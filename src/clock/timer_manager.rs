use super::timer_structs::{TimerGlobs, TypesOfTimers};
use super::notification::Notify;
use super::db::Datab;

use std::time::{Duration, Instant};
use std::sync::mpsc::TryRecvError;


pub fn timer_thread(rx: std::sync::mpsc::Receiver<TypesOfTimers>, tx2: std::sync::mpsc::Sender<u32>) -> i32 {
    let mut now = Instant::now();
    let mut state = TypesOfTimers::None;
    let database = Datab::new();

    let mut timer_vec = generate_timervec(&database);
    let mut stopped_time = Duration::new(0, 0);

    let mut notify = Notify::new(tx2);

    loop {

        if notify.notifier_time.elapsed() >= notify.random_secs {
            notify.call_notifier();
        }
        match rx.try_recv() {
            Ok(TypesOfTimers::Stats) => {
                let stats_per_month = database.read_average_per_month_with_month();
                let stats_per_day = database.read_average();
                for stat in stats_per_month {
                    println!("year: {} month: {} study: {}s work: {}s fun: {}s coffee: {}s.", stat.5, stat.4, stat.0, stat.1, stat.2, stat.3);
                }
                println!("Average per day: study: {}s work: {}s fun: {}s coffee: {}s.", stats_per_day.0, stats_per_day.1, stats_per_day.2, stats_per_day.3);
            },
            Ok(TypesOfTimers::Quit)  => {
                if state != TypesOfTimers::None { 
                    let running_pos = timer_vec_position(&state);
                    let mut elapsed_time;
                    if timer_vec[running_pos]._is_paused() {
                        elapsed_time = stopped_time;
                    }
                    else if stopped_time != Duration::new(0, 0) {
                        elapsed_time = now.elapsed();
                        elapsed_time += stopped_time;
                    }
                    else {
                        elapsed_time = now.elapsed();
                    }
                    timer_vec[running_pos].update_total_timer(elapsed_time);
                    database.db_update_val(&timer_vec[running_pos]._timer_type.to_string(), &(timer_vec[running_pos].total_time.as_secs() as u64));
                    notify.notifier(&TypesOfTimers::Quit);
                }
                println!("Quit -> Terminating.");
                database.db_read_all();
                break;
            },
            Ok(TypesOfTimers::Stop) => {
                if state != TypesOfTimers::None { 
                    let running_pos = timer_vec_position(&state);
                    timer_vec[running_pos].update_running_paused(true, true);
                    stopped_time += now.elapsed();
                }
            },
            Ok(type_of_timer) => {
                if state != TypesOfTimers::None { 
                    let running_pos = timer_vec_position(&state);

                    if timer_vec[running_pos]._is_running() && timer_vec[running_pos]._is_paused() {
                        timer_vec[running_pos].update_running_paused(true, false);
                        now = Instant::now();
                    }
                }
                change_timer(&mut timer_vec , &mut state, &type_of_timer, &mut now, &database, &mut stopped_time);
                notify.notifier(&type_of_timer);
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
        timervec.push(TimerGlobs::new(name, i as u32, database.read_total_time(name)));
    }
    timervec
}

fn change_timer(timer_vec: &mut Vec<TimerGlobs>, state : &mut TypesOfTimers, new_state: &TypesOfTimers, time: &mut Instant, database : &Datab, stopped_time: &mut Duration) {
    if *state == *new_state {
        println!("Timer was already running!");
        return ;
    }

    let new_position = timer_vec_position(&new_state);
    let old_position = timer_vec_position(&state);

    if *state != TypesOfTimers::None {
        let mut elapsed_time;
        if timer_vec[old_position]._is_paused() {
            elapsed_time = *stopped_time;
        }
        else if *stopped_time != Duration::new(0, 0) {
            elapsed_time = time.elapsed();
            elapsed_time += *stopped_time;
        }
        else {
            elapsed_time = time.elapsed();
        }

        timer_vec[old_position].update_current_timer(elapsed_time);
        timer_vec[old_position].update_total_timer(elapsed_time);
        let total_time = &(timer_vec[old_position].total_time.as_secs() as u64);
        database.db_update_val(&timer_vec[old_position]._timer_type.to_string(), &total_time);
        timer_vec[old_position].update_running_paused(false, false);
        timer_update_state(time, state, new_state, &mut timer_vec[new_position], true, false);
        *stopped_time = Duration::new(0, 0);
    }
    else {
        timer_update_state(time, state, new_state, &mut timer_vec[new_position], true, false);
    }
}

fn timer_vec_position(state: &TypesOfTimers) -> usize {
    match state {
        TypesOfTimers::Stop => 7,
        TypesOfTimers::Stats => 6,
        TypesOfTimers::Quit => 5,
        TypesOfTimers::None => 4,
        TypesOfTimers::Study => 0,
        TypesOfTimers::Work => 1,
        TypesOfTimers::Fun => 2,
        TypesOfTimers::Coffee => 3,
    }
}

fn timer_update_state(time: &mut Instant, state : &mut TypesOfTimers, new_state : &TypesOfTimers, running_timer: &mut TimerGlobs,running: bool, paused: bool) {
    *time = Instant::now();
    *state = *new_state;
    running_timer.increment_start_counter();
    running_timer.update_running_paused(running, paused);
}

