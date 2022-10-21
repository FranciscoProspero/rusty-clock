mod clock;
use clock::timer_manager::TimerManager;
use clock::stdin::stdin_parser;
use clock::db::Datab;
use clock::gui::GuiRustyClock;

use std::sync::mpsc::TryRecvError;
use clap::Parser;
use std::thread;
use std::sync::mpsc;

#[derive(Parser,Default,Debug)]
#[clap(author="Francisco Prospero", version, about="Rusty clock")]
struct Arguments {
    /// Start in GUI or cli
    #[arg(short, long)]
    gui: bool,
}


fn main() {
    let args = Arguments::parse();
    let database = Datab::new();
    database.create_table();

    if args.gui == false {      
        start_cli(&database);
    }
    else {
        start_gui();
    }
}

fn start_gui() {
    let gui = GuiRustyClock::new();
    gui.start();
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
        let mut timer_manager = TimerManager::new(rx, tx2);
        timer_manager.timer_thread();
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
