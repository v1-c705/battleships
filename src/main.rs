fn main() {
    let mut array = [[0 as u8; 10];10];
    array[0][0] = 1;

    for (_, row) in array.iter().enumerate(){
        for (_, collumn) in row.iter().enumerate(){
            print!("{}", collumn);
        }
        println!();
    }
}
