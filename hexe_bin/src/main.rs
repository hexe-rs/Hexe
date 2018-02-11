extern crate hexe;

fn main() {
    hexe::engine::Engine::default().uci().start();
}
