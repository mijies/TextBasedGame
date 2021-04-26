use crate::error::GameError;
use super::*;

#[test]
fn check_item_test() -> Result<(), GameError>{
    let mut game = Game::new();
    assert_eq!(check_item(&mut game)?, ());
    Ok(())
}