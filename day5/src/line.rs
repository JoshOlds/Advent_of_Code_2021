use std::fmt;

#[derive(PartialEq, Eq, Hash)]
pub struct Point
{
    pub x: usize,
    pub y: usize
}

#[derive(PartialEq, Eq, Hash)]
pub struct Line
{
    pub start: Point,
    pub end: Point
}

impl Line{

    pub fn is_horizontal(&self) -> bool
    {
        if self.start.y == self.end.y { return true; }
        false
    }

    pub fn is_vertical(&self) -> bool
    {
        if self.start.x == self.end.x { return true; }
        false
    }
}
impl fmt::Display for Point
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl fmt::Display for Line
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

