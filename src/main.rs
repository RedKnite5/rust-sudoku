use std::collections::HashSet;
use std::fmt;
use std::fs;
use core::str::FromStr;
use std::num::IntErrorKind;
use std::path::Path;

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


fn read_json<P: AsRef<Path>>(path: P) -> serde_json::Value {
    let data = fs::read_to_string(path)
        .expect("Unable to read file");

    let json: serde_json::Value = serde_json::from_str(data.as_str())
        .expect("JSON was not well-formatted");
    return json;
}

fn main() {

    let json = read_json("./src/boards.json");

    let easy = json["easy"].to_string();

    let mut chars = easy.chars();
    chars.next();
    chars.next_back();
    let stripped = chars.as_str();

    println!("{}", Board::from_str(stripped).expect("err"));
    






}