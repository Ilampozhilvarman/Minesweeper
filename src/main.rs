#[macro_use]
extern crate rocket;

use rand::seq::SliceRandom;
use rocket::fs::{FileServer, NamedFile, relative};
use rocket::serde::{Serialize, json::Json};
use std::path::Path;

struct File<'a> {
    folder: &'a str,
    name: &'a str,
}

async fn open_file(file: File<'_>) -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!(".")).join(file.folder).join(file.name))
        .await
        .ok()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SquareType {
    Safe(u32),
    Mine,
}

struct Position {
    row: u32,
    column: u32,
}

struct Square {
    s_type: SquareType,
    pos: Position,
}

struct Board {
    width: u32,
    height: u32,
    squares: Vec<Square>,
}

impl Board {
    fn new(width: u32, height: u32, percentage: u32) -> Self {
        let mut board = Board {
            width,
            height,
            squares: Vec::new(),
        };
        board.assign_all_squares_values(percentage);
        board
    }

    fn find_square(&self, row: u32, column: u32) -> Option<&Square> {
        if row < 1 || column < 1 || row > self.height || column > self.width {
            return None;
        }
        self.squares
            .get(((row - 1) * self.width + (column - 1)) as usize)
    }

    fn assign_all_squares_values(&mut self, percentage: u32) {
        let total = self.width * self.height;
        let mine_count = (total * percentage / 100).clamp(1, total - 1) as usize;

        let mut is_mine = vec![false; total as usize];
        for flag in is_mine.iter_mut().take(mine_count) {
            *flag = true;
        }
        is_mine.shuffle(&mut rand::rng());

        self.squares.clear();
        for row in 1..=self.height {
            for column in 1..=self.width {
                let index = ((row - 1) * self.width + (column - 1)) as usize;
                let s_type = if is_mine[index] {
                    SquareType::Mine
                } else {
                    SquareType::Safe(0)
                };
                self.squares
                    .push(Square::new(s_type, Position { row, column }));
            }
        }

        let counts: Vec<u32> = {
            let board = &*self;
            board
                .squares
                .iter()
                .map(|s| {
                    s.pos
                        .find_all(board)
                        .iter()
                        .filter(|t| matches!(t, Some(SquareType::Mine)))
                        .count() as u32
                })
                .collect()
        };

        for (square, count) in self.squares.iter_mut().zip(counts) {
            if !square.s_type.is_mine() {
                square.assign_value(SquareType::Safe(count));
            }
        }
    }
}

impl Position {
    fn type_at_offset(&self, board: &Board, d_row: i32, d_col: i32) -> Option<SquareType> {
        let row = self.row.checked_add_signed(d_row)?;
        let column = self.column.checked_add_signed(d_col)?;
        board.find_square(row, column).map(|s| s.s_type)
    }

    fn type_of_square_left(&self, board: &Board) -> Option<SquareType> {
        self.type_at_offset(board, 0, -1)
    }

    fn type_of_square_right(&self, board: &Board) -> Option<SquareType> {
        self.type_at_offset(board, 0, 1)
    }

    fn type_of_square_up(&self, board: &Board) -> Option<SquareType> {
        self.type_at_offset(board, -1, 0)
    }

    fn type_of_square_down(&self, board: &Board) -> Option<SquareType> {
        self.type_at_offset(board, 1, 0)
    }

    fn type_of_square_top_left(&self, board: &Board) -> Option<SquareType> {
        self.type_at_offset(board, -1, -1)
    }

    fn type_of_square_top_right(&self, board: &Board) -> Option<SquareType> {
        self.type_at_offset(board, -1, 1)
    }

    fn type_of_square_bottom_left(&self, board: &Board) -> Option<SquareType> {
        self.type_at_offset(board, 1, -1)
    }

    fn type_of_square_bottom_right(&self, board: &Board) -> Option<SquareType> {
        self.type_at_offset(board, 1, 1)
    }

    fn find_all(&self, board: &Board) -> [Option<SquareType>; 8] {
        [
            self.type_of_square_left(board),
            self.type_of_square_right(board),
            self.type_of_square_up(board),
            self.type_of_square_down(board),
            self.type_of_square_top_left(board),
            self.type_of_square_top_right(board),
            self.type_of_square_bottom_left(board),
            self.type_of_square_bottom_right(board),
        ]
    }
}

impl SquareType {
    fn is_mine(&self) -> bool {
        matches!(self, SquareType::Mine)
    }
}

impl Square {
    fn assign_value(&mut self, value: SquareType) {
        self.s_type = value;
    }

    fn new(s_type: SquareType, pos: Position) -> Self {
        Self { s_type, pos }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct BoardResponse {
    width: u32,
    height: u32,
    cells: Vec<i32>,
}

fn valid_settings(width: u32, height: u32, mines: u32) -> bool {
    (2..=100).contains(&width) && (2..=100).contains(&height) && (1..=100).contains(&mines)
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    open_file(File {
        folder: "templates",
        name: "index.html",
    })
    .await
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/game?<width>&<height>&<mines>")]
async fn game(width: u32, height: u32, mines: u32) -> Option<NamedFile> {
    if !valid_settings(width, height, mines) {
        return None;
    }
    open_file(File {
        folder: "templates",
        name: "game.html",
    })
    .await
}

#[get("/board?<width>&<height>&<mines>")]
fn board_data(width: u32, height: u32, mines: u32) -> Option<Json<BoardResponse>> {
    if !valid_settings(width, height, mines) {
        return None;
    }
    let board = Board::new(width, height, mines);
    let cells = board
        .squares
        .iter()
        .map(|s| match s.s_type {
            SquareType::Mine => -1,
            SquareType::Safe(n) => n as i32,
        })
        .collect();
    Some(Json(BoardResponse {
        width,
        height,
        cells,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello, game, board_data])
        .mount("/static", FileServer::from(relative!("static")))
}
