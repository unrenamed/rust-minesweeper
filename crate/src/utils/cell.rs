use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cell {
  Num(u8),
  Flag,
  Mine,
  Pressed,
  Closed,
  Opened,
  Exploded,
}

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Cell::Num(num) => write!(f, " {} ", num),
      Cell::Flag => write!(f, "{} ", "🚩"),
      Cell::Mine => write!(f, "{} ", "💣"),
      Cell::Pressed => write!(f, "{} ", "🔲"),
      Cell::Closed => write!(f, "{} ", "⬛"),
      Cell::Opened => write!(f, "{} ", "⬜"),
      Cell::Exploded => write!(f, "{} ", "💥"),
    }
  }
}

pub trait ConvertToTexture {
  fn to_texture_path(&self) -> String;
}

impl ConvertToTexture for Cell {
  fn to_texture_path(&self) -> String {
    let path = match *self {
      Cell::Num(num) => format!("type{}.svg", num),
      Cell::Flag => format!("flag.svg"),
      Cell::Mine => format!("mine.svg"),
      Cell::Pressed => format!("pressed.svg"),
      Cell::Closed => format!("closed.svg"),
      Cell::Opened => format!("type0.svg"),
      Cell::Exploded => format!("mine_red.svg"),
    };

    format!("img/{}", path)
  }
}
