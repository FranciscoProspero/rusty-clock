use iced::{
    button, Alignment, Button, Column, Element, Sandbox, Text, window,
};

#[derive(Default)]
pub struct Popup {
    _running_timer: i32,
}

impl Popup {
    pub fn _new() -> Self {
        println!("Called Popus");
        Self::default()
    }
    
    pub fn _run_popus() {
        let mut testis = window::Settings::default();
        testis.always_on_top = true;
        testis.size = (100, 400);
        testis.position = window::Position::Specific(0,0);

        // Counter2::run(Settings {
        //     window: testis,
        //     ..Settings::default()
        // });
    }

    // pub fn run_main() {
    //     Counter::run(Settings {
    //         window: testis,
    //         ..Settings::default()
    //     });
    // } 
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Study,
    Work,
    Fun,
    Coffee,
    Statistics,
    Quit,
}

#[derive(Default)]
pub struct Counter2 {
    value: i32,
    study_button: button::State,
    work_button: button::State,
    fun_button: button::State,
    coffee_button: button::State,
    statistical_button: button::State,
    quit_button: button::State,
    close : bool,
}


impl Sandbox for Counter2 {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("What you doin'?")
    }

    fn should_exit(&self) -> bool {
        self.close
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Study => {
                println!("Having Studio");
            }
            Message::Work => {
                println!("Having Worko");
            }
            Message::Fun => {
                println!("Having FUNO");
            }
            Message::Coffee => {
                println!("Having Coffio");
            }
            Message::Statistics => {
                println!("Having Statistica");
            }
            Message::Quit => {
                println!("Having quito");
                self.close = true;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.study_button, Text::new("Study"))
                    .on_press(Message::Study),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.work_button, Text::new("Work"))
                    .on_press(Message::Work),
            )
            .push(
                Button::new(&mut self.fun_button, Text::new("Fun"))
                    .on_press(Message::Fun),
            )
            .push(
                Button::new(&mut self.coffee_button, Text::new("Coffee"))
                    .on_press(Message::Coffee),
            )
            .push(
                Button::new(&mut self.statistical_button, Text::new("Statistics"))
                    .on_press(Message::Statistics),
            )
            .push(
                Button::new(&mut self.quit_button, Text::new("Quit"))
                    .on_press(Message::Quit),
            )
            .into()
    }

}

#[derive(Default)]
pub struct Counter {
    value: i32,
    study_button: button::State,
    work_button: button::State,
    fun_button: button::State,
    coffee_button: button::State,
    statistical_button: button::State,
    quit_button: button::State,
    close : bool,
}


impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("What you doin'?")
    }

    fn should_exit(&self) -> bool {
        self.close
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Study => {
                println!("Having Studio");
            }
            Message::Work => {
                println!("Having Worko");
            }
            Message::Fun => {
                println!("Having FUNO");
            }
            Message::Coffee => {
                println!("Having Coffio");
            }
            Message::Statistics => {
                println!("Having Statistica");
            }
            Message::Quit => {
                println!("Having quito");
                self.close = true;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .spacing(20)
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.study_button, Text::new("Study"))
                    .on_press(Message::Study),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.work_button, Text::new("Work"))
                    .on_press(Message::Work),
            )
            .push(
                Button::new(&mut self.fun_button, Text::new("Fun"))
                    .on_press(Message::Fun),
            )
            .push(
                Button::new(&mut self.coffee_button, Text::new("Coffee"))
                    .on_press(Message::Coffee),
            )
            .push(
                Button::new(&mut self.statistical_button, Text::new("Statistics"))
                    .on_press(Message::Statistics),
            )
            .push(
                Button::new(&mut self.quit_button, Text::new("Quit"))
                    .on_press(Message::Quit),
            )
            .into()
    }

}