use crate::error::GameError;
use crate::item::{ItemList, Item};
use crate::room::Room;
use super::*;

macro_rules! items_pushed {
    ($rm:expr, $im:expr, $num:expr) => {{
        let mut items = ItemList::new(Vec::new());
        for _ in $num..Room::item_count() {
            items.push(($rm, $im as Item));
        }
        items
    }};
}

macro_rules! items_dummy {
    ($num:expr) => {
        items_pushed!(Room::Euler, "item", $num)
    };
}


#[test]
fn check_item_test() -> Result<(), GameError> {
    let game = &mut Game::new();
    
    assert_eq!(check_item(game)?, ());

    game.items = items_dummy!(1);
    assert_eq!(check_item(game)?, ());

    game.items = items_dummy!(0);
    assert_eq!(check_item(game), Err(GameError::Over));

    Ok(())
}

#[test]
fn get_item_test() -> Result<(), GameError> {
    let game = &mut Game::new();

    assert_eq!(
        get_item(game, "PointAtInfinity has none"),
        Err(GameError::Item(
            "PointAtInfinity has none".to_owned()
        ))
    );

    game.room = Room::Euclid;
    assert_eq!(
        get_item(game, "invalid"),
        Err(GameError::Item("invalid".to_owned()))
    );

    assert_eq!(
        get_item(game, "The Elements")?, ()
    );

    assert_eq!(
        get_item(game, "The Elements"),
        Err(GameError::Item("The Elements".to_owned()))
    );

    Ok(())
}

#[test]
fn go_test() -> Result<(), GameError> {
    use GameError::{Direction, Over};
    use Room::*;
    let game = &mut Game::new();

    assert_eq!(go(game, "Foo"), Err(Direction));
    assert_eq!(game.room, PointAtInfinity);

    assert_eq!(go(game, "South"), Err(Direction));
    assert_eq!(game.room, PointAtInfinity);

    assert_eq!(go(game, "East")?, ());
    assert_eq!(game.room, Newton);

    assert_eq!(go(game, "North"), Err(Over));
    Ok(())
}

#[test]
fn run_cmd_test() -> Result<(), GameError> {
    use GameError::{Direction, Over};
    let game = &mut Game::new();

    assert_eq!(run_cmd(game, "go", "Foo"), Err(Direction));
    assert_eq!(run_cmd(game, "go", "South"), Err(Direction));
    assert_eq!(run_cmd(game, "go", "East")?, ());
    assert_eq!(run_cmd(game, "go", "North"), Err(Over));
 
    game.room = Room::Newton;
    assert_eq!(run_cmd(game, "get", "Foo"), Err(GameError::Item("Foo".to_owned())));
    assert_eq!(run_cmd(game, "get", "The Principia")?, ());
    assert_eq!(run_cmd(game, "get", "The Principia"), Err(GameError::Item("The Principia".to_owned())));
 
    assert_eq!(run_cmd(game, "Bar", "Foo"), Err(GameError::Item("Foo".to_owned())));
    Ok(())
}