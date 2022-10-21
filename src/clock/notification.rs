use notify_rust::{Notification, Timeout};
use std::time::{Duration, Instant};
use rand::Rng;
use rand::rngs::ThreadRng;


use super::timer_structs::TypesOfTimers;

pub struct Notify {
    rng : ThreadRng,
    pub notifier_time : Instant,
    pub random_secs : Duration,
    tx2 : std::sync::mpsc::Sender<u32>,
}

impl Notify {
    pub fn new( tx : std::sync::mpsc::Sender<u32> ) -> Notify {
        let mut rng = rand::thread_rng();
        let n1 = rng.gen_range(6..20);
        Notify {
            rng : rng,
            notifier_time : Instant::now(),
            random_secs : Duration::new(n1, 0),
            tx2 : tx,
        }
    }

    pub fn call_notifier(&mut self) {
        let _tx_return = self.tx2.send(1);
        self.random_request_notification(self.notifier_time.elapsed());
        let n1 = self.rng.gen_range(6..20);
        self.random_secs = Duration::new(n1, 0);
        self.notifier_time = Instant::now();
    }

    fn random_request_notification(&self ,tiempo: Duration) {
        let tbody = format!("The application is very ran dom! It took this tiempo {:?} babai.", tiempo); 
    
        Notification::new()
            .summary("Very random")
            .body(&tbody)
            .timeout(Timeout::Milliseconds(100)) //milliseconds
            .show().unwrap();
        ()
    }

    pub fn notifier(&self, type_of_timer : &TypesOfTimers) -> i32 {

        let timer = type_of_timer.to_string();
        
        Notification::new()
            .summary(&timer)
            .body("The application is quiting! babai.")
            .timeout(Timeout::Milliseconds(1000)) //milliseconds
            .show().unwrap();
        1
    }
    
}


