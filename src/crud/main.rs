use iced::widget::Column;
use iced::widget::{button, column, radio, row, text_input};
use iced::{theme, window, Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Crud::run(Settings {
        window: window::Settings {
            size: (400, 400),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Default)]
struct Crud {
    selected_name: Option<usize>,
    name: String,
    sur_name: String,
    names: Vec<String>,
    spaces: String,
}

#[derive(Clone, Debug)]
enum Message {
    FilterPrefixChanged(String),
    SelectedName(usize),
    NameChanged(String),
    SurnameChanged(String),
    CreatePressed,
    UpdatePressed,
    DeletePressed,
}

impl Sandbox for Crud {
    type Message = Message;

    fn new() -> Self {
        Self {
            selected_name: None,
            name: String::new(),
            sur_name: String::new(),
            names: Vec::new(),
            spaces: " ".repeat(62),
        }
    }

    fn title(&self) -> String {
        String::from("CRUD")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::FilterPrefixChanged(_prefix) => {}
            Message::SelectedName(name) => {
                self.selected_name = Some(name);
            }
            Message::NameChanged(name) => {
                self.name = name;
            }
            Message::SurnameChanged(name) => {
                self.sur_name = name;
            }
            Message::CreatePressed => {
                if !self.sur_name.is_empty() && !self.name.is_empty() {
                    self.names.push(format!("{}, {}", self.sur_name, self.name))
                }
            }
            Message::UpdatePressed => {
                if !self.sur_name.is_empty() && !self.name.is_empty() {
                    if let Some(name) = self.selected_name {
                        self.names[name] = format!("{}, {}", self.sur_name, self.name);
                    }
                }
            }
            Message::DeletePressed => {
                if let Some(name) = self.selected_name {
                    self.names.remove(name);
                }
                self.selected_name = None;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let spaces_ref: &str = &self.spaces;
        let filter_prefix = row![
            "Filter prefix: ",
            text_input("", &self.name, Message::FilterPrefixChanged),
            spaces_ref,
        ]
        .padding(10)
        .align_items(Alignment::Start);

        let mut names_col = Vec::new();
        for (i, names) in self.names.iter().enumerate() {
            names_col.push(radio(names, i, self.selected_name, Message::SelectedName).into());
        }
        let names_col = Column::with_children(names_col).padding(10).spacing(10);

        let name = row!["Name", text_input("", &self.name, Message::NameChanged),];
        let surname = row![
            "Surname",
            text_input("", &self.sur_name, Message::SurnameChanged),
        ];
        let enter_name = column![name, surname].padding(10).spacing(10);

        let names_box = row![names_col, enter_name,];

        let create = button("Create").on_press(Message::CreatePressed);
        let create = if self.sur_name.is_empty() || self.name.is_empty() {
            create.style(theme::Button::Destructive)
        } else {
            create
        };

        let update = button("Update").on_press(Message::UpdatePressed);
        let update =
            if self.sur_name.is_empty() || self.name.is_empty() || self.selected_name.is_none() {
                update.style(theme::Button::Destructive)
            } else {
                update
            };

        let delete = button("Delete").on_press(Message::DeletePressed);
        let delete = if self.selected_name.is_none() {
            delete.style(theme::Button::Destructive)
        } else {
            delete
        };

        let buttons = row![create, update, delete,]
            .padding(10)
            .spacing(10)
            .align_items(Alignment::Start);

        column![filter_prefix, names_box, buttons]
            .align_items(Alignment::Start)
            .into()
    }
}
