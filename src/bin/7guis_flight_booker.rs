use iced::widget::{button, column, container, pick_list, scrollable, text_input};
use iced::{window, Size};
use iced::{Alignment, Element, Length};

use std::fmt;

/// # Errors
///
/// The application may error.
pub fn main() -> iced::Result {
    iced::application(
        FlightBooker::default,
        FlightBooker::update,
        FlightBooker::view,
    )
    .title("Flight Booker")
    .window(window::Settings {
        size: Size {
            width: 300.0,
            height: 250.0,
        },
        ..Default::default()
    })
    .run()
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

#[derive(Default)]
struct FlightBooker {
    selected_flight: Flight,
    one_way_flight: String,
    one_way_flight_date: Option<Date>,
    return_flight: String,
    return_flight_date: Option<Date>,
    book: bool,
    show_dialogue: bool,
    dialogue_string: String,
    red_background: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Book,
    FlightSelected(Flight),
    OneWayFlightChanged(String),
    ReturnFlightChanged(String),
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
                self.book = year_2 >= year_1 && month_2 >= month_1 && day_2 >= day_1;
            }
            _ => self.book = false,
        }
    }

    fn validate_flight(&mut self) {
        match self.selected_flight {
            Flight::OneWay => {
                self.validate_one_way_flight();
                if !self.one_way_flight.is_empty() && !self.book {
                    self.red_background = true;
                }
            }
            // Fixme: return flights are also supposed to make a red background when invalid.
            Flight::Return => self.validate_return_flight(),
        }
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
                    Flight::OneWay => self.dialogue_string = one_way_string,
                    Flight::Return => self.dialogue_string = return_string,
                }
            }
            Message::FlightSelected(flight) => {
                self.selected_flight = flight;
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
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let pick_list = pick_list(
            &Flight::ALL[..],
            Some(self.selected_flight),
            Message::FlightSelected,
        )
        .width(Length::Fill);

        let one_way_flight = text_input("choose a flight date", &self.one_way_flight)
            .on_input(Message::OneWayFlightChanged);
        if self.red_background {
            // Set the background red.
        }

        let return_flight = if self.selected_flight == Flight::Return {
            text_input("choose a flight date", &self.return_flight)
                .on_input(Message::ReturnFlightChanged)
        } else {
            text_input("", &self.one_way_flight)
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
        .align_x(Alignment::Center)
        .padding(10)
        .spacing(10);

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
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
        1 | 3 | 5 | 7 | 8 | 10 | 12 => match day {
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

        4 | 6 | 9 | 11 => match day {
            1..=30 => (),
            _ => return None,
        },
        _ => return None,
    }

    Some((day, month, year))
}
