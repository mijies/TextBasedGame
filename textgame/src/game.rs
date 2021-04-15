use crate::room::{Room, Direction};
use crate::item::ItemList;
use crate::error::GameError;
use regex::Regex;
use std::io;

pub struct Game {
    room: Room,
    items: ItemList,
}

pub fn run_next(game: &mut Game) -> Result<(), GameError> {
    game.message_input();

    if let Some(caps) = Regex::new(r"(go|get) (.+)")
        .unwrap().captures(&read_line()) {

        return run_cmd(
            game,
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str()
        )
    }
    Err(GameError::Input)
}

fn read_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}

fn run_cmd(game: &mut Game, cmd: &str, arg: &str) -> Result<(), GameError> {
    match cmd {
        "go" => go(game, arg),
        _ => get_item(game, arg),
    }
}

fn go(game: &mut Game, direction: &str) -> Result<(), GameError> {
    game.room = game.room.go(Direction::new(direction)?)?;

    if game.room == Room::LaplaceDemon {
        game.message_status();
        return Err(GameError::Over)
    }
    Ok(())
}

fn get_item(game: &mut Game, arg: &str) -> Result<(), GameError> {
    if own_item(game) {
        return Err(GameError::Item(arg.to_string()))
    }

    if let Some(item) = game.room.item() {
        if item == arg {
            game.items.push((game.room, item));
            game.message_item();

            if game.items.len() == Room::item_count() {
                return Err(GameError::Over);
            }
            return Ok(())
        }
    }
    Err(GameError::Item(arg.to_string()))
}

fn own_item(game: &mut Game) -> bool {
    let mut items = game.items.iter();

    while let Some((room, _)) = items.next() {
        if room == &game.room {
            return true
        }
    }
    false
}

impl Game {
    pub fn new() -> Game {
        Game {
            room: Room::PointAtInfinity,
            items: ItemList::new(Vec::new()),
        }
    }

    pub fn message_start(&self) {
        println!("Historical Math & Physics Text Game");
        println!("Collect { } items to win the game, or be captured by Laplace's demon.", Room::item_count());
        println!("");
        println!("Move commands: go South, go North, go East, go West");
        println!("Add to Inventory: get 'item name'");
    }

    pub fn message_status(&mut self) {
        println!("");
        println!("You are in the {}", self.room.name());
        println!("Inventory : {}", self.items);

        if let Some(item) = self.room.item() {
            if !own_item(self) {
                println!("You see {}", item);
            }
        }
    }

    pub fn message_input(&self) {
        println!("---------------------------");
        println!("Enter your move:");
    }

    pub fn message_item(&self) {
        let (_, ref item) = self.items.last().unwrap();
        println!("{} retrieved!", item);
    }

    pub fn message_end(&self) {
        println!("Thanks for playing the game. Hope you enjoyed it.");
    }

    pub fn win_or_loss(&self) {
        if self.items.len() == Room::item_count() {
            println!("");
            println!("Congratulations! You have collected all items!");
        } else {
            println!("NOM NOM...GAME OVER!");
        }
    }
}
