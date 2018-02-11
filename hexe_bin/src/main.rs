extern crate hexe;
extern crate hexe_core;

fn main() {
    hexe::engine::Engine::default().uci().start();
}
