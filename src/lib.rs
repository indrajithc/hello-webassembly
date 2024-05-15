use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

#[wasm_bindgen]
pub fn get_fibonacci_number(n: u32) {
    let num = fib(n);
    alert(&format!("Hello {n}:{num}"));
}

fn sum(n: u32) -> u32 {
    (1..=n).sum()
}

#[wasm_bindgen]
pub fn get_sum(n: u32) {
    let num = sum(n);
    alert(&format!("Hello {n}:{num}"));
}