use std::str::FromStr;

pub struct Puzzle {
    rotations: Vec<Rotation>,
}

#[derive(Debug)]
pub struct Rotation {
    direction: Direction,
    distance: i32,
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

pub struct Dial {
    position: i32,
    floor: i32,
    ceiling: i32,
}

impl Dial {
    fn apply(&self, rotation: &Rotation) -> Dial {
        let position = match rotation.direction {
            Direction::Left => self.position - rotation.distance,
            Direction::Right => self.position + rotation.distance,
        };

        Dial {
            position: self.correct_position(position),
            floor: self.floor,
            ceiling: self.ceiling,
        }
    }

    fn correct_position(&self, position: i32) -> i32 {
        if position < self.floor {
            let corrected = position + (self.ceiling + 1);
            return self.correct_position(corrected);
        }

        if position > self.ceiling {
            let corrected = position - (self.ceiling + 1);
            return self.correct_position(corrected);
        }

        position
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            position: 50,
            floor: 0,
            ceiling: 99,
        }
    }
}

impl Direction {
    fn from_str(string: &str) -> Option<Self> {
        match string {
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }
    }
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rotations = input
            .lines()
            .filter_map(|x| {
                let (direction_str, distance_str) = x.split_at_checked(1)?;
                let direction = Direction::from_str(direction_str)?;
                let distance = distance_str.parse::<i32>().ok()?;

                Some(Rotation {
                    direction,
                    distance,
                })
            })
            .collect();

        Ok(Puzzle { rotations })
    }
}

impl Puzzle {
    pub fn part_1(&self) -> u32 {
        let default_dial = Dial::default();
        let (_, count) =
            self.rotations
                .iter()
                .fold((default_dial, 0), |(dial, count), rotation| {
                    let new_dial = dial.apply(rotation);
                    match new_dial.position {
                        0 => (new_dial, count + 1),
                        _ => (new_dial, count),
                    }
                });

        count
    }

    pub fn part_2(&self) -> u32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn should_solve_part_1() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_1();

        // Then
        assert_eq!(3, sum);
    }

    #[test]
    fn should_solve_part_2() {
        // Given
        let puzzle = Puzzle::from_str(INPUT).unwrap();

        // When
        let sum = puzzle.part_2();

        // Then
        assert_eq!(0, sum);
    }
}
