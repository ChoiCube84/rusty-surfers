use ggez::*;
use ggez::input::keyboard::KeyCode;
use ggez::graphics::DrawParam;
// use ggez::conf::*;
// use std::cmp;
use std::env;
use std::path;

mod player;
use player::Player;

struct State {
    window_width : i32,
    window_height : i32,

    bg_image : graphics::Image,

    player : Player,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.player.move_left();
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.player.move_right();
        }
        if ctx.keyboard.is_key_just_pressed(KeyCode::Up) {
            self.player.move_upper_lane();
        }
        
        if ctx.keyboard.is_key_just_pressed(KeyCode::Down) {
            self.player.move_lower_lane();
        }
        if ctx.keyboard.is_key_just_pressed(KeyCode::Space) {
            self.player.jump();
        }

        self.player.update();

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let lane_divider1 = graphics::Mesh::new_line(
            ctx, 
            &{
                let line_y_pos = ((self.window_height * 3 / 4) - 50) as f32;
                [
                    mint::Point2{x: 0.0, y: line_y_pos}, 
                    mint::Point2{x: self.window_width as f32, y: line_y_pos}
                ]
            },
            1.0, 
            graphics::Color::WHITE,
        )?;

        let lane_divider2 = graphics::Mesh::new_line(
            ctx, 
            &{
                let line_y_pos = ((self.window_height * 3 / 4) + 50) as f32;
                [
                    mint::Point2{x: 0.0, y: line_y_pos}, 
                    mint::Point2{x: self.window_width as f32, y: line_y_pos}
                ]
            },
            1.0, 
            graphics::Color::WHITE,
        )?;

        let perspective = 0.0;
        let semi_major_axis = 50.0 * (1.0 + perspective * self.player.pos_z as f32);

        let surfing_board = graphics::Mesh::new_ellipse(
            ctx,
            graphics::DrawMode::fill(),
            self.player.get_2d_coordinate(self.window_height, perspective),
            semi_major_axis,
            semi_major_axis * (3.0 / 16.0),
            0.1,
            graphics::Color::YELLOW,
        )?;

        canvas.draw(&self.bg_image, DrawParam::default()
            .scale(mint::Vector2{
                x: self.window_width as f32 / self.bg_image.width() as f32, 
                y: self.window_height as f32 / self.bg_image.height() as f32}));

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

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("rust-surfers", "ChoiCube84")
        .window_setup(conf::WindowSetup::default().title("Rusty Surfers"))
        .window_mode(conf::WindowMode::default().dimensions(window_width as f32, window_height as f32))
        .add_resource_path(resource_dir);

    let (ctx, event_loop) = cb.build().unwrap();

    let character = Player::new(window_width).jump_customize(150.0, 90);

    let state = State {
        window_width : window_width,
        window_height : window_height,
        
        bg_image : graphics::Image::from_path(&ctx, "/sea.jpg").expect("Background Image Loading Failed!"),

        player : character,
    };

    event::run(ctx, event_loop, state);
}