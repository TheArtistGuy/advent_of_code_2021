use std::fs;
use std::path::Path;
use crate::day10::BranchResult::{Corrupted, Unfinished};
//TODO PART 2
pub fn day10(){
        let data = fs::read_to_string(Path::new("resources/day10_data"))
            .expect("could not open file");
        let (corrupt, incomplete) = result(&data);
        println!("day 10 , 1 : {}", &corrupt);
        println!("day 10 , 2 : {}", &incomplete);
}

fn result(data : &String) -> (i32, i64){
    let lines = parse_lines(&data);
    let (_, corruption, incomplete) = check_for_corruption(lines);
    let corruption_value = corruption.iter().sum();
    let incompleteness_value = compute_incompleteness(incomplete);
    (corruption_value, incompleteness_value)
}

fn parse_lines(data: &String) -> Vec<Vec<char>> {
    let mut lines = Vec::new();
    for line in data.lines().into_iter(){
        let mut c = vec!['#'];
        let mut chars : Vec<char> = line.chars().into_iter().collect();
        c.append(&mut chars);
        c.push('/');
        lines.push(c);
    }
    lines
}


fn check_for_corruption(lines: Vec<Vec<char>>) -> (Vec<Vec<char>>, Vec<i32>, Vec<Vec<char>>) {
    let mut ok_lines = Vec::new();
    let mut corruption_level = Vec::new();
    let mut incomplete_lines = Vec::new();
    for line in lines.iter(){
        match new_branch(line)  {
            BranchResult::Ok => {ok_lines.push(line.clone())}
            BranchResult::Rest(_) => {panic!("rest return from root")}
            Unfinished(_, to_complete) => {
                let x = to_complete.iter().map(|x| determine_opposing_char(x)).collect();
                incomplete_lines.push(x)}
            Corrupted(val) => {corruption_level.push(val)}
        }
    }
    (ok_lines, corruption_level, incomplete_lines)
}
fn is_opening_char(c : &char)-> bool{
    match c{
        '(' => true,
        '{' => true,
        '[' => true,
        '<' => true,
        '#' => true, //Hack to determine root
        _ => false
    }
}

fn is_closing_char(c : &char)-> bool{
    match c {
        ')' => true,
        ']' => true,
        '}' => true,
        '>' => true,
        '/' => true, //Hack to determine root
        _ => false
    }
}

enum BranchResult<'a>{
    Ok,
    Rest(&'a [char]),
    Unfinished(&'a [char], Vec<char>),
    Corrupted(i32),
}

fn new_branch<'a>(arr: &'a [char]) -> BranchResult {
    //case empty String
    if arr.is_empty() { return BranchResult::Ok }
    let first = &arr[0];
    let mut rest = &arr[1..];
    let mut inclomplete = Vec::new();
    while ! rest.is_empty(){
        let next_char = &rest[0];
        let rest_akt = &rest[1..];
        if is_closing_char(next_char)
            && *next_char == determine_opposing_char(first){
            if *first == '#' {
                //case_is_root
                    if inclomplete.is_empty(){return BranchResult::Ok;}
                    else {return BranchResult::Unfinished(rest, inclomplete)}
            } else {
                return BranchResult::Rest(rest_akt);
            }
        }
        if is_closing_char(next_char)&& *next_char != determine_opposing_char(first){
            if *next_char == '/' {
                //case_is_root
                inclomplete.push(first.clone());
                return BranchResult::Unfinished(rest, inclomplete);
            } else {
                return BranchResult::Corrupted(determine_corrupton_value(next_char));
            }
        }
        if is_opening_char(next_char){
            match new_branch(rest) {
                BranchResult::Ok => {return BranchResult::Ok}
                BranchResult::Rest(x) => {rest = x}
                Unfinished(r, mut x) => {rest = r;
                                                        inclomplete.append(& mut x);}
                Corrupted(x) => {return BranchResult::Corrupted(x)}
            }
        }
    }
    return {BranchResult::Unfinished(rest, inclomplete)}

}

fn determine_corrupton_value(c: &char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("In - determine_corruption_value- char = {}", &c)
    }
}

fn compute_incompleteness(incomplete: Vec<Vec<char>>) -> i64 {
    let mut resulting = Vec::new();
    for line in incomplete {
        let r = {
            let mut s: i64 = 0;
            for x in line {
                s = s * 5 + determine_incompleteness_value(&x) as i64;
            }
            s
        };
        resulting.push(r);
    }
    resulting.sort();
    let res = resulting.get(resulting.len() / 2).unwrap();
    res.clone()
}

fn determine_incompleteness_value(c: &char) -> i32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("In - determine_incompleteness_value- char = {}", &c)
    }
}


fn determine_opposing_char(character: &char) -> char {
   match character {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '#' => '/',
       '/' => '#',
        _ => 'e'
    }
}

#[cfg(test)]
mod test{
    use std::fs;
    use std::os::unix::raw::ino_t;
    use std::path::Path;
    use crate::day10::{check_for_corruption, compute_incompleteness, determine_incompleteness_value, parse_lines};

    #[test]
    fn test_day10_1(){
        let data = fs::read_to_string(Path::new("resources/day10_test_data"))
            .expect("could not open file");
        let lines = parse_lines(&data);
        assert_eq!(lines.len(), 10);
        for x in lines [0].iter(){
            print!("{}", x);
        }
        println!();
        let (ok_lines, corruption, incomplete) = check_for_corruption(lines);
        println!("ok : {}", &ok_lines.len());
        println!("corruption : {}", &corruption.len());
        let result : i32 = corruption.iter().sum();
        assert_eq!(result, 26397);
        let res = compute_incompleteness(incomplete);
        assert_eq!(res, 288957)
    }
}