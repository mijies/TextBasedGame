use crate::room::{Room, Direction};
use crate::item::{Item, ItemList};
use crate::error::GameError;
use regex::Regex;
use std::io;
use textgame_macro::decorate;
use super::message::*;

pub struct Game {
    room: Room,
    items: ItemList,
}

#[decorate(message_input)]
fn run_next(game: &mut Game) -> Result<(), GameError> {
    if let Some(caps) = Regex::new(r"(go|get) (.+)")
        .unwrap().captures(&read_line(io::stdin().lock())) {

        return run_cmd(
            game,
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str()
        )
    }
    Err(GameError::Input)
}

fn read_line<R>(mut reader: R) -> String
    where R: io::BufRead,
{
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    s
}

fn run_cmd(game: &mut Game, cmd: &str, arg: &str) -> Result<(), GameError> {
    match cmd {
        "go" => go(game, arg),
        _ => get_item(game, arg)
            .and_then(|_| check_item(game)),
    }
}

fn go(game: &mut Game, direction: &str) -> Result<(), GameError> {
    game.room = game.room.go(Direction::new(direction)?)?;

    if game.room == Room::LaplaceDemon {
        return Err(GameError::Over)
    }
    Ok(())
}

#[decorate(message_item)]
fn get_item(game: &mut Game, arg: &str) -> Result<(), GameError> {
    if game.items.contains(&(game.room, arg)) {
        return Err(GameError::Item(arg.to_string()))
    }

    if let Some(item) = game.room.item() {
        if item == arg {
            game.items.push((game.room, item));
            return Ok(())
        }
    }
    Err(GameError::Item(arg.to_string()))
}

fn check_item(game: &mut Game) -> Result<(), GameError> {
    if game.items.len() == Room::item_count() {
        return Err(GameError::Over);
    }
    return Ok(())
}

impl Game {
    #[decorate(message_start)]
    pub fn new() -> Game {
        Game {
            room: Room::PointAtInfinity,
            items: ItemList::new(Vec::new()),
        }
    }

    pub fn run(&mut self) -> &mut Self {
        self.message_status();

        if let Err(e) = run_next(self) {
            if e == GameError::Over {
                return self
            }
            println!("{}", e);
        }
        self.run()
    }

    pub fn win_or_loss(&mut self) {
        if self.items.len() == Room::item_count() {
            println!("");
            println!("Congratulations! You have collected all items!");
        } else {
            self.message_status();
            println!("NOM NOM...GAME OVER!");
        }
        println!("Thanks for playing the game. Hope you enjoyed it.");
    }

    pub fn last_item(&self) -> Item {
        let (_, ref item) = self.items.last().unwrap();
        item
    }

    fn message_status(&mut self) {
        println!("");
        println!("You are in the {}", self.room.name());
        println!("Inventory : {}", self.items);

        if let Some(item) = self.room.item() {
            if !self.items.contains(&(self.room, item)) {
                println!("You see {}", item);
            }
        }
    }
}

#[cfg(test)]
#[path = "./game_test.rs"]
mod game_test;