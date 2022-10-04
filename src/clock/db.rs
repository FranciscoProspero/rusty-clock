use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Timer {
    pub id: i32,
    pub timertype: String,
}


#[derive(Debug)]
pub struct Datab {
    conn : Connection,
    connected : bool,
}

impl Datab {
    pub fn new() -> Datab {
        let conn = Connection::open("timer.db");
        match conn {
            Ok(_) => Datab { conn : conn.unwrap(), connected : true},
            Err(_) => Datab { conn : conn.unwrap(), connected : false},
        }
    }

    pub fn createtable(&self) {
        self.conn.execute(
            "create table if not exists types_of_timers (
                    id integer primary key,
                    name text not null unique
                )",
            rusqlite::NO_PARAMS,
        );
    }

    pub fn dbruntest(&self) {
        let test = Timer {
            id: 1,
            timertype: "Work".to_string(),
        };
        self.conn.execute(
            "INSERT INTO types_of_timers (id, name) VALUES (?1, ?2)",
            (&test.id, &test.timertype),
        );
        let mut stmt = self.conn.prepare("SELECT id, name FROM types_of_timers").unwrap();
            
        let timer_iter = stmt.query_map([], |row| {
            Ok(Timer {
                id: row.get(0)?,
                timertype: row.get(1)?,
            })
        }).unwrap();
        for timer in timer_iter {
            println!("Found timer {:?}", timer.unwrap());
        }
    }

    pub fn db_new_val(&self, test: &Timer) {
        self.conn.execute(
            "INSERT INTO types_of_timers (id, name) VALUES (?1, ?2)",
            (&test.id, &test.timertype),
        );
    }

    pub fn db_update_val(&self) {
        let mut test = self.conn.execute(
            "UPDATE types_of_timers SET name = 'Salmonela' WHERE id = 1",rusqlite::NO_PARAMS, 
        ).unwrap();
    }

    pub fn db_read_all(&self) {
        let mut stmt = self.conn.prepare("SELECT id, name FROM types_of_timers").unwrap();
            
        let timer_iter = stmt.query_map([], |row| {
            Ok(Timer {
                id: row.get(0)?,
                timertype: row.get(1)?,
            })
        }).unwrap();
        for timer in timer_iter {
            println!("Found timer {:?}", timer.unwrap());
        }
    }
}