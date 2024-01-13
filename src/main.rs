use ggez::*;
use ggez::input::keyboard::KeyCode;
// use ggez::conf::*;
// use std::cmp;

mod character;
use character::Character;

struct State {
    window_width : i32,
    window_height : i32,

    character : Character,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const SPEED: f32 = 5.0;

        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.character.move_left(SPEED);
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.character.move_right(SPEED);
        }
        if ctx.keyboard.is_key_just_pressed(KeyCode::Up) {
            self.character.move_upper_lane();
        }
        
        if ctx.keyboard.is_key_just_pressed(KeyCode::Down) {
            self.character.move_lower_lane();
        }
        if ctx.keyboard.is_key_just_pressed(KeyCode::Space) {
            self.character.jump();
        }

        self.character.fall_by_gravity();

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let semi_major_axis = 50.0 * (1.0 + 0.2 * self.character.pos_z as f32);

        let lane_divider1 = graphics::Mesh::new_line(
            ctx, 
            &[
                mint::Point2{x: 0.0, y: ((self.window_height / 2) - 100) as f32}, 
                mint::Point2{x: self.window_width as f32, y: ((self.window_height / 2) - 100) as f32}
            ],
            1.0, 
            graphics::Color::WHITE,
        )?;

        let lane_divider2 = graphics::Mesh::new_line(
            ctx, 
            &[
                mint::Point2{x: 0.0, y: ((self.window_height / 2) + 100) as f32}, 
                mint::Point2{x: self.window_width as f32, y: ((self.window_height / 2) + 100) as f32}
            ],
            1.0, 
            graphics::Color::WHITE,
        )?;

        let surfing_board = graphics::Mesh::new_ellipse(
            ctx,
            graphics::DrawMode::fill(),
            self.character.get_2d_coordinate(self.window_height),
            semi_major_axis,
            semi_major_axis * (3.0 / 16.0),
            0.1,
            graphics::Color::YELLOW,
        )?;

        canvas.draw(&lane_divider1, graphics::DrawParam::default());
        canvas.draw(&lane_divider2, graphics::DrawParam::default());

        canvas.draw(&surfing_board, graphics::DrawParam::default());

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let cb = ggez::ContextBuilder::new("rust-surfers", "ChoiCube84")
        .window_setup(conf::WindowSetup::default().title("Rusty Surfers"))
        .window_mode(conf::WindowMode::default().dimensions(window_width as f32, window_height as f32));

    let character = Character::new(window_width);

    let state = State {
        window_width : window_width,
        window_height : window_height,
        
        character : character,
    };

    let (ctx, event_loop) = cb.build().unwrap();

    event::run(ctx, event_loop, state);
}