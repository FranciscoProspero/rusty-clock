
mod clock;
use clock::timer_manager::timer_thread;
use clock::stdin::stdin_parser;
use clock::timer_structs::{TimerGlobs, TypesOfTimers};
use clock::popup::Popup;

use std::sync::mpsc::TryRecvError;
use iced::{
    button, Alignment, Button, Column, Element, Sandbox, Settings, Text, window,
};


use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

fn main() {
    start_cli();
}

fn start_cli() {

    let timer_names: [TypesOfTimers; 4] = [TypesOfTimers::Study, TypesOfTimers::Work, TypesOfTimers::Fun, TypesOfTimers::Coffee];
    
    let mut test_vec = Vec::with_capacity(4);

    for (i, name) in timer_names.into_iter().enumerate() {
        test_vec.push(TimerGlobs::new(name,i));
    }

    let timer_vec_mtx = Arc::new(Mutex::new(test_vec));
    println!("You have started the Study timer");
    
    let (tx, rx) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let timer_vec = Arc::clone(&timer_vec_mtx);
    let handle = thread::spawn( move || { 
        timer_thread(&timer_vec, rx, tx2)
    });
    let timer_vec2 = Arc::clone(&timer_vec_mtx);
    let handle1 = thread::spawn( move || { 
        stdin_parser(&timer_vec2, tx)
    });
    println!("listening to treda");
    loop {
        match rx2.try_recv() {
            Ok(1) => {
                Popup::run_popus();
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
    handle.join().unwrap();
    handle1.join().unwrap();
}


