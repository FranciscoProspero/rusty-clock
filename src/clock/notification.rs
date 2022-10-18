use notify_rust::{Notification, Timeout};
use std::time::Duration;
use super::timer_structs::TypesOfTimers;


pub fn notifier( type_of_timer : &TypesOfTimers) -> i32 {

    let timer = type_of_timer.to_string();
    
    Notification::new()
        .summary(&timer)
        .body("The application is quiting! babai.")
        .timeout(Timeout::Milliseconds(1000)) //milliseconds
        .show().unwrap();
    1
}

pub fn random_request_notification(tiempo: Duration) {
    let tbody = format!("The application is very ran dom! It took this tiempo {:?} babai.", tiempo);
 
    

    Notification::new()
        .summary("Very random")
        .body(&tbody)
        .timeout(Timeout::Milliseconds(100)) //milliseconds
        .show().unwrap();
    ()
}