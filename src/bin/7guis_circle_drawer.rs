use iced::{
    Color, Element, Point, Rectangle, Size, Theme,
    widget::{
        Stack,
        canvas::{Canvas, Frame, Geometry, Path, Program},
        center, container, text,
    },
};
use iced_aw::{ICED_AW_FONT_BYTES, helpers::card, style};
use sweeten::mouse_area;

/// # Errors
///
/// The application may error.
pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Circle Drawer")
        .font(ICED_AW_FONT_BYTES)
        .window_size(Size {
            width: 800.0,
            height: 800.0,
        })
        .run()
}

#[derive(Clone, Debug, Default)]
struct App {
    circles: Vec<Circle>,
    display_size: Option<Circle>,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Mouse(mouse) => match mouse.event {
                "left press" => self.circles.push(Circle {
                    center: mouse.point,
                    radius: 50.0,
                    selected: true,
                }),
                "moved" => {
                    let mut distance_1 = 1_000.0;
                    let mut index = 0;

                    for (i, circle) in self.circles.iter_mut().enumerate() {
                        let distance_2 = ((mouse.point.x - circle.center.x).powi(2)
                            + (mouse.point.y - circle.center.y).powi(2))
                        .sqrt();

                        if distance_2 < distance_1 {
                            distance_1 = distance_2;
                            index = i;
                        }

                        circle.selected = false;
                    }

                    if let Some(circle) = self.circles.get_mut(index)
                        && distance_1 < circle.radius
                    {
                        circle.selected = true;
                    }
                }
                "right press" => {
                    for circle in &self.circles {
                        if circle.selected {
                            self.display_size = Some(circle.clone());
                        }
                    }
                }
                _ => unreachable!(),
            },
            Message::CloseSize => self.display_size = None,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut stack = Stack::new();

        stack = stack.push(
            center(
                mouse_area(center("").style(container::rounded_box))
                    .on_move(|point| {
                        Message::Mouse(Mouse {
                            event: "moved",
                            point,
                        })
                    })
                    .on_press(|point| {
                        Message::Mouse(Mouse {
                            event: "left press",
                            point,
                        })
                    })
                    .on_right_press(|point| {
                        Message::Mouse(Mouse {
                            event: "right press",
                            point,
                        })
                    }),
            )
            .width(800.0)
            .height(800.0)
            .padding(10),
        );

        stack = stack.push(
            Canvas::new(App {
                circles: self.circles.clone(),
                display_size: None,
            })
            .width(800.0)
            .height(800.0),
        );

        if let Some(circle) = &self.display_size {
            stack = stack.push(
                card(
                    text!("Circle Size {}", circle.radius.round_ties_even()),
                    text!(
                        "Adjust diameter of circle at ({}, {}).",
                        circle.center.x.round_ties_even(),
                        circle.center.y.round_ties_even()
                    ),
                )
                .style(style::card::primary)
                .on_close(Message::CloseSize),
            );
        }

        stack.into()
    }
}

#[derive(Clone, Copy, Debug)]
enum Message {
    CloseSize,
    Mouse(Mouse),
}

#[derive(Clone, Copy, Debug)]
struct Mouse {
    event: &'static str,
    point: Point,
}

impl<Message> Program<Message> for App {
    type State = ();

    fn draw(
        &self,
        (): &(),
        renderer: &iced::Renderer,
        _: &Theme,
        bounds: Rectangle,
        _: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        for circle in &self.circles {
            let point = Point {
                x: circle.center.x,
                y: circle.center.y,
            };

            let path = Path::circle(point, circle.radius);

            if circle.selected {
                frame.fill(&path, Color::BLACK);
            } else {
                frame.fill(&path, Color::WHITE);
            }
        }

        vec![frame.into_geometry()]
    }
}

#[derive(Clone, Debug, Default)]
struct Circle {
    center: Point,
    radius: f32,
    selected: bool,
}
