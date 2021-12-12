use std::fs;
use std::path::Path;
use crate::mat2d::Mat2d;

pub(crate) fn day11(){
    let data = fs::read_to_string(Path::new("resources/day_11_data")).expect("");
    let part1 = advance_to_generation_and_count_flashes(&mut parse_input(&data), 100);
    let part2 = find_synchronized_event(&mut parse_input(&data));
    println!("day 11 , 1 : {}", &part1);
    println!("day 11 , 2 : {}", &part2);
}


fn find_synchronized_event(world: &mut Mat2d<i32>) -> i32{
    let mut counter = 0;
    loop {
        counter = counter +1;
        let mut has_flashed = Mat2d {
            height: world.height,
            width: world.width,
            vector: vec![false; world.width * world.height]
        };

        increase_by_one(world);

        handle_flashes(world, &mut has_flashed);

        set_flashed_to_zero(world);
        let mut found = true;
        for b in has_flashed.vector.iter(){
            if !*b{
                found = false;
            }
        }
        if found {return counter}
    }
}

fn advance_to_generation_and_count_flashes(world: &mut Mat2d<i32>, generations: i32) -> i32 {
    let mut flashes = 0;
    for _ in 0..generations {
        let mut has_flashed = Mat2d {
            height: world.height,
            width: world.width,
            vector: vec![false; world.width * world.height]
        };

        increase_by_one(world);
        handle_flashes(world, &mut has_flashed);
        set_flashed_to_zero(world);

        let new_flashes : i32= has_flashed.vector.iter().map(|x| if *x == true{1} else{0}).sum();
        flashes = flashes + new_flashes;

    }
    flashes
}

fn set_flashed_to_zero(world: &mut Mat2d<i32>) {
    for x in 0..world.width {
        for y in 0..world.height {
            if world.get_value(x, y).unwrap() > &9 {
                world.set_value(x, y, 0).expect("");
            }
        }
    }
}

fn handle_flashes(world: &mut Mat2d<i32>, mut has_flashed: &mut Mat2d<bool>) {
    for x in 0..world.width {
        for y in 0..world.height {
            handle_flash(world, &mut has_flashed, x, y)
        }
    }
}

fn increase_by_one(world: &mut Mat2d<i32>) {
    for x in 0..world.width {
        for y in 0..world.height {
            world.set_value(x, y, world.get_value(x, y).unwrap() + 1 ).expect("");
        }
    }
}

fn handle_flash(mut world: &mut Mat2d<i32>, has_flashed: &mut Mat2d<bool>, x: usize, y: usize) {
    if has_flashed.get_value(x, y) == Some(&false)
        && *world.get_value(x, y).unwrap() > 9 {
        has_flashed.set_value(x, y, true);
        if x + 1 < world.width {
            flash_neighbor(world, has_flashed, x + 1, y);
            if y + 1 < world.height {flash_neighbor(world, has_flashed, x + 1, y +1);}
            if y as i32 - 1 >= 0 {flash_neighbor(world, has_flashed, x+1, (y as i32 - 1) as usize); }
        }
        if x as i32- 1 >= 0 {
            flash_neighbor(world, has_flashed, (x as i32 - 1) as usize, y);
            if y + 1 < world.height {
                flash_neighbor(world, has_flashed, (x as i32 - 1) as usize , y+1);
            }
            if y as i32 - 1 >= 0 {
                flash_neighbor(world, has_flashed, (x as i32 - 1) as usize, (y as i32 - 1) as usize);
            }
        }
        if y + 1 < world.height {
            flash_neighbor(world, has_flashed, x , y+1);
        }
        if y as i32 - 1 >= 0 {
            flash_neighbor(world, has_flashed, x, (y as i32 - 1) as usize);
        }
    }
}

fn flash_neighbor(mut world: &mut Mat2d<i32>, has_flashed: &mut Mat2d<bool>, x: usize, y: usize) {
    if has_flashed.get_value(x , y) == Some(&false) {
        world.set_value(x , y,
                        *world.get_value(x , y).unwrap() + 1)
            .expect("");
        handle_flash(world, has_flashed, x , y);
    }
}

fn parse_input(data: &String) -> Mat2d<i32> {
        let mut vector = Vec::new();
        let lines : Vec<&str> = data.lines().collect();
        let height = lines.len();
        let first_line : Vec<char> = lines[0].chars().collect();
        let width = first_line.len();

        for line in lines{
            for character in line.chars().into_iter(){
                vector.push(character.to_string().parse().unwrap());
            }
        }
        Mat2d{
            height,
            width,
            vector
        }
    }

#[cfg(test)]
mod test{
    use std::fs;
    use std::path::Path;
    use crate::day11::{advance_to_generation_and_count_flashes, find_synchronized_event, parse_input};
    use crate::mat2d::Mat2d;

    #[test]
    fn test11_1(){
        let data  = fs::read_to_string(Path::new("resources/day11_testdata"))
            .expect("could not open file");
        let mut world = parse_input(&data);
        let flashes : i32 = advance_to_generation_and_count_flashes(&mut world, 100);
        assert_eq!(flashes, 1656);
        let part2 = find_synchronized_event(&mut parse_input(&data));
        assert_eq!(part2, 195);
    }
}