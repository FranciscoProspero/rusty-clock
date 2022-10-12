mod clock;
use clock::timer_manager::timer_thread;
use clock::stdin::stdin_parser;
use clock::db::Datab;
use clock::gui::Counter;

use std::sync::mpsc::TryRecvError;
use iced::{
    button, Alignment, Button, Column, Element, Sandbox, Settings, Text, window,
};
use clap::Parser;

#[derive(Parser,Default,Debug)]
#[clap(author="Francisco Prospero", version, about="Rusty clock")]
struct Arguments {
    /// Start in GUI or cli
    #[arg(short, long)]
    gui: bool,

}

use std::thread;
use std::sync::mpsc;

fn main() {
    let args = Arguments::parse();
    let database = Datab::new();
    database.create_table();

    if args.gui == false {      
        start_cli(&database);
    }
    else {
        start_gui(&database);
    }
}

fn start_gui(database :  &Datab) {
    let mut testis = window::Settings::default();
    testis.always_on_top = true;
    testis.size = (150, 450);
    testis.position = window::Position::Specific(0,0);

    println!("Start GUI");
    Counter::run(Settings {
        window: testis,
        ..Settings::default()
    });
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
