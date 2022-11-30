use iced::widget::{row, text, text_input};
use iced::{window, Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    TemperatureConverter::run(Settings {
        window: window::Settings {
            size: (600, 80),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Default)]
struct TemperatureConverter {
    celsius: String,
    fahrenheit: String,
}

#[derive(Clone, Debug)]
enum Message {
    CelsiusChanged(String),
    FahrenheitChanged(String),
}

impl TemperatureConverter {
    fn empty(&mut self) {
        self.celsius = "".into();
        self.fahrenheit = "".into();
    }
}

impl Sandbox for TemperatureConverter {
    type Message = Message;

    fn new() -> Self {
        Self {
            celsius: "".into(),
            fahrenheit: "".into(),
        }
    }

    fn title(&self) -> String {
        String::from("Temperature Converter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CelsiusChanged(s) => {
                if s.is_empty() {
                    self.empty();
                } else {
                    self.celsius = s.clone();
                    match s.parse::<f64>() {
                        Ok(celsius) => {
                            let fahrenheit = celsius * (9. / 5.) + 32.;
                            self.fahrenheit = fahrenheit.to_string();
                        }
                        Err(_) => self.empty(),
                    }
                }
            }
            Message::FahrenheitChanged(s) => {
                if s.is_empty() {
                    self.empty();
                } else {
                    self.fahrenheit = s.clone();
                    match s.parse::<f64>() {
                        Ok(fahrenheit) => {
                            let celsius = (fahrenheit - 32.) * (5. / 9.);
                            self.celsius = celsius.to_string();
                        }
                        Err(_) => self.empty(),
                    };
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        row![
            text_input("", &self.celsius, Message::CelsiusChanged),
            text("Celsius =").size(50),
            text_input("", &self.fahrenheit, Message::FahrenheitChanged),
            text("Fahrenheit").size(50),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
