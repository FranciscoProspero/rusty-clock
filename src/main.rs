
mod clock;
use clock::timer_manager::timer_thread;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use std::io;
use std::sync::mpsc;

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


