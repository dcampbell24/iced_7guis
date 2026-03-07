use iced::{
    Color, Element, Point, Rectangle, Size, Theme,
    widget::{
        Stack,
        canvas::{Canvas, Frame, Geometry, Path, Program},
        center, container,
    },
};
use sweeten::mouse_area;

/// # Errors
///
/// The application may error.
pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Circle Drawer")
        .window_size(Size {
            width: 800.0,
            height: 800.0,
        })
        .run()
}

#[derive(Clone, Debug, Default)]
struct App {
    circles: Vec<Circle>,
}

impl App {
    fn update(&mut self, message: Message) {
        for circle in &mut self.circles {
            circle.selected = false;
        }

        match message.mouse_event {
            "left press" => self.circles.push(Circle {
                center: message.mouse_point,
                radius: 50.0,
                selected: true,
            }),
            "moved" => {
                let mut distance_1 = 1_000.0;
                let mut index = 0;

                for (i, circle) in self.circles.iter_mut().enumerate() {
                    let distance_2 = ((message.mouse_point.x - circle.center.x).powi(2)
                        + (message.mouse_point.y - circle.center.y).powi(2))
                    .sqrt();

                    if distance_2 < distance_1 {
                        distance_1 = distance_2;
                        index = i;
                    }
                }

                if let Some(circle) = self.circles.get_mut(index)
                    && distance_1 < circle.radius
                {
                    circle.selected = true;
                }
            }
            "right press" => self.circles.push(Circle {
                center: message.mouse_point,
                radius: 40.0,
                selected: true,
            }),
            _ => unreachable!(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut stack = Stack::new();

        stack = stack.push(
            center(
                mouse_area(center("").style(container::rounded_box))
                    .on_move(|mouse_point| Message {
                        mouse_event: "moved",
                        mouse_point,
                    })
                    .on_press(|mouse_point| Message {
                        mouse_event: "left press",
                        mouse_point,
                    })
                    .on_right_press(|mouse_point| Message {
                        mouse_event: "right press",
                        mouse_point,
                    }),
            )
            .width(800.0)
            .height(800.0)
            .padding(10),
        );

        stack = stack.push(
            Canvas::new(App {
                circles: self.circles.clone(),
            })
            .width(800.0)
            .height(800.0),
        );

        stack.into()
    }
}

#[derive(Clone, Copy, Debug)]
struct Message {
    mouse_event: &'static str,
    mouse_point: Point,
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
