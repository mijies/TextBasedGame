use crate::error::GameError;
use crate::item::{ItemList, Item};
use crate::room::Room;
use super::*;

#[test]
fn check_item_test() -> Result<(), GameError>{
    assert_eq!(
        check_item(&mut Game::new())?, ()
    );

    let mut items = ItemList::new(Vec::new());
    for _ in 1..Room::item_count() {
        items.push((Room::Euler, "item" as Item));
    }
    assert_eq!(
        check_item(&mut Game {
            room: Room::PointAtInfinity,
            items: items,
        })?, ()
    );

    let mut items = ItemList::new(Vec::new());
    for _ in 0..Room::item_count() {
        items.push((Room::Euler, "item" as Item));
    }
    assert_eq!(
        check_item(&mut Game {
            room: Room::PointAtInfinity,
            items: items,
        }), Err(GameError::Over)
    );

    Ok(())
}