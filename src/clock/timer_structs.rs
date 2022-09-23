use std::time::Duration;

#[derive(Debug)]
pub enum TypesOfTimers {
    Study,
    Work,
    Fun,
    Coffee,
    Quit
}

#[derive(Debug)]
pub struct TimerGlobs {
    _timer_type: TypesOfTimers,
    _id: usize,
    total_time: Duration,
    current_time: Duration,
    _alert_timer: i32,
    nr_of_start: i32
}

impl TimerGlobs {
    pub fn new(type_of_timer: TypesOfTimers, idx : usize) -> TimerGlobs {
        TimerGlobs {_timer_type: type_of_timer, _id: idx, total_time: Duration::new(0,0), current_time: Duration::new(0,0), _alert_timer: 0, nr_of_start: 0}
    }

    pub fn update_current_timer(&mut self, elapsed_time : Duration) -> (){
        self.current_time = elapsed_time;
    }

    pub fn update_total_timer(&mut self, elapsed_time : Duration) -> (){
        self.total_time = self.total_time + elapsed_time;
    }

    pub fn increment_start_counter(&mut self) -> (){
        self.nr_of_start += 1;
    }
}
