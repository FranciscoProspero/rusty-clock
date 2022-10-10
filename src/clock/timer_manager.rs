use super::timer_structs::{TimerGlobs, TypesOfTimers};
use super::notification::{notifier, random_request_notification};
use super::db::{Datab, Timer};
use rand::Rng;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::TryRecvError;

pub fn timer_thread(mtx:&Arc<Mutex<Vec<TimerGlobs>>>, rx: std::sync::mpsc::Receiver<TypesOfTimers>, tx2: std::sync::mpsc::Sender<u32>) -> i32 {
    let mut now = Instant::now();
    let mut running_pos : usize = 50;
    let database = Datab::new();
    let mut rng = rand::thread_rng();

    let mut n1: u64 = rng.gen_range(6..20);
    let mut notifier_time = Instant::now();
    let mut random_seconds = Duration::new(n1, 0);

    loop {

        if notifier_time.elapsed() >= random_seconds {
            let _tx_return = tx2.send(1);
            random_request_notification(notifier_time.elapsed());
            n1 = rng.gen_range(6..20);
            random_seconds = Duration::new(n1, 0);
            notifier_time = Instant::now();
        }
        match rx.try_recv() {
            Ok(TypesOfTimers::Study) => {
                change_timer(&mtx , &mut running_pos, 0, &mut now, &database);
                notifier(TypesOfTimers::Study);
            },
            Ok(TypesOfTimers::Work) => {
                change_timer(&mtx , &mut running_pos, 1, &mut now, &database);
                notifier(TypesOfTimers::Work);
            },
            Ok(TypesOfTimers::Fun) => {
                change_timer(&mtx , &mut running_pos, 2, &mut now, &database);
                notifier(TypesOfTimers::Fun);
            },
            Ok(TypesOfTimers::Coffee) => {
                change_timer(&mtx , &mut running_pos, 3, &mut now, &database);
                notifier(TypesOfTimers::Coffee);
            },
            Ok(TypesOfTimers::Quit)  => {
                let elapsed_time = now.elapsed();      
                let mut num = mtx.lock().unwrap();
                if running_pos < 5 {
                    num[running_pos].update_total_timer(elapsed_time);
                }
                let totaltime = num[running_pos].total_time.as_millis() as u64;
                database.db_update_val(&totaltime, &num[running_pos].id);
                notifier(TypesOfTimers::Quit);
                println!("Quit -> Terminating.");
                break;
            }
            Err(TryRecvError::Disconnected) => {
                    println!("Error timer thread disconnected.")
            }
            Err(TryRecvError::Empty) => {}
        };
    }
    0
}


fn change_timer(mtx:&Arc<Mutex<Vec<TimerGlobs>>>, position : &mut usize, new_position: usize, time: &mut Instant, database : &Datab) {
    if *position == new_position {
        println!("Timer was already running!");
    }
    else if *position < 5 {
        let elapsed_time = time.elapsed();      
        let mut num = mtx.lock().unwrap();
        num[*position].update_current_timer(elapsed_time);
        num[*position].update_total_timer(elapsed_time);
        
        let totaltime = *&num[*position].total_time.as_millis() as u64;
        database.db_update_val(&totaltime, &(*&num[*position].id as u32));

        *time = Instant::now();
        *position = new_position;
        num[*position].increment_start_counter();
    }
    else {
        *time = Instant::now();
        *position = new_position;
        let mut num = mtx.lock().unwrap();
        num[*position].increment_start_counter();
    }
}
