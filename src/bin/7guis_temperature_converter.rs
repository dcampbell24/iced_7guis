use iced::widget::{row, text, text_input};
use iced::{window, Alignment, Element, Size};

pub fn main() -> iced::Result {
    iced::application(
        "Temperature Converter",
        TemperatureConverter::update,
        TemperatureConverter::view,
    )
    .window(window::Settings {
        size: Size {
            width: 600.0,
            height: 80.0,
        },
        ..Default::default()
    })
    .run()
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
        self.celsius = String::new();
        self.fahrenheit = String::new();
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CelsiusChanged(s) => {
                if s.is_empty() {
                    self.empty();
                } else {
                    self.celsius.clone_from(&s);
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
                    self.fahrenheit.clone_from(&s);
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
            text_input("", &self.celsius).on_input(Message::CelsiusChanged),
            text(" Celsius = "),
            text_input("", &self.fahrenheit).on_input(Message::FahrenheitChanged),
            text(" Fahrenheit"),
        ]
        .padding(20)
        .align_y(Alignment::Center)
        .into()
    }
}
