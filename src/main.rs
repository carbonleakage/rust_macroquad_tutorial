use macroquad::prelude::*;

#[macroquad::main("Sarav's Game")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        next_frame().await
    }
}