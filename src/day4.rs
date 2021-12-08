use std::borrow::BorrowMut;
use std::fs;
use std::path::Path;

#[derive(Clone)]
struct Field{
    pub number: i32,
    pub called: bool
}
#[derive(Clone)]
struct Board{
    pub height: usize,
    pub width: usize,
    vector : Vec<Field>
}

impl Board {
    fn new(width : usize, height : usize) ->Self{
        Board{
            vector: vec![Field{
                number: 0,
                called: false
            }; (width*height) as usize],
            width,
            height
        }
    }

    fn get_field(&self, col : usize, row : usize) -> Option<&Field> {
        if col >= self.width || row >= self.height{
            return None;
        }
        self.vector.get(col + (self.width * row))
    }

    fn set_field_number(&mut self, col : usize, row : usize, value : i32) -> Result<bool, &str> {
        if col >= self.width || row >= self.height{
            return Err("No Such field in World");
        }
        self.vector.remove(col + (self.width * row));
        self.vector.insert(col + (self.width * row), Field{
            number: value,
            called: false
        });
        Ok(true)
    }

    fn set_field_called(&mut self, col : usize, row : usize, value :bool) -> Result<bool, &str> {
        if  col >= self.width || row >= self.height{
            return Err("No Such field in World");
        }
        let number = self.vector.get(col + (self.width * row)).unwrap().number;
        self.vector.remove(col + (self.width * row));
        self.vector.insert(col + (self.width * row), Field{
            number,
            called: value
        });
        Ok(true)
    }

}

pub fn day4(){
    let data = fs::read_to_string(Path::new("resources/day4_data")).expect("could not open file");
    println!("day 4 , 1 : {}", bingo(&data));
    println!("day 4 , 2 : {}", bingo_presience(&data));
}

fn bingo(data: &String) -> i32 {
    let (mut numbers, mut boards) = parse_input(data);
    let (winning_board, winning_number) = call_bingo_numbers(&mut numbers, &mut boards);
    let result = compute_result(winning_board, winning_number);
    result
}

fn bingo_presience(data: &String) -> i32 {
    let (mut numbers, mut boards) = parse_input(data);
    let (winning_board, winning_number) = call_bingo_numbers_to_last_board(&mut numbers, &mut boards);
    let result = compute_result(winning_board, winning_number);
    result
}

fn compute_result(winning_board: Option<Board>, winning_number: i32) -> i32 {
    let result = match winning_board {
        Some(x) => {
            let mut sum = 0;
            for field in x.vector.into_iter() {
                if !field.called {
                    sum = sum + field.number;
                }
            }
            sum * winning_number
        },
        _ => 0
    };
    result
}

fn parse_input(data: &String) -> (Vec<i32>, Vec<Board>) {
    let mut numbers: Vec<i32> = Vec::new();
    let mut boards = Vec::new();
    let lines: Vec<&str> = data.lines().into_iter().collect();
    for i in 0..lines.len() {
        let line = lines.get(i).unwrap();
        if i == 0 {
            for num in line.split(",") {
                numbers.push(num.parse().expect("error parsing"));
            }
        } else if i % 6 == 0 {
            //parse board
            let mut b = Board::new(5, 5);
            for j in 0..5 {
                let index = i - 4 + j;
                let l = lines.get(index ).unwrap();
                let mut counter = 0;
                assert!(!l.is_empty());
                for val in l.split(" ").into_iter() {
                    if counter < 5 && !val.is_empty() {
                        b.set_field_number(counter.clone(), j.clone(), val.parse().expect("could not parse, line")).expect("could not set field");
                        counter = counter + 1;
                    }
                }
            }
            boards.push(b);
        }
    }
    (numbers, boards)
}

fn call_bingo_numbers_to_last_board(numbers: &mut Vec<i32>, boards: &mut Vec<Board>) -> (Option<Board>, i32){
    let mut boards = boards.clone();
    let mut boards_to_remove = boards.len().clone();
    let mut to_remove:Vec<usize> = Vec::new();
    for x in numbers {
        let b = &mut boards;
        for (i, mut board) in b.into_iter().borrow_mut().enumerate() {
            let success = check_board(&mut board, &x);
            if success {
                boards_to_remove = boards_to_remove - 1;
                let is_last = boards_to_remove == 0;
                if is_last {
                    return (Some(board.clone()), x.clone())
                } else {
                    to_remove.push(i);
                }
            }
        }
        if !to_remove.is_empty() {
            // remove finished boards from list
            to_remove.sort_by(|a,b| b.partial_cmp(a).unwrap()); //sort from big to small, to prevent a element isnt there
            for i in to_remove {
            boards.remove(i);
        }
        to_remove = Vec::new();
    }
    }
    (None, 0)
}

fn call_bingo_numbers(numbers: &mut Vec<i32>, boards: &mut Vec<Board>) -> (Option<Board>, i32){
    let mut boards = boards.clone();
    for x in numbers {
        let b = &mut boards;
        for mut board in b.into_iter().borrow_mut() {
            let success = check_board(& mut board, &x);
            if success { return (Some(board.clone()),x.clone()) }
        }
    }
    (None, 0)
}

fn check_board(board: &mut Board, num: &i32) -> bool {
    let changed= check_number_on_board(board, num);
    if changed {
        if check_diagonals(board){
            return true;
        }
        if check_horizontals(board){
            return true
        }
    }
    false
}

fn check_horizontals(board: & Board) -> bool {
    for row in 0..board.width {
        let mut row_count = 0;
        for col in 0..board.height {
            if board.get_field(col, row).unwrap().called == true {
                row_count = row_count + 1;
            }
        }
            if row_count == board.height {
                return true;
            }
    }
    false
}

fn check_diagonals(board: &mut Board) -> bool{
    for col in 0..board.width {
        let mut col_count = 0;
        for row in 0..board.height {
            if board.get_field(col, row).unwrap().called == true {
                col_count = col_count + 1;
            }
            }
            if col_count == board.width {
                return true;
        }
    }
    false
}

fn check_number_on_board(board: &mut Board, num: &i32) -> bool {
    for col in 0..board.width {
        for row in 0..board.height {
            if board.get_field(col, row).unwrap().number == *num {
                board.set_field_called(col, row, true).expect("could not set field");
                return true
            }
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use crate::day4::{bingo, bingo_presience, call_bingo_numbers, call_bingo_numbers_to_last_board, compute_result, parse_input};

    #[test]
    fn day_4_testbingo() {
        let data = fs::read_to_string(Path::new("resources/day4_test_data")).expect("could not open file");
        let (mut numbers, mut boards) = parse_input(&data);
        assert_eq!(boards.len(), 3);
        assert_eq!(numbers.len(), 27);
        let (winning_board, winning_number) = call_bingo_numbers(&mut numbers, &mut boards);
        assert_eq!(winning_number, 24);
        assert_eq!(winning_board.unwrap().get_field(0,0).unwrap().number, 14);
        let (winning_board, winning_number) = call_bingo_numbers(&mut numbers, &mut boards);
        assert_eq!(compute_result(winning_board, winning_number), 4512);
        assert_eq!(bingo(&data), 4512);
    }

    #[test]
    fn day_4_testbingo_pres() {
        let data = fs::read_to_string(Path::new("resources/day4_test_data")).expect("could not open file");
        let (mut numbers, mut boards) = parse_input(&data);
        let (_winning_board, winning_number) = call_bingo_numbers_to_last_board(&mut numbers, &mut boards);
        assert_eq!(winning_number, 13);
        assert_eq!(bingo_presience(&data), 1924);
    }
}