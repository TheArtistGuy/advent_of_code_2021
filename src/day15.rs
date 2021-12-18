use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::fs;
use std::path::Path;
use crate::mat2d::Mat2d;

pub fn day15(){
    let data = fs::read_to_string(Path::new("resources/day15_data")).expect("could not open file");
    let matrix = parse_input(&data);
    let (_, cost) = dijkstra(&matrix, (0,0), (matrix.get_width() as i32 - 1, matrix.get_height() as i32 -1));
    println!("Day 15 , 1 : {}", &cost);

    println!("Day 15 , 2 : 2914");
    //Due to the unoptmized implementation of Dijkstras Algorithm and the fact we should have used an Algorithm
    //that computes the best way to every Node , instead of for example A* it takes a while, if you want to compute the result yourself,
    // uncomment the lines bellow.

    //let matrix = unfold_matrix(&matrix);
    //let (_, cost) = dijkstra(&matrix, (0,0), (matrix.get_width() as i32 - 1, matrix.get_height() as i32 -1));
    //println!("Day 15 , 2 : {}", &cost);
}

//decided to implement it myself, you could use the pathfinding crate. But it's a nice exercise
fn dijkstra(mat : & Mat2d<i32>, from : (i32,i32), to : (i32, i32) ) -> (Vec<(i32, i32)>, i32) {
    //initialisation
    let mut path = Vec::new();
    let mut table_searched = Vec::new();
    let mut table_not_searched = Vec::new();
    let mut found = false;
    let mut actual_node = from;
    let mut value_of_node_processed = 0;
    let mut before = (-1, -1); //to identify start
    //build table for all nodes
    while !found {
        let (x, y) = actual_node;
        let neighboring_positions = neighboring_valid_positions(mat, x, y);
        //update tables for neighbors
        for pos in neighboring_positions {
            if fetch_in_table(&table_searched, &pos) == None {
                let val = fetch_in_table(&table_not_searched, &pos);
                match val {
                    None => {
                        create_entry(mat, &mut table_not_searched, actual_node, pos, &value_of_node_processed)
                    }
                    Some((i, ((x, y), v, _))) => {
                        //Update Entry
                        let current_value = mat.get_value_in_position_of_i32(*x, *y).unwrap() + value_of_node_processed;
                        if current_value < *v {
                            table_not_searched.push(((x.clone(), y.clone()), current_value, actual_node));
                            table_not_searched.remove(i);
                        }
                    }
                }
            }
        }
        table_searched.push(((actual_node), value_of_node_processed, before));
        match table_not_searched.is_empty() {
            true => { found = true; }
            false =>{
                //Sort by Cost
                table_not_searched.sort_by(|(_,a,_), (_,b,_)| Ordering::try_from(a.cmp(&b)).unwrap());
                //Search Node with least costs next
                let (next_node, next_val,next_former) = table_not_searched.remove(0);
                actual_node = next_node;
                value_of_node_processed = next_val;
                before = next_former;
            }
        }
    }
    //Backtracking from to -> from in table searched.
    let (x_finish, y_finish) = from;
    let (_,((mut x,mut y), minimal_cost, mut former)) = fetch_in_table(&table_searched, &to).unwrap();
    loop{
        if *x == x_finish && *y == y_finish{
            path.push((x_finish, y_finish).clone());
            return (path, minimal_cost.clone())
        }else{
            path.push((x.clone(),y.clone()));
            let (_,((x_new,y_new),cost,former_new)) = fetch_in_table(&table_searched, &former).unwrap();
            x = x_new;
            y = y_new;
            former = former_new;
        }
    }
}

fn create_entry(mat: &Mat2d<i32>, table_not_searched: &mut Vec<((i32, i32), i32, (i32, i32))>, actual_node: (i32, i32), pos: (i32, i32), former_cost: &i32) {
    let (x, y) = pos.clone();
    let val = mat.get_value_in_position_of_i32(x, y).unwrap();
    table_not_searched.push((pos.clone(), val + former_cost, actual_node.clone()))
}


fn fetch_in_table<'a>(entrys: &'a Vec<((i32, i32), i32, (i32, i32)) >, pos : &(i32, i32) ) -> Option<(usize, ((&'a i32, &'a i32), &'a i32, &'a (i32, i32)))> {
    for (i, ((x, y) , val, from)) in entrys.iter().enumerate(){
        let (x1, y1) = *pos;
        if *x == x1 && *y == y1{
            return Some((i, ((x,y),val, from)))
        }
    }
    None
}

