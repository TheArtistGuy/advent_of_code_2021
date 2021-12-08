use std::fs;
use std::path::Path;

pub(crate) fn day3(){
    let data = fs::read_to_string(Path::new("resources/day3_data")).expect("could not open file");
    println!("day 3 , 1 : {}", diagnostic(&data));
    println!("day 3 , 2 : {}", diag_2(data));
}

///solves part1
fn diagnostic(data: &String) -> i32 {
    let (mut count, mut v) = parse_lines(data);
    let (gamma_bin, epsilon_bin) = determinate_gamma_and_epsilon(&mut count, &mut v);
    let gamma = bin_to_dec(gamma_bin);
    let epsilon = bin_to_dec(epsilon_bin);
    gamma * epsilon
}

///solves part 2
fn diag_2 (data: String) -> i32 {
    let mut data_int:Vec<Vec<i32>> = vec![];
    parse_to_matrix(data, &mut data_int);
    let length = data_int.get(0).unwrap().len();
    let oxygen  = determine_oxygen(&data_int, &length);
    let co2 = determine_co2(&data_int, &length);

    oxygen * co2
}

fn parse_to_matrix(data: String, data_int: &mut Vec<Vec<i32>>) {
    for line in data.lines().into_iter() {
        let mut v = Vec::new();
        for c in line.chars().into_iter() {
            let x = match c {
                '1' => 1,
                _ => 0
            };
            v.push(x);
        }
        data_int.push(v);
    }
}

///determines oxygen by filtering the matrix by maximum argument
fn determine_oxygen(data_int: &Vec<Vec<i32>>, length: &usize) -> i32 {
    let mut d = data_int.clone();
    for i in 0..*length {
        let count = d.len();
        let mut det = 0;
        for x in &d { if x.get(i).unwrap() == &1 { det = det + 1 } };
        let max = match (det as f32 / count as f32) < 0.5 {
            true => 0,
            _ => 1
        };
        let mut d_buffer = Vec::new();
        for x in d.into_iter() {
            if x.get(i).unwrap() == &max {
                d_buffer.push(x.clone());
            }
        }
        d = d_buffer;
        if d.len() == 1 {break}
    }
    assert_eq!(d.len(), 1);
    let result =  d.get(0).unwrap();
    bin_to_dec(Box::new(result.to_owned()))
}

///determines co2 by filtering the matrix by minimum argument
fn determine_co2(data_int: & Vec<Vec<i32>>, length: &usize) -> i32 {
    let mut d = data_int.clone();
    for i in 0..*length {
        let count = d.len();
        let mut det = 0;
        for x in &d { if x.get(i).unwrap() == &1 { det = det + 1 } };
        let min = match (det as f32 / count as f32) < 0.5 {
            true => 1,
            _ => 0
        };
        let mut d_buffer = Vec::new();
        for x in d.into_iter() {
            if x.get(i).unwrap() == &min {
                d_buffer.push(x.clone());
            }
        }
        d = d_buffer;
        if d.len() == 1 {break};
    }
    assert_eq!(d.len(), 1);
    let result =  d.get(0).unwrap();
    bin_to_dec(Box::new(result.to_owned()))
}


///Converts vec of 0/1 to a decimal number
fn bin_to_dec(res_bin_in: Box<Vec<i32>>) -> i32 {
    let res_bin = *res_bin_in;
    let mut result = 0;
    for i in 0..res_bin.len() {
        result = result + (res_bin[i] << (res_bin.len() - 1 - i));
    }
    result
}


fn parse_lines(data: &String) -> (i32, Box<Vec<i32>>) {
    let mut count = 0;
    let mut v = Vec::new();
    for (k, line) in data.lines().into_iter().enumerate() {
        if k== 0 {
            for _ in line.chars().into_iter() {
                v.push(0)
            }
        }
        for  (i,c) in line.chars().into_iter().enumerate() {
            let x = match c {
                '1' => 1,
                _ => 0
            };
            v[i] = v[i] + x;

        }
        count = count + 1;
    }
    (count, Box::new(v))
}

fn determinate_gamma_and_epsilon(count: &mut i32, v_in: &mut Box<Vec<i32>>) -> (Box<Vec<i32>>, Box<Vec<i32>>){
    let v = &**v_in;
    let mut gamma_bin = Vec::new();
    let mut epsilon_bin = Vec::new();
    for _ in 0..v.len(){
        gamma_bin.push(0);
        epsilon_bin.push(0)
    }
    for i in 0..v.len() {
        gamma_bin[i] = if (v[i] as f32/ *count as f32) < 0.5 { 0 } else { 1 };
        epsilon_bin[i] = match gamma_bin[i] {0 => 1 , _ => 0 };
    }
    (Box::new(gamma_bin), Box::new(epsilon_bin))
}


#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use crate::day3::{determinate_gamma_and_epsilon, determine_co2, determine_oxygen, diag_2, diagnostic, parse_lines, parse_to_matrix};

    #[test]
    fn test_diagnostic(){
        let data = fs::read_to_string(Path::new("../resources/day3_test_data")).expect("could not open file");
        let (mut c, mut v)= parse_lines(&data);
        assert_eq!(determinate_gamma_and_epsilon(&mut c, &mut v), (Box::new(Vec::from([1,0,1,1,0])), Box::new(Vec::from([0,1,0,0,1]))));
        assert_eq!(diagnostic(&data), 22*9);
    }
    #[test]
    fn test_diagnostic2(){
        let data = fs::read_to_string(Path::new("../resources/day3_test_data")).expect("could not open file");
        let mut data_int:Vec<Vec<i32>> = vec![];
        parse_to_matrix(data, &mut data_int);
        let length = data_int.get(0).unwrap().len();
        let ox = determine_oxygen(&data_int, &length);
        let co2 = determine_co2(&data_int, &length);
        assert_eq!(ox, 23);
        assert_eq!(co2, 10);
        let data = fs::read_to_string(Path::new("../resources/day3_test_data")).expect("could not open file");
        assert_eq!(diag_2(data), 230);
        }

}