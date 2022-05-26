mod engine;
use crate::engine::Engine;
fn main() {
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
