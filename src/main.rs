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
                turn_count += 1;
                Stone::Black
            } else {
                turn_count -= 1;
                Stone::White
            },
        );
    }
}

fn turn(field: &mut [[Stone; 10]; 10], stone: Stone) {
    print_field(&field);
    println!("Enter a move (ex: A1): ");
    loop {
        let (x, y) = get_input();
        if is_placeable(field, x, y, stone) {
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
fn is_placeable(field: &mut [[Stone; 10]; 10], x: u32, y: u32, stone: Stone) -> bool {
    if x > 9 || y > 9 {
        return false;
    }
    if field[y as usize][x as usize] != Stone::Empty {
        return false;
    }
    let left = left_placeable(field, x, y, stone);
    let right = right_placeable(field, x, y, stone);
    let upper = upper_placeable(field, x, y, stone);
    let lower = lower_placeable(field, x, y, stone);
    let left_upper = left_upper_placeable(field, x, y, stone);
    let right_upper = right_upper_placeable(field, x, y, stone);
    let left_lower = left_lower_placeable(field, x, y, stone);
    let right_lower = right_lower_placeable(field, x, y, stone);

    left || right || upper || lower || left_upper || right_upper || left_lower || right_lower
}

fn left_upper_placeable(field: &mut [[Stone; 10]; 10], x: u32, y: u32, stone: Stone) -> bool {
    let mut i = 2;
    loop {
        if x < i || y < i {
            return false;
        }
        if field[(y - 1) as usize][(x - 1) as usize] == stone.enemy()
            && field[(y - i) as usize][(x - i) as usize] == stone
        {
            for j in 1..i {
                field[(y - j) as usize][(x - j) as usize] = stone;
            }
            return true;
        } else if field[(y - i) as usize][(x - i) as usize] == Stone::Empty {
            return false;
        }
        i += 1;
    }
}

fn right_upper_placeable(field: &mut [[Stone; 10]; 10], x: u32, y: u32, stone: Stone) -> bool {
    let mut i = 2;
    loop {
        if x + i > 9 || y < i {
            return false;
        }
        if field[(y - 1) as usize][(x + 1) as usize] == stone.enemy()
            && field[(y - i) as usize][(x + i) as usize] == stone
        {
            for j in 1..i {
                field[(y - j) as usize][(x + j) as usize] = stone;
            }
            return true;
        } else if field[(y - i) as usize][(x + i) as usize] == Stone::Empty {
            return false;
        }
        i += 1;
    }
}

fn right_placeable(field: &mut [[Stone; 10]; 10], x: u32, y: u32, stone: Stone) -> bool {
    let mut i = 2;
    loop {
        if x + i > 9 {
            return false;
        }
        if field[y as usize][(x + 1) as usize] == stone.enemy()
            && field[y as usize][(x + i) as usize] == stone
        {
            for j in 1..i {
                field[y as usize][(x + j) as usize] = stone;
            }
            return true;
        } else if field[y as usize][(x + i) as usize] == Stone::Empty {
            return false;
        }
        i += 1;
    }
}

fn right_lower_placeable(field: &mut [[Stone; 10]; 10], x: u32, y: u32, stone: Stone) -> bool {
    let mut i = 2;
    loop {
        if x + i > 9 || y + i > 9 {
            return false;
        }
        if field[(y + 1) as usize][(x + 1) as usize] == stone.enemy()
            && field[(y + i) as usize][(x + i) as usize] == stone
        {
            for j in 1..i {
                field[(y + j) as usize][(x + j) as usize] = stone;
            }
            return true;
        } else if field[(y + i) as usize][(x + i) as usize] == Stone::Empty {
            return false;
        }
        i += 1;
    }
}

fn left_placeable(field: &mut [[Stone; 10]; 10], x: u32, y: u32, stone: Stone) -> bool {
    let mut i = 2;
    loop {
        if x < i {
            return false;
        }
        if field[y as usize][(x - 1) as usize] == stone.enemy()
            && field[y as usize][(x - i) as usize] == stone
        {
            for j in 1..i {
                field[y as usize][(x - j) as usize] = stone;
            }
            return true;
        } else if field[y as usize][(x - i) as usize] == Stone::Empty {
            return false;
        }
        i += 1;
    }
}

fn left_lower_placeable(field: &mut [[Stone; 10]; 10], x: u32, y: u32, stone: Stone) -> bool {
    let mut i = 2;
    loop {
        if x < i || y + i > 9 {
            return false;
        }
        if field[(y + 1) as usize][(x - 1) as usize] == stone.enemy()
            && field[(y + i) as usize][(x - i) as usize] == stone
        {
            for j in 1..i {
                field[(y + j) as usize][(x - j) as usize] = stone;
            }
            return true;
        } else if field[(y + i) as usize][(x - i) as usize] == Stone::Empty {
            return false;
        }
        i += 1;
    }
}

fn lower_placeable(field: &mut [[Stone; 10]; 10], x: u32, y: u32, stone: Stone) -> bool {
    let mut i = 2;
    loop {
        if y + i > 9 {
            return false;
        }
        if field[(y + 1) as usize][x as usize] == stone.enemy()
            && field[(y + i) as usize][x as usize] == stone
        {
            for j in 1..i {
                field[(y + j) as usize][x as usize] = stone;
            }
            return true;
        } else if field[(y + i) as usize][x as usize] == Stone::Empty {
            return false;
        }
        i += 1;
    }
}

fn upper_placeable(field: &mut [[Stone; 10]; 10], x: u32, y: u32, stone: Stone) -> bool {
    let mut i = 2;
    loop {
        if y < i {
            return false;
        }
        if field[(y - 1) as usize][x as usize] == stone.enemy()
            && field[(y - i) as usize][x as usize] == stone
        {
            for j in 1..i {
                field[(y - j) as usize][x as usize] = stone;
            }
            return true;
        } else if field[(y - i) as usize][x as usize] == Stone::Empty {
            return false;
        }
        i += 1;
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
