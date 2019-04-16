extern crate cfg_if;
extern crate tensorflow;
extern crate wasm_bindgen;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

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
    fn jsSum(_: i32, _: i32) -> i32;
}

#[wasm_bindgen]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn sum_with_js(a: i32, b: i32) -> i32 {
    jsSum(a, b)
}
