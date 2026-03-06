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
    position: Option<Point>,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Mouse(event, p) => {
                if event == "Left press" {
                    self.position = Some(p);
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
                position: self.position,
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
        if let Some(position) = self.position {
            let point = Point {
                x: position.x,
                y: position.y,
            };

            let path = Path::circle(point, 50.0);
            frame.fill(&path, Color::BLACK);
        }

        vec![frame.into_geometry()]
    }
}
