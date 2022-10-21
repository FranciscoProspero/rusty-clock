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
    // get clap args
    let args = Arguments::parse();

    // open database, if there is none create it.
    let database = Datab::new();
    database.create_table();

    // open CLI
    if args.gui == false {      
        start_cli(&database);
    }
    // opn GUI
    else {
        start_gui();
    }
}

fn start_gui() {
    let gui = GuiRustyClock::new();
    gui.start();
}

fn start_cli(database : &Datab ) {
    // launch timer manager thread and stdin thread.
    let (rx, handlers) = launch_threads();

    main_loop(rx);

    // waits for the other threads to exit before exiting the main thread
    for handle in handlers {
        handle.join().unwrap();
    }

    // print current values of the database
    database.db_read_all();
}

// returns the receiver so that the main thread can be notifyied by the timer manager thread.
// also returns a vector with the thread handlers so that the main thread can exit only after the other threads are finished
fn launch_threads() -> (std::sync::mpsc::Receiver<u32>, Vec<thread::JoinHandle<()>>){

    // tx for stdin to send the timer type to the rx on the timer manager
    let (tx, rx) = mpsc::channel();

    // tx2 for timer manager to send notifications to the main thread
    let (tx2, rx2) = mpsc::channel();

    let mut handlers = vec![];

    // launch thread that manages the timers and database updates
    let handle = thread::spawn( move || { 
        let mut timer_manager = TimerManager::new(rx, tx2);
        timer_manager.timer_thread();
    });
    handlers.push(handle);

    // launch stdin thread that reads user input and sends them to the timer manager thread
    let handle = thread::spawn( move || { 
        stdin_parser(tx);
    });
    handlers.push(handle);

    (rx2, handlers)
}

fn main_loop (rx: std::sync::mpsc::Receiver<u32>) {
    // waits for the notification from timer manager to run the popup
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
    // Exits when the thread is disconnected from the timer threads
}
