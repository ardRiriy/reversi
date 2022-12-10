#[derive(Clone, Copy)]
enum Stone {
    Empty,
    Black,
    White,
}

fn main() {
    let mut field: [[Stone; 10]; 10] = [[Stone::Empty; 10]; 10];
    field[4][4] = Stone::Black;
    field[4][5] = Stone::White;
    field[5][4] = Stone::White;
    field[5][5] = Stone::Black;

    print_field(&field);
}

fn print_field(field: &[[Stone; 10]; 10]) {
    println!("  A B C D E F G H I J");
    let mut i = 0;

    for row in field {
        print!("{}", i);
        for stone in row {
            print!(" ");
            match stone {
                Stone::Empty => print!("."),
                Stone::Black => print!("X"),
                Stone::White => print!("O"),
            }
        }
        println!("");
        i += 1;
    }
}
