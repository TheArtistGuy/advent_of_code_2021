use std::fs;
use std::path::Path;

pub fn day7(){
    let data = fs::read_to_string(Path::new("resources/day7_data")).expect("could not open file");
    println!("day 7 , 1 : {}", part1(&data));
    println!("day 7 , 2 : {}", part2(&data));
}

fn part1(data : &String) -> i32{
    let mut init_state: Vec<i32> = parse_input(&data);
    let mut ways = compute_ways(&mut init_state);
    let optimal = get_minimal(&mut ways);
    optimal.fuel
}


fn part2(data : &String) -> i32{
    let mut init_state: Vec<i32> = parse_input(&data);
    let mut ways = compute_ways_higher_costs(&mut init_state);
    let optimal = get_minimal(&mut ways);
    optimal.fuel
}

///Parses the input String
fn parse_input(input: &String) -> Vec<i32> {
    let mut out = Vec::new();
    for num in input.split(","){
        out.push(num.parse().expect("could not parse number"));
    }
    out
}

#[derive(Clone)]
struct Way {
    position : usize,
    fuel : i32
}

fn compute_ways(points: &mut Vec<i32>) -> Vec<Way> {
    points.sort();
    let max = points.get(points.len()-1).unwrap().clone() as usize; //get last Element which is the biggest
    let mut ways = Vec::new();
    for i in 0.. max{
        let fuel = points.iter().map(|x| (x- i as i32).abs()).sum();
        ways.push(Way { position: i, fuel});
    }
    ways
}

fn compute_ways_higher_costs(points: &mut Vec<i32>) -> Vec<Way> {
    points.sort();
    let max = points.get(points.len()-1).unwrap().clone() as usize; //get last Element which is the biggest
    let mut ways = Vec::new();
    for i in 0.. max{
        let fuel = points.iter().map(|x| {
                let (start, end) = if (x.clone() as usize) < i  {(x.clone() as usize,i)} else {(i, x.clone() as usize)};
                let mut counter = 0;
                let mut result = 0;
                for _ in start..end{
                    result = result +1 + counter;
                    counter = counter +1;
                }
                result
            }
        ).sum();
        ways.push(Way { position: i, fuel});
    }
    ways
}


fn get_minimal(ways: &mut Vec<Way>) -> Way {
    ways.sort_by(|a,b| a.fuel.partial_cmp(&b.fuel).unwrap());
    let min = ways.get(0).unwrap();
    min.clone()
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::path::Path;
    use crate::day7::{compute_ways, compute_ways_higher_costs, get_minimal, parse_input, Way};

    #[test]
    fn test_7_1() {
        let data = fs::read_to_string(Path::new("resources/day7_test_data")).expect("could not open file");
        let mut init_state: Vec<i32> = parse_input(&data);
        let mut ways = compute_ways(&mut init_state);
        let optimal = get_minimal(&mut ways);
        assert_eq!(optimal.position, 2);
        assert_eq!(optimal.fuel, 37);
    }
    fn test_7_2() {
        let data = fs::read_to_string(Path::new("resources/day7_test_data")).expect("could not open file");
        let mut init_state: Vec<i32> = parse_input(&data);
        let mut ways = compute_ways_higher_costs(&mut init_state);
        let optimal = get_minimal(&mut ways);
        assert_eq!(optimal.position, 5);
        assert_eq!(optimal.fuel, 168);
    }

}