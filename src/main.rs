use std::time::{Duration, Instant};
use std::thread;
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
    fn new(name: TypesOfTimers, idx : usize) -> TimerGlobs {
        TimerGlobs {timer_type: name, id: idx, total_time: 0, current_time: 0, alert_timer: 0}
    }
}

fn main() {
    let now = Instant::now();

    thread::spawn(|| {
        wait_for_start();
    });

    wait_for_start();

    let elapsed_time = now.elapsed();
    println!("You have been practicing for {} seconds.", elapsed_time.as_secs());
}

fn wait_for_start() {
    let timer_names: [TypesOfTimers; 4] = [TypesOfTimers::Study, TypesOfTimers::Work, TypesOfTimers::Fun, TypesOfTimers::Coffee];

    let mut test_vec = Vec::with_capacity(4);

    for (i, name) in timer_names.into_iter().enumerate() {
        test_vec.push(TimerGlobs::new(name,i));
    }

    for strt in &test_vec{
        println!("olé - {:?}",strt.id)
    }
    println!("You have started the Study timer");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        
        match input.trim() {
            "Study" => count_zi_time(&test_vec[0]),
            "Work" => println!("2 - {}", input),
            "Fun" => println!("3 - {}", input),
            "Coffee" => println!("4 - {}", input),
            "Quit" => break,
            _ => println!("Ain't a commanda bruh"),
        }
    }
}

fn count_zi_time(batch:&TimerGlobs) {
    println!("zi time iz counts{:?}",batch.timer_type);

}