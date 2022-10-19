use super::timer_manager::timer_thread;
use super::timer_structs::TypesOfTimers;
use super::db::Datab;

use iced::{
    alignment, button, executor, time, Alignment, Application, Button, Column,
    Command, Container, Element, Length, Row, Settings, Subscription, Text, window
};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;

pub struct GuiRustyClock {
    flag: bool,
}

impl GuiRustyClock {
    pub fn new() -> GuiRustyClock {
        GuiRustyClock {
            flag: true,
        }
    }
    
    pub fn start(&self)  {
        if self.flag == true {
            let mut testis = window::Settings::default();
            // testis.always_on_top = true;
            testis.size = (600, 250);
            //testis.position = window::Position::Specific(0,0);

            RustyClock::run(Settings {
                    window: testis,
                    ..Settings::default()
                });
        }
    }
}

pub struct Gui {
    flag: bool,
    tx: std::sync::mpsc::Sender<TypesOfTimers>,
    rx2: std::sync::mpsc::Receiver<u32>,
    database : Datab,
}

impl Gui {
    pub fn new() -> Gui {
        let (tx, rx) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        let db = Datab::new();
        let handle = thread::spawn( move || { 
            timer_thread(rx, tx2);
        });

        Gui {
            flag : true,
            tx: tx,
            rx2: rx2,
            database: db,
        }
    }

    pub fn send(&self, type_of_timer: TypesOfTimers) {
        self.tx.send(type_of_timer);
    }

    // with timer as input return time in seconds




}

struct RustyClock {
    duration: Duration,
    state: State,
    current_timer: Timer,
    study_timer: u64,
    work_timer: u64,
    fun_timer: u64,
    coffee_timer: u64,
    stop: button::State,
    study: button::State,
    work: button::State,
    fun: button::State,
    coffee: button::State,
    quit: button::State,
    stats: button::State,
    exit: bool,
    gui: Gui,
}

enum Timer {
    Stop,
    Study,
    Work,
    Fun,
    Coffee,
    Stats,
}

enum State {
    Idle,
    Ticking { last_tick: Instant },
}

#[derive(Debug, Clone)]
enum Message {
    Stop,
    Stats,
    Study,
    Work,
    Fun,
    Coffee,
    Quit,
    Tick(Instant),
}

impl Application for RustyClock {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (RustyClock, Command<Message>) {
        let gui = Gui::new();
        (
        RustyClock {
            duration: Duration::default(),
            state: State::Idle,
            current_timer: Timer::Stop,
            study_timer: gui.database.read_today_time(TypesOfTimers::Study),
            work_timer: gui.database.read_today_time(TypesOfTimers::Work),
            fun_timer: gui.database.read_today_time(TypesOfTimers::Fun),
            coffee_timer: gui.database.read_today_time(TypesOfTimers::Coffee),
            stop: button::State::new(),
            study: button::State::new(),
            work: button::State::new(),
            fun: button::State::new(),
            coffee: button::State::new(),
            quit: button::State::new(),
            stats: button::State::new(),
            exit: false,
            gui: gui,
        },
        Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rusty Clock")
    }

