use criterion::{black_box, criterion_group, criterion_main, Criterion};








use std::collections::HashSet;
use std::fmt;
use std::fs;
use core::str::FromStr;
use std::num::IntErrorKind;
use std::path::Path;
use std::mem;

#[derive(Debug)]
struct CustomParseError {
    kind: IntErrorKind,
    character: char,
}

impl fmt::Display for CustomParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse '{}': {:?}", self.character, self.kind)
    }
}

impl std::error::Error for CustomParseError {}

impl From<(IntErrorKind, char)> for CustomParseError {
    fn from((kind, character): (IntErrorKind, char)) -> Self {
        CustomParseError {kind, character}
    }
}


#[derive(Debug, Clone)]
struct Board {
    size: u8,
    arr: [[HashSet<u8>; 9]; 9]
}

impl Default for Board {
    fn default() -> Board {
        let mut array: [[HashSet<u8>; 9]; 9] = Default::default();

        for row in array.iter_mut() {
            for i in row.iter_mut() {
                *i = (1..=9).collect();
            }
        }

        return Board {size: 9, arr: array};
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let res = self.arr.iter().map(
            |row| row.iter().map(
                |set| match set.len() {
                    1 => set.iter().next().unwrap().to_string(),
                    _ => "0".to_string()}
                ).collect::<Vec<String>>()
                .join("") + "\n"
            ).collect::<Vec<String>>().join("");

        write!(f, "{}", res)
    }
}

impl FromStr for Board {
    type Err = CustomParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b: Board = Default::default();
        let mut line = 0;
        let mut column = 0;
        for c in s.chars() {
            if c == '0' {
                column += 1;
                continue;
            } else if c == '-' {
                line += 1;
                column = 0;
                continue;
            }

            let err: CustomParseError = (IntErrorKind::InvalidDigit, c).into();

            let digit_u32 = c.to_digit(10).ok_or(err)?;
            let digit: u8 = digit_u32 as u8;

            let set = HashSet::from([digit]);

            b.arr[line][column] = set;

            column += 1;
        }
        return Ok(b);
    }
}


fn transpose_clone(board: &Board) -> Board {
    let mut new_board: Board = Default::default();
    for i_u8 in 0..board.size {
        let i = usize::from(i_u8);
        for j_u8 in 0..board.size {
            let j = usize::from(j_u8);
            new_board.arr[i][j] = board.arr[j][i].clone();
        }
    }
    return new_board;
}



fn transpose_swap(board: &Board) -> Board {
    let mut new_board = (*board).clone();
    for i_u8 in 1..board.size {
        let i = usize::from(i_u8);
        for j in 0..i {
            let (x, y) = new_board.arr.split_at_mut(i);
            mem::swap(&mut x[j][i], &mut y[0][j]);
        }
    }
    return new_board;
}

fn detailed_display(board: &Board) -> String {
    let mut s: String = "—".repeat(4 * usize::from(board.size) + 1) + "\n";
    for i_u8 in 0..board.size {
        let i = usize::from(i_u8);
        
        for x in 0..3 {
            s = s + "|";
            for j_u8 in 0..board.size {
                let j = usize::from(j_u8);
                for y in 0..3 {
                    let num = (3*x + y + 1).to_string();
                    s = s + if board.arr[i][j].contains(&(3*x + y + 1)) {&num} else {" "};
                }
                s = s + "|";
            }
            s = s + "\n";
        }
        s = s + "—".repeat(4 * usize::from(board.size) + 1).as_str() + "\n";
    }

    s
}


fn read_json<P: AsRef<Path>>(path: P) -> serde_json::Value {
    let data = fs::read_to_string(path)
        .expect("Unable to read file");

    let json: serde_json::Value = serde_json::from_str(data.as_str())
        .expect("JSON was not well-formatted");
    return json;
}





fn transpose_swap2(board: &Board) -> Board {
    let mut new_board = (*board).clone();
    for i in 1..usize::from(board.size) {
        for j in 0..i {
            let (x, y) = new_board.arr.split_at_mut(i);
            mem::swap(&mut x[j][i], &mut y[0][j]);
        }
    }
    return new_board;
}









pub fn criterion_benchmark(c: &mut Criterion) {
    
    let json = read_json("./src/boards.json");

    let easy = json["easy"].to_string();

    let mut chars = easy.chars();
    chars.next();
    chars.next_back();
    let stripped = chars.as_str();

    let board = Board::from_str(stripped).expect("err");

    //println!("{}", board);
    //println!("{:?}", board);

    //println!("{}", detailed_display(&board));
    //println!("{}", detailed_display(&transpose(&board)));

    
    c.bench_function("transpose swap2", |x| x.iter(|| transpose_swap2(&board)));
    c.bench_function("transpose swap", |x| x.iter(|| transpose_swap(&board)));
    c.bench_function("transpose swap2b", |x| x.iter(|| transpose_swap2(&board)));
    
    c.bench_function("transpose swapb", |x| x.iter(|| transpose_swap(&board)));
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

