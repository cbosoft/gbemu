mod word;
mod registers;
mod memory;
mod cpu;
mod instructions;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::*;

use cpu::LR35902;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub fn log(s: &str) {
    println!("{s}");
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
pub fn run(rom: Vec<u8>) {
    LR35902::open(rom).run();
}