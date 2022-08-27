use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
  if n <= 1 {
    n
  } else {
    fibonacci(n-1) + fibonacci(n-2)
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fibonacci(10);
        assert_eq!(result, 55);
    }
}
