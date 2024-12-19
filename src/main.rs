use iced::{widget::{button, column, container, row, text}, Element, Length::Fill, Theme};
use rand::{self, seq::SliceRandom};

pub fn main() -> iced::Result{
    iced::application("test", update, view)
    .theme(|_| Theme::Dracula)
    .run()
}

struct AnArray{
    array: [[u8; 10]; 10],
}

fn array_2d() -> String {
    let mut seed = rand::thread_rng();

    let chance : [u8;3]= [0,0,1];

    let array = [[0 as u8; 10];10];
    let mut string_array = String::new();
    let mut array0 = AnArray{
        array: array,
    };


    for (row_iter, row) in array.iter().enumerate(){
        for (column_iter, mut collumn) in row.iter().enumerate(){
            collumn = chance.choose(&mut seed).unwrap();
            print!("{}", collumn);
            if column_iter % 10 == 0 {
                string_array.push_str("\n");
                string_array.push_str(collumn.to_string().as_str());
            } else {
                string_array.push_str(collumn.to_string().as_str());
            }
            array0.array[row_iter][column_iter] = *collumn;
        }
        println!();
    }
    string_array
}

#[derive(Debug, Clone)]
enum Message{
    Show,
}

fn update(value: &mut String, message: Message){
    match message{
        Message::Show => value.push_str(&array_2d())
    }
}

fn view(value: &String) -> Element<Message> {
    container(
        column![
            row![
                button(text("0")).on_press(Message::Show),
                button(text("0")).on_press(Message::Show),
                button(text("0")).on_press(Message::Show),
                button(text("0")).on_press(Message::Show)
            ]
            .spacing(2),

            button(text("0")).on_press(Message::Show),
            button(text("0")).on_press(Message::Show),
            button(text("0")).on_press(Message::Show),
            button(text("0")).on_press(Message::Show),

            text(value),

        ]
        .spacing(2)
        .padding(64)
    )
    .center(Fill)
    .into()
    
}

// use iced::widget::{button, column, text, Column};

// pub fn main() -> iced::Result {
//     iced::run("A counter", update, view)
// }

// #[derive(Debug, Clone)]
// enum Message {
//     Increment,
// }

// fn update(value: &mut u64, message: Message) {
//     match message {
//         Message::Increment => *value += 1,
//     }
// }

// fn view(value: &u64) -> Column<Message> {
//     column![
//         text(value),
//         button("+").on_press(Message::Increment),
//     ]
// }