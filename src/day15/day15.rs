use std::collections::HashSet;
// use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn manhattan_dist(pt_a: &Point<i64>, pt_b: &Point<i64>) -> u64 {
    ((pt_a.x - pt_b.x).abs() + (pt_a.y - pt_b.y).abs()) as u64
}

#[derive(Debug)]
struct Sensor {
    location: Point<i64>,
    dist: u64,
}
impl Sensor {
    fn range_on_horizontal_within_sensor_range(&self, y: i64) -> Option<CoordinateRange> {
        let y_dist = (self.location.y - y).abs() as u64;
        if y_dist > self.dist {
            return None;
        }
        let offset = self.dist - y_dist;
        let start_x = self.location.x - offset as i64;
        let stop_x = self.location.x + offset as i64;
        Some(CoordinateRange {
            start: start_x,
            end: stop_x,
        })
    }
}

fn string_to_sensor_and_beacon(line: &str) -> (Sensor, Point<i64>) {
    if let Ok((sensor_x, sensor_y, beacon_x, beacon_y)) = scan_fmt!(
        line,
        "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
        i64,
        i64,
        i64,
        i64
    ) {
        let sensor_pt = Point {
            x: sensor_x,
            y: sensor_y,
        };
        let beacon_pt = Point {
            x: beacon_x,
            y: beacon_y,
        };
        let dist = manhattan_dist(&sensor_pt, &beacon_pt);

        (
            Sensor {
                location: sensor_pt,
                dist,
            },
            beacon_pt,
        )
    } else {
        panic!("Error reading sensors");
    }
}

#[derive(Debug)]
struct CoordinateRange {
    start: i64,
    end: i64,
}
impl CoordinateRange {
    fn span(&self) -> u64 {
        (self.end - self.start) as u64 + 1
    }
}

fn simplify_ranges(sorted_ranges: &Vec<CoordinateRange>) -> Vec<CoordinateRange> {
    // Assume sorted range vec (by start)
    let mut simplified = vec![];
    let mut candidate_range: Option<(i64, i64)> = None;
    for range in sorted_ranges {
        match candidate_range {
            None => {
                candidate_range = Some((range.start, range.end));
            }
            Some((cand_start, cand_end)) => {
                if range.start > cand_end {
                    simplified.push(CoordinateRange {
                        start: cand_start,
                        end: cand_end,
                    });
                    candidate_range = Some((range.start, range.end));
                } else if range.end > cand_end {
                    candidate_range = Some((cand_start, range.end))
                }
            }
        }
    }
    match candidate_range {
        None => (),
        Some((cand_start, cand_end)) => simplified.push(CoordinateRange {
            start: cand_start,
            end: cand_end,
        }),
    }
    return simplified;
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
    let mut known_beacons: HashSet<Point<i64>> = HashSet::new();
    let mut sensors: Vec<Sensor> = vec![];
    // let mut min_x: i64 = 0;
    // let mut max_x: i64 = 0;
    for line in input.lines() {
        let (sensor, beacon) = string_to_sensor_and_beacon(line);
        // if (sensor.location.x - sensor.dist as i64) < min_x {
        //     min_x = sensor.location.x - sensor.dist as i64;
        // }
        // if (sensor.location.x + sensor.dist as i64) > max_x {
        //     max_x = sensor.location.x + sensor.dist as i64;
        // }
        sensors.push(sensor);
        known_beacons.insert(beacon);
    }
    let horizontal: i64 = 2_000_000;
    let mut ranges: Vec<CoordinateRange> = vec![];
    for sensor in sensors {
        if let Some(range) = sensor.range_on_horizontal_within_sensor_range(horizontal) {
            ranges.push(range);
        }
    }
    ranges.sort_by(|x, y| x.start.cmp(&y.start));
    // dbg!(&ranges);
    let simplified_ranges = simplify_ranges(&ranges);
    // dbg!(&simplified_ranges);
    let mut beacons_on_horizontal = 0;
    for beacon in known_beacons {
        if beacon.y == horizontal {
            beacons_on_horizontal += 1;
        }
    }
    let mut invalid_positions = 0;
    for range in simplified_ranges {
        invalid_positions += range.span();
    }
    println!("Positions: {}", invalid_positions - beacons_on_horizontal);
}

pub fn part2(input: String) {
    println!("Part 2");
    // let input = fs::read_to_string("src/day11/testinput")
    //     .expect("Should have been able to read the file");
    let mut known_beacons: HashSet<Point<i64>> = HashSet::new();
    let mut sensors: Vec<Sensor> = vec![];
    // let mut min_x: i64 = 0;
    // let mut max_x: i64 = 0;
    for line in input.lines() {
        let (sensor, beacon) = string_to_sensor_and_beacon(line);
        // if (sensor.location.x - sensor.dist as i64) < min_x {
        //     min_x = sensor.location.x - sensor.dist as i64;
        // }
        // if (sensor.location.x + sensor.dist as i64) > max_x {
        //     max_x = sensor.location.x + sensor.dist as i64;
        // }
        sensors.push(sensor);
        known_beacons.insert(beacon);
    }
    // let horizontal: i64 = 2_000_000;
    let min: i64 = 0;
    let max: i64 = 4_000_000;
    for y in min..=max {
        let mut ranges: Vec<CoordinateRange> = vec![];
        for sensor in &sensors {
            if let Some(mut range) = sensor.range_on_horizontal_within_sensor_range(y) {
                if range.end < min || range.start > max {
                    continue;
                }
                if range.end > max {
                    range.end = max;
                }
                if range.start < min {
                    range.start = min;
                }
                ranges.push(range);
            }
        }
        ranges.sort_by(|x, y| x.start.cmp(&y.start));
        // dbg!(&ranges);
        let simplified_ranges = simplify_ranges(&ranges);
        if simplified_ranges.len() == 2 {
            println!("SPLIT HERE: {}", y);
            // dbg!(simplified_ranges);
            let beacon_x = simplified_ranges[0].end + 1;
            println!("Frequency: {}", beacon_x * max + y);
            break;
        }
    }
    // dbg!(simplified_ranges);
}

#[cfg(test)]
mod test_sensor {
    use super::*;

    #[test]
    fn range_is_none_when_signal_too_far_from_horizontal() {
        let horizontal: i64 = 0;
        let sensor_y = 4;
        let sensor_dist = 2;
        let sensor = Sensor {
            location: Point { x: 0, y: sensor_y },
            dist: sensor_dist,
        };
        let returned_range = sensor.range_on_horizontal_within_sensor_range(horizontal);
        assert!(returned_range.is_none());
    }

    #[test]
    fn range_len_is_one_plus_distance_difference() {
        let horizontal: i64 = 0;
        let sensor_y = 4;
        let sensor_dist = 6;
        let sensor = Sensor {
            location: Point { x: 0, y: sensor_y },
            dist: sensor_dist,
        };
        let returned_range = sensor.range_on_horizontal_within_sensor_range(horizontal);
        let diff_between_sensor_dist_and_dist_to_horizontal =
            sensor_dist - (sensor_y - horizontal) as u64;
        assert_eq!(
            returned_range.unwrap().span(),
            (diff_between_sensor_dist_and_dist_to_horizontal * 2) + 1
        );
    }
}
