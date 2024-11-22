use iced::widget::{button, column, radio, row, scrollable, text_input};
use iced::widget::{container, Column};
use iced::{window, Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Crud::run(Settings {
        window: window::Settings {
            size: (580, 280),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Default)]
struct Crud {
    filter_prefix: String,
    selected_name: Option<usize>,
    name: String,
    sur_name: String,
    names: Vec<String>,
    display_names: Vec<String>,
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
            filter_prefix: String::new(),
            selected_name: None,
            name: String::new(),
            sur_name: String::new(),
            names: Vec::new(),
            display_names: Vec::new(),
            spaces: " ".repeat(90),
        }
    }

    fn title(&self) -> String {
        String::from("CRUD")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::FilterPrefixChanged(prefix) => {
                self.filter_prefix = prefix;
                self.selected_name = None;
            }
            Message::SelectedName(index) => {
                self.selected_name = Some(index);
            }
            Message::NameChanged(name) => {
                self.name = name;
            }
            Message::SurnameChanged(name) => {
                self.sur_name = name;
            }
            Message::CreatePressed => {
                self.names.push(format!("{}, {}", self.sur_name, self.name));
                self.selected_name = None;
            }
            Message::UpdatePressed => {
                if let Some(index) = self.selected_name {
                    let name_chosen = &self.display_names[index];
                    let mut j = 0;
                    for (i, name) in self.names.iter().enumerate() {
                        if name_chosen == name {
                            j = i;
                            break;
                        }
                    }
                    self.names[j] = format!("{}, {}", self.sur_name, self.name);
                }
            }
            Message::DeletePressed => {
                if let Some(index) = self.selected_name {
                    let name_chosen = self.display_names.remove(index);
                    let mut j = 0;
                    for (i, name) in self.names.iter().enumerate() {
                        if &name_chosen == name {
                            j = i;
                            break;
                        }
                    }
                    self.names.remove(j);
                }
                self.selected_name = None;
            }
        }

        self.display_names = Vec::new();
        for name in &self.names {
            if name[..self.filter_prefix.len()] == self.filter_prefix {
                self.display_names.push(name.into());
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let spaces_ref: &str = &self.spaces;
        let filter_prefix = row![
            "Filter prefix: ",
            text_input("", &self.filter_prefix, Message::FilterPrefixChanged),
            spaces_ref,
        ]
        .padding(10)
        .align_items(Alignment::Start);

        let mut names_col = Vec::new();
        for (i, names) in self.display_names.iter().enumerate() {
            names_col.push(radio(names, i, self.selected_name, Message::SelectedName).into());
        }
        let names_col = Column::with_children(names_col).padding(10).spacing(10);
        let names_col = scrollable(names_col).height(iced::Length::Fixed(200.0));
        let names_col = container(names_col).width(iced::Length::Fixed(300.0));

        let name = row!["Name", text_input("", &self.name, Message::NameChanged),];
        let surname = row![
            "Surname",
            text_input("", &self.sur_name, Message::SurnameChanged),
        ];
        let enter_name = column![name, surname].padding(10).spacing(10);

        let names_box = row![names_col, enter_name,];

        let create = button("Create");
        let create = if self.sur_name.is_empty() || self.name.is_empty() {
            create
        } else {
            create.on_press(Message::CreatePressed)
        };

        let update = button("Update");
        let update =
            if self.sur_name.is_empty() || self.name.is_empty() || self.selected_name.is_none() {
                update
            } else {
                update.on_press(Message::UpdatePressed)
            };

        let delete = button("Delete");
        let delete = if self.selected_name.is_none() {
            delete
        } else {
            delete.on_press(Message::DeletePressed)
        };

        let buttons = row![create, update, delete]
            .padding(10)
            .spacing(10)
            .align_items(Alignment::Start);

        column![filter_prefix, names_box, buttons]
            .align_items(Alignment::Start)
            .into()
    }
}
