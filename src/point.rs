#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y}
    }

    pub fn transform(&self, direction: Direction, times: u16)  {
        let times = times as i16;
        let transformation = match direction {
            Direction::Up => (0, -times),
            Direction::Right => (times, 0),
            Directions::Down => (0, times),
            Direction::Left => (-times, 0),
        };

        Self::new(
        )

    }

    fn transform_value(value: u16, by: i16) -> u16 {
        if by.is_negative() && by.abs() as u16 > value {
            panic!("Transforming value {} by {} would result in a negative number", value, by);
        } else {
            (value as i16 + by) as u16
        }
    }
}