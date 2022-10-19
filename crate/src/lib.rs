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
    render();

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
    build_header_elem()?;
    let grid_section = build_grid_section_elem(&data)?;

    // Manufacture the field we're gonna append
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            let elem = DOCUMENT.with(|document| document.borrow_mut().create_element("a"))?;
            elem.set_class_name("cell");
            elem.set_attribute("href", "#")?;
            elem.set_inner_html(data[y][x]);

            let listener = Closure::wrap(Box::new(move || {
                MINESWEEPER.with(|ms| ms.borrow_mut().open((x, y)));
                render();
            }) as Box<dyn FnMut()>);

            elem.add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())?;

            grid_section.append_child(&elem)?;

            listener.forget();
        }
    }

    Ok(())
}

fn build_header_elem() -> Result<Element, JsValue> {
    let header = DOCUMENT.with(|document| document.borrow_mut().create_element("header"))?;
    header.set_id("grid");

    let listener = Closure::wrap(Box::new(move || {
        MINESWEEPER.with(|ms| ms.borrow_mut().reset());
        render();
    }) as Box<dyn FnMut()>);
    let button = DOCUMENT.with(|document| document.borrow_mut().create_element("button"))?;
    button.set_id("reset");
    button.set_inner_html(&String::from("Reset"));
    button.add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())?;

    header.append_child(&button)?;
    listener.forget();

    let win_status = DOCUMENT.with(|document| document.borrow_mut().create_element("h1"))?;
    win_status.set_id("win_status");
    MINESWEEPER.with(|ms| {
        if ms.borrow().is_game_over() {
            win_status.set_attribute("style", &String::from("color: red"));
            win_status.set_inner_html(&String::from("You lost 😞"));
        } else if ms.borrow().is_game_finished() {
            win_status.set_attribute("style", &String::from("color: green"));
            win_status.set_inner_html(&String::from("You won 😎"));
        }
    });
    header.append_child(&win_status)?;

    ROOT.with(|root| root.borrow_mut().append_child(&header))?;
    Ok(header)
}

fn build_grid_section_elem(data: &Vec<Vec<&str>>) -> Result<Element, JsValue> {
    let section = DOCUMENT.with(|document| document.borrow_mut().create_element("section"))?;
    let section_style = format!(
        "display:inline-grid; grid-template:repeat({}, auto)/repeat({}, auto)",
        data.len(),
        data[0].len()
    );

    section.set_id("grid");
    section.set_attribute("style", &section_style[..])?;

    ROOT.with(|root| root.borrow_mut().append_child(&section))?;
    Ok(section)
}
