use iced::{widget::{button, column, container, image, row, Column, Image}, Background, Border, Color, Element, Theme};
//use rand::{self, seq::SliceRandom};

pub fn main() -> iced::Result{
    iced::application("test", update, view)
    .theme(|_| Theme::Dark)
    .run()
}

struct ButtonStyle;
struct ContainerStyle;

impl ButtonStyle {
    fn button_style(_theme: &Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                background: Some(Background::Color(Color::WHITE)),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(Color::from_rgb8(161, 161, 161))),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            },
            button::Status::Disabled => button::Style{..Default::default()},
            button::Status::Pressed => button::Style{..Default::default()},
        }
    }

    fn button_image(path:&str) -> Image {
        image(path).into()
    }
}

impl ContainerStyle {
    fn base_container_style(_theme: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
    fn main_container_style(_theme: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::WHITE)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 5.0.into(),
            },
            ..Default::default()
        }
    }
}

// fn array_2d() -> String {
//     let mut seed = rand::thread_rng();

//     let chance : [u8;3]= [0,0,1];

//     let array = [[0 as u8; 10];10];


//     for (row_iter, row) in array.iter().enumerate(){
//         for (column_iter, mut collumn) in row.iter().enumerate(){
//             collumn = chance.choose(&mut seed).unwrap();
//             print!("{}", collumn);
//             array[row_iter][column_iter] = *collumn;
//         }
//         println!();
//     }
//     string_array
// }

#[derive(Debug, Clone)]
enum Message{
    Label(String),
}

fn update(value: &mut String, message: Message){
    match message{
        Message::Label(label) => *value = label
    }
}

fn view(_value: &String) -> Element<Message> {
    let grid: Column<Message> = column(
        ('a'..='j').map(|row_index| {
            row(
                (1..=10).map(|col_index| {
                    let label = format!("{}-{}", row_index, col_index);
                    button(ButtonStyle::button_image("assets/water.png").content_fit(iced::ContentFit::Cover))
                    .style(ButtonStyle::button_style)
                        .width(64)
                        .height(64)
                        .padding(2)
                        .on_press(Message::Label(label.clone()))
                        .into()
                })
            )
            .spacing(0)         // Spacing for Rows
            .into()
        })
    )
    .spacing(0);            // Spacing for Columns

    container(
        container(
            column![
                grid
            ]
            .spacing(2)
            .padding(7),
        )
        .style(ContainerStyle::main_container_style)
        .center(iced::Length::Shrink)
        .padding(0)
    )
    .style(ContainerStyle::base_container_style)
    .center(iced::Length::Fill)
    .padding(0)
    .into()
}