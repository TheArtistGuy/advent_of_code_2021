use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub(crate) fn day14(){
    let data = fs::read_to_string(Path::new("resources/day14_data")).expect("could not open file");
    println!("Day 14 , 1 : {}",&part1(&data));
    println!("Day 14 , 2 : {}",&part2(&data));
}

fn part1(data: &String) -> i64 {
    let (polymer, instruction_table) = parse_input(&data);
    let polymer_10: Vec<char> = to_step(&polymer, &instruction_table, 10);
    let (least_common, most_common) = determine_numbers(polymer_10);
    most_common-least_common
}

//this is brute forcing, takes a long time.
fn part2(data: &String) -> i64 {
    let (polymer, instruction_table) = parse_input(&data);
    println!("Step");
    let polymer: Vec<char> = to_step(&polymer, &instruction_table, 40);
    println!("counting");
    let (least_common, most_common) = determine_numbers(polymer);
    most_common-least_common
}

fn determine_numbers(polymer: Vec<char>) -> (i64, i64) {
    let mut map: HashMap<char, i64> = HashMap::new();
    for x in polymer.iter(){
        let exist = map.get(&x);
        match exist{
            None => {map.insert(*x, 0);}
            Some(y) => {map.insert(*x, y+1);}
        }
    }
    let mut as_vec:Vec<i64>= map.iter().map(|(x,y)| *y).collect();
    as_vec.sort();
    (*as_vec.get(0).unwrap(), *as_vec.get((as_vec.len() as i32 -1) as usize).unwrap())
}

fn parse_input(data: &String) -> (Vec<char>, Vec<(String, char)>) {
    let mut polymer = Vec::new();
    let mut instructions = Vec::new();
    for (i, line) in data.lines().into_iter().enumerate(){
        if i == 0 {
            polymer = line.chars().into_iter().collect();
        } else if !line.is_empty(){read_instruction_line(&mut instructions, line);
        }
    }
    (polymer, instructions)
}

fn read_instruction_line(instructions: &mut Vec<(String, char)>, line: &str) {
    let temp: Vec<&str> = line.split(" -> ").collect();
    let from = temp.get(0).unwrap().to_string();
    let to: Vec<char> = temp.get(1).unwrap().chars().collect();
    let to = to.get(0).unwrap().clone();
    instructions.push((from, to));
}


fn to_step(in_polymer: &Vec<char>, instructions : &Vec<(String, char)>, steps: usize) -> Vec<char> {
    let mut polymer = in_polymer.clone();
    for i in 0..steps {
        print!("{} ", &i);
        let mut counter = 0;
        while counter + 1 < polymer.len() {
            let pattern = polymer[counter..counter + 2].iter().cloned().collect::<String>();
            let actual_instruction = fetch_actual_instruction(pattern, instructions);
            match actual_instruction {
                None => {}
                Some(x) => {
                    polymer.insert(counter + 1, x);
                    counter = counter +1;}

            }
            counter = counter + 1;
        }
    }
    println!();
    polymer
}

fn fetch_actual_instruction(input: String, instructions: &Vec<(String, char)>) -> Option<char> {
    for (pattern, created) in instructions.iter(){
        if input.eq(pattern){
            return Some(created.clone());
        }
    }
    None
}


#[cfg(test)]
mod test{
    use std::fs;
    use crate::day14::{determine_numbers, parse_input, to_step};

    #[test]
    fn test_iteration(){
        let mut v = vec!['N','N','N'];
        let mut counter = 0;
        while counter +1 < v.len(){
            if v[counter..counter+2] == ['N','N']{
                v.insert(counter+1, 'A');

            }
            counter = counter +1;
        }
        let s = v.iter().cloned().collect::<String>();
        assert_eq!(s, "NANAN".to_string());
    }

    #[test]
    fn test_day14(){
        let data = fs::read_to_string("resources/day14_test_data").expect("could not open file");
        let (polymer, instruction_table) = parse_input(&data);
        let s = polymer.iter().cloned().collect::<String>();
        assert_eq!(s, "NNCB");
        let polymer_10: Vec<char> = to_step(&polymer, &instruction_table, 10);
        assert_eq!(polymer_10.len(), 3073 );
        let (least_common, most_common) = determine_numbers(polymer_10);
        assert_eq!(most_common - least_common, 1588);
        //let polymer_40 = to_step(&polymer, &instruction_table, 40);
        //let (least_common, most_common) = determine_numbers(polymer_40);
        //assert_eq!(least_common, 2192039569602);
        //assert_eq!(most_common, 3849876073);
        //assert_eq!(most_common-least_common, 2188189693529)
    }

}