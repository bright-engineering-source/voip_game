use macroquad::prelude::{next_frame, *};

#[macroquad::main("VoIP")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(DARKPURPLE);

        let delta_time = get_frame_time();

        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }

        draw_rectangle(x, y, 16.0, 16.0, BLUE);

        next_frame().await;
    }
}
