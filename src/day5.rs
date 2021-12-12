use std::fs;
use std::path::Path;
use crate::mat2d::Mat2d;


pub fn day5(){
    let data = fs::read_to_string(Path::new("resources/day5_data")).expect("day5: could not open file");
    println!("day 5 , 1 : {}", part1(&data));
    println!("day 5 , 1 : {}", part2(&data));
}

fn part1(input: &String) -> i32 {
    let lines = parse_input(&input);
    let matrix = draw_ways_to_matrix(&lines, false);
    let result = count_overlap(&matrix);
    result
}
fn part2(input : &String)-> i32{
    let lines = parse_input(&input);
    let matrix = draw_ways_to_matrix(&lines, true);
    let result = count_overlap(&matrix);
    result
}


fn parse_input(input : &String) -> Vec<Line> {
    let mut ways = Vec::new();
    for line in input.lines().into_iter(){
        let split :Vec<&str> = line.split(" -> ").collect();
        let l1 :Vec<&str>= split.get(0).unwrap().split(",").collect();
        let l2 :Vec<&str>= split.get(1).unwrap().split(",").collect();
        let x1 = l1.get(0).unwrap().parse().expect("could not parse number");
        let y1 = l1.get(1).unwrap().parse().expect("could not parse number");
        let x2 = l2.get(0).unwrap().parse().expect("could not parse number");
        let y2 = l2.get(1).unwrap().parse().expect("could not parse number");
        let way = Line{
            x1,
            y1,
            x2,
            y2
        };
        ways.push(way);
    }
    ways
}


#[derive(Debug, Copy, Clone)]
struct Line{
    x1 : i32,
    y1 : i32,
    x2 : i32,
    y2 : i32
}

fn draw_ways_to_matrix(ways: &Vec<Line>, is_drawing_diagonals: bool) -> Mat2d<i32> {
    let height: usize = max_y(&ways) +1;
    let width: usize = max_x(&ways) +1;
    let mut mat = Mat2d::from(vec![0; width*height], width, height);
    for line in ways.iter(){
        if &line.x1 == &line.x2 {
            draw_horizontal_line(&mut mat, &line);
        } else if &line.y1 == &line.y2{
            draw_vertical_line(&mut mat, &line);
        } else if is_drawing_diagonals{
            draw_diagonal_line(&mut mat, &line)
        }
    }
    mat
}

fn draw_diagonal_line(matrix: &mut Mat2d<i32>, line: &&Line) {
    let (x_start, x_end,y_start, y_end) = init_diagonal_path(line);
    if x_end < x_start {
        draw_line_to_top_left(matrix, x_end, y_end, x_start, y_start)
    } else {
        draw_line_to_top_right(matrix, x_end, y_end, x_start, y_start)
    }
}

fn draw_line_to_top_right(matrix: &mut Mat2d<i32>, x_end: i32, y_end: i32, mut x: i32, mut y: i32) {
    while x_end + 1 != x  && y_end +1  != y{
        matrix.set_value(x as usize, y as usize,
                         matrix.get_value(x as usize, y as usize).unwrap() + 1)
            .expect("could not set field");
        x = x + 1;
        y = y + 1;
    }
}

fn draw_line_to_top_left(matrix: &mut Mat2d<i32>, x_end: i32, y_end: i32, mut x: i32, mut y: i32) {
    while x_end  != x +1 && y_end +1  != y {
        matrix.set_value(x as usize, y as usize,
                         matrix.get_value(x as usize, y as usize).unwrap() + 1)
            .expect("could not set field");
        x = x - 1;
        y = y + 1;
    }
}

/// returns (x_start, x_end, y_start, y_end) y start is always smaller end, x_start is the corresponding coordinate.
/// so a line can always be drawn from bottom to top.
fn init_diagonal_path(line: &&Line) -> (i32, i32, i32, i32) {

    if &line.y1 < &line.y2 {
        (line.x1.clone(), line.x2.clone(), line.y1.clone(), line.y2.clone())
    } else {
        (line.x2.clone(), line.x1.clone(), line.y2.clone(), line.y1.clone())
    }
}

