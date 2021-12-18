pub fn day17(){
    let target = TargetArea::new(85, 145, -163, -108);
    let (number_hits, max_y) = determine_maximum_height(&target);
    println!("Day 17 , 1 : {}", &max_y);
    println!("Day 17 , 2 : {}", &number_hits);
}

struct TargetArea{
    x_from : i64,
    x_to : i64,
    y_from : i64,
    y_to : i64,
}

impl TargetArea {
    fn new(x_from: i64, x_to: i64, y_from: i64, y_to: i64) -> Self {
        TargetArea {
            x_from,
            x_to,
            y_from,
            y_to
        }
    }

    fn is_hit(&self, x: &i64, y: &i64) -> bool {
        x >= &self.x_from && x <= &self.x_to && y >= &self.y_from && y <= &self.y_to
    }
}
fn next_iteration(x: &mut i64, x_velocity: &mut i64, y: &mut i64, y_velocity: &mut i64) {
    *x += *x_velocity;
    *y += *y_velocity;
    *x_velocity = if *x_velocity == 0 {0}
        else if *x_velocity > 0 { *x_velocity - 1 }
        else {*x_velocity + 1};
    *y_velocity -=  1;
}

fn determine_maximum_height(target_area: &TargetArea) -> (i32, i64) {
        let mut number_hits = 0;
        let mut max_y = i64::MIN;
        for init_velocity_x in 1..target_area.x_to +1{
            for init_velocity_y in target_area.y_from..400{
                let (hit, local_min) = follow_trajectory(target_area, &init_velocity_x, &init_velocity_y);
                if hit {
                    number_hits += 1;
                    max_y = i64::max(max_y, local_min);
                }
            }
        }
    (number_hits, max_y)
}

fn follow_trajectory(target_area: &TargetArea, init_velocity_x: &i64, init_velocity_y: &i64) -> (bool, i64) {
    let mut x_vel = init_velocity_x.clone();
    let mut y_vel = init_velocity_y.clone();
    let mut x = 0;
    let mut y = 0;
    let mut local_max = i64::MIN;
    let mut hit = false;
    while x < target_area.x_to && y >= target_area.y_from && !hit{
        next_iteration(&mut x, &mut x_vel, &mut y, &mut y_vel);
        if target_area.is_hit(&x, &y){hit = true}
        local_max = i64::max(y, local_max);
    }
    (hit, local_max)
}

#[cfg(test)]
mod test {
    use crate::day17::{determine_maximum_height, follow_trajectory, TargetArea};

    #[test]
    fn test_17_1(){
        let target_area = TargetArea::new(20,30,-10,-5);
        let (hit, max) =  follow_trajectory(&target_area, &6,&9);
        assert_eq!(max, 45);
        assert_eq!(hit, true);
        let (number_hits, max_y) = determine_maximum_height(&target_area);
        assert_eq!(max_y, 45);
        assert_eq!(number_hits, 112);
    }
}