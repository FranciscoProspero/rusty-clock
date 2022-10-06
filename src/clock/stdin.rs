use std::io;
use std::sync::{Arc, Mutex};
use super::timer_structs::{TimerGlobs, TypesOfTimers};

pub fn stdin_parser(mtx:&Arc<Mutex<Vec<TimerGlobs>>>, tx: std::sync::mpsc::Sender<TypesOfTimers>) {
    let input_possibilities = vec!["Study", "study", "Work", "work", "Fun", "fun", "Coffee", "coffee"];
    let input_exit = vec!["Exit", "exit", "Quit", "quit", "End", "end", "Terminate", "terminate", "Q", "q"];

    loop {
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        {
            let num = mtx.lock().unwrap();
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
            break;
        }
    }
    {
        let num = mtx.lock().unwrap();
        println!("Finali {:?}", *num); 
    }
} 