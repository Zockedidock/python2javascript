mod engine;
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::engine::Engine;
        let p_code = String::from(
            "
def main():
    print(test)
    if test == 1:
        print(test)
    else:
        print(test)
        print(test)
    print(test)
");
        let mut engine = Engine::new(p_code);
        engine.run();
    }
}

#[wasm_bindgen]
pub fn compile(code: String) -> String {
    use crate::engine::Engine;
    let mut engine = Engine::new(code);
    engine.run();
    engine.get_JS()
}