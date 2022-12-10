#[derive(Clone, Copy, PartialEq)]
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

    loop {
        let mut turn_count = 1;
        }
    }
}

fn turn(field: &[[Stone; 10]; 10], stone: Stone) {
    print_field(&field);
    println!("Enter a move (ex: A1): ");
    loop {
        let (x, y) = get_input();
        if is_valid_move(&field, x, y) {
            field[y as usize][x as usize] = stone;
            break;
        } else {
            println!("Can't place stone there or invaild input. Try again!");
        }
    }
}

fn get_input() -> (u8, u8) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let x = input.chars().nth(0).unwrap() as u8 - 'A' as u8;
    let y = input.chars().nth(1).unwrap() as u8 - '0' as u8;
    (x, y)
}

fn is_valid_move(field: &[[Stone; 10]; 10], x: u8, y: u8) -> bool {
    if x > 9 || y > 9 {
        return false;
    }
    if field[y as usize][x as usize] != Stone::Empty {
        return false;
    }
    true
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
