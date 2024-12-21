use iced::{widget::{button, column, container, image, row, text, Column, Image}, Background, Border, Color, Element, Theme};
//use rand::{self, seq::SliceRandom};

pub fn main() -> iced::Result{
    iced::application("test", update, view)
    .theme(|_| Theme::Dark)
    .run()
}

struct ButtonStyle;
struct ContainerStyle;

impl ButtonStyle {
    fn button_style(_theme: &Theme, _status: button::Status) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                color: Color::WHITE,
                width: 2.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }

    fn button_image(path:&str) -> Image {
        image(path).into()
    }
}

impl ContainerStyle {
    fn container_style(_theme: &Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 5.0.into(),
            },
            ..Default::default()
        }
    }
}

// struct ButtonStyle{
//     border: Border
// }

// impl ButtonStyle {   
// }

// struct ButtonStyle;

// impl ButtonStyle {
//     fn status(&self) -> button::Style {
//         match button::Status{
//             button::Status::Active => {
//                 let a_style = button::Style::default();
//                 a_style.border.rounded(0);
//                 a_style
//             }
//         }
//         // button::Style { 
//         //     background: Some(Background::Color(Color::from_rgb(8.0, 8.0, 8.0))), 
//         //     ..Default::default()
//         // }
//     }
// }

// struct AnArray{
//     array: [[u8; 10]; 10],
// }

// fn array_2d() -> String {
//     let mut seed = rand::thread_rng();

//     let chance : [u8;3]= [0,0,1];

//     let array = [[0 as u8; 10];10];
//     let mut string_array = String::new();
//     let mut array0 = AnArray{
//         array
//     };


//     for (row_iter, row) in array.iter().enumerate(){
//         for (column_iter, mut collumn) in row.iter().enumerate(){
//             collumn = chance.choose(&mut seed).unwrap();
//             print!("{}", collumn);
//             if column_iter % 10 == 0 {
//                 string_array.push_str("\n");
//                 string_array.push_str(collumn.to_string().as_str());
//             } else {
//                 string_array.push_str(collumn.to_string().as_str());
//             }
//             array0.array[row_iter][column_iter] = *collumn;
//         }
//         println!();
//     }
//     string_array
// }

#[derive(Debug, Clone)]
enum Message{
    //Show,
    Label(String),
}

fn update(value: &mut String, message: Message){
    match message{
        //Message::Show => value.push_str("Show"),
        Message::Label(label) => *value = label
    }
}

fn view(value: &String) -> Element<Message> {
    let grid: Column<Message> = column(
        ('a'..='j').map(|row_index| {
            row(
                (1..=10).map(|col_index| {
                    let label = format!("{}-{}", row_index, col_index);
                    button(ButtonStyle::button_image("assets/water.png").content_fit(iced::ContentFit::Cover))
                    .style(ButtonStyle::button_style)
                        .width(64)
                        .height(64)
                        .padding(1)
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
        column![
            grid,
            text(value)
        ]
        .spacing(2)
        .padding(5),
    )
    .style(ContainerStyle::container_style)
    .center(iced::Length::Fill)
    .padding(0)
    .into()
}