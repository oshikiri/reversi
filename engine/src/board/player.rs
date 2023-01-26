use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub enum Player {
    First,
    Second,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::First => Player::Second,
            Player::Second => Player::First,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::player::Player;
    #[test]
    fn opponent() {
        let first = Player::First;
        assert_eq!(first.opponent().clone(), Player::Second);

        let second = Player::Second;
        assert_eq!(second.opponent(), Player::First);
    }
}
