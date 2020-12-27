mod prime;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> () {
    alert("Hello Pikachu!");
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn isPrime(n: u64) -> bool {
    prime::is_prime(n)
}
