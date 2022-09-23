use super::timer_structs::{TimerGlobs, TypesOfTimers};
use super::notification::{notifier, random_request_notification};
use rand::Rng;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::TryRecvError;

pub fn timer_thread(mtx:&Arc<Mutex<Vec<TimerGlobs>>>, rx: std::sync::mpsc::Receiver<TypesOfTimers>) -> i32 {
    println!("yes my name is burrito");
    let mut now = Instant::now();
    let mut running_pos : usize = 50;

    let mut rng = rand::thread_rng();

    let mut n1: u64 = rng.gen_range(6..20);
    let mut notifier_time = Instant::now();
    let mut random_seconds = Duration::new(n1, 0);
    loop {

        if notifier_time.elapsed() >= random_seconds {
            random_request_notification(notifier_time.elapsed());
            n1 = rng.gen_range(60..3600);
            random_seconds = Duration::new(n1, 0);
            notifier_time = Instant::now();
        }
        match rx.try_recv() {
            Ok(TypesOfTimers::Study) => {
                change_timer(&mtx , &mut running_pos, 0, &mut now);
                notifier(TypesOfTimers::Study);
                println!("Study")
            },
            Ok(TypesOfTimers::Work) => {
                change_timer(&mtx , &mut running_pos, 1, &mut now);
                notifier(TypesOfTimers::Work);
                println!("Work")
            },
            Ok(TypesOfTimers::Fun) => {
                change_timer(&mtx , &mut running_pos, 2, &mut now);
                notifier(TypesOfTimers::Fun);
                println!("Fun")
            },
            Ok(TypesOfTimers::Coffee) => {
                change_timer(&mtx , &mut running_pos, 3, &mut now);
                notifier(TypesOfTimers::Coffee);
            },
            Ok(TypesOfTimers::Quit)  => {
                let elapsed_time = now.elapsed();      
                let mut num = mtx.lock().unwrap();
                if running_pos < 5 {
                    num[running_pos].update_total_timer(elapsed_time);
                }
                notifier(TypesOfTimers::Quit);
                println!("Quit -> Terminating.");
                break;
            }
            Err(TryRecvError::Disconnected) => {
                    println!("Error Disconetiooni.")
            }
            Err(TryRecvError::Empty) => {}
        };
    }
    println!("It is finito con este treda");
    0
}


fn change_timer(mtx:&Arc<Mutex<Vec<TimerGlobs>>>, position : &mut usize, new_position: usize, time: &mut Instant) {
    if *position == new_position {
        println!("Timer was already running!");
    }
    else if *position < 5 {
        let elapsed_time = time.elapsed();      
        let mut num = mtx.lock().unwrap();
        num[*position].update_current_timer(elapsed_time);
        num[*position].update_total_timer(elapsed_time);
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
