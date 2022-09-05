use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io;

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

struct TimerGlobs {
    timer_type: TypesOfTimers,
    id: usize,
    total_time: i32,
    current_time: i32,
    alert_timer: i32
}

impl TimerGlobs {
    fn new(type_of_timer: TypesOfTimers, idx : usize) -> TimerGlobs {
        TimerGlobs {timer_type: type_of_timer, id: idx, total_time: 0, current_time: 0, alert_timer: 0}
    }
}


fn main() {

    let now = Instant::now();
    start_cli();

    let elapsed_time = now.elapsed();
    println!("You have been practicing for {} seconds.", elapsed_time.as_secs());
}

fn start_cli() {
    let timer_names: [TypesOfTimers; 4] = [TypesOfTimers::Study, TypesOfTimers::Work, TypesOfTimers::Fun, TypesOfTimers::Coffee];

    let mut test_vec = Vec::with_capacity(4);

    for (i, name) in timer_names.into_iter().enumerate() {
        test_vec.push(TimerGlobs::new(name,i));
    }

    let timer_vec_mtx = Arc::new(Mutex::new(test_vec));
    println!("You have started the Study timer");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        let timer_vec = Arc::clone(&timer_vec_mtx);
        match input.trim() {
            "Study" => spawn_control(&timer_vec, 0),
            "Work" => spawn_control(&timer_vec, 1),
            "Fun" => spawn_control(&timer_vec, 2),
            "Coffee" => spawn_control(&timer_vec, 3),
            "Quit" => break,
            _ => -1,
        };
    }
}

fn spawn_control( batch:&Arc<Mutex<Vec<TimerGlobs>>>, position: usize ) -> i32 {
    let mtx = Arc::clone(&batch);
    let handle = thread::spawn(move || { 
       let mut num = mtx.lock().unwrap();
       count_zi_time((*num)[position].id); 
    });
    handle.join().unwrap();
    0
}

fn count_zi_time(idy: usize) {
    let now = Instant::now();
    println!("zi time iz counts{:?}", idy);

    let elapsed_time = now.elapsed();
}