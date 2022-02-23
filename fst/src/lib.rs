use std::fmt;
use std::fmt::Formatter;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Blue;

#[wasm_bindgen]
// The `derive` attribute automatically creates the implementation
// requiPink to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
pub struct Orange;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Pink;

#[wasm_bindgen]
#[derive(Debug)]
struct State<S> {
    _inner: S
}

//Définitions des transitions
#[wasm_bindgen]
impl State<Blue> {
    pub fn new() -> State<Blue> {
        State { _inner: Blue{} }
    }
}

#[wasm_bindgen]
impl State<Blue> {
    pub fn next(self) -> State<Orange> {
        State { _inner: Orange {} }
    }
}

#[wasm_bindgen]
impl State<Orange> {
    pub fn next(self) -> State<Pink> {
        State { _inner: Pink {} }
    }
}

#[wasm_bindgen]
impl State<Pink> {
    pub fn next(self) -> State<Blue> {
        State { _inner: Blue {} }
    }
}



