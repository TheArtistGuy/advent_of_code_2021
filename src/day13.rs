use std::fs;
use std::path::Path;
use crate::mat2d::Mat2d;

pub(crate) fn day13(){
    let data = fs::read_to_string(Path::new("resources/day13_data")).expect("");
    println!("day 13 , 1 : {}", part1(&data));
    part2(&data);
}

fn part1(data :&str) -> i32 {
    let data = data.to_string();
    let (mat, instructions) = parse_input(&data);
    let mat = process_instructions(mat, &instructions[..1].to_vec());
    let points :i32 = mat.get_as_vector().into_iter().map(|x|if x == &true{1} else {0}).sum();
    points
}
fn part2(data :&str){
    let data = data.to_string();
    let (mat, instructions) = parse_input(&data);
    let mat = process_instructions(mat, &instructions);
    println!("Day 13 , 2 : ");
    print_mat(&mat);
}


fn parse_input(data: &String) ->  (Mat2d<bool>, Vec<(String, usize)>){
    let mut instructions = Vec::new();
    let mut positions:Vec<(i32,i32)> = Vec::new();
    let mut mat = Mat2d::empty();
    for line in data.lines().into_iter(){
        if line.starts_with("fold along"){
            instructions.push(parse_instruction_line( line));
        }else if !line.is_empty(){
            positions.push(parse_position_line(line));
        }
    }
    mat = initialize_matrix(&mut positions);
    (mat, instructions)
}

fn parse_position_line(line: &str) -> (i32, i32){
    let numbers: Vec<&str> = line.split(",").collect();
    let x: i32 = numbers[0].parse().expect("");
    let y: i32 = numbers[1].parse().expect("");
    (x,y)
}

fn parse_instruction_line(line: &str) -> (String, usize){
    let line: Vec<&str> = line.split(" ").collect();
    let inst: Vec<&str> = line[2].split("=").collect();

    let position = inst[0];
    let number: usize = inst[1].parse().expect("");

    (position.to_string(), number)
}

fn initialize_matrix(mut positions: &mut Vec<(i32, i32)>) -> Mat2d<bool> {
    let (max_x, max_y) = determine_max_positions(&mut positions);

    let mut  mat = Mat2d {
        height: max_y,
        width: max_x,
        vector: vec![false; max_x * max_y]
    };

    for (x, y) in positions.iter() {
        mat.set_value(*x as usize, *y as usize, true);
    }
    mat

}

fn determine_max_positions(positions: &mut Vec<(i32, i32)>) -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y) in positions.iter() {
        if max_x < *x { max_x = x.clone() }
        if max_y < *y { max_y = y.clone() }
    }
    ((max_x +1)as usize, (max_y +1)as usize)
}

fn process_instructions(matrix: Mat2d<bool>, instructions: &Vec<(String, usize)>) -> Mat2d<bool> {
    let mut mat = matrix;
    for (instruction, position) in instructions.into_iter(){
        if instruction.eq("x"){
            let (left, right) = mat.split_vertical(*position).unwrap();
            let (_, right) = right.split_vertical(1).unwrap();
            let right = right.vertical_inverted_copy();

            if right.get_width() < left.get_width(){
                mat = fold_to_bigger(right, left);
            } else {
                mat = fold_to_bigger(left, right);
            }
        } else if instruction.eq("y"){
            let  (top, bottom) = mat.split_horizontal(*position).unwrap();
            let (_, bottom) = bottom.split_horizontal(1).unwrap();
            let bottom = bottom.horizontal_inverted_copy();
            if bottom.get_height() < top.get_height(){
                mat  = fold_to_bigger(bottom, top);
            } else{
                mat = fold_to_bigger(top, bottom);
            }
        }
    }
    mat
}

fn fold_to_bigger(smaller: Mat2d<bool>, mut bigger: Mat2d<bool>) -> Mat2d<bool> {
    for row in 0..smaller.get_height() {
        for col in 0..smaller.get_width() {
            let t = bigger.get_value(col, row).unwrap() == &true|| smaller.get_value(col, row).unwrap() == &true;
            if t {
                bigger.set_value(col, row, true);
            }
        }
    }
    bigger
}

fn print_mat(mat : &Mat2d<bool>){
    for y in 0..mat.get_height(){
        for x in 0..mat.get_width(){
            let val = if mat.get_value(x,y).unwrap() == &true{'#'} else {'.'};
            print!("{} ", val);
        }
        println!();
    }
}
#[cfg(test)]
mod test{
    use std::fs;
    use std::fs::metadata;
    use crate::day13::{parse_input, print_mat, process_instructions};
    use crate::mat2d::Mat2d;

    #[test]
    fn test_day13_1(){
        let data = fs::read_to_string("resources/day_13_testdata").expect("could not open file");
        let (mat, instructions) = parse_input(&data);
        assert_eq!(mat.get_height(), 15);
        assert_eq!(mat.get_width(), 11);
        let mat = process_instructions(mat, &instructions[..1].to_vec());
        let points :i32 = mat.get_as_vector().into_iter().map(|x|if x == &true {1} else {0}).sum();
        print_mat(&mat);
        //assert_eq!(mat.get_height(), 7);
        //assert_eq!(mat.get_width(), 5);
        println!("{}", &points);
        assert_eq!(points, 17);
    }

}