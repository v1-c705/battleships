use std::{fs::OpenOptions, io::Read, sync::{LazyLock, Once}};
use toml;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use iced::{widget::{button, column, container, image, row, Column, Image}, window::Settings, Background, Border, Color, Element, Size, Theme};
use rand::{self, seq::SliceRandom};

pub fn main() -> iced::Result{
    iced::application("test", Application::update, Application::view)
    .window(Settings{icon: Some(iced::window::icon::from_file("assets/water.png").unwrap()), min_size:Some(Size::new(680.0, 680.0)), ..Default::default()})
    .theme(|_| Theme::Dracula)
    .resizable(true)
    .window_size(Size{width: 1200.0, height: 720.0})
    .run()
}

struct ButtonStyle;
struct ContainerStyle;

impl ButtonStyle {
    fn button_style(_theme: &Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => {
                button::Style {
                background: Some(Background::Color(Color::WHITE)),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }},
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(Color::from_rgb8(255, 231, 153))),
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
        // match self.status {
        //     button::Status::Active => image(path).into(),
        //     button::Status::Hovered => image(path).opacity(0.5).into(),
        //     _ => image(path).into(),
        // }

        image(path).into()
    }

    // fn status(self) -> button::Status {
    //     self.status
    // }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AnArray{
    array: [[i32; 10]; 10],
}

fn array_2d() -> AnArray{
    let mut seed = rand::thread_rng();

    let chance : [i32;4]= [0,0,0,1];
    let ref_array = [[0 as i32; 10];10];

    let mut array = AnArray{
        array: ref_array,
    };


    for (row_iter, row) in ref_array.iter().enumerate(){
        for (column_iter, mut collumn) in row.iter().enumerate(){
            collumn = chance.choose(&mut seed).unwrap();
            array.array[row_iter][column_iter] = *collumn;
        }
    }

    array
}

#[derive(Debug, Clone)]
enum Message{
    Label(String),
    Array(AnArray, usize, usize),
}

struct Application;

// #[derive(serde::Serialize)]
// struct Test{
//     first: [[i32; 2]; 2],
// }

impl Application{
    fn update(value: &mut String, message: Message){
        match message{
            Message::Label(label) => {
                *value = label;
                println!("{:?}", value);
            },
            Message::Array(mut array, rowdex, coldex) => {
                match array.array[coldex-1][rowdex-1] {
                    0 => array.array[coldex-1][rowdex-1] = 1,
                    1 => array.array[coldex-1][rowdex-1] = 0,
                    _ => ()
                }
                let toml_string = toml::to_string(&array).expect("Failed to serialize to TOML");
                let mut file = match OpenOptions::new().read(true).write(true).open("utils/bot_grid.toml") {
                    Ok(file) => {
                        println!("File opened successfully.");
                        file
                    }
                    Err(err) => {
                        // If there's an error (e.g., file not found), create the file
                        println!("Error opening file: {}. Creating a new file...", err);
                        File::create("utils/bot_grid.toml").expect("Error creating the file")
                    }
                };

                file.write_all(toml_string.as_bytes()).expect("Unable to write data");
            }
        }
    }

    

    fn view(_value: &String) -> Element<Message> {
        // let test = Test{
        //     first: [[1,2],[3,4]]
        // };

        fn update_grid(array: &AnArray){
            let toml_string = toml::to_string(&array).expect("Failed to serialize to TOML");
            let mut file = match OpenOptions::new().read(true).write(true).open("utils/bot_grid.toml") {
                Ok(file) => {
                    println!("File opened successfully.");
                    file
                }
                Err(err) => {
                    // If there's an error (e.g., file not found), create the file
                    println!("Error opening file: {}. Creating a new file...", err);
                    File::create("utils/bot_grid.toml").expect("Error creating the file")
                }
            };

            file.write_all(toml_string.as_bytes()).expect("Unable to write data");
        }

        fn fetch_array()-> std::io::Result<AnArray>{
            let mut file = File::open("utils/bot_grid.toml").expect("error");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("Error while reading file content");

            let array:AnArray = toml::de::from_str(&content).expect("Error while converting file content to specified type AnArray");
            Ok(array)
        }

        static ARRAY: LazyLock<AnArray> = LazyLock::new(|| array_2d());
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {update_grid(&*ARRAY);});

        let array = fetch_array().unwrap();
        

        let grid: Column<Message> = column(
            (1..=10).map(|row_index| {
                row(
                    (1..=10).map(|col_index| {
                        let tile = match array.array[row_index-1][col_index-1]{
                            0 => {
                                button(ButtonStyle::button_image("assets/water.png").content_fit(iced::ContentFit::Cover))
                            },
                            1 => {
                                button(ButtonStyle::button_image("assets/ahegao_button.png").content_fit(iced::ContentFit::Cover))
                            },
                            _ => {
                                button(ButtonStyle::button_image("assets/water.png").content_fit(iced::ContentFit::Cover))
                            }
                        };
                        let label = format!("{}-{}", row_index, col_index);
                        tile.style(ButtonStyle::button_style)
                            .width(64)
                            .height(64)
                            .padding(2)
                            .on_press(Message::Label(label.clone()))
                            .on_press(Message::Array(array.clone(), col_index, row_index))
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
                .spacing(0)
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
}

