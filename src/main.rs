use std::time::{Duration, Instant};
use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::io;
use std::sync::mpsc::{self, TryRecvError};

// Objetivos:
//   Ter um cli
//   Receber um comando para dar start de timer especifico
//   Tipos de timers -> Study, Work, Fun, Coffee
//   Apenas um timer pode estar ativo
//   Dá para mudar de timer sem ter de terminar o outro primeiro
//   Gravar valores por dia, semana, mes, ano e fazer estatisticas
//   Avisar a cada x minutos que timer está a decorrer, pode ser especificado ao correr o comando
//   Notificar de y em y a perguntar o que se está a fazer
//   Ter num nice gui :) probably tui-rs

// Ordem Tarefas:
//   1 - cli
//   2 - types of timers
//   3 - start timer commands
//   4 - stop timer commands
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
    timer_type: TypesOfTimers,
    id: usize,
    total_time: Duration,
    current_time: Duration,
    alert_timer: i32
}

impl TimerGlobs {
    fn new(type_of_timer: TypesOfTimers, idx : usize) -> TimerGlobs {
        TimerGlobs {timer_type: type_of_timer, id: idx, total_time: Duration::new(0,0), current_time: Duration::new(0,0), alert_timer: 0}
    }

    fn update_current_timer(&mut self, elapsed_time : Duration) -> (){
        self.current_time = elapsed_time;
    }

    fn update_total_timer(&mut self, elapsed_time : Duration) -> (){
        self.total_time = self.total_time + elapsed_time;
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
    //handle.join().unwrap();
    loop {
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        let mut num = timer_vec_mtx.lock().unwrap();
        println!("{:?}", *num); 
        if input_possibilities.contains( &input.trim() ) {
            
            let mtx = Arc::clone(&timer_vec_mtx);
            match input.trim() {
                "Study" | "study" => tx.send(TypesOfTimers::Study),
                "Work" | "work" => tx.send(TypesOfTimers::Work),
                "Fun" | "fun" => tx.send(TypesOfTimers::Fun),
                "Coffee" | "coffee" => tx.send(TypesOfTimers::Coffee),
                _ => Ok(()),
            };
        } 
        else if input_exit.contains(&input.trim())
        {
            //let mut num = timer_vec_mtx.lock().unwrap();
            //println!("{:?}", *num); 
            let _ = tx.send(TypesOfTimers::Quit);

            break;
        }
    }
    
}

fn timer_thread(mtx:&Arc<Mutex<Vec<TimerGlobs>>>, rx: std::sync::mpsc::Receiver<TypesOfTimers>) -> i32 {
    println!("yes my name is burrito");
    let now = Instant::now();
    let mut running_pos : usize = 50;
    let fifty_ms = time::Duration::from_millis(50);

    loop {
        thread::sleep(fifty_ms);
        if (running_pos < 5) {
            let elapsed_time = now.elapsed();      
            let mut num = mtx.lock().unwrap();
            num[running_pos].update_current_timer(elapsed_time);
        }
        match rx.try_recv() {
            Ok(TypesOfTimers::Study) => {
                running_pos = 0;
                println!("Study")
            },
            Ok(TypesOfTimers::Work) => {
                running_pos = 1;
                println!("Work")
            },
            Ok(TypesOfTimers::Fun) => {
                running_pos = 2;
                println!("Fun")
            },
            Ok(TypesOfTimers::Coffee) => {
                running_pos = 3;
                println!("Coffee")
            },
            Ok(TypesOfTimers::Quit)  => {
                let elapsed_time = now.elapsed();      
                let mut num = mtx.lock().unwrap();
                num[running_pos].update_total_timer(elapsed_time);
                println!("Quit -> Terminating.");
                break;
            },
            Ok(_) => {
                println!("Nota sure");
                break;
            }
            Err(TryRecvError::Disconnected) => {
                    println!("Error Disconetiooni."); 
                    break;
                }
            Err(TryRecvError::Empty) => {}
        }
    }
    0
}
