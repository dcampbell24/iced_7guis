use iced::widget::{button, column, container, pick_list, scrollable, text_input};
use iced::window;
use iced::{Alignment, Element, Length, Sandbox, Settings};

use std::fmt;

pub fn main() -> iced::Result {
    FlightBooker::run(Settings {
        window: window::Settings {
            size: (200, 200),
            ..Default::default()
        },
        ..Default::default()
    })
}

type Date = (u32, u32, u32);

#[derive(Clone, Debug)]
struct PrettyDate {
    day: u32,
    month: u32,
    year: u32,
}

impl fmt::Display for PrettyDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}.{:02}.{:04}", self.day, self.month, self.year)
    }
}

struct FlightBooker {
    selected_flight: Option<Flight>,
    one_way_flight: String,
    one_way_flight_date: Option<Date>,
    return_flight: String,
    return_flight_date: Option<Date>,
    book: bool,
    show_dialogue: bool,
    dialogue_string: String,
}

impl Default for FlightBooker {
    fn default() -> Self {
        FlightBooker {
            selected_flight: Some(Flight::OneWay),
            one_way_flight: String::new(),
            one_way_flight_date: None,
            return_flight: String::new(),
            return_flight_date: None,
            book: false,
            show_dialogue: false,
            dialogue_string: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Book,
    FlightSelected(Flight),
    OneWayFlightChanged(String),
    ReturnFlightChanged(String),
    None(String),
}

impl FlightBooker {
    fn validate_one_way_flight(&mut self) {
        match &self.one_way_flight_date {
            Some(_) => self.book = true,
            None => self.book = false,
        }
    }

    fn validate_return_flight(&mut self) {
        match (&self.one_way_flight_date, &self.return_flight_date) {
            (Some((day_1, month_1, year_1)), Some((day_2, month_2, year_2))) => {
                if year_2 >= year_1 && month_2 >= month_1 && day_2 >= day_1 {
                    self.book = true;
                } else {
                    self.book = false;
                }
            }
            _ => self.book = false,
        }
    }

    fn validate_flight(&mut self) {
        match self.selected_flight {
            Some(Flight::OneWay) => self.validate_one_way_flight(),
            Some(Flight::Return) => self.validate_return_flight(),
            None => self.book = false,
        }
    }
}

impl Sandbox for FlightBooker {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Flight Booker".into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Book => {
                self.show_dialogue = true;

                let one_way_flight_date = match self.one_way_flight_date {
                    Some((day, month, year)) => PrettyDate { day, month, year },
                    None => PrettyDate {
                        day: 0,
                        month: 0,
                        year: 0,
                    },
                };
                let return_flight_date = match self.return_flight_date {
                    Some((day, month, year)) => PrettyDate { day, month, year },
                    None => PrettyDate {
                        day: 0,
                        month: 0,
                        year: 0,
                    },
                };

                let one_way_string =
                    format!("You have booked a one-way flight on {one_way_flight_date}");

                let return_string = format!(
                    "You have booked a flight leaving on {one_way_flight_date} and returning on {return_flight_date}"
                );

                match self.selected_flight {
                    Some(Flight::OneWay) => self.dialogue_string = one_way_string,
                    Some(Flight::Return) => self.dialogue_string = return_string,
                    None => self.dialogue_string = String::new(),
                }
            }
            Message::FlightSelected(flight) => {
                self.selected_flight = Some(flight);
                self.show_dialogue = false;
                self.validate_flight();
            }
            Message::OneWayFlightChanged(date) => {
                self.show_dialogue = false;
                self.one_way_flight_date = validate_date(&date);
                self.one_way_flight = date;
                self.validate_flight();
            }
            Message::ReturnFlightChanged(date) => {
                self.show_dialogue = false;
                self.return_flight_date = validate_date(&date);
                self.return_flight = date;
                self.validate_flight();
            }
            Message::None(_) => {}
        }
    }

    fn view(&self) -> Element<Message> {
        let pick_list = pick_list(
            &Flight::ALL[..],
            self.selected_flight,
            Message::FlightSelected,
        )
        .width(Length::Fill);

        // When there is an error, supposed to make the background red.
        // let one_way_flight = if !self.one_way_flight.is_empty() && self.one_way_flight_date.is_none() {
        let one_way_flight = text_input("", &self.one_way_flight, Message::OneWayFlightChanged);

        let return_flight = if self.selected_flight == Some(Flight::Return) {
            text_input("", &self.return_flight, Message::ReturnFlightChanged)
        } else {
            text_input(&self.one_way_flight, "", Message::None)
        };

        let book = button("                            Book").width(Length::Fill);
        let book = if self.book {
            book.on_press(Message::Book)
        } else {
            book
        };

        let content = column![
            pick_list,
            one_way_flight,
            return_flight,
            book,
            if self.book && self.show_dialogue {
                &self.dialogue_string
            } else {
                ""
            }
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Flight {
    #[default]
    OneWay,
    Return,
}

impl Flight {
    const ALL: [Flight; 2] = [Flight::OneWay, Flight::Return];
}

impl std::fmt::Display for Flight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Flight::OneWay => "one-way flight",
                Flight::Return => "return flight",
            }
        )
    }
}

fn validate_date(date: &str) -> Option<(u32, u32, u32)> {
    let date = date.split('.').collect::<Vec<_>>();
    if date.len() != 3 {
        return None;
    }

    let day = match date[0].parse::<u32>() {
        Ok(num) => {
            if num == 0 {
                return None;
            }
            num
        }
        Err(_) => return None,
    };

    let month = match date[1].parse::<u32>() {
        Ok(num) => {
            if num == 0 {
                return None;
            }
            num
        }
        Err(_) => return None,
    };

    let year = match date[2].parse::<u32>() {
        Ok(num) => {
            if num == 0 {
                return None;
            }
            num
        }
        Err(_) => return None,
    };

    let mut leap_year = year % 4 == 0;

    if year % 100 == 0 {
        leap_year = year % 400 == 0;
    }

    match month {
        1 => match day {
            1..=31 => (),
            _ => return None,
        },

        2 => {
            if leap_year {
                match day {
                    1..=29 => (),
                    _ => return None,
                }
            } else {
                match day {
                    1..=28 => (),
                    _ => return None,
                }
            }
        }

        3 => match day {
            1..=31 => (),
            _ => return None,
        },
        4 => match day {
            1..=30 => (),
            _ => return None,
        },
        5 => match day {
            1..=31 => (),
            _ => return None,
        },
        6 => match day {
            1..=30 => (),
            _ => return None,
        },
        7 => match day {
            1..=31 => (),
            _ => return None,
        },
        8 => match day {
            1..=31 => (),
            _ => return None,
        },
        9 => match day {
            1..=30 => (),
            _ => return None,
        },
        10 => match day {
            1..=31 => (),
            _ => return None,
        },
        11 => match day {
            1..=30 => (),
            _ => return None,
        },
        12 => match day {
            1..=31 => (),
            _ => return None,
        },
        _ => return None,
    }

    Some((day, month, year))
}
