use std::{fmt, str::FromStr};

use wasm_bindgen::JsValue;
use web_sys::Element;

use crate::{
  render,
  utils::{cell::Cell, cell::ConvertToTexture},
};

const CLASSIC_VIEW_TYPE_NAME: &str = "classic";
const TERMINAL_VIEW_TYPE_NAME: &str = "terminal";

#[derive(Debug, Clone, Copy)]
pub enum ViewType {
  Terminal,
  Classic,
}

impl fmt::Display for ViewType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ViewType::Classic => write!(f, "{}", CLASSIC_VIEW_TYPE_NAME),
      ViewType::Terminal => write!(f, "{}", TERMINAL_VIEW_TYPE_NAME),
    }
  }
}

impl FromStr for ViewType {
  type Err = ();

  fn from_str(s: &str) -> Result<ViewType, ()> {
    match s {
      CLASSIC_VIEW_TYPE_NAME => Ok(ViewType::Classic),
      TERMINAL_VIEW_TYPE_NAME => Ok(ViewType::Terminal),
      _ => Ok(ViewType::Classic),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct TerminalView;
#[derive(Debug, Clone, Copy)]
pub struct ClassicView;

pub trait CellWasmElement {
  fn get_cell_elem(field_type: &Cell) -> Result<Element, JsValue>;
}

impl CellWasmElement for TerminalView {
  fn get_cell_elem(cell: &Cell) -> Result<Element, JsValue> {
    let elem = render::create_dom_element("a")?;
    elem.set_class_name("cell");
    elem.set_attribute("href", "#")?;
    elem.set_inner_html(&cell.to_string());

    Ok(elem)
  }
}

impl CellWasmElement for ClassicView {
  fn get_cell_elem(cell: &Cell) -> Result<Element, JsValue> {
    let elem = render::create_dom_element("div")?;
    elem.set_class_name("cell");
    let style = format!(
      "width:24px; height:24px; background:center / contain url({})",
      cell.to_texture_path(),
    );
    elem.set_attribute("style", &style[..])?;
    elem.set_inner_html("");

    Ok(elem)
  }
}

pub struct View {
  _type: ViewType,
}

impl View {
  pub fn new() -> Self {
    Self {
      _type: ViewType::Terminal,
    }
  }

  pub fn get_type(&mut self) -> ViewType {
    self._type
  }

  pub fn switch(&mut self, _type: ViewType) {
    self._type = _type;
  }

  pub fn get_cell_elem(&mut self, field_type: &Cell) -> Result<Element, JsValue> {
    match self.get_type() {
      ViewType::Classic => ClassicView::get_cell_elem(field_type),
      ViewType::Terminal => TerminalView::get_cell_elem(field_type),
    }
  }
}
