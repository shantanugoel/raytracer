mod canvas;
mod color;
mod projectiles;
mod tuple;
mod utils;

fn main() {
    projectiles::tick_run();
    println!("Hello, world!");
}
