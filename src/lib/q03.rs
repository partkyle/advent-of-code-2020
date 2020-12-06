use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Space {
    Empty,
    Tree,
}

impl FromStr for Space {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Space::Empty),
            "#" => Ok(Space::Tree),
            _ => Err(format!("Not a board type: {}", s)),
        }
    }
}

pub struct Board<Space> {
    pub board: Vec<Vec<Space>>,
}

impl Board<Space> {
    pub fn new(input: String) -> Result<Board<Space>, String> {
        let board_result: Result<Vec<Vec<Space>>, _> = input
            .lines()
            .map(|s| s.chars().map(|c| c.to_string().parse()).collect())
            .collect();

        match board_result {
            Err(e) => Err(format!("error creating board state: {}", e)),
            Ok(result) => Ok(Board { board: result }),
        }
    }

    fn get_at(&self, x: usize, y: usize) -> Space {
        self.board[y][x].clone()
    }

    pub fn get_at_wrapping(&self, x: usize, y: usize) -> Space {
        let new_x = x % self.board[y].len();
        self.get_at(new_x, y)
    }

    pub fn count_trees_from_slope(&self, dx: usize, dy: usize) -> usize {
        let (mut x, mut y) = (0, 0);
        let mut total = 0;
        while y < self.board.len() {
            total += match self.get_at_wrapping(x, y) {
                Space::Tree => 1,
                Space::Empty => 0,
            };
            x += dx;
            y += dy;
        }
        total
    }
}
