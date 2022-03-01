use int_enum::IntEnum;
use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
enum Color {
    Red = 0,
    Orange = 1,
    Yellow = 2,
    Green = 3,
    Blue = 4,
    Purple = 5,
}

impl fmt::Display for Color {
    fn fmt(&self, c: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, c)
    }
}

#[derive(Copy, Clone, Debug)]
struct Tower {
    height: i8,
    color: Color,
    inuse: bool,
}

impl Tower {
    fn new(height: i8, color: Color, inuse: bool) -> Self {
        Tower {
            height: height,
            color: color,
            inuse: inuse,
        }
    }
}

#[derive(Clone, Copy)]
struct Position {
    row: i8,
    column: i8,
}

impl Position {
    fn new(row: i8, column: i8) -> Self {
        Position {
            row: row,
            column: column,
        }
    }
}

fn initialize_freetowers() -> Vec<Tower> {
    let mut towers: Vec<Tower> = Vec::new();

    for i in 1..7 {
        for j in 0..6 {
            towers.push(Tower::new(i, Color::from_int(j).unwrap(), false));
        }
    }
    return towers;
}

fn main() {
    let board_height = vec![
        vec![0, 3, 4, 2, 1, 5],
        vec![2, 1, 5, 0, 3, 4],
        vec![5, 4, 2, 3, 0, 1],
        vec![4, 1, 3, 0, 5, 2],
        vec![3, 5, 1, 4, 2, 0],
        vec![1, 2, 0, 5, 4, 3],
    ];

    let mut board = vec![Tower::new(1, Color::Red, false); 36];

    let mut freetowers = initialize_freetowers();

    let mut position = Position::new(0, 5);

    loop {
        if position.row == -1 || position.column == -1 {
            break;
        }

        position = evaluate_cube(&mut board, &mut freetowers, &board_height, position);

        if freetowers.len() == 0 {
            print_board(&board);
        }
    }
}

fn evaluate_cube(
    board: &mut Vec<Tower>,
    freetowers: &mut Vec<Tower>,
    board_height: &Vec<Vec<i32>>,
    p: Position,
) -> Position {
    if board[(p.row * 6 + p.column) as usize].inuse
        && board[(p.row * 6 + p.column) as usize].color == Color::Purple
    {
        board[(p.row * 6 + p.column) as usize].inuse = false;
        freetowers.push(board[(p.row * 6 + p.column) as usize]);
        return back_position(p);
    }

    let mut i = 0;

    if board[(p.row * 6 + p.column) as usize].inuse {
        i = board[(p.row * 6 + p.column) as usize].color.int_value() + 1;
    }

    while i < 6 {
        let index = find_available_tower(
            &freetowers,
            Tower::new(
                (6 - board_height[p.row as usize][p.column as usize])
                    .try_into()
                    .unwrap(),
                Color::from_int(i).unwrap(),
                false,
            ),
        );

        if index > -1 && color_is_free(&board, p, Color::from_int(i).unwrap()) {
            if board[(p.row * 6 + p.column) as usize].inuse {
                freetowers.push(board[(p.row * 6 + p.column) as usize]);
            }

            board[(p.row * 6 + p.column) as usize] = Tower::new(
                (6 - board_height[p.row as usize][p.column as usize])
                    .try_into()
                    .unwrap(),
                Color::from_int(i).unwrap(),
                true,
            );
            freetowers.remove(index.try_into().unwrap());
            return advance_position(p);
        }
        i += 1;
    }

    if board[(p.row * 6 + p.column) as usize].inuse {
        board[(p.row * 6 + p.column) as usize].inuse = false;
        freetowers.push(board[(p.row * 6 + p.column) as usize]);
    }
    return back_position(p);
}

fn print_board(board: &Vec<Tower>) {
    //    println!("{:?}", board);
    if board[3 * 6 + 1].color == Color::Yellow && board[3 * 6 + 3].color == Color::Orange {
        for i in 0..6 {
            for j in 0..6 {
                if board[i * 6 + j].inuse {
                    print!(
                        "{}",
                        board[i * 6 + j].color.to_string().chars().next().unwrap()
                    );
                    print!("{}", board[i * 6 + j].height);
                    print!("{}", ' ');
                } else {
                    print!("{}", "  ");
                }
            }
            println!();
        }
        println!();
    }
}

fn find_available_tower(freetowers: &Vec<Tower>, t: Tower) -> i32 {
    for i in 0..freetowers.len() {
        if freetowers[i].height == t.height && freetowers[i].color == t.color {
            return i.try_into().unwrap();
        }
    }
    return -1;
}

fn color_is_free(board: &Vec<Tower>, p: Position, color: Color) -> bool {
    for i in 0..6 {
        if board[(p.row * 6 + i) as usize].color == color
            && board[(p.row * 6 + i) as usize].inuse == true
        {
            return false;
        }
    }

    for i in 0..6 {
        if board[i * 6 + p.column as usize].color == color
            && board[(i * 6 + (p.column as usize)) as usize].inuse == true
        {
            return false;
        }
    }
    return true;
}

fn advance_position(p: Position) -> Position {
    if p.row == 5 && p.column == 0 {
        return back_position(p);
    }

    let column = p.column - 1;
    if column < 0 {
        return Position::new(p.row + 1, 5);
    }
    return Position::new(p.row, p.column - 1);
}

fn back_position(p: Position) -> Position {
    let column = p.column + 1;
    if column > 5 {
        return Position::new(p.row - 1, 0);
    }
    return Position::new(p.row, p.column + 1);
}