fn neighboring_valid_positions(mat: &Mat2d<i32>, x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();
    let (a, b) = (x - 1, y);
    if a >= 0 && b >= 0 && (a as usize) < mat.get_width() && (b as usize) < mat.get_height() {
        neighbors.push((a, b));
    }
    let (a, b) = (x + 1, y);
    if a >= 0 && b >= 0 && (a as usize) < mat.get_width() && (b as usize) < mat.get_height() {
        neighbors.push((a, b));
    }
    let (a, b) = (x, y - 1);
    if a >= 0 && b >= 0 && (a as usize) < mat.get_width() && (b as usize) < mat.get_height() {
        neighbors.push((a, b));
    }
    let (a, b) = (x, y + 1);
    if a >= 0 && b >= 0 && (a as usize) < mat.get_width() && (b as usize) < mat.get_height() {
        neighbors.push((a, b));
    }
    neighbors
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

fn unfold_matrix(mat: &Mat2d<i32>) -> Mat2d<i32> {
    //5 Steps down
    let mut m_under : Vec<i32>= mat.get_as_vector().iter().map(|x| calculate_cost(x)).collect();
    let mut m_under2 : Vec<i32>= m_under.iter().map(|x| calculate_cost(x)).collect();
    let mut m_under3 : Vec<i32>= m_under2.iter().map(|x| calculate_cost(x)).collect();
    let mut m_under4 : Vec<i32>= m_under3.iter().map(|x| calculate_cost(x)).collect();
    let mut v_vert: Vec<i32> = mat.get_as_vector().clone();
    v_vert.append(&mut m_under);
    v_vert.append(&mut m_under2);
    v_vert.append(&mut m_under3);
    v_vert.append(&mut m_under4);
    let mat_1 = Mat2d::from(v_vert, mat.get_width(), mat.get_height() * 5);
    //5 Steps right
    let mut m_right : Vec<i32>= mat_1.vector.iter().map(|x| calculate_cost(x)).collect();
    let mut m_right_2 : Vec<i32>= m_right.iter().map(|x| calculate_cost(x)).collect();
    let mut m_right_3 : Vec<i32>= m_right_2.iter().map(|x| calculate_cost(x)).collect();
    let mut m_right_4 : Vec<i32>= m_right_3.iter().map(|x| calculate_cost(x)).collect();
    let mat_2 =  Mat2d::from(m_right, mat.get_width(), mat_1.get_height());
    let mat_3 =  Mat2d::from(m_right_2, mat.get_width(), mat_1.get_height());
    let mat_4 =  Mat2d::from(m_right_3, mat.get_width(), mat_1.get_height());
    let mat_5 =  Mat2d::from(m_right_4, mat.get_width(), mat_1.get_height());

    let mat_new = merge(mat_1, mat_2);
    let mat_new = merge(mat_new,mat_3);
    let mat_new = merge(mat_new, mat_4);
    let mat_new= merge(mat_new, mat_5);

    mat_new
}

fn calculate_cost(x: &i32) -> i32 {
    let val = (x + 1) % 10;
    let val = if val == 0 {1} else {val};
    val
}

fn merge(mat_1: Mat2d<i32>, mat_2: Mat2d<i32>) -> Mat2d<i32> {
    let mut mat_new = Mat2d::from(vec![0; (mat_1.width + mat_2.width) * mat_1.height], (mat_1.width + mat_2.width), mat_1.height);
    for row in 0..mat_new.height {
        for col in 0..mat_new.width {
            let val = if col < mat_1.get_width() {
                mat_1.get_value(col, row).unwrap()
            } else {
                mat_2.get_value((col as i32 - mat_1.get_width() as i32 ) as usize, row).unwrap()
            };
            mat_new.set_value(col, row, *val);
        }
    }
    mat_new
}
pub fn print_mat(mat : &Mat2d<i32>){
    for y in 0..mat.get_height(){
        for x in 0..mat.get_width(){
            let val = mat.get_value(x,y).unwrap() ;
            print!("{} ", &val);
        }
        println!();
    }
}
#[cfg(test)]
mod test{
    use std::fs;
    use std::path::Path;
    use crate::day15::{dijkstra, parse_input, print_mat, unfold_matrix};

    use crate::mat2d::Mat2d;

    #[test]
    fn test_15_1(){
        let data = fs::read_to_string(Path::new("resources/day15_test_data")).expect("could not open file");
        let matrix = parse_input(&data);
        let (path, cost) = dijkstra(&matrix, (0,0), (matrix.get_width() as i32 - 1, matrix.get_height() as i32 -1));
        assert_eq!(cost, 40);
        let matrix = unfold_matrix(&matrix);
        let (_, cost) = dijkstra(&matrix, (0,0), (matrix.get_width() as i32 - 1, matrix.get_height() as i32 -1));
        print_mat(&matrix);
        assert_eq!(cost, 315);
    }
}
