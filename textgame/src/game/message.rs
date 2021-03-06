use crate::error::GameError;
use crate::room::Room;
use super::game::Game;

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

pub fn message_item<F>(f: F, game: &mut Game, arg: &str) -> Result<(), GameError>
    where F: Fn(&mut Game, &str) -> Result<(), GameError>
{
    f(game, arg)?;
    println!("{} retrieved!", game.last_item());
    Ok(())
}
