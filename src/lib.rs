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

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
