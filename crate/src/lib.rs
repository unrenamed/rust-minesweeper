mod minesweeper;
mod random;
mod utils;

use std::{cell::RefCell, fmt, str::FromStr};

use minesweeper::Minesweeper;

#[macro_use]
extern crate cfg_if;

extern crate wasm_bindgen;
extern crate web_sys;
use utils::{FieldToTexturePath, FieldType, MineFieldBuilder};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, Element, Event, MouseEvent};

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

thread_local! {
    static MINESWEEPER: RefCell<Minesweeper> = RefCell::new(Minesweeper::new(20, 20, 30));
    static DOCUMENT: RefCell<Document> = RefCell::new(
        web_sys::window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document on window"));
    static ROOT: RefCell<Element> = RefCell::new(
        DOCUMENT.with(|document| document.borrow().get_element_by_id("root").expect("document should have a root div"))
    );
    static APP: RefCell<App> = RefCell::new(App::new());
}

#[derive(Debug, Clone, Copy)]
enum ViewType {
  Terminal,
  Classic,
}

impl fmt::Display for ViewType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ViewType::Classic => write!(f, "classic"),
      ViewType::Terminal => write!(f, "terminal"),
    }
  }
}

impl FromStr for ViewType {
  type Err = ();

  fn from_str(s: &str) -> Result<ViewType, ()> {
    match s {
      "classic" => Ok(ViewType::Classic),
      "terminal" => Ok(ViewType::Terminal),
      _ => Ok(ViewType::Classic),
    }
  }
}

#[derive(Debug, Clone, Copy)]
struct TerminalView;
#[derive(Debug, Clone, Copy)]
struct ClassicView;

trait SquareElementCreate {
  fn create(field_type: &FieldType) -> Result<Element, JsValue>;
}

impl SquareElementCreate for TerminalView {
  fn create(cell: &FieldType) -> Result<Element, JsValue> {
    let elem = create_dom_element("a")?;
    elem.set_class_name("cell");
    elem.set_attribute("href", "#")?;
    elem.set_inner_html(&cell.to_string());

    Ok(elem)
  }
}

impl SquareElementCreate for ClassicView {
  fn create(cell: &FieldType) -> Result<Element, JsValue> {
    let elem = create_dom_element("div")?;
    elem.set_class_name("cell");
    let style = format!(
      "width:24px; height:24px; background:center / contain url({})",
      cell.to_path(),
    );
    elem.set_attribute("style", &style[..])?;
    elem.set_inner_html("");

    Ok(elem)
  }
}

struct App {
  view: ViewType,
}

impl App {
  fn new() -> Self {
    Self {
      view: ViewType::Terminal,
    }
  }

  fn view(&mut self) -> ViewType {
    self.view
  }

  fn switch(&mut self, view: ViewType) {
    self.view = view;
  }

  fn create(&mut self, field_type: &FieldType) -> Result<Element, JsValue> {
    match self.view() {
      ViewType::Classic => ClassicView::create(field_type),
      ViewType::Terminal => TerminalView::create(field_type),
    }
  }
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
  // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
  // it will do nothing.
  set_panic_hook();

  // Render content
  render()?;

  Ok(())
}

fn render() -> Result<(), JsValue> {
  // Clear root content
  ROOT.with(|root| root.borrow_mut().set_inner_html(""));

  let ms_field = MINESWEEPER.with(|ms| ms.borrow().build());

  // Set up inner HTML
  render_header()?;
  let grid_section = render_grid_section(&ms_field)?;

  // Manufacture the field we're gonna append
  for y in 0..ms_field.len() {
    for x in 0..ms_field[y].len() {
      let cell = APP.with(|app| app.borrow_mut().create(&ms_field[y][x]))?;
      add_mousedown_listener_to_cell(&cell, (x, y))?;
      add_mouseup_listener_to_cell(&cell)?;
      add_contexmenu_listener_to_cell(&cell)?;
      grid_section.append_child(&cell)?;
    }
  }

  Ok(())
}

fn render_header() -> Result<Element, JsValue> {
  let header = create_dom_element("header")?;
  header.set_id("grid");

  let reset_button = get_reset_button_elem()?;
  let win_status = get_win_status_elem()?;
  let view_select = get_view_select()?;
  header.append_child(&reset_button)?;
  header.append_child(&win_status)?;
  header.append_child(&view_select)?;

  append_child_to_root(&header)?;
  Ok(header)
}

fn render_grid_section(data: &Vec<Vec<FieldType>>) -> Result<Element, JsValue> {
  let section = create_dom_element("section")?;
  let section_style = format!(
    "display:inline-grid; grid-template:repeat({}, auto)/repeat({}, auto)",
    data.len(),
    data[0].len()
  );

  section.set_id("grid");
  section.set_attribute("style", &section_style[..])?;

  append_child_to_root(&section)?;
  Ok(section)
}

