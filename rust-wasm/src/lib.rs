use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Person {
    age: u32,
    height: u32
}

#[wasm_bindgen]
impl Person {
    pub fn new(age: u32, height: u32) -> Person {
        Person {
            age,
            height
        }
    }

    pub fn get_age(self) -> u32 {
        self.age
    }

    pub fn get_height(self) -> u32 {
        self.height
    }
}