use super::timer_structs::{TimerGlobs, TypesOfTimers};

use rusqlite::Connection;
use chrono::{Datelike, Timelike, Utc, Date, NaiveDate, Duration};

#[derive(Debug)]
pub struct Timer {
    pub id: i32,
    pub date: String,
    pub study: u64,
    pub work: u64,
    pub fun: u64,
    pub coffee: u64,
}

#[derive(Debug)]
pub struct Count {
    pub nr: i32,
}

#[derive(Debug)]
pub struct Test {
    pub date: String,
}

#[derive(Debug)]
pub struct Datab {
    conn : Connection,
    today : Date<Utc>,
    _connected : bool,
}

impl Datab {
    pub fn new() -> Datab {

        let today = Utc::today();
        let conn = Connection::open("rusty-clock.db");
        match conn {
            Ok(_) => Datab { conn : conn.unwrap(), today: today, _connected : true},
            Err(_) => Datab { conn : conn.unwrap(), today: today, _connected : false},
        }
    }

    pub fn create_table(&self) {
        self.conn.execute(
            "create table if not exists timers (
                    id integer primary key,
                    date text not null unique,
                    study integer,
                    work integer,
                    fun integer,
                    coffee integer
                )",
            [],
        );
        if self.is_table_empty() {
            self.populate();
        }
        if self.is_today_empty() {
            let formatted_date = format!("{}", self.today.format("%Y-%m-%d"));
            let id = self.last_unique_id() + 1;
            self.db_new_val(id, formatted_date, 0, 0, 0, 0);
        }
    }

    fn count_db_lines(&self, query: String) -> bool {
        let mut stmt = self.conn.prepare(&query.to_string()).unwrap();
        let mut rows = stmt.query_map([], |row| {
                Ok(Count {
                    nr: row.get(0).unwrap(),
                })
            }).unwrap();
        let count_of_rows = rows.nth(0).unwrap().unwrap().nr;

        if count_of_rows == 0 {
            true
        }
        else {
            false
        }
    }

    fn is_table_empty(&self) -> bool{
        let empty = self.count_db_lines("SELECT count(*) FROM timers".to_string());
        if empty {
            true
        }
        else {
            false
        }
    }

    fn is_today_empty(&self) -> bool{
        let formatted = format!("{}", self.today.format("%Y-%m-%d"));
        let query = "SELECT count(*) FROM timers WHERE date = \"".to_string();
        let query = format!("{}{}{}", query, formatted, "\"".to_string());

        let empty = self.count_db_lines(query);
        if empty {
            true
        }
        else {
            false
        }
    }

    fn populate(&self) {
        let formatted_date = format!("{}", self.today.format("%Y-%m-%d"));
        self.db_new_val(0, formatted_date, 0, 0, 0, 0);
        self.db_read_all();
        // let mut today = self.today;
        // for i in 0..137 as i32{
        //     today += Duration::days(1);
        //     let formatted_date = format!("{}", today.format("%Y-%m-%d"));
        //     self.db_new_val(i, formatted_date, i, i, i, i);
        // }
        // self.db_read_all();
    }

    fn db_new_val(&self, id: i32, date: String, study: i32, work: i32, fun: i32, coffee: i32 ) {
        self.conn.execute(
            "INSERT INTO timers (id, date, study, work, fun, coffee) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (id, date, study, work, fun, coffee),
        );
    }

    pub fn db_update_val(&self, timer: &String, totaltime : &u64) {
        let query = "UPDATE timers SET ";
        let query = format!("{}{}{}", query, timer.to_lowercase(),  "= (?1) WHERE id = (?2)".to_string());
        self.conn.execute( &query, (totaltime, self.last_unique_id()) );
    }

    pub fn db_read_all(&self) {
        let mut stmt = self.conn.prepare("SELECT id, date, study, work, fun, coffee FROM timers").unwrap();
            
        let timer_iter = stmt.query_map([], |row| {
            Ok(Timer {
                id: row.get(0)?,
                date: row.get(1)?,
                study: row.get(2)?,
                work: row.get(3)?,
                fun: row.get(4)?,
                coffee: row.get(5)?,
            })
        }).unwrap();
        for timer in timer_iter {
            println!("DB: {:?}", timer.unwrap());
        }
    }

    pub fn read_total_time(&self, timer: TypesOfTimers) -> u64 {
        let query = "SELECT id, date, study, work, fun, coffee FROM timers WHERE id = ";
        let query = format!("{}{}", query, self.last_unique_id() );
        let mut stmt = self.conn.prepare(&query).unwrap();
            
        let timer_iter = stmt.query_map([], |row| {
            Ok(Timer {
                id: row.get(0)?,
                date: row.get(1)?,
                study: row.get(2)?,
                work: row.get(3)?,
                fun: row.get(4)?,
                coffee: row.get(5)?,
            })
        }).unwrap();
        for iter_item in timer_iter {
            match timer {
                TypesOfTimers::Study => return iter_item.unwrap().study,
                TypesOfTimers::Work => return iter_item.unwrap().work,
                TypesOfTimers::Fun => return iter_item.unwrap().fun,
                TypesOfTimers::Coffee => return iter_item.unwrap().coffee,
                _ => 0
            };
        }
        0
    }

    pub fn last_unique_id(&self) -> i32 {
        let mut stmt = self.conn.prepare("SELECT id, date, study, work, fun, coffee FROM timers").unwrap();
            
        let timer_iter = stmt.query_map([], |row| {
            Ok(Timer {
                id: row.get(0)?,
                date: row.get(1)?,
                study: row.get(2)?,
                work: row.get(3)?,
                fun: row.get(4)?,
                coffee: row.get(5)?,
            })
        }).unwrap();
        let mut max_id = 0;
        for timer in timer_iter {
            match timer {
                Ok(timer_val) => {
                    if timer_val.id > max_id  {
                       max_id = timer_val.id;
                    }
                },
                Err(_) => return 0,
            }
        }
        max_id
    }

    // receive as input year and return an average of timers on that year
    pub fn read_year_average(&self, year: i32) -> (u64, u64, u64, u64) {
        let mut stmt = self.conn.prepare("SELECT id, date, study, work, fun, coffee FROM timers").unwrap();
            
        let timer_iter = stmt.query_map([], |row| {
            Ok(Timer {
                id: row.get(0)?,
                date: row.get(1)?,
                study: row.get(2)?,
                work: row.get(3)?,
                fun: row.get(4)?,
                coffee: row.get(5)?,
            })
        }).unwrap();
        let mut study_sum = 0;
        let mut work_sum = 0;
        let mut fun_sum = 0;
        let mut coffee_sum = 0;
        let mut count = 0;
        for timer in timer_iter {
            match timer {
                Ok(timer_val) => {
                    let date = NaiveDate::parse_from_str(&timer_val.date, "%Y-%m-%d").unwrap();
                    if date.year() == year {
                        study_sum += timer_val.study;
                        work_sum += timer_val.work;
                        fun_sum += timer_val.fun;
                        coffee_sum += timer_val.coffee;
                        count += 1;
                    }
                },
                Err(_) => return (0, 0, 0, 0),
            }
        }
        (study_sum/count, work_sum/count, fun_sum/count, coffee_sum/count)
    }

    // return the average of timers per month with corresponding month from database
    pub fn read_average_per_month_with_month(&self) -> Vec<(u64, u64, u64, u64, u32, i32)> {
        let mut stmt = self.conn.prepare("SELECT id, date, study, work, fun, coffee FROM timers").unwrap();
            
        let timer_iter = stmt.query_map([], |row| {
            Ok(Timer {
                id: row.get(0)?,
                date: row.get(1)?,
                study: row.get(2)?,
                work: row.get(3)?,
                fun: row.get(4)?,
                coffee: row.get(5)?,
            })
        }).unwrap();
        let mut study_sum = 0;
        let mut work_sum = 0;
        let mut fun_sum = 0;
        let mut coffee_sum = 0;
        let mut count = 0;
        let mut month = 0;
        let mut year = 0;
        let mut result = Vec::new();
        for timer in timer_iter {
            match timer {
                Ok(timer_val) => {
                    let date = NaiveDate::parse_from_str(&timer_val.date, "%Y-%m-%d").unwrap();
                    if date.month() == month && date.year() == year {
                        study_sum += timer_val.study;
                        work_sum += timer_val.work;
                        fun_sum += timer_val.fun;
                        coffee_sum += timer_val.coffee;
                        count += 1;
                    } else {
                        if count != 0 {
                            result.push((study_sum/count, work_sum/count, fun_sum/count, coffee_sum/count, month, year));
                        }
                        study_sum = timer_val.study;
                        work_sum = timer_val.work;
                        fun_sum = timer_val.fun;
                        coffee_sum = timer_val.coffee;
                        count = 1;
                        month = date.month();
                        year = date.year();
                    }
                },
                Err(_) => return Vec::new(),
            }
        }
        result
    }

    // get all timers from db and divide by all occurences
    pub fn read_average(&self) -> (u64, u64, u64, u64) {
        let mut stmt = self.conn.prepare("SELECT id, date, study, work, fun, coffee FROM timers").unwrap();
            
        let timer_iter = stmt.query_map([], |row| {
            Ok(Timer {
                id: row.get(0)?,
                date: row.get(1)?,
                study: row.get(2)?,
                work: row.get(3)?,
                fun: row.get(4)?,
                coffee: row.get(5)?,
            })
        }).unwrap();
        let mut study_sum = 0;
        let mut work_sum = 0;
        let mut fun_sum = 0;
        let mut coffee_sum = 0;
        let mut count = 0;
        for timer in timer_iter {
            match timer {
                Ok(timer_val) => {
                    study_sum += timer_val.study;
                    work_sum += timer_val.work;
                    fun_sum += timer_val.fun;
                    coffee_sum += timer_val.coffee;
                    count += 1;
                },
                Err(_) => return (0, 0, 0, 0),
            }
        }
        (study_sum/count, work_sum/count, fun_sum/count, coffee_sum/count)
    }

    
}





