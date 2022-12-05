use iced::executor;
use iced::widget::{button, column, progress_bar, row, slider, text};
use iced::{window, Application, Command, Element, Settings, Subscription, Theme};

use std::time::{Duration, Instant};

pub fn main() -> iced::Result {
    Timer::run(Settings {
        window: window::Settings {
            size: (200, 180),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug)]
struct Timer {
    t0: Instant,
    t1: Instant,
    time: Vec<Duration>,
    starting: bool,
    duration: Duration,
    duration_max: f32,
    elapsed_time: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Reset,
    SliderChanged(f32),
    Tick(Instant),
}

impl Application for Timer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Timer {
                t0: Instant::now(),
                t1: Instant::now(),
                time: Vec::new(),
                starting: true,
                duration: Default::default(),
                duration_max: 20.0,
                elapsed_time: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Timer")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Reset => {
                self.time.clear();
                self.t0 = Instant::now();
                self.starting = true;
            }
            Message::SliderChanged(max) => {
                self.duration_max = max;

                if self.starting && self.elapsed_time >= max {
                    self.starting = false;
                    self.time.push(self.t1 - self.t0);
                }

                if !self.starting && self.elapsed_time < max {
                    self.starting = true;
                    self.t0 = Instant::now();
                }
            }
            Message::Tick(instant) => {
                if self.starting {
                    self.t1 = instant;

                    if self.elapsed_time >= self.duration_max {
                        self.starting = false;
                        self.time.push(self.t1 - self.t0);
                    }
                }

                self.duration = self.time.iter().sum();
                if self.starting {
                    self.duration += self.t1 - self.t0;
                }

                self.elapsed_time = self.duration.as_secs_f32();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let elapsed_time = row![
            "Elapsed Time",
            progress_bar(0.0..=self.duration_max, self.elapsed_time),
        ]
        .padding(10);

        let time_seconds = row![text(format!("{:.1}s", self.elapsed_time)),].padding(10);

        let duration = row![
            "Duration",
            slider(0.0..=60.0, self.duration_max, Message::SliderChanged),
        ]
        .padding(10);

        let reset = row![button("Reset").on_press(Message::Reset),].padding(10);

        column![elapsed_time, time_seconds, duration, reset,].into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(100)).map(|_| Message::Tick(Instant::now()))
    }
}
