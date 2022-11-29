use iced::widget::{button, text, row};
use iced::{window, Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Counter::run(Settings {
        window: window::Settings {
            size: (200, 100),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Default)]
struct Counter {
    value: u32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        row![
            text(self.value).size(50),
            button("Count").on_press(Message::IncrementPressed),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}