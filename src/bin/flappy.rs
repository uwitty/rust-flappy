fn main() {
    flappy::scene::root::run().unwrap_or_else(|e| panic!("flappy is finished with: {}", e));
}
