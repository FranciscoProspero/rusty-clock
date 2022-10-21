use super::timer_structs::{TimerGlobs, TypesOfTimers};
use super::notification::Notify;
use super::db::Datab;

use std::time::{Duration, Instant};
use std::sync::mpsc::TryRecvError;

pub struct TimerManager {
    now: Instant,
    state: TypesOfTimers,
    database: Datab,
    timer_vec: Vec<TimerGlobs>,
    stopped_time: Duration,
    receive_message: std::sync::mpsc::Receiver<TypesOfTimers>,
    notify: Notify,
}

impl TimerManager {
    pub fn new(rx: std::sync::mpsc::Receiver<TypesOfTimers>, tx2: std::sync::mpsc::Sender<u32>) -> TimerManager {
        let database = Datab::new();
        let timer_vec = generate_timervec(&database);
        TimerManager {
            now: Instant::now(),
            state: TypesOfTimers::None,
            database: database,
            timer_vec: timer_vec,
            stopped_time: Duration::new(0, 0),
            receive_message: rx,
            notify: Notify::new(tx2),
        }
    }
    
    pub fn timer_thread(&mut self) -> i32 {
        loop {
            //When the random amount of time passed notify user to update what timer should be running
            if self.notify.notifier_time.elapsed() >= self.notify.random_secs {
                self.notify.call_notifier();
            }
            // When a message is received from the gui or cli run the corresponding task
            match self.receive_message.try_recv() {
                Ok(TypesOfTimers::Stats) => {
                    self.show_stats(&self.database);
                },
                Ok(TypesOfTimers::Quit)  => {
                    // If some timer is running, get current timer and save it to db
                    if self.state != TypesOfTimers::None {
                        let running_pos = timer_vec_position(&self.state); 
                        self.update_timer_on_db(running_pos);
                    }
                    println!("Quit -> Terminating.");
                    self.database.db_read_all();
                    break;
                },
                Ok(TypesOfTimers::Stop) => {
                    // If some timer is running, stop it and store current elapsed time
                    if self.state != TypesOfTimers::None { 
                        let running_pos = timer_vec_position(&self.state);
                        self.timer_vec[running_pos].update_running_paused(true, true);
                        self.stopped_time += self.now.elapsed();
                    }
                },
                Ok(type_of_timer) => {
                    if self.state != TypesOfTimers::None { 
                        let running_pos = timer_vec_position(&self.state);

                        if self.timer_vec[running_pos]._is_running() && self.timer_vec[running_pos]._is_paused() {
                            self.timer_vec[running_pos].update_running_paused(true, false);
                            self.now = Instant::now();
                        }
                    }
                    self.change_timer(&type_of_timer );
                    self.notify.notifier(&type_of_timer);
                },
                Err(TryRecvError::Disconnected) => {
                        println!("Error timer thread disconnected.")
                }
                Err(TryRecvError::Empty) => {}
            };
        }
        0
    }

    fn change_timer(&mut self, new_state: &TypesOfTimers ) {
        if self.state == *new_state {
            println!("Timer was already running!");
            return ;
        }

        let new_position = timer_vec_position(&new_state);
        let old_position = timer_vec_position(&self.state);

        if self.state != TypesOfTimers::None {
            let mut elapsed_time;
            if self.timer_vec[old_position]._is_paused() {
                elapsed_time = self.stopped_time;
            }
            else if self.stopped_time != Duration::new(0, 0) {
                elapsed_time = self.now.elapsed();
                elapsed_time += self.stopped_time;
            }
            else {
                elapsed_time = self.now.elapsed();
            }

            self.timer_vec[old_position].update_current_timer(elapsed_time);
            self.timer_vec[old_position].update_total_timer(elapsed_time);
            let total_time = &(self.timer_vec[old_position].total_time.as_secs() as u64);
            self.database.db_update_val(&self.timer_vec[old_position]._timer_type.to_string(), &total_time);
            self.timer_vec[old_position].update_running_paused(false, false);
            self.timer_update_state(&new_state, new_position, true, false);
            self.stopped_time = Duration::new(0, 0);
        }
        else {
            self.timer_update_state(&new_state, new_position, true, false);
        }
    }

    fn timer_update_state(&mut self, new_state : &TypesOfTimers, running_timer: usize, running: bool, paused: bool) {
        self.now = Instant::now();
        self.state = *new_state;
        self.timer_vec[running_timer].increment_start_counter();
        self.timer_vec[running_timer].update_running_paused(running, paused);
    }

    fn show_stats(&self, database : &Datab) {
        let stats_per_month = database.read_average_per_month_with_month();
        let stats_per_day = database.read_average();
        for stat in stats_per_month {
            println!("year: {} month: {} study: {}s work: {}s fun: {}s coffee: {}s.", stat.5, stat.4, stat.0, stat.1, stat.2, stat.3);
        }
        println!("Average per day: study: {}s work: {}s fun: {}s coffee: {}s.", stats_per_day.0, stats_per_day.1, stats_per_day.2, stats_per_day.3);
    }

    fn update_timer_on_db(&mut self, running_timer: usize) {
        let mut elapsed_time;
        if self.timer_vec[running_timer]._is_paused() {
            elapsed_time = self.stopped_time;
        }
        else if self.stopped_time != Duration::new(0, 0) {
            elapsed_time = self.now.elapsed();
            elapsed_time += self.stopped_time;
        }
        else {
            elapsed_time = self.now.elapsed();
        }
        self.timer_vec[running_timer].update_total_timer(elapsed_time);
        self.database.db_update_val(&self.timer_vec[running_timer]._timer_type.to_string(), &(self.timer_vec[running_timer].total_time.as_secs() as u64));
        self.notify.notifier(&TypesOfTimers::Quit);
    }
}


fn generate_timervec(database : &Datab) -> Vec<TimerGlobs> {
    let timer_names: [TypesOfTimers; 4] = [TypesOfTimers::Study, TypesOfTimers::Work, TypesOfTimers::Fun, TypesOfTimers::Coffee];
    let mut timervec = Vec::with_capacity(4);

    for (i, name) in timer_names.into_iter().enumerate() {
        timervec.push(TimerGlobs::new(name, i as u32, database.read_total_time(name)));
    }
    timervec
}

fn timer_vec_position(state: &TypesOfTimers) -> usize {
    match *state {
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
