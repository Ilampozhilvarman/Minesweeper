#[macro_use]
extern crate rocket;

use rocket::{
    fs::{FileServer, NamedFile, relative},
    time::format_description::modifier::WeekNumberIso,
};
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
    squares: Vec<Square>,
}

impl Board {
    fn find_square(&self, row: u32, column: u32) -> Option<&Square> {
        self.squares
            .iter()
            .find(|s| s.pos.row == row && s.pos.column == column)
    }

    fn assign_all_squares_values(&mut self, percentage: u32, grid_size: u32) {
        let square_size: u32 = grid_size * grid_size;
        let mut current_pos = Position { row: 1, column: 1 };
        for x in 1..=square_size {
            for y in 1..square_size {
                SquareType::new(s_type: SquareType::Mine, pos: Position {

                });
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
    open_file(File {
        folder: "templates",
        name: "game.html",
    })
    .await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello, game])
        .mount("/static", FileServer::from(relative!("static")))
}
