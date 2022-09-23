use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io;
use std::sync::mpsc::{self, TryRecvError};
use notify_rust::{Notification, Timeout};
use rand::Rng;
// Objetivos:
// x  Ter um cli
// x  Receber um comando para dar start de timer especifico
// x  Tipos de timers -> Study, Work, Fun, Coffee
// x  Apenas um timer pode estar ativo
// x  Dá para mudar de timer sem ter de terminar o outro primeiro
//   Gravar valores por dia, semana, mes, ano e fazer estatisticas
//   Avisar a cada x minutos que timer está a decorrer, pode ser especificado ao correr o comando
//   Notificar de y em y a perguntar o que se está a fazer
//   Ter num nice gui :) probably tui-rs

// Ordem Tarefas:
//  kinda check  1 - cli 
//  kinda check  2 - types of timers
//  kinda check  3 - start timer commands
//  kinda check  4 - stop timer commands
//   5 - save data
//   6 - notifiyer
//   7 - gui


#[derive(Debug)]
enum TypesOfTimers {
    Study,
    Work,
    Fun,
    Coffee,
    Quit
}

#[derive(Debug)]
struct TimerGlobs {
    _timer_type: TypesOfTimers,
    _id: usize,
    total_time: Duration,
    current_time: Duration,
    _alert_timer: i32,
    nr_of_start: i32
}

impl TimerGlobs {
    fn new(type_of_timer: TypesOfTimers, idx : usize) -> TimerGlobs {
        TimerGlobs {_timer_type: type_of_timer, _id: idx, total_time: Duration::new(0,0), current_time: Duration::new(0,0), _alert_timer: 0, nr_of_start: 0}
    }

    fn update_current_timer(&mut self, elapsed_time : Duration) -> (){
        self.current_time = elapsed_time;
    }

    fn update_total_timer(&mut self, elapsed_time : Duration) -> (){
        self.total_time = self.total_time + elapsed_time;
    }

    fn increment_start_counter(&mut self) -> (){
        self.nr_of_start += 1;
    }
}


fn main() {
    start_cli();
}

fn start_cli() {

    let timer_names: [TypesOfTimers; 4] = [TypesOfTimers::Study, TypesOfTimers::Work, TypesOfTimers::Fun, TypesOfTimers::Coffee];
    let input_possibilities = vec!["Study", "study", "Work", "work", "Fun", "fun", "Coffee", "coffee"];
    let input_exit = vec!["Exit", "exit", "Quit", "quit", "End", "end", "Terminate", "terminate", "Q", "q"];
    
    let mut test_vec = Vec::with_capacity(4);

    for (i, name) in timer_names.into_iter().enumerate() {
        test_vec.push(TimerGlobs::new(name,i));
    }

    let timer_vec_mtx = Arc::new(Mutex::new(test_vec));
    println!("You have started the Study timer");
    
    let (tx, rx) = mpsc::channel();
    let timer_vec = Arc::clone(&timer_vec_mtx);
    let handle = thread::spawn(move || { 
        timer_thread(&timer_vec, rx)
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        {
            let num = timer_vec_mtx.lock().unwrap();
            println!("{:?}", *num); 
        }
        if input_possibilities.contains( &input.trim() ) {
            let res = match input.trim() {
                "Study" | "study" => tx.send(TypesOfTimers::Study),
                "Work" | "work" => tx.send(TypesOfTimers::Work),
                "Fun" | "fun" => tx.send(TypesOfTimers::Fun),
                "Coffee" | "coffee" => tx.send(TypesOfTimers::Coffee),
                &_ => todo!(),
            };

            match res {
                Ok(_) => (),
                Err(error) => panic!("Error message: {}", error),
            }
        } 
        else if input_exit.contains(&input.trim())
        {
            let res = tx.send(TypesOfTimers::Quit);
            res.unwrap();
            handle.join().unwrap();
            break;
        }
    }
    {
        let num = timer_vec_mtx.lock().unwrap();
        println!("Finali {:?}", *num); 
        }
}

fn timer_thread(mtx:&Arc<Mutex<Vec<TimerGlobs>>>, rx: std::sync::mpsc::Receiver<TypesOfTimers>) -> i32 {
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

fn notifier( type_of_timer : TypesOfTimers) -> i32 {

    let timer = match type_of_timer {
        TypesOfTimers::Study => "Study",
        TypesOfTimers::Work => "Work",
        TypesOfTimers::Fun => "Fun",
        TypesOfTimers::Coffee => "Coffee",
        TypesOfTimers::Quit => "Quit"
    };
    
    Notification::new()
        .summary(timer)
        .body("The application is quiting! babai.")
        .timeout(Timeout::Milliseconds(1000)) //milliseconds
        .show().unwrap();
    1
}

fn random_request_notification(tiempo: Duration) {
    let tbody = format!("The application is very ran dom! It took this tiempo {:?} babai.", tiempo);
    Notification::new()
        .summary("Very random")
        .body(&tbody)
        .timeout(Timeout::Milliseconds(100)) //milliseconds
        .show().unwrap();
    ()
}