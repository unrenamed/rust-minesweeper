use crate::utils::{cell::Cell, view::ViewType};
use crate::{minesweeper::MineFieldBuilder, DOCUMENT, MINESWEEPER, ROOT, VIEW};
use std::str::FromStr;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, Event, MouseEvent};

pub fn render() -> Result<(), JsValue> {
  // Clear root content
  ROOT.with(|root| root.borrow_mut().set_inner_html(""));

  let ms_field = MINESWEEPER.with(|ms| ms.borrow().build());

  // Set up inner HTML
  render_header()?;
  let grid_section = render_grid_section(&ms_field)?;

  // Manufacture the field we're gonna append
  for y in 0..ms_field.len() {
    for x in 0..ms_field[y].len() {
      let cell = VIEW.with(|app| app.borrow_mut().get_cell_elem(&ms_field[y][x]))?;
      add_mousedown_listener_to_cell(&cell, (x, y))?;
      add_mouseup_listener_to_cell(&cell)?;
      add_contexmenu_listener_to_cell(&cell)?;
      grid_section.append_child(&cell)?;
    }
  }

  Ok(())
}

pub fn create_dom_element(local_name: &str) -> Result<Element, JsValue> {
  DOCUMENT.with(|document| document.borrow_mut().create_element(local_name))
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

fn render_grid_section(data: &Vec<Vec<Cell>>) -> Result<Element, JsValue> {
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

  let view_type = VIEW.with(|app| app.borrow_mut().get_type());
  match view_type {
    ViewType::Classic => option_classic.set_attribute("selected", "true")?,
    ViewType::Terminal => option_terminal.set_attribute("selected", "true")?,
  };

  let listener = Closure::wrap(Box::new(move |e: Event| {
    e.prevent_default();
    if let Some(input) = e.target().unwrap().dyn_ref::<web_sys::HtmlSelectElement>() {
      let view_type = ViewType::from_str(&input.value()[..]).unwrap();
      VIEW.with(|app| app.borrow_mut().switch(view_type));
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
    MINESWEEPER.with(|ms| ms.borrow_mut().clear_depressed_cells());
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

fn register_event_listener(
  elem: &Element,
  event: &str,
  listener: Closure<dyn FnMut(MouseEvent)>,
) -> Result<(), JsValue> {
  elem.add_event_listener_with_callback(event, listener.as_ref().unchecked_ref())?;
  listener.forget();
  Ok(())
}
