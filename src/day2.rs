use std::fs;
use std::path::Path;

pub fn day2(){
    let data = fs::read_to_string(Path::new("../resources/day2_data")).expect("could not open file");
    compute_and_print_part1(&data);
    compute_and_print_part2(&data);
}

fn compute_and_print_part1(data: &String) {
    let (horizontal, depth) = steer_ship(&data);
    let result = horizontal * depth;
    println!("day 2 , 1 : {}", result);
}


fn compute_and_print_part2(data: &String) {
    let (horizontal, depth) = steer_ship_with_aim(&data);
    let result = horizontal * depth;
    println!("day 2 , 2 : {}", result);
}

///reads direction from string and steers ship to new position from 0,0
fn steer_ship(input: &String) -> (i32 , i32) {
    //initialisation
    let mut horizontal = 0;
    let mut depth = 0;
    for line in input.lines().into_iter(){
        //parse inputs
        let command : Vec<&str> = line.split(" ").collect();
        let direction = *command.get(0).unwrap();
        let speed : i32 = command.get(1).unwrap().parse().expect("could not parse number");
        //steer, according to inputs
        match direction {
            "forward" => {horizontal = horizontal + speed;},
            "up" => {depth = depth - speed;},
            "down" => {depth = depth + speed;},
            _ => {},
        }
    }
    (horizontal, depth)
}

///reads direction from string and steers ship to new position from 0,0
fn steer_ship_with_aim(input: &String) -> (i32, i32){
    //initialisation
    let mut horizontal = 0;
    let mut aim = 0;
    let mut depth = 0;
    for line in input.lines().into_iter(){
        //parse inputs
        let command : Vec<&str> = line.split(" ").collect();
        let direction = *command.get(0).unwrap();
        let speed : i32 = command.get(1).unwrap().parse().expect("could not parse number");
        //steer ship according to inputs
        match direction {
            "forward" => {horizontal = horizontal + speed;
                            depth = depth + (aim * speed)},
            "up" => {aim = aim - speed;},
            "down" => {aim = aim + speed;},
            _ => {},
        }
    }
    (horizontal, depth)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use crate::day2::{steer_ship, steer_ship_with_aim};

    #[test]
    fn test_control(){
        let data = fs::read_to_string(Path::new("resources/day2_test_data")).expect("could not open file");
        let(horizontal, depth)= steer_ship(&data);
        assert_eq!(horizontal, 15);
        assert_eq!(depth, 10);
        assert_eq!(horizontal  * depth, 150);
    }
    #[test]
    fn test_control_with_aim(){
        let data = fs::read_to_string(Path::new("resources/day2_test_data")).expect("could not open file");
        let(horizontal, depth)= steer_ship_with_aim(&data);
        assert_eq!(horizontal, 15);
        assert_eq!(depth, 60);
        assert_eq!(horizontal  * depth, 900);
    }

}