fn get_reset_button_elem() -> Result<Element, JsValue> {
  let button = create_dom_element("button")?;
  button.set_id("reset");
  button.set_inner_html(&String::from("Reset"));
  add_click_listener_to_reset_button(&button)?;
  Ok(button)
}

fn get_win_status_elem() -> Result<Element, JsValue> {
  let win_status = create_dom_element("h1")?;
  win_status.set_id("win_status");

  MINESWEEPER.with(|ms| {
    if ms.borrow().is_game_over() {
      win_status.set_attribute("style", "color: red").unwrap();
      win_status.set_inner_html(&String::from("You lose 😞"));
    } else if ms.borrow().is_game_finished() {
      win_status.set_attribute("style", "color: green").unwrap();
      win_status.set_inner_html(&String::from("You win! 😎"));
    }
  });

  Ok(win_status)
}

fn get_view_select() -> Result<Element, JsValue> {
  let select = create_dom_element("select")?;
  select.set_attribute("name", "view_type")?;
  select.set_id("view_type");

  let option_classic = create_dom_element("option")?;
  option_classic.set_attribute("value", &ViewType::Classic.to_string())?;
  option_classic.set_inner_html("Classic");

  let option_terminal = create_dom_element("option")?;
  option_terminal.set_attribute("value", &ViewType::Terminal.to_string())?;
  option_terminal.set_inner_html("Terminal");

  select.append_child(&option_terminal)?;
  select.append_child(&option_classic)?;

  let view_type = APP.with(|app| app.borrow_mut().view());
  match view_type {
    ViewType::Classic => option_classic.set_attribute("selected", "true")?,
    ViewType::Terminal => option_terminal.set_attribute("selected", "true")?,
  };

  let listener = Closure::wrap(Box::new(move |e: Event| {
    e.prevent_default();
    if let Some(input) = e.target().unwrap().dyn_ref::<web_sys::HtmlSelectElement>() {
      let view_type = ViewType::from_str(&input.value()[..]).unwrap();
      APP.with(|app| app.borrow_mut().switch(view_type));
      render().unwrap();
    }
  }) as Box<dyn FnMut(Event)>);
  select.add_event_listener_with_callback("change", listener.as_ref().unchecked_ref())?;
  listener.forget();

  Ok(select)
}

fn append_child_to_root(child: &Element) -> Result<(), JsValue> {
  ROOT.with(|root| root.borrow_mut().append_child(child))?;
  Ok(())
}

fn add_click_listener_to_reset_button(elem: &Element) -> Result<(), JsValue> {
  let listener = Closure::wrap(Box::new(move |e: MouseEvent| {
    e.prevent_default();
    MINESWEEPER.with(|ms| ms.borrow_mut().reset());
    render().unwrap();
  }) as Box<dyn FnMut(MouseEvent)>);

  register_event_listener(elem, "click", listener)?;
  Ok(())
}

fn add_mousedown_listener_to_cell(elem: &Element, pos: (usize, usize)) -> Result<(), JsValue> {
  let listener = Closure::wrap(Box::new(move |e: MouseEvent| {
    e.prevent_default();
    match e.button() {
      0 => MINESWEEPER.with(|ms| ms.borrow_mut().open(pos)),
      2 => MINESWEEPER.with(|ms| ms.borrow_mut().toggle_flag(pos)),
      _ => (),
    }
    render().unwrap();
  }) as Box<dyn FnMut(MouseEvent)>);

  register_event_listener(elem, "mousedown", listener)?;
  Ok(())
}

fn add_mouseup_listener_to_cell(elem: &Element) -> Result<(), JsValue> {
  let listener = Closure::wrap(Box::new(move |e: MouseEvent| {
    e.prevent_default();
    MINESWEEPER.with(|ms| ms.borrow_mut().clear_depressed_fields());
    render().unwrap();
  }) as Box<dyn FnMut(MouseEvent)>);

  register_event_listener(elem, "mouseup", listener)?;
  Ok(())
}

fn add_contexmenu_listener_to_cell(elem: &Element) -> Result<(), JsValue> {
  let listener = Closure::wrap(Box::new(move |e: MouseEvent| {
    e.prevent_default();
  }) as Box<dyn FnMut(MouseEvent)>);

  register_event_listener(elem, "contextmenu", listener)?;
  Ok(())
}

fn create_dom_element(local_name: &str) -> Result<Element, JsValue> {
  DOCUMENT.with(|document| document.borrow_mut().create_element(local_name))
}

fn register_event_listener(
  elem: &Element,
  event: &str,
  listener: Closure<dyn FnMut(MouseEvent)>,
) -> Result<(), JsValue> {
  elem.add_event_listener_with_callback(event, listener.as_ref().unchecked_ref())?;
  listener.forget();
  Ok(())
}
