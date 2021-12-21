use std::cmp::Ordering;
use std::ops::{ RangeInclusive};

struct Target {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
}

struct Velocity {
    x: isize,
    y: isize,
}

impl Target {
    fn within(&self, pos: &(isize, isize)) -> bool {
        self.x.contains(&pos.0) && self.y.contains(&pos.1)
    }

    fn overshoot(&self, pos: &(isize, isize)) -> bool {
        &pos.0 > self.x.end() || &pos.1 < self.y.start()
    }
}

impl Velocity {
    fn drag_and_gravity(&mut self) {
        match self.x.cmp(&0) {
            Ordering::Greater => self.x -= 1,
            Ordering::Less => self.x += 1,
            Ordering::Equal => {}
        }
        self.y -= 1;
    }
}

fn shoot_area(target: Target) -> (isize, u32) {
    let mut max_y = 0;
    let mut total_hits = 0;

    for x in 0..=*target.x.end() {
        for y in *target.y.start()..=target.y.start().abs() {
            let mut velocity = Velocity { x, y };
            let mut tracking_point = (0, 0);
            let mut trajectory_y = max_y;

            while !target.overshoot(&tracking_point) {
                tracking_point.0 += velocity.x;
                tracking_point.1 += velocity.y;

                if tracking_point.1 > trajectory_y {
                    trajectory_y = tracking_point.1;
                }


                if target.within(&tracking_point) {
                    if trajectory_y > max_y {
                        max_y = trajectory_y;
                    }
                    total_hits += 1;
                    break;
                }
                velocity.drag_and_gravity();
            }
        }
    }

    (max_y, total_hits)
}

#[cfg(test)]
mod tests {
    use crate::day_17::{shoot_area, Target};

    #[test]
    fn should_find_max_y() {
        assert_eq!(
            45,
            shoot_area(Target {
                x: (20..=30),
                y: (-10..=-5),
            }).0
        );
    }

    #[test]
    fn should_find_max_y_test_data() {
        assert_eq!(
            9730,
            shoot_area(Target {
                x: (117..=164),
                y: (-140..=-89),
            }).0
        );
    }

    #[test]
    fn should_find_total_hits_test_data() {
        assert_eq!(
            4110,
            shoot_area(Target {
                x: (117..=164),
                y: (-140..=-89),
            }).1
        );
    }
}
