use std::collections::HashSet;
use std::fmt;
use std::fs;

use serde::{Serialize, Deserialize};


struct Board {
    arr: [[HashSet<u8>; 10]; 10]
}

impl Default for Board {
    fn default() -> Board {
        let mut array: [[HashSet<u8>; 10]; 10] = Default::default();

        for row in array.iter_mut() {
            for i in row.iter_mut() {
                *i = (1..=9).collect();
            }
        }

        return Board {arr: array};
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




fn main() {
    let mut board: Board = Default::default();

    let data = fs::read_to_string("./src/boards.json")
        .expect("Unable to read file");

    let json: serde_json::Value = serde_json::from_str(data.as_str())
        .expect("JSON was not well-formatted");

    println!("{}", board);
    println!("{}", json["easy"]);
    






}