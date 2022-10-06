use rusqlite::{Connection, Result};
use super::timer_structs::{TimerGlobs, TypesOfTimers};
use std::time::SystemTime;

#[derive(Debug)]
pub struct Timer {
    pub id: i32,
    pub timertype: String,
    pub time: f64,
}


#[derive(Debug)]
pub struct Datab {
    conn : Connection,
    connected : bool,
}

impl Datab {
    pub fn new() -> Datab {
        let conn = Connection::open("rusty-clock.db");
        match conn {
            Ok(_) => Datab { conn : conn.unwrap(), connected : true},
            Err(_) => Datab { conn : conn.unwrap(), connected : false},
        }
    }

    pub fn createtable(&self) {
        self.conn.execute(
            "create table if not exists timers (
                    id integer primary key,
                    type_of_timer text not null unique,
                    total_time integer
                )",
            rusqlite::NO_PARAMS,
        );
        
    }

    pub fn db_new_val(&self, timerglobs: &TimerGlobs, timertype: String) {
        let totaltime = *&timerglobs.total_time.as_millis() as u64;
        self.conn.execute(
            "INSERT INTO timers (id, type_of_timer, total_time) VALUES (?1, ?2, ?3)",
            (&timerglobs.id, &timertype, totaltime),
        );
    }

    pub fn db_update_val(&self, totaltime : &u64, id: &u32) {
        let mut test = self.conn.execute(
            "UPDATE timers SET total_time = (?1) WHERE id = (?2)",(totaltime,id), 
        ).unwrap();
    }

    pub fn db_read_all(&self) {
        let mut stmt = self.conn.prepare("SELECT id, type_of_timer, total_time FROM timers").unwrap();
            
        let timer_iter = stmt.query_map([], |row| {
            Ok(Timer {
                id: row.get(0)?,
                timertype: row.get(1)?,
                time: row.get(2)?,
            })
        }).unwrap();
        for timer in timer_iter {
            println!("DB: {:?}", timer.unwrap());
        }
    }

    pub fn read_total_time(&self, id: i32) -> u64 {
        let mut stmt = self.conn.prepare("SELECT id, type_of_timer, total_time FROM timers").unwrap();
            
        let timer_iter = stmt.query_map([], |row| {
            Ok(Timer {
                id: row.get(0)?,
                timertype: row.get(1)?,
                time: row.get(2)?,
            })
        }).unwrap();
        for timer in timer_iter {
            match timer {
                Ok(T) => {
                    if T.id == id {
                       return T.time as u64
                    }
                },
                Err(_) => return 0,
            }
        }
        0
    }
}