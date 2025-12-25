use chrono::offset::LocalResult;
use chrono::{DateTime, TimeZone, Utc};
// use iced::widget::text_input::Style;
use iced::widget::{button, column, container, pick_list, scrollable, text_input};
use iced::{window, Size};
// use iced::{color, window, Background, Size};
use iced::{Alignment, Element, Length};

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

#[derive(Default)]
struct FlightBooker {
    selected_flight: Flight,
    one_way_flight: String,
    one_way_flight_date: Option<DateTime<Utc>>,
    return_flight: String,
    return_flight_date: Option<DateTime<Utc>>,
    book: bool,
    show_dialogue: bool,
    dialogue_string: String,
    _red_background: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Book,
    FlightSelected(Flight),
    OneWayFlightChanged(String),
    ReturnFlightChanged(String),
}

impl FlightBooker {
    fn _print_flights(&self) {
        println!(
            "flight: {}, return_flight: {}",
            self.one_way_flight, self.return_flight
        );
    }

    fn validate_flights(&mut self) -> anyhow::Result<()> {
        match self.selected_flight {
            Flight::OneWay => match validate_flight(&self.one_way_flight) {
                Ok(flight) => {
                    self.one_way_flight_date = Some(flight);
                    Ok(())
                }
                Err(error) => {
                    self.one_way_flight_date = None;
                    Err(error)
                }
            },
            Flight::Return => {
                if let (Ok(flight), Ok(return_flight)) = (
                    validate_flight(&self.one_way_flight),
                    validate_flight(&self.return_flight),
                ) {
                    self.one_way_flight_date = Some(flight);
                    self.return_flight_date = Some(return_flight);

                    if self.one_way_flight_date <= self.return_flight_date {
                        Ok(())
                    } else {
                        Err(anyhow::Error::msg(
                            "the return flight date is before the flight date",
                        ))
                    }
                } else {
                    self.one_way_flight_date = None;
                    self.return_flight_date = None;
                    Err(anyhow::Error::msg("invalid date"))
                }
            }
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Book => {
                if let Err(error) = self.validate_flights() {
                    self.show_dialogue = false;
                    eprintln!("error: {error}");
                } else {
                    self.show_dialogue = true;
                    let one_way_string = format!(
                        "You have booked a one-way flight on {}",
                        self.one_way_flight
                    );

                    let return_string = format!(
                        "You have booked a flight leaving on {} and returning on {}",
                        self.one_way_flight, self.return_flight,
                    );

                    match self.selected_flight {
                        Flight::OneWay => self.dialogue_string = one_way_string,
                        Flight::Return => self.dialogue_string = return_string,
                    }
                }
            }
            Message::FlightSelected(flight) => {
                self.show_dialogue = false;
                self.selected_flight = flight;
            }
            Message::OneWayFlightChanged(date) => {
                self.show_dialogue = false;
                self.one_way_flight = date;
                self.book = self.validate_flights().is_ok();
            }
            Message::ReturnFlightChanged(date) => {
                self.show_dialogue = false;
                self.return_flight = date;
                self.book = self.validate_flights().is_ok();
            }
        }

        // self.print_flights();
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

        let return_flight = if self.selected_flight == Flight::Return {
            text_input("choose a flight date", &self.return_flight)
                .on_input(Message::ReturnFlightChanged)
        } else {
            text_input("", &self.one_way_flight)
        };

        /*
        if self.red_background {
            return_flight = return_flight.style(|_state, _theme| Style {
                background: Background::Color(color!(255, 0, 0)),
                border: Default::default(),
                icon: Default::default(),
                placeholder: Default::default(),
                value: Default::default(),
                selection: Default::default(),
            })
        } else {
            return_flight = return_flight.style(|_state, _theme| Style {
                background: Background::Color(Default::default()),
                border: Default::default(),
                icon: Default::default(),
                placeholder: Default::default(),
                value: Default::default(),
                selection: Default::default(),
            })
        }
        */

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

fn validate_flight(string: &str) -> anyhow::Result<DateTime<Utc>> {
    let mut year_month_day = string.split('.');
    let Some(day) = year_month_day.next() else {
        return Err(anyhow::Error::msg("invalid day string"));
    };
    let Some(month) = year_month_day.next() else {
        return Err(anyhow::Error::msg("invalid month string"));
    };
    let Some(year) = year_month_day.next() else {
        return Err(anyhow::Error::msg("invalid year string"));
    };

    if let LocalResult::Single(flight_date) =
        Utc.with_ymd_and_hms(year.parse()?, month.parse()?, day.parse()?, 0, 0, 0)
    {
        Ok(flight_date)
    } else {
        Err(anyhow::Error::msg("invalid time string"))
    }
}
