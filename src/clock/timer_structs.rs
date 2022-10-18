use std::time::Duration;
use std::fmt;
#[derive(Debug)]

#[derive(PartialEq,Copy, Clone)]
pub enum TypesOfTimers {
    Study,
    Work,
    Fun,
    Coffee,
    Quit,
    Stats,
    None
}

impl fmt::Display for TypesOfTimers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypesOfTimers::Study => write!(f, "Study"),
            TypesOfTimers::Work => write!(f, "Work"),
            TypesOfTimers::Fun => write!(f, "Fun"),
            TypesOfTimers::Coffee => write!(f, "Coffee"),
            TypesOfTimers::Quit => write!(f, "Quit"),
            TypesOfTimers::Stats => write!(f, "Stats"),
            TypesOfTimers::None => write!(f, "None"),
        }
    }
}

#[derive(Debug)]
pub struct TimerGlobs {
    pub _timer_type: TypesOfTimers,
    pub id: u32,
    pub total_time: Duration,
    current_time: Duration,
    _alert_timer: i32,
    nr_of_start: i32
}

impl TimerGlobs {
    pub fn new(type_of_timer: TypesOfTimers, idx : u32, total_time : u64) -> TimerGlobs {
        TimerGlobs {_timer_type: type_of_timer, id: idx, total_time: Duration::from_secs(total_time), current_time: Duration::new(total_time,0), _alert_timer: 0, nr_of_start: 0}
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
