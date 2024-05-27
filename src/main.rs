use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
	clear_background(BLACK);

	next_frame().await
    }
}
