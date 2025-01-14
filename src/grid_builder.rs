use rand::{self, seq::SliceRandom};
use crate::enums::{AnArray, Size, Orientation};


pub fn array_2d() -> AnArray{
    let mut seed = rand::thread_rng();

    let pop_chance : [i32;4]= [0,0,0,1];
    let size_chance: [Size;4] = [Size::TwoTiles, Size::ThreeTiles, Size::FourTiles, Size::FiveTiles];
    let orientation_chance: [Orientation;4] = [Orientation::HorizontalLeft, Orientation::HorizontalRight, Orientation::VerticalDown, Orientation::VerticalUp];
    
    let ref_array = [[0 as i32; 10];10];
    let mut array = AnArray{
        array: ref_array,
    };

    fn handle_orientation(mut array: AnArray, row_iter: usize, column_iter: usize, size: Size, orientation: Orientation) {
        match orientation {
            Orientation::HorizontalLeft => {
                (0..(size as usize + 1 as usize)).for_each(|iter| {
                    println!("{:?} HorLeft", row_iter);
                    if row_iter > size as usize {
                        array.array[row_iter - iter as usize][column_iter] = 1;
                    }
                });
            },
            Orientation::HorizontalRight => {
                (0..(size as usize + 1 as usize)).for_each(|iter| {
                    println!("{:?} HorRight", row_iter);
                    if row_iter + iter < 10 {
                        array.array[row_iter + iter][column_iter] = 1;
                    }
                });
            },
            Orientation::VerticalDown => {
                (0..(size as usize + 1 as usize)).for_each(|iter| {
                    println!("{:?} VertDown", row_iter);
                    if column_iter + iter < 10 {
                        array.array[row_iter][column_iter + iter] = 1;
                    }
                });
            },
            Orientation::VerticalUp => {
                (0..(size as usize + 1 as usize)).for_each(|iter| {
                    println!("{:?} VertUP", row_iter);
                    if column_iter > size as usize {
                        array.array[row_iter][column_iter - iter] = 1;
                    }
                });
            }
        }
    }

    for (row_iter, row) in ref_array.iter().enumerate(){
        for (column_iter, mut tile) in row.iter().enumerate(){
            tile = pop_chance.choose(&mut seed).unwrap();
            
            loop{
                let size = match tile {
                    0 => Size::NoTiles,
                    1 => *size_chance.choose(&mut seed).unwrap(),
                    _ => Size::NoTiles  
                };

                if size == Size::NoTiles {break}
                else {
                    match size {
                        Size::TwoTiles => loop{
                            let orientation = *orientation_chance.choose(&mut seed).unwrap();
                            handle_orientation(array.clone(), row_iter, column_iter, size, orientation);
                            break
                        },
                        Size::ThreeTiles => loop{
                            let orientation = *orientation_chance.choose(&mut seed).unwrap();
                            handle_orientation(array.clone(), row_iter, column_iter, size, orientation);
                            break
                        },
                        Size::FourTiles => loop{
                            let orientation = *orientation_chance.choose(&mut seed).unwrap();
                            handle_orientation(array.clone(), row_iter, column_iter, size, orientation);
                            break
                        },
                        Size::FiveTiles => loop{
                            let orientation = *orientation_chance.choose(&mut seed).unwrap();
                            handle_orientation(array.clone(), row_iter, column_iter, size, orientation);
                            break
                        },
                        Size::NoTiles => (),
                    }
                }
                break
            }

            
            array.array[row_iter][column_iter] = *tile;
        }
    }

    array
}

