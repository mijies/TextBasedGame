use crate::error::GameError;

#[derive(Clone, Copy, PartialEq)]
pub enum Room {
    PointAtInfinity,
    Euclid,
    Newton,
    Euler,
    LaplaceDemon,
}
use Room::*;

pub enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

impl Direction {
    pub fn new(direction: &str) -> Result<Direction, GameError> {
        match direction {
            "North" => Ok(North),
            "East" => Ok(East),
            "South" => Ok(South),
            "West" => Ok(West),
            _ => Err(GameError::Direction),
        }
    }
}

impl Room {
    const ROOM_LENGTH: usize = 5;
    pub const ITEM_LENGTH: usize = Room::ROOM_LENGTH - 2;

    pub fn name(&self) -> &str {
        match self {
            PointAtInfinity => "Point at Infinity",
            Euclid => "Euclid",
            Newton => "Isaac Newton",
            Euler => "Leonhard Euler",
            LaplaceDemon => "Laplace's Demon",
        }
    }

    // can't return GameError::Item as it takes String
    pub fn item(&self) -> Option<&'static str> {
        match self {
            Euclid => Some("The Elements"),
            Newton => Some("The Principia"),
            Euler => Some("The Opera Omnia"),
            _ => None
        }
    }

    pub fn go(&self, direction: Direction) -> Result<Room, GameError> {
        macro_rules! m {
            ( $( ($dir:path, $room:expr) ),* ) => {
                match direction {
                    $( $dir => Ok($room), )*
                    _ => Err(GameError::Direction),
                }
            }
        }

        match self {
            PointAtInfinity => m!(
                (North, Euler),
                (West, Euclid),
                (East, Newton)
            ),
            Euclid => m!(
                (East, PointAtInfinity)
            ),
            Newton => m!(
                (North, LaplaceDemon),
                (West, PointAtInfinity)
            ),
            Euler => m!(
                (South, PointAtInfinity)
            ),
            _ => Err(GameError::Direction)
        }
    }
}
