use regex::Regex;
pub struct Solution;

impl Solution {
    const NORTH: i8 = 0;
    const EAST: i8 = 1;
    const SOUTH: i8 = 2;
    const WEST: i8 = 3;
    const MAX_DIRECTION: i8 = 4;

    pub fn walking_bot(walking_code: String) -> Option<String> {
        let mut x = 0;
        let mut y = 0;
        let mut direction: i8 = 0;

        let re = Regex::new(r"L|R|W\d+").unwrap();
        let command_vec: Vec<&str> = re
            .find_iter(&walking_code)
            .map(|mat| mat.as_str())
            .collect();

        for command in command_vec {
            if command == "R" {
                Self::move_right(&mut direction);
            } else if command == "L" {
                Self::move_left(&mut direction);
            } else {
                let distance = &command[1..]
                    .parse::<i32>()
                    .expect("Failed to convert string to integer");

                if direction == Self::NORTH {
                    y = y + distance;
                } else if direction == Self::SOUTH {
                    y = y - distance;
                } else if direction == Self::EAST {
                    x = x + distance;
                } else if direction == Self::WEST {
                    x = x - distance;
                }
            }
        }

        match Self::get_direction_string(direction) {
            Some(direct) => {
                let result = format!("X:{}, Y:{}, Direction:{}", x, y, direct);
                Some(result)
            }
            None => None,
        }
    }

    fn move_right(direction: &mut i8) {
        *direction += 1;
        *direction %= Self::MAX_DIRECTION
    }

    fn move_left(direction: &mut i8) {
        *direction -= 1;
        *direction %= Self::MAX_DIRECTION
    }

    fn get_direction_string(direction: i8) -> Option<String> {
        if direction == Solution::NORTH {
            return Some("NORTH".to_string());
        } else if direction == Solution::SOUTH {
            return Some("SOUTH".to_string());
        } else if direction == Solution::EAST {
            return Some("EAST".to_string());
        } else if direction == Solution::WEST {
            return Some("WEST".to_string());
        }
        return None;
    }
}
