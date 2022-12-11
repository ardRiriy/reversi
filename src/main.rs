#[derive(Clone, Copy, PartialEq)]
enum Stone {
    Empty,
    Black,
    White,
}

impl Stone {
    fn enemy(&self) -> Stone {
        match self {
            Stone::Empty => Stone::Empty,
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        }
    }
}

fn main() {
    /*
     * field[x][y]は
     * xが上下，yが左右になっているので注意
     */
    let mut field: [[Stone; 10]; 10] = [[Stone::Empty; 10]; 10];
    field[4][4] = Stone::Black;
    field[4][5] = Stone::White;
    field[5][4] = Stone::White;
    field[5][5] = Stone::Black;

    let mut turn_count = 0;
    loop {
        turn(
            &mut field,
            if turn_count % 2 == 0 {
                Stone::Black
            } else {
                Stone::White
            },
        );
        turn_count += 1;
    }
}

fn turn(field: &mut [[Stone; 10]; 10], stone: Stone) {
    print_field(&field);
    println!("Enter a move (ex: A1): ");
    loop {
        let (x, y) = get_input();
        if is_placeable(&field, x, y, stone, 0) {
            field[y as usize][x as usize] = stone;
            break;
        } else {
            println!("Can't place stone there or invaild input. Try again!");
        }
    }
}

fn get_input() -> (u32, u32) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let x = input.chars().nth(0).unwrap() as u32 - 'A' as u32;
    let y = input.chars().nth(1).unwrap() as u32 - '0' as u32;
    (x, y)
}

// TODO: もっときれいに実装する
fn is_placeable(field: &[[Stone; 10]; 10], x: u32, y: u32, stone: Stone, dir: u32) -> bool {
    let mut i = 2;
    let mut j = 2;
    if x > 9 || y > 9 {
        return false;
    }
    if field[x as usize][y as usize] != Stone::Empty {
        return false;
    }
    loop {
        match dir {
            0 => {
                if x + i > 9 || field[(x + 1) as usize][y as usize] != stone.enemy() {
                    return is_placeable(field, x, y, stone, dir + 1);
                } else if field[(x + i) as usize][y as usize] == stone {
                    return true;
                }
            }
            1 => {
                if x + i > 9
                    || y + j > 9
                    || field[(x + 1) as usize][(y + 1) as usize] != stone.enemy()
                {
                    return is_placeable(field, x, y, stone, dir + 1);
                } else if field[(x + i) as usize][(y + j) as usize] == stone {
                    return true;
                }
            }
            2 => {
                if y + j > 9 || field[x as usize][(y + 1) as usize] != stone.enemy() {
                    return is_placeable(field, x, y, stone, dir + 1);
                } else if field[x as usize][(y + j) as usize] == stone {
                    return true;
                }
            }
            3 => {
                if x < i || y + j > 9 || field[(x - 1) as usize][(y + 1) as usize] != stone.enemy()
                {
                    return is_placeable(field, x, y, stone, dir + 1);
                } else if field[(x - i) as usize][(y + j) as usize] == stone {
                    return true;
                }
            }
            4 => {
                if x < i || field[(x - 1) as usize][y as usize] != stone.enemy() {
                    return is_placeable(field, x, y, stone, dir + 1);
                } else if field[(x - i) as usize][y as usize] == stone {
                    return true;
                }
            }
            5 => {
                if x < i || y < j || field[(x - 1) as usize][(y - 1) as usize] != stone.enemy() {
                    return is_placeable(field, x, y, stone, dir + 1);
                } else if field[(x - i) as usize][(y - j) as usize] == stone {
                    return true;
                }
            }
            6 => {
                if y < j || field[x as usize][(y - 1) as usize] != stone.enemy() {
                    return is_placeable(field, x, y, stone, dir + 1);
                } else if field[x as usize][(y - j) as usize] == stone {
                    return true;
                }
            }
            7 => {
                if x + i > 9 || y < j || field[(x + 1) as usize][(y - 1) as usize] != stone.enemy()
                {
                    return is_placeable(field, x, y, stone, dir + 1);
                } else if field[(x + i) as usize][(y - j) as usize] == stone {
                    return true;
                }
            }
            _ => return false,
        }
        i += 1;
        j += 1;
    }
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
