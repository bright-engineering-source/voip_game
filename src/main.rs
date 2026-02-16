use macroquad::prelude::{next_frame, *};

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    
}

#[macroquad::main("VoIP")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;

    rand::srand(miniquad::date::now() as u64);

    let mut squares = vec![];

    let mut circle = Shape {
        size: 32.0;
        speed: MOVEMENT_SPEED;
        x: screen_width() / 2.0;
        y: screen_height() / 2.0;
    };

    loop {
        clear_background(DARKPURPLE);

        let delta_time = get_frame_time();

        if is_key_down(KeyCode::Left) {
            circle.x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Right) {
            circle.x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += MOVEMENT_SPEED * delta_time;
        }

        // Clamp X and Y to be within the screen
        circle.x = circle.x.clamp(0.0, screen_width() - circle.size);
        circle.y = circle.y.clamp(0.0, screen_height() - circle.size);

        // Generate a new square
        if rand::gen_range(0, 100) >=95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(0.0, screen_width() - size),
                y: -size,
            });
        }

        // Move squares and check for collisions
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }

        // Remove squares that are off-screen
        squares.retain(|square| square.y < screen_height() + square.size);

        // Draw everything
        draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }
        
        next_frame().await;
    }
}
