use std::borrow::Borrow;
use std::cell::RefCell;
use std::fs;
use std::ops::Deref;
use std::os::linux::raw::stat;
use std::path::Path;
use std::rc::Rc;

pub fn day12(){
    let data = fs::read_to_string(Path::new("resources/day12_data")).expect("could not open file");
    let vertices = parse_to_vertices(&data);
    let ways = check_ways(&vertices, false);
    let part1 = ways.len();
    println!("day 12 , 1 : {}", part1);
    let ways = check_ways(&vertices, true);
    let part2 = ways.len();
    println!("day 12 , 2 : {}", &part2);
}

fn parse_to_vertices(data: &String) -> Vec<(&str, &str)> {
    let mut vertices = Vec::new();
    for line in data.lines(){
        let temp : Vec<&str>= line.split("-").collect();
        vertices.push((temp[0].clone(), temp[1].clone()));
    }
    vertices
}

struct Node{
    pub name : String,
    pub adjacent_nodes : Vec<Rc<RefCell<Node>>>,
}

fn check_ways(vertices: &Vec<(&str, &str)>, allow_second_time: bool) -> Vec<Vec<Rc<RefCell<Node>>>> {
    let mut nodes = Vec::new();
    for (from, to) in vertices{
        create_nodes_with_vertices(&mut nodes, from, to)
    }
    let start = get_start(&nodes).unwrap();
    let mut visited_nodes:Vec<Rc<RefCell<Node>>> = Vec::new();
    let ways = visit_next_node(&start, visited_nodes, allow_second_time);
    ways
}
//TODO
fn visit_next_node(node: &Rc<RefCell<Node>>, mut visited: Vec<Rc<RefCell<Node>>>, allow_second_time: bool) -> Vec<Vec<Rc<RefCell<Node>>>> {
    let mut ways: Vec<Vec<Rc<RefCell<Node>>>> = Vec::new();
    let way = vec![Rc::clone(node)];
    if ! node.deref().borrow().name.eq("end") {
        let count = add_to_visited(&node, &mut visited, allow_second_time);
        let allow_second_time = determine_visit_policy(&node, allow_second_time, count);
        for next in node.deref().borrow().adjacent_nodes.iter() {
            let (element_in_visited_list, count) = determine_is_in_visited(next, &mut visited, allow_second_time);
            match element_in_visited_list {
                None => {

                    let mut before = visit_next_node(next, visited.clone(), allow_second_time);
                    let before = filter_ways_which_do_reach_end(&mut before);
                    for mut x in before.iter() {
                        let mut w = way.clone();
                        for cell in x.iter() {
                            w.push(Rc::clone(&cell));
                        }
                        ways.push(w);
                    }
                }
                _ => {}
            }
        }
    }
    ways.push(way.clone());
    let ways_to_end = filter_ways_which_do_reach_end(&mut ways);
    ways_to_end
}

fn determine_visit_policy(node: &&Rc<RefCell<Node>>, allow_second_time: bool, count: i32) -> bool {
    let allow_second_time = if node.deref().deref().borrow().name.eq("start") || is_big_cavern(&node) || count < 1 {
        allow_second_time
    } else {
        false
    };
    allow_second_time
}

fn add_to_visited(node: &&Rc<RefCell<Node>>, mut visited: &mut Vec<Rc<RefCell<Node>>>, allow_second_time: bool) -> i32 {
    let (element_in_visited_list, count) = determine_is_in_visited(node, &mut visited, allow_second_time);
    match element_in_visited_list {
        None => { if !is_big_cavern(&node) { visited.push(Rc::clone(&node)); } }
        _ => {}
    }
    count
}

fn filter_ways_which_do_reach_end(ways: &mut Vec<Vec<Rc<RefCell<Node>>>>) -> Vec<Vec<Rc<RefCell<Node>>>> {
    let mut ways_to_end = Vec::new();
    for x in ways.iter() {
        if to_end(&x) {
            ways_to_end.push(x.clone());
        }
    }
    ways_to_end
}

