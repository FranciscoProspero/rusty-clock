use super::timer_manager::timer_thread;
use super::timer_structs::TypesOfTimers;
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
}

impl Gui {
    pub fn new() -> Gui {
        let (tx, rx) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        let handle = thread::spawn( move || { 
            timer_thread(rx, tx2);
        });

        Gui {
            flag : true,
            tx: tx,
            rx2: rx2,
        }
    }

    pub fn send(&self, type_of_timer: TypesOfTimers) {
        self.tx.send(type_of_timer);
    }

}

struct RustyClock {
    duration: Duration,
    state: State,
    current_timer: Timer,
    stop: button::State,
    study: button::State,
    work: button::State,
    fun: button::State,
    coffee: button::State,
    quit: button::State,
    exit: bool,
    gui: Gui,
}

enum Timer {
    Stop,
    Study,
    Work,
    Fun,
    Coffee,
}

enum State {
    Idle,
    Ticking { last_tick: Instant },
}

#[derive(Debug, Clone)]
enum Message {
    Stop,
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
            stop: button::State::new(),
            study: button::State::new(),
            work: button::State::new(),
            fun: button::State::new(),
            coffee: button::State::new(),
            quit: button::State::new(),
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

        let study_button = {
            let color = match self.current_timer {
                Timer::Study => style::Button::Primary,
                _ => style::Button::Secondary,
            };
            button(&mut self.study, "Study", color).on_press(Message::Study)
        };

        let work_button = {
            let color = match self.current_timer {
                Timer::Work => style::Button::Primary,
                _ => style::Button::Secondary,
            };
            button(&mut self.work, "Work", color).on_press(Message::Work)
        };

        let fun_button = {
            let color = match self.current_timer {
                Timer::Fun => style::Button::Primary,
                _ => style::Button::Secondary,
            };
            button(&mut self.fun, "Fun", color).on_press(Message::Fun)
        };

        let coffee_button = {
            let color = match self.current_timer {
                Timer::Coffee => style::Button::Primary,
                _ => style::Button::Secondary,
            };
            button(&mut self.coffee, "Coffee", color).on_press(Message::Coffee)
        };

        let controls = Row::new()
            .spacing(20)
            .push(stop_button)
            .push(study_button)
            .push(work_button)
            .push(fun_button)
            .push(coffee_button);

        let quit_button =
            button(&mut self.quit, "Quit", style::Button::Destructive)
                .on_press(Message::Quit);
          
        let quit_controls = Row::new()
            .spacing(20)
            .push(quit_button);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(duration)
            .push(controls)
            .push(quit_controls);

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
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                    Button::Destructive => Color::from_rgb(0.8, 0.2, 0.2),
                    Button::Halt => Color::from_rgb(0.80, 0.40, 0.1 ),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}