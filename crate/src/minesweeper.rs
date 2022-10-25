use crate::utils::{cell::Cell, random::random_range};
use std::{collections::HashSet, fmt::Display};

type Position = (usize, usize);

#[derive(Debug)]
pub struct Minesweeper {
  pub width: usize,
  pub height: usize,
  mines: HashSet<Position>,
  open_cells: HashSet<Position>,
  flagged_cells: HashSet<Position>,
  depressed_cells: HashSet<Position>,
  game_over: bool,
}

impl Minesweeper {
  pub fn new(width: usize, height: usize, mines_count: usize) -> Self {
    Self {
      width,
      height,
      mines: Self::gen_rand_mines(width, height, mines_count),
      open_cells: HashSet::new(),
      flagged_cells: HashSet::new(),
      depressed_cells: HashSet::new(),
      game_over: false,
    }
  }

  pub fn reset(&mut self) {
    self.mines = Self::gen_rand_mines(self.width, self.height, self.mines.len());
    self.open_cells = HashSet::new();
    self.flagged_cells = HashSet::new();
    self.depressed_cells = HashSet::new();
    self.game_over = false;
  }

  pub fn is_game_over(&self) -> bool {
    self.game_over
  }

  pub fn is_game_finished(&self) -> bool {
    self.height * self.width - self.open_cells.len() == self.mines.len()
  }

  pub fn open(&mut self, pos: Position) {
    if self.is_game_over() || self.is_game_finished() || self.flagged_cells.contains(&pos) {
      return;
    }

    let mines_count = self.adjacent_mines_count(pos);
    let flags_count = self.adjacent_flags_count(pos);

    if self.open_cells.contains(&pos) {
      if mines_count == flags_count {
        self.open_closed_neighbors(pos);
      } else if flags_count > 0 && flags_count < mines_count {
        self.depress_neighbors(pos);
      }
      return;
    }

    self.open_cells.insert(pos);

    let is_mine = self.mines.contains(&pos);
    if is_mine {
      self.game_over = true;
      return;
    }

    if mines_count == 0 {
      self.open_closed_neighbors(pos);
    };
  }

  pub fn toggle_flag(&mut self, pos: Position) {
    if self.is_game_over() || self.is_game_finished() || self.open_cells.contains(&pos) {
      return;
    }

    if !self.flagged_cells.contains(&pos) && self.flagged_cells.len() < self.mines.len() {
      self.flagged_cells.insert(pos);
      return;
    }

    self.flagged_cells.remove(&pos);
  }

  pub fn clear_depressed_cells(&mut self) {
    if self.depressed_cells.len() < 1 {
      return;
    }

    self.depressed_cells.clear();
  }

  fn gen_rand_mines(width: usize, height: usize, mines_count: usize) -> HashSet<(usize, usize)> {
    let mut mines = HashSet::new();

    while mines.len() < mines_count {
      mines.insert((random_range(0, width), random_range(0, height)));
    }

    mines
  }

  fn adjacent_mines_count(&self, pos: Position) -> u8 {
    self
      .iter_neighbors(pos)
      .filter(|pos| self.mines.contains(pos))
      .count() as u8
  }

  fn adjacent_flags_count(&self, pos: Position) -> u8 {
    self
      .iter_neighbors(pos)
      .filter(|pos| self.flagged_cells.contains(pos))
      .count() as u8
  }

  fn open_closed_neighbors(&mut self, pos: Position) {
    for neighbor in self.iter_neighbors(pos) {
      if !self.open_cells.contains(&neighbor) {
        self.open(neighbor);
      }
    }
  }

  fn depress_neighbors(&mut self, pos: Position) {
    for neighbor in self.iter_neighbors(pos) {
      if !self.open_cells.contains(&neighbor) && !self.flagged_cells.contains(&neighbor) {
        self.depress(neighbor);
      }
    }
  }

  fn depress(&mut self, pos: Position) {
    if self.depressed_cells.contains(&pos) {
      return;
    }
    self.depressed_cells.insert(pos);
  }

  fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
    let width = self.width;
    let height = self.height;

    (x.max(1) - 1..=(x + 1).min(width - 1))
      .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
      .filter(move |&pos| pos != (x, y))
  }
}

impl Display for Minesweeper {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for y in 0..self.height {
      for x in 0..self.width {
        let pos = (x, y);

        if !self.open_cells.contains(&pos) {
          if self.game_over && self.mines.contains(&pos) {
            write!(f, "{}", Cell::Mine)?;
          } else if self.flagged_cells.contains(&pos) {
            write!(f, "{}", Cell::Flag)?;
          } else if self.depressed_cells.contains(&pos) {
            write!(f, "{}", Cell::Pressed)?;
          } else {
            write!(f, "{}", Cell::Closed)?;
          }
        } else if self.mines.contains(&pos) {
          write!(f, "{}", Cell::Exploded)?;
        } else {
          let mine_count = self.adjacent_mines_count((x, y));
          if mine_count == 0 {
            write!(f, "{}", Cell::Opened)?;
          } else {
            write!(f, "{}", Cell::Num(mine_count))?;
          }
        }
      }
      write!(f, "\n")?;
    }

    Ok(())
  }
}

pub trait MineFieldBuilder {
  fn build(&self) -> Vec<Vec<Cell>>;
}

impl MineFieldBuilder for Minesweeper {
  fn build(&self) -> Vec<Vec<Cell>> {
    let mut field = vec![vec![Cell::Closed; self.width]; self.height];

    for y in 0..self.height {
      for x in 0..self.width {
        let pos = (x, y);

        if !self.open_cells.contains(&pos) {
          if self.game_over && self.mines.contains(&pos) {
            field[y][x] = Cell::Mine;
          } else if self.flagged_cells.contains(&pos) {
            field[y][x] = Cell::Flag;
          } else if self.depressed_cells.contains(&pos) {
            field[y][x] = Cell::Pressed;
          } else {
            field[y][x] = Cell::Closed;
          }
        } else if self.mines.contains(&pos) {
          field[y][x] = Cell::Exploded;
        } else {
          let mine_count = self.adjacent_mines_count((x, y));
          if mine_count == 0 {
            field[y][x] = Cell::Opened;
          } else {
            field[y][x] = Cell::Num(mine_count);
          }
        }
      }
    }

    field
  }
}

#[cfg(test)]
mod tests {
  use crate::Minesweeper;

  #[test]
  fn test() {
    let mut ms = Minesweeper::new(10, 10, 5);
    ms.open((5, 5));
    ms.toggle_flag((0, 0));
    ms.open((0, 0));

    println!("{}", ms);
  }
}