fn determine_is_in_visited(node: &Rc<RefCell<Node>>, visited: &mut Vec<Rc<RefCell<Node>>>, allow_second_time: bool) -> (Option<Rc<RefCell<Node>>>, i32) {
    let (element_in_visited_list, count) =
        if allow_second_time && !node.deref().borrow().name.eq("start"){
            get_node_and_count(&visited, &&*node.deref().borrow().name, 2)
        } else {
            get_node_and_count(&visited, &&*node.deref().borrow().name, 1)
        };
    (element_in_visited_list, count)
}

fn to_end(way: &&Vec<Rc<RefCell<Node>>>) -> bool {
    way.get((way.len() as i32- 1) as usize).unwrap().deref().borrow().name.eq("end")
}

fn is_big_cavern(node: &&Rc<RefCell<Node>>) -> bool {
    let x : String = node.deref().deref().borrow().name.clone();
    if x.eq( &x.to_uppercase()){
        return true
    }
    false
}

fn get_start(nodes: &Vec<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    for n in nodes{
        if n.deref().borrow().name.eq(&"start".to_string()){
            return Some(Rc::clone(&n));
        }
    }
    None
}

fn create_nodes_with_vertices(nodes: &mut Vec<Rc<RefCell<Node>>>, from: &&str, to: &&str) {
    let mut f = get_node(&nodes, from);
    let mut t = get_node(&nodes, to);
    match (f, t) {
        (Some(mut x), Some(mut y)) => {
            x.deref().borrow_mut().adjacent_nodes.push(Rc::clone(&y));
            y.deref().borrow_mut().adjacent_nodes.push(Rc::clone(&x));
        },
        (Some(mut x), None) => {
            let n = Rc::new(RefCell::new(Node {
                name: to.to_string(),
                adjacent_nodes: vec![Rc::clone(&x)]
            }));
            x.deref().borrow_mut().adjacent_nodes.push(Rc::clone(&n));
            nodes.push(Rc::clone(&n));
        },
        (None, Some(mut y)) => {
            let n = Rc::new(RefCell::new(Node {
                name: from.to_string(),
                adjacent_nodes: vec![Rc::clone(&y)]
            }));
            y.deref().borrow_mut().adjacent_nodes.push(Rc::clone(&n));
            nodes.push(Rc::clone(&n));
        }
        (None, None) => {
            let mut n = Rc::new(RefCell::new(Node {
                name: from.to_string(),
                adjacent_nodes: Vec::new()
            }));
            let mut m = Rc::new(RefCell::new(Node {
                name: to.to_string(),
                adjacent_nodes: Vec::new()
            }));
            n.deref().borrow_mut().adjacent_nodes.push(Rc::clone(&m));
            m.deref().borrow_mut().adjacent_nodes.push(Rc::clone(&n));
            nodes.push(Rc::clone(&m));
            nodes.push(Rc::clone(&n));
        }
    }
}

fn get_node<'a>(nodes: &Vec<Rc<RefCell<Node>>>, name: &&str) -> Option<Rc<RefCell<Node>>> {
    for n in nodes.iter(){
        if n.deref().borrow().name.eq(name){
            return Some(Rc::clone(n))
        }
    }
    None
}

fn get_node_and_count<'a>(nodes: &Vec<Rc<RefCell<Node>>>, name: &&str, max_count : i32) -> (Option<Rc<RefCell<Node>>>, i32) {
    let mut counter = 0;
    for n in nodes.iter(){
        if n.deref().borrow().name.eq(name){
            counter = counter +1;
            if counter == max_count{
                return (Some(Rc::clone(n)) , counter);
            }
        }
    }
    (None,counter)
}


#[cfg(test)]
mod test{
    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::fs;
    use std::ops::Deref;
    use std::path::Path;
    use std::rc::Rc;
    use crate::day12::{check_ways, parse_to_vertices};

    #[test]
    fn test_12_1(){
        let data = fs::read_to_string(Path::new("resources/day12_testdata")).expect("could not open file");
        let vertices = parse_to_vertices(&data);
        let ways = check_ways(&vertices, false);
        assert_eq!(ways.len(), 10);

        let ways = check_ways(&vertices, true);
        for x in ways.iter() {
            for c in x.iter() {
                print!("{} , ", &&c.deref().borrow().name);
            }
            println!();
        }
        assert_eq!(ways.len(), 36);
    }
}