    fn should_exit(&self) -> bool {
        self.exit
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let old_duration = self.duration;
        match message {
            Message::Tick(now) => match &mut self.state {
                State::Ticking { last_tick } => {
                    self.duration += now - *last_tick;
                    *last_tick = now;
                }
                _ => {}
            },
            Message::Stop => {
                self.state = State::Idle;
            },
            Message::Quit => {
                self.gui.send(TypesOfTimers::Quit);
                let onehundredms = time::Duration::from_millis(100);

                thread::sleep(onehundredms);
                self.exit = true;
            },
            Message::Study => {
                match self.state {
                    State::Idle => {
                        self.state = State::Ticking {
                            last_tick: Instant::now(),
                        };
                    },
                    _ => {}
                };
                match self.current_timer {
                    Timer::Study => {} ,
                    _ => { 
                        self.current_timer = Timer::Study;
                        self.duration = Duration::default();
                        self.gui.send(TypesOfTimers::Study);
                    },
                };
            },
            Message::Work => {
                match self.state {
                    State::Idle => {
                        self.state = State::Ticking {
                            last_tick: Instant::now(),
                        };
                    },
                    _ => {}
                };
                match self.current_timer {
                    Timer::Work => {} ,
                    _ => {
                        self.current_timer = Timer::Work;
                        self.duration = Duration::default();
                        self.gui.send(TypesOfTimers::Work);
                        match self.state {
                            State::Idle => {
                                self.state = State::Ticking {
                                    last_tick: Instant::now(),
                                };
                            },
                            _ => {}
                        };
                    },
                };
            },
            Message::Fun => {
                match self.state {
                    State::Idle => {
                        self.state = State::Ticking {
                            last_tick: Instant::now(),
                        };
                    },
                    _ => {}
                };
                match self.current_timer {
                    Timer::Fun => {} ,
                    _ => {
                    self.current_timer = Timer::Fun;
                    self.duration = Duration::default();
                    self.gui.send(TypesOfTimers::Fun);
                    match self.state {
                        State::Idle => {
                            self.state = State::Ticking {
                                last_tick: Instant::now(),
                            };
                        },
                        _ => {}
                    };
                    },
                };
            },
            Message::Coffee => {
                match self.state {
                    State::Idle => {
                        self.state = State::Ticking {
                            last_tick: Instant::now(),
                        };
                    },
                    _ => {}
                };
                match self.current_timer {
                    Timer::Coffee => {} ,
                    _ => {
                        self.current_timer = Timer::Coffee;
                        self.duration = Duration::default();
                        self.gui.send(TypesOfTimers::Coffee);
                        match self.state {
                            State::Idle => {
                                self.state = State::Ticking {
                                    last_tick: Instant::now(),
                                };
                            },
                            _ => {}
                        };
                    },
                };
            }
            Message::Stats => {
                self.gui.send(TypesOfTimers::Stats);
            }
        }
        if self.duration.as_secs() > old_duration.as_secs() {
            match self.current_timer {
                Timer::Study => {
                    self.study_timer += 1;
                },
                Timer::Work => {
                    self.work_timer += 1;
                },
                Timer::Fun => {
                    self.fun_timer += 1;
                },
                Timer::Coffee => {
                    self.coffee_timer += 1;
                },
                _ => {}
            }
        }
        
        Command::none()   
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => {
                time::every(Duration::from_millis(10)).map(Message::Tick)
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        const MINUTE: u64 = 60;
        const HOUR: u64 = 60 * MINUTE;

        let seconds = self.duration.as_secs();

        let duration = Text::new(format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.duration.subsec_millis() / 10,
        ))
        .size(40);

        let button = |state, label, style| {
            Button::new(
                state,
                Text::new(label)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
            .padding(10)
            .width(Length::Units(80))
            .style(style)
        };

        let stop_button =
            button(&mut self.stop, "Stop", style::Button::Halt)
                .on_press(Message::Stop);

        let quit_button =
            button(&mut self.quit, "Quit", style::Button::Destructive)
                .on_press(Message::Quit);
          
        let stop_column = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(stop_button)
            .push(quit_button);


        let study_button = {
            let color = match self.current_timer {
                Timer::Study => style::Button::Primary,
                _ => style::Button::Secondary,
            };
            button(&mut self.study, "Study", color).on_press(Message::Study)
        };

        let timer_study = self.study_timer.to_string();

        let study_timer = Text::new(timer_study)
            .size(40);

        let study_column = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(study_button)
            .push(study_timer);
            

        let work_button = {
            let color = match self.current_timer {
                Timer::Work => style::Button::Primary,
                _ => style::Button::Secondary,
            };
            button(&mut self.work, "Work", color).on_press(Message::Work)
        };

        let timer_work = self.work_timer.to_string();

        let work_timer = Text::new(timer_work)
            .size(40);

        let work_column = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(work_button)
            .push(work_timer);


        let fun_button = {
            let color = match self.current_timer {
                Timer::Fun => style::Button::Primary,
                _ => style::Button::Secondary,
            };
            button(&mut self.fun, "Fun", color).on_press(Message::Fun)
        };

        let timer_fun = self.fun_timer.to_string();

        let fun_timer = Text::new(timer_fun)
            .size(40);

        let fun_column = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(fun_button)
            .push(fun_timer);


        let coffee_button = {
            let color = match self.current_timer {
                Timer::Coffee => style::Button::Primary,
                _ => style::Button::Secondary,
            };
            button(&mut self.coffee, "Coffee", color).on_press(Message::Coffee)
        };

        let timer_coffee = self.coffee_timer.to_string();

        let coffee_timer = Text::new(timer_coffee)
            .size(40);

        let coffee_column = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(coffee_button)
            .push(coffee_timer);


        let controls = Row::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(stop_column)
            .push(study_column)
            .push(work_column)
            .push(fun_column)
            .push(coffee_column);
         

        let stats_button = button(&mut self.stats, "Stats", style::Button::Info).on_press(Message::Stats);


        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(duration)
            .push(controls)
            .push(stats_button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
        Destructive,
        Halt,
        Info,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                    Button::Destructive => Color::from_rgb(0.8, 0.2, 0.2),
                    Button::Halt => Color::from_rgb(0.80, 0.40, 0.1 ),
                    Button::Info => Color::from_rgb(0.10, 0.80, 0.1 ),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}

