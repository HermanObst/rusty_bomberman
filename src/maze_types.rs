use crate::errors::MazeError;
use std::fs::File;
use std::io::Read;
use std::usize;

#[derive(PartialEq, Debug)]
pub enum BombType {
    Normal,
    Penetrating,
}
#[derive(PartialEq, Debug)]
pub enum FireDirection {
    Up,
    Down,
    Right,
    Left,
}
#[derive(PartialEq, Debug)]
pub enum MazeElement {
    Enemy{health: u8},
    Bomb{reach: u8, kind: BombType},
    Rock,
    Wall,
    Deviate{direction: FireDirection},
    Empty
}

enum NextAction {
    Continue,
    Stop,
    Bomb,
    ChangeDirection(FireDirection),
}

pub struct Maze {
    pub matrix: Vec<Vec<MazeElement>>
}

#[allow(dead_code)]
impl Maze {
    pub fn new(path: &str) -> Result<Maze, MazeError> {
        let file_string = read_file(path).map_err(|e| MazeError::FileNotFound(e.to_string()))?;
        let matrix = process_file(&file_string)?;
        Ok(Maze{matrix})
    }

    fn get_maze_element(&self, x: usize, y: usize) -> Option<&MazeElement> {
        self.matrix.get(y)?.get(x)
    }

    pub fn detonate_bomb(&self, x: usize, y: usize) -> Result<(), MazeError> {
        let element = self.get_maze_element(x, y).ok_or(MazeError::OutOfBounds)?;
        match element {
           MazeElement::Bomb {..} => Ok(()),
           _ => Err(MazeError::NoBomb), 
        }
    }
    
    fn advance_fire_bursts(&mut self, x_initial: usize, y_initial: usize, reach: &u8, bomb_type: &BombType) {
        todo!();
    }

    fn advance_fire_burst(&mut self, remaining_steps: u8, x_burst: usize, y_burst: usize, bomb_type: &BombType, direction: FireDirection) {
        if remaining_steps == 0 {
            return;
        }

        let next_action = self.calulate_next_action(x_burst, y_burst, bomb_type, direction);
        todo!()
    }

    fn calulate_next_action(&self, x_burst: usize, y_burst: usize, bomb_type: &BombType, direction: FireDirection) -> NextAction {
        let (next_x, next_y) = next_indexes(x_burst, y_burst, direction);
        let next_action: NextAction = match (next_x, next_y) {
            (Some(x), Some(y)) => {
                todo!()
            },
            (_,_) => NextAction::Stop
        };
        let element = self.get_maze_element(next_x, next_y);

        todo!();
    }
}

// Returns next indeces. If some of it is out of bounds, return None
fn next_indexes(x_burst: usize, y_burst: usize, direction: FireDirection) -> (Option<usize>, Option<usize>) {
    let (next_x, next_y) = match direction {
        FireDirection::Up => (Some(x_burst), y_burst.checked_sub(1)),
        FireDirection::Down => (Some(x_burst), y_burst.checked_add(1)),
        FireDirection::Left => (x_burst.checked_sub(1), Some(y_burst)),
        FireDirection::Right => (x_burst.checked_add(1), Some(y_burst)),
    };
    (next_x,next_y)
}

// reads a file containing a board and convers it to a String.
fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn process_file(file_string: &str) -> Result<Vec<Vec<MazeElement>>, MazeError> {
    file_string
        .lines()
        .map(|line| { 
            line.split(' ')
                .map(str_to_maze_element)
                .collect::<Result<Vec<MazeElement>, MazeError>>()
            })
        .collect::<Result<Vec<Vec<MazeElement>>, MazeError>>()
}

