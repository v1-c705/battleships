use iced::{widget::{button, column, container, row, text, Column}, Element, Length::Fill, Theme};
//use rand::{self, seq::SliceRandom};

pub fn main() -> iced::Result{
    iced::application("test", update, view)
    .theme(|_| Theme::Dracula)
    .run()
}

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
enum Actions{
    //Show,
    Label(String),
}

fn update(value: &mut String, actions: Actions){
    match actions{
        //Actions::Show => value.push_str("Show"),
        Actions::Label(label) => *value = label
    }
}

fn view(value: &String) -> Element<Actions> {
    let grid: Column<Actions> = column(
        ('a'..='j').map(|row_index| {
            row(
                (1..=10).map(|col_index| {
                    let label = format!("{}-{}", row_index, col_index);
                    button("\u{200b}")
                        .width(32)
                        .height(32)
                        .on_press(Actions::Label(label.clone()))
                        .into()
                })
            )
            .spacing(2)
            .into()
        })
    )
    .spacing(2);

    container(
        column![
            grid,
            text(value),
        ]
        .spacing(2)
        .padding(64),
    )
    .center(Fill)
    .into()
}