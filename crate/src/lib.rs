mod minesweeper;
mod random;

use std::cell::RefCell;

use minesweeper::Minesweeper;

#[macro_use]
extern crate cfg_if;

extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::Element;

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

    // Get field data
    let ms_string = MINESWEEPER.with(|ms| ms.borrow().to_string());
    let data: Vec<Vec<&str>> = ms_string
        .split('\n')
        .map(|s| {
            s.trim()
                .split(char::is_whitespace)
                .filter(|ch| ch != &"")
                .collect::<Vec<&str>>()
        })
        .filter(|vec| !Vec::is_empty(vec))
        .collect();

    // Set up inner HTML
    render_header()?;
    let grid_section = render_grid_section(&data)?;

    // Manufacture the field we're gonna append
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            let cell = get_cell_elem(data[y][x])?;
            add_click_listener_to_cell(&cell, (x, y))?;
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
    header.append_child(&reset_button)?;
    header.append_child(&win_status)?;

    append_child_to_root(&header)?;
    Ok(header)
}

fn render_grid_section(data: &Vec<Vec<&str>>) -> Result<Element, JsValue> {
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
            win_status
                .set_attribute("style", &String::from("color: red"))
                .unwrap();
            win_status.set_inner_html(&String::from("You lost 😞"));
        } else if ms.borrow().is_game_finished() {
            win_status
                .set_attribute("style", &String::from("color: green"))
                .unwrap();
            win_status.set_inner_html(&String::from("You won 😎"));
        }
    });

    Ok(win_status)
}

fn get_cell_elem(cell_content: &str) -> Result<Element, JsValue> {
    let elem = create_dom_element("a")?;
    elem.set_class_name("cell");
    elem.set_attribute("href", "#")?;
    elem.set_inner_html(cell_content);

    Ok(elem)
}

fn append_child_to_root(child: &Element) -> Result<(), JsValue> {
    ROOT.with(|root| root.borrow_mut().append_child(child))?;
    Ok(())
}

fn add_click_listener_to_reset_button(elem: &Element) -> Result<(), JsValue> {
    let listener = Closure::wrap(Box::new(move || {
        MINESWEEPER.with(|ms| ms.borrow_mut().reset());
        render().unwrap();
    }) as Box<dyn FnMut()>);

    register_event_listener(elem, "click", listener)?;
    Ok(())
}

fn add_click_listener_to_cell(elem: &Element, pos: (usize, usize)) -> Result<(), JsValue> {
    let listener = Closure::wrap(Box::new(move || {
        MINESWEEPER.with(|ms| ms.borrow_mut().open(pos));
        render().unwrap();
    }) as Box<dyn FnMut()>);

    register_event_listener(elem, "click", listener)?;
    Ok(())
}

fn create_dom_element(local_name: &str) -> Result<Element, JsValue> {
    DOCUMENT.with(|document| document.borrow_mut().create_element(local_name))
}

fn register_event_listener(
    elem: &Element,
    event: &str,
    listener: Closure<dyn FnMut()>,
) -> Result<(), JsValue> {
    elem.add_event_listener_with_callback(event, listener.as_ref().unchecked_ref())?;
    listener.forget();
    Ok(())
}
