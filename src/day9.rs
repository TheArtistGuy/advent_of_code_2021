use std::fs;
use std::path::Path;
use crate::mat2d::Mat2d;

pub fn day9(){
    let data = fs::read_to_string(Path::new("resources/day9_data.rs"))
        .expect("could not open file");
    println!("day 9 , 1 : {}", part1(&data));
    println!("day 9 , 3 : {}", part2(&data));
}

fn part1(data: &String) -> i32 {
    let sea_map = parse(data);
    let heat = create_heat_map(&sea_map);
    heat
}
fn part2(data : &String) -> i32{
    let sea_map = parse(data);
    let basins = find_basins(&sea_map);
    basins
}

fn parse(data: &String) -> Mat2d<i32> {
    let mut vector = Vec::new();
    let lines : Vec<&str>= data.lines().collect();
    let height = lines.len();
    let line1 :Vec<char>= lines[0].chars().collect();
    let width = line1.len();

    for line in lines{
        for num in line.chars(){
            vector.push(num.to_string().parse().expect("could not parse number"));
        }
    }
    Mat2d{
        height,
        width,
        vector
    }
}


fn create_heat_map(mat: &Mat2d<i32>) -> i32 {
    let mut heat = 0;
    for x in 0..mat.width{
        for y in 0..mat.height{
            let number = mat.get_field(x,y).unwrap();
            if  is_lower_than_all_neighbours(mat, &number, x as i32, y as i32){
                heat = heat + 1 + number;
            }

        }
    }
    heat
}

fn is_lower_than_all_neighbours(mat :&Mat2d<i32>, number : &i32, ix : i32, iy : i32) -> bool{
    is_lower(number, mat.get_field_i32(ix-1,iy))
        &&is_lower(number, mat.get_field_i32(ix+1, iy))
        &&is_lower(number, mat.get_field_i32(ix, iy+1))
        &&is_lower(number, mat.get_field_i32(ix, iy-1))
}

fn is_lower(num: &i32, neighbour: Option<&i32>) -> bool {
    if neighbour == None{
        return true
    }
    if num < neighbour.unwrap(){
        return true
    }
    false
}


fn find_basins(mat: &Mat2d<i32>) -> i32 {
    let mut found_map = Mat2d{
        height: mat.height,
        width: mat.width,
        vector: vec![false; mat.height*mat.width]
    };
    let mut basins = Vec::new();

    for x in 0..mat.width{
        for y in 0..mat.height{
            if mat.get_field(x,y).unwrap() < &9 && found_map.get_field(x, y).unwrap() == &false{
                let mut basin = vec![];
                scan_basin(mat, &mut found_map, x, y, &mut basin);
                basins.push(basin);
            }
        }
    }
    basins.sort_by(|a,b| b.len().partial_cmp(&a.len()).unwrap());

    //multiply the size of the 3 biggest basins
    (basins.get(0).unwrap().len() * basins.get(1).unwrap().len() *basins.get(2).unwrap().len()) as i32
}

fn scan_basin(mat: &Mat2d<i32>, found_map: &mut Mat2d<bool>, x: usize, y: usize, basin: &mut Vec<i32>) {
    if mat.get_field(x,y).unwrap() < &9 && found_map.get_field(x, y).unwrap() == &false {
        found_map.set_field(x, y, true);
        basin.push(mat.get_field(x,y).unwrap().clone());
        let ix = x as i32;
        let iy = y as i32;
        try_neighbor(mat, found_map, basin, ix-1, iy);
        try_neighbor(mat, found_map, basin, ix+1, iy);
        try_neighbor(mat, found_map, basin, ix, iy-1);
        try_neighbor(mat, found_map, basin, ix, iy+1);
    }
}

fn try_neighbor(mat: &Mat2d<i32>, found_map: &mut Mat2d<bool>, basin: &mut Vec<i32>, ix: i32, iy: i32) {
    match mat.get_field_i32(ix, iy) {
        None => {}
        Some(val) => {
            scan_basin(mat, found_map, ix as usize, iy as usize, basin);
        }
    }
}

#[cfg(test)]
mod test{
    use crate::day9::{create_heat_map, find_basins, parse};
    use crate::mat2d::Mat2d;

    #[test]
    fn test9_1(){
        let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678".to_string();
        let mat :Mat2d<i32>= parse(&input);
        assert_eq!(mat.height,5);
        let heat_map :i32 = create_heat_map(&mat);
        assert_eq!(heat_map, 15);
        assert_eq!(find_basins(&mat), 1134);
    }

}