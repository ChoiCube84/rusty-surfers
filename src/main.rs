use ggez::*;
use ggez::input::keyboard::KeyCode;
use ggez::conf::*;
use std::cmp;

mod character;
use character::Character;

struct State {
    window_width : i32,
    window_height : i32,

    character : Character,

    color : usize,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const SPEED: f32 = 5.0;

        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.character.pos_x += SPEED * (0.8 + 0.2 * self.character.pos_z as f32);
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.character.pos_x -= SPEED * (0.8 + 0.2 * self.character.pos_z as f32);
        }
        if ctx.keyboard.is_key_just_pressed(KeyCode::Up) {
            self.character.pos_z = cmp::max(self.character.pos_z - 1, -1);
        }
        
        if ctx.keyboard.is_key_just_pressed(KeyCode::Down) {
            self.character.pos_z = cmp::min(self.character.pos_z + 1, 1);
        }

        if ctx.keyboard.is_key_just_pressed(KeyCode::Space) {
            self.color += 1;
            self.color %= 4;
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let color_vec = Vec::from([graphics::Color::RED, graphics::Color::YELLOW, graphics::Color::GREEN, graphics::Color::BLUE]);
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2{x: self.character.pos_x, y: ((self.window_height / 2) + self.character.pos_z * 200) as f32},
            50.0 * (0.8 + 0.2 * self.character.pos_z as f32),
            0.1,
            color_vec[self.color],
        )?;

        canvas.draw(&circle, graphics::DrawParam::default());
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

    let character = Character {
        pos_x : (window_width / 2) as f32,
        pos_y : 0.0,
        pos_z: 0,
        on_the_air : false,
    };

    let state = State {
        window_width : window_width,
        window_height : window_height,
        
        character : character,
        
        color : 0,
    };

    let (ctx, event_loop) = cb.build().unwrap();

    event::run(ctx, event_loop, state);
}