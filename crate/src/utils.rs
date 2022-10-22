use std::fmt;

pub trait MineFieldBuilder {
  fn build(&self) -> Vec<Vec<FieldType>>;
}

pub trait FieldToTexturePath {
  fn to_path(&self) -> String;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FieldType {
  Num(u8),
  Flag,
  Mine,
  Pressed,
  Closed,
  Opened,
  Exploded,
}

impl fmt::Display for FieldType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      FieldType::Num(num) => write!(f, " {} ", num),
      FieldType::Flag => write!(f, "{} ", "🚩"),
      FieldType::Mine => write!(f, "{} ", "💣"),
      FieldType::Pressed => write!(f, "{} ", "🔲"),
      FieldType::Closed => write!(f, "{} ", "⬛"),
      FieldType::Opened => write!(f, "{} ", "⬜"),
      FieldType::Exploded => write!(f, "{} ", "💥"),
    }
  }
}

impl FieldToTexturePath for FieldType {
  fn to_path(&self) -> String {
    let path = match *self {
      FieldType::Num(num) => format!("type{}.svg", num),
      FieldType::Flag => format!("flag.svg"),
      FieldType::Mine => format!("mine.svg"),
      FieldType::Pressed => format!("pressed.svg"),
      FieldType::Closed => format!("closed.svg"),
      FieldType::Opened => format!("type0.svg"),
      FieldType::Exploded => format!("mine_red.svg"),
    };

    format!("img/{}", path)
  }
}
