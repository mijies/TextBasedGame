use crate::game::Game;
use crate::room::Room;
use crate::error::GameError;

pub fn message_start<F>(f: F) -> Game
    where F: Fn() -> Game
{
    println!("Historical Math & Physics Text Game");
    println!("Collect { } items to win the game, or be captured by Laplace's demon.", Room::item_count());
    println!("");
    println!("Move commands: go South, go North, go East, go West");
    println!("Add to Inventory: get 'item name'");
    f()
}

pub fn message_input<F>(f: F, game: &mut Game) -> Result<(), GameError>
    where F: Fn(&mut Game) -> Result<(), GameError>
{
    println!("---------------------------");
    println!("Enter your move:");
    f(game)
}

pub fn message_item<F>(f: F, game: &mut Game) -> Result<(), GameError>
    where F: Fn(&mut Game) -> Result<(), GameError>
{
    // let (_, ref item) = game.items.last().unwrap();
    // let (_, ref item) = game.last_item().unwrap();
    println!("{} retrieved!", game.last_item());
    f(game)
}
