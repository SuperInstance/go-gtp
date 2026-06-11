//! Go Text Protocol (GTP) command parser and basic board.
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stone { Black, White }
impl Stone {
    pub fn opposite(&self) -> Self { match self { Stone::Black => Stone::White, Stone::White => Stone::Black } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vertex(pub u8, pub u8);

#[derive(Debug, Clone)]
pub struct GtpCommand {
    pub id: Option<u32>,
    pub name: String,
    pub args: Vec<String>,
}

impl std::str::FromStr for GtpCommand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let line = s.trim().split('#').next().unwrap_or("").trim();
        if line.is_empty() { return Err(()); }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let (id, rest) = if tokens[0].chars().all(|c| c.is_ascii_digit()) && !tokens[0].is_empty() {
            (Some(tokens[0].parse().map_err(|_| ())?), &tokens[1..])
        } else { (None, &tokens[..]) };
        if rest.is_empty() { return Err(()); }
        Ok(GtpCommand {
            id,
            name: rest[0].to_lowercase(),
            args: rest[1..].iter().map(|s| s.to_string()).collect(),
        })
    }
}

pub const BOARD_SIZE: usize = 19;

#[derive(Debug, Clone)]
pub struct GoBoard {
    grid: [[Option<Stone>; BOARD_SIZE]; BOARD_SIZE],
}

impl GoBoard {
    pub fn new() -> Self { Self { grid: [[None; BOARD_SIZE]; BOARD_SIZE] } }
    pub fn place(&mut self, v: Vertex, s: Stone) -> Result<(), &'static str> {
        let Vertex(x, y) = v;
        if self.grid[y as usize][x as usize].is_some() { return Err("occupied"); }
        self.grid[y as usize][x as usize] = Some(s);
        Ok(())
    }
    pub fn get(&self, v: Vertex) -> Option<Stone> {
        let Vertex(x, y) = v;
        self.grid[y as usize][x as usize]
    }
    pub fn clear(&mut self) {
        self.grid = [[None; BOARD_SIZE]; BOARD_SIZE];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_command() {
        let cmd: GtpCommand = "1 play black D4".parse().unwrap();
        assert_eq!(cmd.id, Some(1));
        assert_eq!(cmd.name, "play");
        assert_eq!(cmd.args, vec!["black", "D4"]);
    }
}
