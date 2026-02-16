use macroquad::prelude::{next_frame, *};

#[macroquad::main("VoIP")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(DARKPURPLE);

        if is_key_down(KeyCode::Left) {
            x -= 2.0;
        }
        if is_key_down(KeyCode::Right) {
            x += 2.0;
        }
        if is_key_down(KeyCode::Up) {
            y -= 2.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 2.0;
        }

        draw_rectangle(x, y, 16.0, 16.0, BLUE);

        next_frame().await;
    }
}
