use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::str::FromStr;
use std::array::IntoIter;
use std::num::IntErrorKind;
use std::path::Path;
use std::mem;
use std::slice::IterMut;
use std::slice::Iter;
use std::error::Error;

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

impl Error for CustomParseError {}

impl From<(IntErrorKind, char)> for CustomParseError {
    fn from((kind, character): (IntErrorKind, char)) -> Self {
        CustomParseError {kind, character}
    }
}


#[derive(Debug)]
struct FindBifricateCellError;

impl Error for FindBifricateCellError {}

impl fmt::Display for FindBifricateCellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find square with less than 2 options")
    }
}

fn get_rand_elem<T>(set: &HashSet<T>) -> &T {
    set.iter().next().unwrap()
}

#[derive(Debug)]
struct Cell {
    val: u8,
    x: usize,
    y: usize
}

impl Cell {
    fn new(val: u8, x: usize, y: usize) -> Self {
        Cell {val, x, y}
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
                    1 => get_rand_elem(set).to_string(),
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

impl IntoIterator for Board {
    type Item = [HashSet<u8>; 9];
    type IntoIter = IntoIter<Self::Item, 9>;
    
    fn into_iter(self) -> Self::IntoIter {
        return IntoIterator::into_iter(self.arr);
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = &'a [HashSet<u8>; 9];
    type IntoIter = Iter<'a, [HashSet<u8>; 9]>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.iter()
    }
}

impl<'a> IntoIterator for &'a mut Board {
    type Item = &'a mut [HashSet<u8>; 9];
    type IntoIter = IterMut<'a, [HashSet<u8>; 9]>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.iter_mut()
    }
}


fn invalid(board: &Board) -> bool {
    board.into_iter().flatten().any(|set| set.len() == 0)
}

fn finished(board: &Board) -> bool {
    // solved
    !board.to_string().contains('0')
}

fn transpose(board: &Board) -> Board {
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

fn transpose_mut(board: &mut Board) -> &mut Board {
    for i_u8 in 1..board.size {
        let i = usize::from(i_u8);
        for j in 0..i {
            let (x, y) = board.arr.split_at_mut(i);
            mem::swap(&mut x[j][i], &mut y[0][j]);
        }
    }
    return board;
}

fn find_known(board: &Board) -> Vec<Cell> {
    let mut ret: Vec<Cell> = Default::default();
    for (i, row) in board.into_iter().enumerate() {
        for (j, s) in row.iter().enumerate() {
            if s.len() == 1 {
                let num = get_rand_elem(s);
                ret.push(Cell::new(*num, j, i));
            }
        }
    }

    ret
}

fn simple_elim(board: &mut Board) -> bool {
    let singles = find_known(&board);
    let mut changed = false;

    //println!("Singles: {:?}", singles);

    for cell in singles {
        //println!("\n{}", board);
        //println!("row: {:?}, y: {}", board.arr[cell.y], cell.y);
        for (x, square) in &mut board.arr[cell.y].iter_mut().enumerate() {
            if x == cell.x {
                continue;
            }
            if square.remove(&cell.val) {
                changed = true;
            }

            //if square.len() == 0 {
            //    println!("square: {:?}, Val: {}, x: {}, cell.x: {}", square, cell.val, x, cell.x);
            //}
        }

        for (y, square) in &mut transpose_mut(board).arr[cell.x].iter_mut().enumerate() {
            if y == cell.y {
                continue;
            }
            if square.remove(&cell.val) {
                changed = true;
            }
        }
        transpose_mut(board);

        let modified = box_elim(board, cell);
        if modified {
            changed = true;
        }
    }

    changed
}


fn box_elim(board: &mut Board, cell: Cell) -> bool {
    let xbox = cell.x / 3;
    let ybox = cell.y / 3;

    let xstart = xbox * 3;
    let ystart = ybox * 3;

    let xend = xstart + 3;
    let yend = ystart + 3;

    let mut changed = false;

    for i in ystart..yend {
        for j in xstart..xend {
            if i == cell.y && j == cell.x {
                continue;
            }

            if board.arr[i][j].remove(&cell.val) {
                changed = true;
            }
        }
    }

    return changed;
}

fn full_elim(board: &mut Board) -> bool {
    let mut ret = false;
    loop {
        let changed = simple_elim(board);
        if !changed {
            break;
        }
        ret = true;
    }
    return ret;
}

fn find_bifrication_candidate(board: &Board) -> Result<(usize, usize), FindBifricateCellError> {
    let mut cell = (board.arr[0][0].len(), 0, 0);
    if cell.0 == 2 {
        return Ok((cell.1, cell.2));
    }

    //println!("finding cand\n{}", detailed_display(&board));

    for (i, row) in board.into_iter().enumerate() {
        for (j, square) in row.iter().enumerate() {
            println!("cell: {:?}, square len: {:?}", cell, square.len());
            if cell.0 == 1 && square.len() > 1 {
                cell = (square.len(), j, i);
            } else if cell.0 > 2 && 1 < square.len() && square.len() < cell.0 {
                println!("cell: {:?}, new cell: {:?}", cell, (square.len(), j, i));
                cell = (square.len(), j, i);

            }

            if cell.0 == 2 {
                return Ok((cell.1, cell.2));
            }

        }
    }

    if cell.0 < 2 {
        println!("panicing, cell: {:?}", cell);
        return Err(FindBifricateCellError);
    }

    return Ok((cell.1, cell.2));
}

fn bifricate(board: &mut Board) {
    let mut test_board = board.clone();

    let test_res = find_bifrication_candidate(&test_board);
    println!("Found: {:?}", test_res);

    match test_res {
        Ok(test) => {
            let set = test_board.arr[test.1][test.0].clone();

            let elem = get_rand_elem(&set);
        
            test_board.arr[test.1][test.0].remove(elem);
        
            
            println!("bifricating on {}, {}", test.0, test.1);
        
            solve(&mut test_board);
        
            if invalid(&test_board) {
                board.arr[test.1][test.0] = HashSet::from([*elem]);
            } else if finished(&test_board) {
                board.arr = test_board.arr;
            } else {
                panic!("Bifricate failed! No looping");
            }
        }
        Err(_) => {
            return;
        }
    }

}


fn solve(board: &mut Board) {
    loop {
        full_elim(board);
        if finished(board) || invalid(board) {
            return;
        }
        println!("Enter");
        bifricate(board);
        println!("Xit");
    }

}

fn sqrtceil(x: u8) -> u8 {
    let rt = (x as f64).sqrt().floor() as u8;
    if rt * rt == x {
        return rt;
    }
    return rt + 1;
}

fn detailed_display(board: &Board) -> String {
    let size_rt = sqrtceil(board.size) as usize;
    let mut s: String = "—".repeat((size_rt + 1) * usize::from(board.size) + 1) + "\n";
    for i_u8 in 0..board.size {
        let i = usize::from(i_u8);
        
        for x in 0..size_rt {
            s = s + "|";
            for j_u8 in 0..board.size {
                let j = usize::from(j_u8);
                for y in 0..size_rt {
                    let num = (size_rt*x + y + 1).to_string();
                    s = s + if board.arr[i][j].contains(&((size_rt*x + y + 1) as u8)) {&num} else {" "};
                }
                s = s + "|";
            }
            s = s + "\n";
        }
        s = s + "—".repeat((size_rt + 1) * usize::from(board.size) + 1).as_str() + "\n";
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

fn main() {

    let json = read_json("./src/boards.json");

    let easy = json["evil"].to_string();

    let mut chars = easy.chars();
    chars.next();
    chars.next_back();
    let stripped = chars.as_str();

    let mut board = Board::from_str(stripped).expect("err");

    println!("{}", board);

    solve(&mut board);

    println!("{}", board);

    println!("{}", detailed_display(&board));


    if finished(&board) {
        println!("Finished!");
    }

    if invalid(&board) {
        println!("Invalid!");
    }

}
