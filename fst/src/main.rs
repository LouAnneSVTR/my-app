use std::fmt;
use std::fmt::Formatter;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


fn main() {
    let state = State::new(); // Blue.
    let state = state.next(); // Orange.
    let state = state.next(); // Pink.
    let state = state.next(); // Blue.

    //Imprime et renvoie la valeur d'une expression donnée pour un débogage.
    dbg!(state);
}