use iced::widget::{button, row, text};
use iced::{window, Alignment, Element, Size};

pub fn main() -> iced::Result {
    iced::application("Counter", Counter::update, Counter::view)
        .window(window::Settings {
            size: Size {
                width: 200.0,
                height: 110.0,
            },
            ..Default::default()
        })
        .run()
}

#[derive(Default)]
struct Counter {
    value: u32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
}

impl Counter {
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
        .spacing(50)
        .align_y(Alignment::Center)
        .into()
    }
}
