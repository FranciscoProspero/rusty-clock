
mod clock;
use clock::timer_manager::timer_thread;
use clock::stdin::stdin_parser;
use clock::db::Datab;
//use clock::popup::Popup;

use std::sync::mpsc::TryRecvError;
// use iced::{
//     button, Alignment, Button, Column, Element, Sandbox, Settings, Text, window,
// };

use std::thread;
use std::sync::mpsc;

fn main() {
    let database = Datab::new();
    database.create_table();
    start_cli(&database);
}

fn start_cli(database : &Datab ) {

    let (rx, handlers) = launch_threads();

    main_loop(rx);

    database.db_read_all();
    
    for handle in handlers {
        handle.join().unwrap();
    }
}

fn launch_threads() -> (std::sync::mpsc::Receiver<u32>, Vec<thread::JoinHandle<()>>){

    let (tx, rx) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let mut handlers = vec![];
    let handle = thread::spawn( move || { 
        timer_thread(rx, tx2);
    });
    handlers.push(handle);

    let handle = thread::spawn( move || { 
        stdin_parser(tx);
    });
    handlers.push(handle);
    (rx2, handlers)
}

fn main_loop (rx: std::sync::mpsc::Receiver<u32>) {
    loop {
        match rx.try_recv() {
            Ok(1) => {
                //Popup::run_popus();
            },
            Ok(_) => {
                println!("Whateba")
            }
            Err(TryRecvError::Disconnected) => {
                    println!("Disconnected from timer thread.");
                    break;
            }
            Err(TryRecvError::Empty) => {}
        };
    }
}
