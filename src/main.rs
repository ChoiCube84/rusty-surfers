use ggez::*;
use ggez::input::keyboard::KeyCode;

struct State {
    pos_x: f32,
    pos_y: f32,
    color : usize,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const SPEED: f32 = 5.0;

        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.pos_x += SPEED;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.pos_x -= SPEED;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.pos_y -= SPEED;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.pos_y += SPEED;
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
            mint::Point2{x: self.pos_x, y: self.pos_y},
            50.0,
            0.1,
            color_vec[self.color],
        )?;

        canvas.draw(&circle, graphics::DrawParam::default());
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() {
    let state = State {
        pos_x : 0.0,
        pos_y : 0.0,
        color : 0,
    };
    
    let cb = ggez::ContextBuilder::new("rust-surfers", "ChoiCube84");
    let (ctx, event_loop) = cb.build().unwrap();

    event::run(ctx, event_loop, state);
}