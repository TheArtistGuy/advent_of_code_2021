use std::fs;
use std::path::Path;

pub fn day1(){
    let input1 = fs::read_to_string(Path::new("resources/day1_data")).unwrap();
    let result1 = sonar_sweep(&input1);
    let result2 = sonar_sweep_summed(&input1);
    println!("Result day 1 part 1: {}", result1);
    println!("Result day 1 part 2: {}", result2);
}

///solution to the first part
fn sonar_sweep(input_string : &String) -> u32 {
    let mut input = Vec::new();
    parse_input(input_string, &mut input);
    let result = sweep(&input);
    result
}

///checks how many numbers in the vector are bigger than the previous one
fn sweep(input: &Vec<i32>) -> u32 {
    let mut result : u32 = 0;
    let mut last = &input.get(0).unwrap().clone();
    for i in input.iter() {
        if i > last {
            result = result + 1;
        }
        last = i;
    }
    result
}

///solution to the second part
fn sonar_sweep_summed(input_string: &String) -> u32 {
    let mut input = Vec::new();
    parse_input(input_string, &mut input);
    let summed_inputs = sum_inputs(&mut input);
    let result = sweep(&summed_inputs);
    result
}

///sums a object in the vector with the following 2
fn sum_inputs(input: &mut Vec<i32>) -> Vec<i32> {
    let mut summed_inputs = Vec::new();
    for i in 0..input.len() - 2 {
        let sum = input.get(i).unwrap() + input.get(i + 1).unwrap() + input.get(i + 2).unwrap();
        summed_inputs.push(sum);
    }
    summed_inputs
}

fn parse_input(input_string: &String, input: &mut Vec<i32>) {
    for x in input_string.lines().into_iter() {
        let num = x.parse::<i32>().unwrap();
        input.push(num);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use crate::day1::{parse_input, sonar_sweep, sonar_sweep_summed};

    # [test]
    fn test_sonar_sweep(){
        let input = fs::read_to_string(Path::new("resources/day1_test_data")).expect("could not open file");
        assert_eq!(sonar_sweep(&input), 7);
    }
    #[test]
    fn test_parse_input(){
        let input = fs::read_to_string(Path::new("resources/day1_test_data")).expect("could not open file");
        let mut control_vector = Vec::new();
        parse_input(&input, &mut control_vector);
        assert_eq!(control_vector.len(), 10);
    }
    #[test]
    fn test_sonar_sweep_summed(){
        let input = fs::read_to_string(Path::new("resources/day1_test_data")).expect("could not open file");
        assert_eq!(sonar_sweep_summed(&input), 5);
    }

}