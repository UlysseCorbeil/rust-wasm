#![recursion_limit = "256"]

extern crate cfg_if;
extern crate wasm_bindgen;

mod api;
mod app;
mod components;
mod pages;
mod route;
mod types;
mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}
