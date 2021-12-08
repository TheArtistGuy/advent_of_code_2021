use std::fs;
use std::path::Path;

pub fn day8(){
    let data = fs::read_to_string(Path::new("resources/day8_data")).expect("could not open file");
    println!("day 8 , 1 : {}", part1(&data));
    println!("day 8 , 2 : {}", part2(&data));
}

fn part1(input: &String) -> i32 {
    let mut result = 0;
    for entry in input.lines().into_iter() {
        let temp : Vec<&str>=  entry.split("|").collect();
        let wires = temp[0];
        let output = temp[1];
        for x in output.split(" "){
            if x.len() == 2 {result = result +1}; // Number : 1
            if x.len() == 4 {result = result +1}; // NUmber : 4
            if x.len() == 3 {result = result +1}; // Number : 7
            if x.len() == 7 {result = result +1}; // Number : 8
        }
    }
    result
}


fn part2(input: &String) -> i32 {
    let mut result = 0;
    for entry in input.lines().into_iter() {
        let (wires, output) = split_wires_and_output(entry);
        let numbers = determine_wiring_of_numbers(wires);
        let value = determine_displayed_number(output, numbers);
        result = result + value;
        }
    result
}

fn split_wires_and_output(entry: &str) -> (Vec<&str>, Vec<&str>) {
    let temp: Vec<&str> = entry.split("|").collect();
    let wires: Vec<&str> = temp[0].split(" ").collect();
    let output: Vec<&str> = temp[1].split(" ").collect();
    (wires, output)
}

fn determine_displayed_number(output: Vec<&str>, numbers: [&str; 10]) -> i32 {
    let mut value = 0;
    for signal in output {
        if !signal.is_empty() {
            for (i, x) in numbers.iter().enumerate() {
                if contains_all_chars(&x.trim(), &signal.trim()) && contains_all_chars(&signal.trim(), &x.trim()) {
                    value = value * 10 + i as i32;
                }
            }
        }
    }
    value
}

fn determine_wiring_of_numbers(wires: Vec<&str>) -> [&str; 10] {
    let mut numbers = [""; 10];
    //round 1 determines the obvious numbers
    for x in wires.iter() {
        if x.len() == 2 { numbers[1] = x.clone() }; // Number : 1
        if x.len() == 4 { numbers[4] = x.clone() }; // Number : 4
        if x.len() == 3 { numbers[7] = x.clone() }; // Number : 7
        if x.len() == 7 { numbers[8] = x.clone() }; // Number : 8
    }
    //round 2
    for x in wires.iter() {
        if x.len() == 6 && !contains_all_chars(&x, &numbers[4]) && contains_all_chars(&x, &numbers[7]) { numbers[0] = x.clone() }; // Number : 0
        if x.len() == 5 && contains_all_chars(&x, &numbers[7]) { numbers[3] = x.clone() }; //Number: 3
        if x.len() == 6 && !contains_all_chars(&x, &numbers[7]) && !contains_all_chars(&x, &numbers[4]) { numbers[6] = x.clone() };//Number : 6
        if x.len() == 6 && contains_all_chars(&x, &numbers[4]) { numbers[9] = x.clone() }; //Number : 9
    }
    // round 3 determines 2 and 5
    for x in wires.iter() {
        if x.len() == 5

            && contains_all_chars(&numbers[8], &x)
            && !contains_all_chars(&numbers[9], &x)
        { numbers[2] = x.clone() };     //Number : 2
        if x.len() == 5
            && !contains_all_chars(&x, &numbers[4])
            && !contains_all_chars(&numbers[3], &x)
            && contains_all_chars(&numbers[9], &x)
        { numbers[5] = x.clone() };//Number : 5
    }
    numbers
}

///determines if the 1. Argument contains all characters of the 2. Argument
fn contains_all_chars(p0: &&str, p1: &&str) -> bool {
    for x in p1.chars(){
        let mut is_not_found = true;
        for y in p0.chars(){
            if x.eq(&y){is_not_found = false}
        }
        if is_not_found {return false}
    }
    true
}


#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use crate::day8::{contains_all_chars, part1, part2};

    #[test]
    fn test_8_1(){
        let data = fs::read_to_string(Path::new("../resources/day8_test_data")).expect("could not open file");
        let result : i32 = part1(&data);
        assert_eq!(result, 26);
    }
    #[test]
    fn test_8_2(){
        let data = fs::read_to_string(Path::new("../resources/day8_test_data")).expect("could not open file");
        let result : i32 = part2(&data);
        let a = "asd";
        let b = "as";
        let c = "abd";
        assert!(contains_all_chars(&a, &b));
        assert!(!contains_all_chars(&b, &a));
        assert!(!contains_all_chars(&a,&c));
        assert_eq!(result, 61229);
    }

}