fn str_to_maze_element(element_str: &str) -> Result<MazeElement, MazeError> {
    let chars: Vec<char> = element_str.chars().collect();
    match &chars[..] {
        [c1] => {
            match c1 {
                '_' => Ok(MazeElement::Empty),
                'W' => Ok(MazeElement::Wall),
                'R'=> Ok(MazeElement::Rock),
                _ => Err(MazeError::InvalidFormat(String::from("Not allowed character found")))
            }
        }
        [c1,c2] => {
            match c1 {
                'B' => {
                    if !c2.is_ascii_digit() {
                        Err(MazeError::InvalidFormat(String::from("Not allowed character found")))
                    } else {
                        Ok(MazeElement::Bomb {reach: (*c2 as u8 - '0' as u8), kind: (BombType::Normal)})
                    }
                },
                'S' => {
                    if !c2.is_ascii_digit() {
                        Err(MazeError::InvalidFormat(String::from("Not allowed character found")))
                    } else {
                        Ok(MazeElement::Bomb {reach: (*c2 as u8 - '0' as u8), kind: (BombType::Penetrating)})
                    }
                },
                'F' => {
                    if !c2.is_ascii_digit() {
                        Err(MazeError::InvalidFormat(String::from("Not allowed character found")))
                    } else {
                        Ok(MazeElement::Enemy {health:(*c2 as u8 - '0' as u8)})
                    }
                },
                'D' => {
                    match c2 {
                        'L' => Ok(MazeElement::Deviate{direction:(FireDirection::Left)}),
                        'R' => Ok(MazeElement::Deviate{direction:(FireDirection::Right)}),
                        'U' => Ok(MazeElement::Deviate{direction:(FireDirection::Up)}),
                        'D' => Ok(MazeElement::Deviate{direction:(FireDirection::Down)}),
                        _ => Err(MazeError::InvalidFormat(String::from("Not allowed character found"))),
                    }
                }
                _ => Err(MazeError::InvalidFormat(String::from("Not allowed character found")))
            }
        },
        _ => Err(MazeError::InvalidFormat(String::from("Incorrect maze MazeElement file")))

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reading() {
        let result = read_file("src/input.txt").unwrap();
        assert_eq!(result, "B2 R R _ F1 _ _\n_ W R W _ W _\nB5 _ _ _ B2 _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _");
    }

    #[test]
    fn test_maze_build() {
        let result = read_file("src/input.txt").unwrap();
        let matrix = process_file(&result).unwrap();
        let expected_maze = [
            vec![
                MazeElement::Bomb {reach:(2), kind:(BombType::Normal)},
                MazeElement::Rock,
                MazeElement::Rock,
                MazeElement::Empty,
                MazeElement::Enemy {health: (1)},
                MazeElement::Empty,
                MazeElement::Empty,
            ],
            vec![
                MazeElement::Empty,
                MazeElement::Wall,
                MazeElement::Rock,
                MazeElement::Wall,
                MazeElement::Empty,
                MazeElement::Wall,
                MazeElement::Empty,
            ],
            vec![
                MazeElement::Bomb {reach:(5), kind:(BombType::Normal)},
                MazeElement::Empty,
                MazeElement::Empty,
                MazeElement::Empty,
                MazeElement::Bomb {reach:(2), kind:(BombType::Normal)},
                MazeElement::Empty,
                MazeElement::Empty,
            ],
            vec![
                MazeElement::Empty,
                MazeElement::Wall,
                MazeElement::Empty,
                MazeElement::Wall,
                MazeElement::Empty,
                MazeElement::Wall,
                MazeElement::Empty,
            ],
            vec![
                MazeElement::Empty,
                MazeElement::Empty,
                MazeElement::Empty,
                MazeElement::Empty,
                MazeElement::Empty,
                MazeElement::Empty,
                MazeElement::Empty,
            ],
            vec![
                MazeElement::Empty,
                MazeElement::Wall,
                MazeElement::Empty,
                MazeElement::Wall,
                MazeElement::Empty,
                MazeElement::Wall,
                MazeElement::Empty,
            ],  
            vec![
                MazeElement::Empty,
                MazeElement::Empty,
                MazeElement::Empty,
                MazeElement::Empty,
                MazeElement::Empty,
                MazeElement::Empty,
                MazeElement::Empty,
            ],
        ];
        assert_eq!(matrix, expected_maze);

    }
}