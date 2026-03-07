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
        match message {
            Message::Mouse(event, point) => {
                if event == "Left press" {
                    self.circles.push(Circle {
                        center: point,
                        closest: false,
                    });
                }

                let mut distance_1 = 1_000.0;
                let mut index = 0;

                for (i, circle) in self.circles.iter_mut().enumerate() {
                    let distance_2 = ((point.x - circle.center.x).powi(2)
                        + (point.y - circle.center.y).powi(2))
                    .sqrt();

                    if distance_2 < distance_1 {
                        distance_1 = distance_2;
                        index = i;
                    }

                    circle.closest = false;
                }

                if let Some(circle) = self.circles.get_mut(index) {
                    circle.closest = true;
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut stack = Stack::new();

        stack = stack.push(
            center(
                mouse_area(center("").style(container::rounded_box))
                    .on_enter(|p| Message::Mouse("Entered", p))
                    .on_exit(|p| Message::Mouse("Exited", p))
                    .on_press(|p| Message::Mouse("Left press", p)),
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
enum Message {
    Mouse(&'static str, Point),
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

            let path = Path::circle(point, 50.0);

            if circle.closest {
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
    closest: bool,
}
