mod minesweeper;
mod render;
mod utils;

use minesweeper::Minesweeper;
use std::cell::RefCell;

#[macro_use]
extern crate cfg_if;

extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

use crate::utils::view::View;

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
    static VIEW: RefCell<View> = RefCell::new(View::new());
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
  // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
  // it will do nothing.
  set_panic_hook();

  // Render content
  render::render()?;

  Ok(())
}
