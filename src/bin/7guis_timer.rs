use iced::widget::{button, column, progress_bar, row, slider, text};
use iced::{window, Element, Size, Subscription};

use std::time::{Duration, Instant};

pub fn main() -> iced::Result {
    iced::application("Timer", Timer::update, Timer::view)
        .window(window::Settings {
            size: Size {
                width: 200.0,
                height: 180.0,
            },
            ..Default::default()
        })
        .subscription(Timer::run_timer)
        .run()
}

#[derive(Debug)]
struct InstantExt {
    inner: Instant,
}

impl Default for InstantExt {
    fn default() -> Self {
        InstantExt {
            inner: Instant::now(),
        }
    }
}

#[derive(Debug, Default)]
struct Timer {
    t0: InstantExt,
    t1: InstantExt,
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

impl Timer {
    fn update(&mut self, message: Message) {
        match message {
            Message::Reset => {
                self.time.clear();
                self.t0 = InstantExt::default();
                self.starting = true;
            }
            Message::SliderChanged(max) => {
                self.duration_max = max;

                if self.starting && self.elapsed_time >= max {
                    self.starting = false;
                    self.time.push(self.t1.inner - self.t0.inner);
                }

                if !self.starting && self.elapsed_time < max {
                    self.starting = true;
                    self.t0 = InstantExt::default();
                }
            }
            Message::Tick(instant) => {
                if self.starting {
                    self.t1 = InstantExt { inner: instant };

                    if self.elapsed_time >= self.duration_max {
                        self.starting = false;
                        self.time.push(self.t1.inner - self.t0.inner);
                    }
                }

                self.duration = self.time.iter().sum();
                if self.starting {
                    self.duration += self.t1.inner - self.t0.inner;
                }

                self.elapsed_time = self.duration.as_secs_f32();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let elapsed_time = row![
            "Elapsed Time: ",
            progress_bar(0.0..=self.duration_max, self.elapsed_time),
        ]
        .padding(10);

        let time_seconds = row![text(format!("{:.1}s", self.elapsed_time)),].padding(10);

        let duration = row![
            "Duration: ",
            slider(0.0001..=60.0, self.duration_max, Message::SliderChanged),
        ]
        .padding(10);

        let reset = row![button("Reset").on_press(Message::Reset),].padding(10);

        column![elapsed_time, time_seconds, duration, reset,].into()
    }

    fn run_timer(_self: &Self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(100)).map(|_| Message::Tick(Instant::now()))
    }
}
