use super::timer_structs::{TimerGlobs, TypesOfTimers};
use super::notification::{notifier, random_request_notification};
use super::db::Datab;
use rand::Rng;
use std::time::{Duration, Instant};
use std::sync::mpsc::TryRecvError;

#[derive(PartialEq)]
enum TimerState {
    None,
    Study,
    Work,
    Fun,
    Coffee,
}

pub fn timer_thread(rx: std::sync::mpsc::Receiver<TypesOfTimers>, tx2: std::sync::mpsc::Sender<u32>) -> i32 {
    let mut now = Instant::now();
    let mut state = TimerState::None;
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
            Ok(TypesOfTimers::Study) => {
                change_timer(&mut timer_vec , &mut state, TimerState::Study, &mut now, &database);
                notifier(TypesOfTimers::Study);
            },
            Ok(TypesOfTimers::Work) => {
                change_timer(&mut timer_vec , &mut state, TimerState::Work, &mut now, &database);
                notifier(TypesOfTimers::Work);
            },
            Ok(TypesOfTimers::Fun) => {
                change_timer(&mut timer_vec , &mut state, TimerState::Fun, &mut now, &database);
                notifier(TypesOfTimers::Fun);
            },
            Ok(TypesOfTimers::Coffee) => {
                change_timer(&mut timer_vec , &mut state, TimerState::Coffee, &mut now, &database);
                notifier(TypesOfTimers::Coffee);
            },
            Ok(TypesOfTimers::Quit)  => {
                if state != TimerState::None { 
                    let running_pos = timer_vec_position(&state);
                    let elapsed_time = now.elapsed();      
                    timer_vec[running_pos].update_total_timer(elapsed_time);
                    database.db_update_val(&(timer_vec[running_pos].total_time.as_millis() as u64), &timer_vec[running_pos].id);
                    notifier(TypesOfTimers::Quit);
                }
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

fn generate_timervec(database : &Datab) -> Vec<TimerGlobs> {
    let timer_names: [TypesOfTimers; 4] = [TypesOfTimers::Study, TypesOfTimers::Work, TypesOfTimers::Fun, TypesOfTimers::Coffee];
    let mut timervec = Vec::with_capacity(4);

    for (i, name) in timer_names.into_iter().enumerate() {
        let total_time = database.read_total_time(i as i32);
        timervec.push(TimerGlobs::new(name, i as u32, total_time));
    }
    timervec
}

fn change_timer(timer_vec: &mut Vec<TimerGlobs>, state : &mut TimerState, new_state: TimerState, time: &mut Instant, database : &Datab) {
    if *state == new_state {
        println!("Timer was already running!");
        return ;
    }

    let position = timer_vec_position(&new_state);

    if *state != TimerState::None {
        let elapsed_time = time.elapsed(); 
        timer_vec[position].update_current_timer(elapsed_time);
        timer_vec[position].update_total_timer(elapsed_time);
        
        database.db_update_val(&(timer_vec[position].total_time.as_millis() as u64), &timer_vec[position].id);

        timer_update_state(time, state, new_state, &mut timer_vec[position]);
    }
    else {
        timer_update_state(time, state, new_state, &mut timer_vec[position]);
    }
}

fn timer_vec_position(state: &TimerState) -> usize {
    match state {
        TimerState::None => 4,
        TimerState::Study => 0,
        TimerState::Work => 1,
        TimerState::Fun => 2,
        TimerState::Coffee => 3,
    }
}

fn timer_update_state(time: &mut Instant, state : &mut TimerState, new_state : TimerState, running_timer: &mut TimerGlobs) {
    *time = Instant::now();
    *state = new_state;
    running_timer.increment_start_counter();
}

