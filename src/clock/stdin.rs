use std::io;
use super::timer_structs::TypesOfTimers;

pub fn stdin_parser(tx: std::sync::mpsc::Sender<TypesOfTimers>) {
    let input_possibilities = vec!["Study", "study", "Work", "work", "Fun", "fun", "Coffee", "coffee", "Stop", "stop"];
    let input_exit = vec!["Exit", "exit", "Quit", "quit", "End", "end", "Terminate", "terminate", "Q", "q"];
    let input_stats = vec!["Stats", "stats", "Statistics", "statistics", "Stat", "stat", "S", "s"];
    loop {
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");

        if input_possibilities.contains( &input.trim() ) {
            let res = match input.trim() {
                "Study" | "study" => tx.send(TypesOfTimers::Study),
                "Work" | "work" => tx.send(TypesOfTimers::Work),
                "Fun" | "fun" => tx.send(TypesOfTimers::Fun),
                "Coffee" | "coffee" => tx.send(TypesOfTimers::Coffee),
                "Stop" | "stop" => tx.send(TypesOfTimers::Stop),
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
            break;
        }
        else if input_stats.contains(&input.trim())
        {
            let res = tx.send(TypesOfTimers::Stats);
            res.unwrap();
        }
        else {
            println!("Invalid input, please try again.");
        }
    }
    println!("Exit stdin parser");
} 