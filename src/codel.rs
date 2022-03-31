
use pietcolor::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Codel {
    pub color: PietColor,
    pub x: usize,
    pub y: usize,
}

impl Codel {
    pub fn diff_to(self, other: &Codel) -> (usize, usize) {
        self.color.diff_to(&other.color)
    }

    pub fn compare_to<'a>(
        self: &'a Codel,
        other: &'a Codel,
        dp: Direction,
        cc: Direction,
    ) -> &'a Codel {
        if *self == *other {
            self
        } else {
            if let Some(codel) = dp.choose_codel(&self, other) {
                codel
            } else {
                cc.relative_to(dp).choose_codel(&self, other).unwrap()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl Direction {
    pub fn to_vector(self) -> (isize, isize) {
        use Direction::*;
        match self {
            Right => (1, 0),
            Left => (-1, 0),
            Down => (0, 1),
            Up => (0, -1),
        }
    }

    pub fn rotate(self) -> Direction {
        use Direction::*;
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    pub fn opposite(self) -> Direction {
        use Direction::*;
        match self {
            Right => Left,
            Down => Up,
            Left => Right,
            Up => Down,
        }
    }

    pub fn relative_to(self, other: Direction) -> Direction {
        use Direction::*;
        match other {
            Right => {
                if self == Right {
                    Down
                } else if self == Left {
                    Up
                } else {
                    panic!("error: relative_to({:?}, {:?}", Right, Left)
                }
            },
            Down => {
                if self == Right {
                    Left
                } else if self == Left {
                    Right
                } else {
                    panic!("error: relative_to({:?}, {:?})", Left, Right)
                }
            },
            Left => {
                if self == Right {
                    Up
                } else if self == Left {
                    Down
                } else {
                    panic!("error: relative_to({:?}, {:?})", Up, Down)
                }
            },
            Up => {
                if self == Right {
                    Right
                } else if self == Left {
                    Left
                } else {
                    panic!("error: relative_to({:?}, {:?})", Right, Left)
                }
            },
        }
    }

    pub fn choose_codel<'a>(self, c1: &'a Codel, c2: &'a Codel) -> Option<&'a Codel> {
        use Direction::*;
        match self {
            Right => {
                if c1.x > c2.x {
                    Some(c1)
                } else if c1.x < c2.x {
                    Some(c2)
                } else {
                    None
                }
            },
            Left => {
                if c1.x < c2.x {
                    Some(c1)
                } else if c1.x > c2.x {
                    Some(c2)
                } else {
                    None
                }
            },
            Up => {
                if c1.y < c2.y {
                    Some(c1)
                } else if c1.y > c2.y {
                    Some(c2)
                } else {
                    None
                }
            },
            Down => {
                if c1.y > c2.y {
                    Some(c1)
                } else if c1.y < c2.y {
                    Some(c2)
                } else {
                    None
                }
            },
        }
    }
}
