use std::fs;
use std::path::Path;

pub fn day6(){
    let data = fs::read_to_string(Path::new("resources/day6_data")).expect("could not open file");
    println!("day 6 , 1 : {}", part_1(&data));
    println!("day 6 , 2 : {}", part_2(&data));
}
fn part_1(data: &String) -> u64 {
    let mut init_state: Vec<u8> = parse_input(&data);
    let bins = sort_to_bins(&init_state);
    let last_state = advance(&bins, 80);
    count_fish_in_bins(&last_state)
}
fn part_2(data: &String) -> u64 {
    let mut init_state: Vec<u8> = parse_input(&data);
    let bins = sort_to_bins(&init_state);
    let last_state = advance(&bins, 256);
    count_fish_in_bins(&last_state)
}

///Parses the input String
pub(crate) fn parse_input(input: &String) -> Vec<u8> {
    let mut out = Vec::new();
    for num in input.split(","){
       out.push(num.parse().expect("could not parse number"));
    }
    out
}
///Sorts the fish to bins according to their age
fn sort_to_bins(pond: &Vec<u8>) -> [u64;9] {
    let mut bins:[u64;9] = [0;9];
    for fish in pond{
        bins[*fish as usize] = bins[*fish as usize] +1;
    }
    bins
}

///advances the state to day : days
fn advance(bins: &[u64; 9], days: i32) -> [u64; 9] {
    let mut bins = bins.clone();
    for _ in 0..days{
        let reproducing_fish = bins[0];
        for i in 1..bins.len(){
            bins[i-1] = bins[i];
        }
        bins[6] = bins[6] + reproducing_fish;
        bins[8] = reproducing_fish;
    }
    bins
}

fn count_fish_in_bins(bins: &[u64; 9]) -> u64 {
    let sum = bins.iter().sum();
    sum
}

#[cfg(test)]
mod test{
    use std::fs;
    use std::path::Path;
    use crate::day6::{advance, count_fish_in_bins, parse_input, sort_to_bins};

    #[test]
    fn test_6_1(){
        let data = fs::read_to_string(Path::new("resources/day6_test_data")).expect("could not open file");
        let mut init_state: Vec<u8> = parse_input(&data);
        assert_eq!(init_state.len(), 5);
        let bins = sort_to_bins(&init_state);
        let last_state = advance(&bins, 80);
        assert_eq!(count_fish_in_bins(&last_state), 5934);
        let last_state = advance(&bins, 18);
        assert_eq!(count_fish_in_bins(&last_state), 26);

    }
    #[test]
    fn test_6_2() {
        let data = fs::read_to_string(Path::new("resources/day6_test_data")).expect("could not open file");
        let pond  = parse_input(&data);
        let bins = sort_to_bins(&pond);
        let bins_new = advance(&bins, 256);
        let count = count_fish_in_bins(&bins_new);
        assert_eq!(count, 26984457539);
    }

}