
mod clock;
use clock::timer_manager::timer_thread;
use clock::stdin::stdin_parser;
use clock::timer_structs::{TimerGlobs, TypesOfTimers};
use clock::db::{Datab, Timer};
//use clock::popup::Popup;

use std::sync::mpsc::TryRecvError;
// use iced::{
//     button, Alignment, Button, Column, Element, Sandbox, Settings, Text, window,
// };

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

fn main() {
    let database = Datab::new();
    database.createtable();
    start_cli(&database);
}

fn start_cli(database : &Datab ) {
    database.dbruntest();
    let (rx, handlers) = launch_threads();

    main_loop(rx);

    for handle in handlers {
        handle.join().unwrap();
    }
}

fn launch_threads() -> (std::sync::mpsc::Receiver<u32>, Vec<thread::JoinHandle<()>>){
    let test_vec = generate_timervec();

    let timer_vec_mtx = Arc::new(Mutex::new(test_vec));
    let timer_vec = Arc::clone(&timer_vec_mtx);
    let (tx, rx) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let mut handlers = vec![];
    let handle = thread::spawn( move || { 
        timer_thread(&timer_vec, rx, tx2);
    });
    handlers.push(handle);

    let timer_vec2 = Arc::clone(&timer_vec_mtx);
    let handle = thread::spawn( move || { 
        stdin_parser(&timer_vec2, tx);
    });
    handlers.push(handle);
    (rx2, handlers)
}

fn generate_timervec() -> Vec<TimerGlobs> {
    let timer_names: [TypesOfTimers; 4] = [TypesOfTimers::Study, TypesOfTimers::Work, TypesOfTimers::Fun, TypesOfTimers::Coffee];
    let mut timervec = Vec::with_capacity(4);

    for (i, name) in timer_names.into_iter().enumerate() {
        timervec.push(TimerGlobs::new(name,i));
    }
    timervec
}

fn main_loop (rx: std::sync::mpsc::Receiver<u32>) {
    loop {
        match rx.try_recv() {
            Ok(1) => {
                //Popup::run_popus();
                println!("exit the popus")
            },
            Ok(_) => {
                println!("Whateba")
            }
            Err(TryRecvError::Disconnected) => {
                    println!("Error Disconetiooni.");
                    break;
            }
            Err(TryRecvError::Empty) => {}
        };
    }
}