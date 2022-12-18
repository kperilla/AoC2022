use std::collections::HashMap;
use std::fs;
use std::ops::Index;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
}

struct Sensor {
    location: Point,
    dist: u64,
}
impl Sensor {
    fn range_on_horizontal_within_sensor_range(&self, y: i64) -> RangeInclusive<> {

    }
}


pub fn part1(input: String) {
    println!("Part 1");
    // let input = fs::read_to_string("src/day14/testinput")
    //     .expect("Should have been able to read the file");

    // Put each sensor in a struct with its dist
    // Check x boundaries
    // Something like leftmost - its distance and same for rightmost
    // Along y=2000000, for from boundary to boundary
    // for each sensor, if mandist(s, pt) <= s.dist add 1, break
    // if it comes into the range of that sensor, you can assume that any point
    //   on y within it's dist can be added (subtracting any beacons)
    // (maybe sort sensors by distance to y=2000000 ?)

    // Or maybe just go through each sensor and see how many points on the y line
    //   count
    // Could be a range, which should be easy to tell overlaps
}

pub fn part2(input: String){
    println!("Part 2");
    // let input = fs::read_to_string("src/day11/testinput")
    //     .expect("Should have been able to read the file");
}
