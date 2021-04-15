use crate::room::Room;
use std::ops::{Deref, DerefMut};
use std::fmt;

type Item = &'static str;
pub struct ItemList(Vec<(Room, Item)>);

impl ItemList {
    pub fn new(items: Vec<(Room, Item)>) -> ItemList {
        ItemList(items)
    }
}

impl Deref for ItemList {
    type Target = Vec<(Room, Item)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ItemList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for ItemList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut items = self.iter();
        let mut sep = false;

        write!(f, "[")?;
        while let Some((_, item)) = items.next() {
            if sep {
                write!(f, ", '{}'", item)?;
            } else {
                write!(f, "'{}'", item)?;
                sep = true;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}