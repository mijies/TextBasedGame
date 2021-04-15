use std::{error::Error, fmt};

#[derive(PartialEq)]
pub enum GameError {
    Over,
    Input,
    Direction,
    Item(String),
}

impl Error for GameError {}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GameError::*;
        let e = {
            match self {
                Over => String::new(), // won't occure
                Input => "Invalid Input!".to_string(),
                Direction => "You can't go that way!".to_string(),
                Item(e) => "Can't get ".to_string() + e,
            }
        };
        write!(f, "{}", e)?;
        Ok(())
    }
}

impl fmt::Debug for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)?;
        Ok(())
    }
}
