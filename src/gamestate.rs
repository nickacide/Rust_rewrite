use std::str::FromStr;

// use crate::square::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameState {
    castling: (bool, bool, bool, bool),
    en_pessant: Option<usize>,
}
pub struct ParseFenError;
impl FromStr for GameState {
    type Err = ParseFenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<&str> = s.split(" ").collect();
        let castle_rights = parsed[2].to_owned();
        let mut castling = (false, false, false, false);
        castling.0 = castle_rights.contains("Q");
        castling.1 = castle_rights.contains("K");
        castling.2 = castle_rights.contains("q");
        castling.3 = castle_rights.contains("k");
        let en_pessant = match parsed[3] {
            "-" => None,
            str => {
                let mut chars = str.chars();
                let file = chars.next().unwrap() as usize - 97; // 'a'
                let rank = chars.next().unwrap() as usize - 49; // '1'
                Some(file + 8 * rank)
            }
        };
        Ok(GameState {
            castling,
            en_pessant,
        })
    }
}