fn draw_horizontal_line(mat: &mut Mat2d<i32>, line: &&Line) {
    if &line.y1 < &line.y2 {
        draw_line_from_bottom_to_top(mat, line)
    } else {
        draw_line_from_top_to_bottom(mat, line)
    }
}

fn draw_line_from_top_to_bottom(mat: &mut Mat2d<i32>, line: &&Line) {
    for i in (line.y2 as usize)..((line.y1 + 1) as usize) {
        mat.set_value(line.x1 as usize, i,
                      *mat.get_value(line.x1 as usize, i).unwrap() + 1)
            .expect("could not set field");
    }
}

fn draw_line_from_bottom_to_top(mat: &mut Mat2d<i32>, line: &&Line) {
    for i in (line.y1 as usize)..((line.y2 + 1) as usize) {
        mat.set_value(line.x1 as usize, i,
                      *mat.get_value(line.x1 as usize, i).unwrap() + 1)
            .expect("could not set field");
    }
}

fn draw_vertical_line(mat: &mut Mat2d<i32>, line: &&Line) {
    if &line.x1 < &line.x2 {
        draw_line_from_left_to_right(mat, line)
    } else {
        draw_line_from_right_to_left(mat, line)
    }
}

fn draw_line_from_right_to_left(mat: &mut Mat2d<i32>, line: &&Line) {
    for i in (line.x2 as usize)..((line.x1 + 1) as usize) {
        mat.set_value(i, line.y1 as usize,
                      *mat.get_value(i, line.y1 as usize).unwrap() + 1)
            .expect("could not set field");
    }
}

fn draw_line_from_left_to_right(mat: &mut Mat2d<i32>, line: &&Line) {
    for i in (line.x1 as usize)..((line.x2 + 1) as usize) {
        mat.set_value(i, line.y1 as usize,
                      *mat.get_value(i, line.y1 as usize).unwrap() + 1)
            .expect("could not set field");
    }
}

///determines the highest x coordinate of all Lines
fn max_x(ways: &&Vec<Line>) -> usize {
    let mut max = 0;
    for line in ways.iter(){
        if line.x1 > max { max = line.x1.clone()}
        if line.x2 > max { max = line.x2.clone()}
    }
    max as usize
}

///determines the highest y coordinate of all Lines
fn max_y(ways: &&Vec<Line>) -> usize {
    let mut max = 0;
    for line in ways.iter(){
        if line.y1 > max { max = line.y1.clone()}
        if line.y2 > max { max = line.y2.clone()}
    }
    max as usize
}


fn count_overlap(matrix: &Mat2d<i32>) -> i32 {
    let mut result = 0;
    for field in matrix.vector.iter(){
        if *field > 1{
            result = result +1;
        }
    }
    result
}

#[cfg(test)]
mod test{
    use std::fs;
    use std::path::Path;
    use crate::day5::{count_overlap, draw_ways_to_matrix, Line, parse_input};
    use crate::mat2d::Mat2d;

    #[test]
    fn test_day5_1(){
        let data = fs::read_to_string(Path::new("resources/day5_test_data")).expect("could not open file");
        let lines :Vec<Line> = parse_input(&data);
        assert_eq!(lines.len(), 10);
        let mat : Mat2d<i32> = draw_ways_to_matrix(&lines, false);
        draw_matrix(&mat);
        assert_eq!(mat.height, 10);
        assert_eq!(mat.width, 10);
        assert_eq!(mat.get_value(0, 9).unwrap(), &2);
        let result:i32 = count_overlap(&mat);
        assert_eq!(result, 5);
    }

    fn draw_matrix(mat: &Mat2d<i32>) {
        for i in 0..mat.width {
            for j in 0..mat.height {
                print!("{} ", &mat.get_value(i, j).unwrap());
            }
            println!()
        }
    }

    #[test]
    fn test_day5_2(){
        let data = fs::read_to_string(Path::new("resources/day5_test_data")).expect("could not open file");
        let lines :Vec<Line> = parse_input(&data);
        let matrix = draw_ways_to_matrix(&lines, true);
        draw_matrix(&matrix);
        for i in lines{
            println!("{}, {} | {}, {}", &i.x1, &i.y1, &i.x2, &i.y2);
        }
        let result = count_overlap(&matrix);
        assert_eq!(result, 12)

    }

}