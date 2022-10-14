use super::timer_structs::{TimerGlobs, TypesOfTimers};
use super::notification::{notifier, random_request_notification};
use super::db::Datab;
use rand::Rng;
use std::time::{Duration, Instant};
use std::sync::mpsc::TryRecvError;


pub fn timer_thread(rx: std::sync::mpsc::Receiver<TypesOfTimers>, tx2: std::sync::mpsc::Sender<u32>) -> i32 {
    let mut now = Instant::now();
    let mut state = TypesOfTimers::None;
    let database = Datab::new();
    let mut rng = rand::thread_rng();

    let mut n1: u64 = rng.gen_range(6..20);
    let mut notifier_time = Instant::now();
    let mut random_seconds = Duration::new(n1, 0);

    let mut timer_vec = generate_timervec(&database);

    loop {

        if notifier_time.elapsed() >= random_seconds {
            let _tx_return = tx2.send(1);
            random_request_notification(notifier_time.elapsed());
            n1 = rng.gen_range(6..20);
            random_seconds = Duration::new(n1, 0);
            notifier_time = Instant::now();
        }
        match rx.try_recv() {
            Ok(TypesOfTimers::Quit)  => {
                if state != TypesOfTimers::None { 
                    let running_pos = timer_vec_position(&state);
                    let elapsed_time = now.elapsed();      
                    timer_vec[running_pos].update_total_timer(elapsed_time);
                    database.db_update_val(&(timer_vec[running_pos].total_time.as_millis() as u64), &timer_vec[running_pos].id);
                    notifier(&TypesOfTimers::Quit);
                }
                println!("Quit -> Terminating.");
                database.db_read_all();
                break;
            },
            Ok(type_of_timer) => {
                change_timer(&mut timer_vec , &mut state, &type_of_timer, &mut now, &database);
                notifier(&type_of_timer);
            },
            Err(TryRecvError::Disconnected) => {
                    println!("Error timer thread disconnected.")
            }
            Err(TryRecvError::Empty) => {}
        };
    }
    0
}

fn generate_timervec(database : &Datab) -> Vec<TimerGlobs> {
    let timer_names: [TypesOfTimers; 4] = [TypesOfTimers::Study, TypesOfTimers::Work, TypesOfTimers::Fun, TypesOfTimers::Coffee];
    let mut timervec = Vec::with_capacity(4);

    for (i, name) in timer_names.into_iter().enumerate() {
        timervec.push(TimerGlobs::new(name, i as u32, database.read_total_time(i as i32)));
    }
    timervec
}

fn change_timer(timer_vec: &mut Vec<TimerGlobs>, state : &mut TypesOfTimers, new_state: &TypesOfTimers, time: &mut Instant, database : &Datab) {
    if *state == *new_state {
        println!("Timer was already running!");
        return ;
    }

    let new_position = timer_vec_position(&new_state);
    let old_position = timer_vec_position(&state);

    if *state != TypesOfTimers::None {
        let elapsed_time = time.elapsed(); 
        timer_vec[old_position].update_current_timer(elapsed_time);
        timer_vec[old_position].update_total_timer(elapsed_time);
        
        database.db_update_val(&(timer_vec[old_position].total_time.as_millis() as u64), &timer_vec[old_position].id);

        timer_update_state(time, state, new_state, &mut timer_vec[new_position]);
    }
    else {
        timer_update_state(time, state, new_state, &mut timer_vec[new_position]);
    }
}

fn timer_vec_position(state: &TypesOfTimers) -> usize {
    match state {
        TypesOfTimers::Quit => 5,
        TypesOfTimers::None => 4,
        TypesOfTimers::Study => 0,
        TypesOfTimers::Work => 1,
        TypesOfTimers::Fun => 2,
        TypesOfTimers::Coffee => 3,
    }
}

fn timer_update_state(time: &mut Instant, state : &mut TypesOfTimers, new_state : &TypesOfTimers, running_timer: &mut TimerGlobs) {
    *time = Instant::now();
    *state = *new_state;
    running_timer.increment_start_counter();
}

