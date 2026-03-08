use std::sync::{Arc, Mutex};

use iced::{
    Color, Element, Point, Rectangle, Size, Theme,
    widget::{
        Column, Stack, button,
        canvas::{Canvas, Frame, Geometry, Path, Program},
        center, container, row, slider, text,
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
    circles: Vec<Arc<Mutex<Circle>>>,
    circles_undo: Vec<Arc<Mutex<Circle>>>,
    display_size: Option<Arc<Mutex<Circle>>>,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Mouse(mouse) => match mouse.event {
                "left press" => {
                    self.circles.push(Arc::new(Mutex::new(Circle {
                        center: mouse.point,
                        radius: 50.0,
                        selected: true,
                    })));

                    self.circles_undo.clear();
                }
                "moved" => {
                    let mut distance_1 = 1_000.0;
                    let mut index = 0;

                    for (i, circle) in self.circles.iter_mut().enumerate() {
                        let Ok(mut circle) = circle.lock() else {
                            return;
                        };

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
                        && let Ok(mut circle) = circle.lock()
                        && distance_1 < circle.radius
                    {
                        circle.selected = true;
                    }
                }
                "right press" => {
                    for circle_arc in &self.circles {
                        let Ok(circle) = circle_arc.lock() else {
                            return;
                        };

                        if circle.selected {
                            self.display_size = Some(circle_arc.clone());
                        }
                    }
                }
                _ => unreachable!(),
            },
            Message::CloseSize => self.display_size = None,
            Message::Redo => {
                if let Some(circle) = self.circles_undo.pop() {
                    self.circles.push(circle);
                }
            }
            Message::SizeChange(radius) => {
                if let Some(circle) = &mut self.display_size {
                    let Ok(mut circle) = circle.lock() else {
                        unreachable!();
                    };

                    circle.radius = radius;
                }
            }
            Message::Undo => {
                if let Some(circle) = self.circles.pop() {
                    self.circles_undo.push(circle);
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut stack = Stack::new();

        let mut column_1 = Column::new();

        let mut undo = button("Undo");
        if !self.circles.is_empty() {
            undo = undo.on_press(Message::Undo);
        }

        let mut redo = button("Redo");
        if !self.circles_undo.is_empty() {
            redo = redo.on_press(Message::Redo);
        }

        column_1 = column_1.push(row![undo, redo].padding(10).spacing(10));

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
            .height(800.0),
        );

        stack = stack.push(
            Canvas::new(App {
                circles: self.circles.clone(),
                circles_undo: Vec::new(),
                display_size: None,
            })
            .width(800.0)
            .height(800.0),
        );

        if let Some(circle) = &self.display_size
            && let Ok(circle) = circle.lock()
        {
            let mut column_2 = Column::new();
            column_2 = column_2.push(text!(
                "Adjust radius of circle at ({}, {}).",
                circle.center.x.round_ties_even(),
                circle.center.y.round_ties_even()
            ));
            column_2 = column_2.push(slider(10.0..=100.0, circle.radius, Message::SizeChange));

            stack = stack.push(
                card(
                    text!("Circle Radius {}", circle.radius.round_ties_even()),
                    column_2,
                )
                .style(style::card::primary)
                .on_close(Message::CloseSize),
            );
        }

        column_1.push(stack).into()
    }
}

#[derive(Clone, Copy, Debug)]
enum Message {
    CloseSize,
    Mouse(Mouse),
    Redo,
    SizeChange(f32),
    Undo,
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
            let Ok(circle) = circle.lock() else {
                continue;
            };

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
