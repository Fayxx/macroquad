use macroquad::prelude::*;

#[macroquad::main("BasicShaes")]
async fn main() {
    loop {
	clear_background(BLACK);

	next_frame().await
    }
}
