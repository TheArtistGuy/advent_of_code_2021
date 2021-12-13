use crate::mat2d::Mat2d;

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
        height: max_y as usize,
        width: max_x as usize,
        vector: vec![false; (max_x * max_y) as usize]
    };

    for (x, y) in positions.iter() {
        mat.set_value(*x as usize, *y as usize, true);
    }
    mat

}

fn determine_max_positions(positions: &mut Vec<(i32, i32)>) -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y) in positions.iter() {
        if max_x < *x { max_x = x.clone() }
        if max_y < *y { max_y = y.clone() }
    }
    (max_x, max_y)
}


#[cfg(test)]
mod test{
    use std::fs;
    use crate::day13::parse_input;

    #[test]
    fn test_day13_1(){
        let data = fs::read_to_string("resources/day_13_testdata").expect("could not open file");
        let (mat, instructions) = parse_input(&data);
    }

}