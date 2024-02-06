pub enum BombType {
    Normal,
    Penetrating,
}

pub enum DeviationDirection {
    Top,
    Down,
    Right,
    Left,
}

pub enum MazeElement {
    Enemy{health: u8},
    Bomb{reach: u8, kind: BombType},
    Rock,
    Wall,
    Deviate{direction: DeviationDirection}
}

pub struct Maze {
    matrix: Vec<Vec<MazeElement>>
}