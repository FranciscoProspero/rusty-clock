use rusqlite::{Connection, Result};
use super::timer_structs::{TimerGlobs, TypesOfTimers};
use std::time::SystemTime;

#[derive(Debug)]
pub struct Timer {
    pub id: i32,
    pub timertype: String,
    pub time: u64,
}

#[derive(Debug)]
pub struct Count {
    pub nr: i32,
}

#[derive(Debug)]
pub struct Datab {
    conn : Connection,
    _connected : bool,
}

impl Datab {
    pub fn new() -> Datab {
        let conn = Connection::open("rusty-clock.db");
        let db = match conn {
            Ok(_) => Datab { conn : conn.unwrap(), _connected : true},
            Err(_) => Datab { conn : conn.unwrap(), _connected : false},
        };
        db.create_table();
        db
    }

    pub fn create_table(&self) {
        let create_table = self.conn.execute(
            "create table if not exists timers (
                    id integer primary key,
                    type_of_timer text not null unique,
                    total_time integer
                )",
            [],
        );
        if self.is_empty() {
            self.populate();
        }
    }
    
    fn is_empty(&self) -> bool{
        let mut stmt = self.conn.prepare("SELECT count(*) FROM timers").unwrap();
        let rows = stmt.query_map([], |row| {
                Ok(Count {
                    nr: row.get(0).unwrap(),
                })
            }).unwrap();

        for name_result in rows {
            if name_result.unwrap().nr == 0 {
                return true;
            }
            else {
                return false;
            }
        }
        false
    }

    fn populate(&self) {
        let test = TimerGlobs::new(TypesOfTimers::Study,0,0);
        self.db_new_val(&test, TypesOfTimers::Study.to_string());
        let test = TimerGlobs::new(TypesOfTimers::Work,1,0);
        self.db_new_val(&test, TypesOfTimers::Work.to_string());
        let test = TimerGlobs::new(TypesOfTimers::Fun,2,0);
        self.db_new_val(&test, TypesOfTimers::Fun.to_string());
        let test = TimerGlobs::new(TypesOfTimers::Coffee,3,0);
        self.db_new_val(&test, TypesOfTimers::Coffee.to_string());
        self.db_read_all();
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
                Ok(timer_val) => {
                    if timer_val.id == id {
                       return timer_val.time as u64
                    }
                },
                Err(_) => return 0,
            }
        }
        0
    }
}