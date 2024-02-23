use crate::errors::MazeError;
use std::fs::File;
use std::io::Read;

#[derive(PartialEq, Debug)]
pub enum BombType {
    Normal,
    Penetrating,
}
#[derive(PartialEq, Debug)]
pub enum DeviationDirection {
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
    Deviate{direction: DeviationDirection},
    Empty
}

pub struct Maze {
    pub matrix: Vec<Vec<MazeElement>>
}

impl Maze {
    pub fn new(path: &str) -> Result<Maze, MazeError> {
        let file_string = read_file(path).map_err(|e| MazeError::FileNotFound(e.to_string()))?;
        let matrix = process_file(&file_string)?;
        Ok(Maze{matrix})
    }
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
                        'L' => Ok(MazeElement::Deviate{direction:(DeviationDirection::Left)}),
                        'R' => Ok(MazeElement::Deviate{direction:(DeviationDirection::Right)}),
                        'U' => Ok(MazeElement::Deviate{direction:(DeviationDirection::Up)}),
                        'D' => Ok(MazeElement::Deviate{direction:(DeviationDirection::Down)}),
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