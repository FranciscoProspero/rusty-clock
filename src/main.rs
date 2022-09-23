
mod clock;
use clock::timer_manager::timer_thread;
use clock::timer_structs::{TimerGlobs, TypesOfTimers};

use std::thread;
use std::sync::{Arc, Mutex};
use std::io;
use std::sync::mpsc;

fn main() {
    start_cli();
}

fn start_cli() {

    let timer_names: [TypesOfTimers; 4] = [TypesOfTimers::Study, TypesOfTimers::Work, TypesOfTimers::Fun, TypesOfTimers::Coffee];
    let input_possibilities = vec!["Study", "study", "Work", "work", "Fun", "fun", "Coffee", "coffee"];
    let input_exit = vec!["Exit", "exit", "Quit", "quit", "End", "end", "Terminate", "terminate", "Q", "q"];
    
    let mut test_vec = Vec::with_capacity(4);

    for (i, name) in timer_names.into_iter().enumerate() {
        test_vec.push(TimerGlobs::new(name,i));
    }

    let timer_vec_mtx = Arc::new(Mutex::new(test_vec));
    println!("You have started the Study timer");
    
    let (tx, rx) = mpsc::channel();
    let timer_vec = Arc::clone(&timer_vec_mtx);
    let handle = thread::spawn(move || { 
        timer_thread(&timer_vec, rx)
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        {
            let num = timer_vec_mtx.lock().unwrap();
            println!("{:?}", *num); 
        }
        if input_possibilities.contains( &input.trim() ) {
            let res = match input.trim() {
                "Study" | "study" => tx.send(TypesOfTimers::Study),
                "Work" | "work" => tx.send(TypesOfTimers::Work),
                "Fun" | "fun" => tx.send(TypesOfTimers::Fun),
                "Coffee" | "coffee" => tx.send(TypesOfTimers::Coffee),
                &_ => todo!(),
            };

            match res {
                Ok(_) => (),
                Err(error) => panic!("Error message: {}", error),
            }
        } 
        else if input_exit.contains(&input.trim())
        {
            let res = tx.send(TypesOfTimers::Quit);
            res.unwrap();
            handle.join().unwrap();
            break;
        }
    }
    {
        let num = timer_vec_mtx.lock().unwrap();
        println!("Finali {:?}", *num); 
        }
}


