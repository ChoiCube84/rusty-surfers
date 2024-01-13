use std::cmp;
use ggez::mint;

#[derive(Copy, Clone)]
pub struct Character {
    pub pos_x : f32,
    pub pos_y : f32,
    pub pos_z: i32,

    velocity_y : f32,
    acceleration_y : f32,
    on_the_air : bool,
}

impl Character {
    pub fn new(window_width : i32) -> Character {
        Character {
            pos_x : (window_width / 2) as f32,
            pos_y : 0.0,
            pos_z: 0,

            velocity_y : 0.0,
            acceleration_y : 0.0,
            on_the_air : false,
        }
    }

    pub fn fall_by_gravity(&mut self) {
        if self.on_the_air {
            self.acceleration_y = -0.3;
            self.velocity_y += self.acceleration_y;
            self.pos_y += self.velocity_y;

            if self.pos_y <= 0.0 {
                self.acceleration_y = 0.0;
                self.velocity_y = 0.0;
                self.pos_y = 0.0;

                self.on_the_air = false;
            }
        }
    }

    pub fn move_left(&mut self, speed : f32) {
        self.pos_x -= speed * (0.8 + 0.2 * self.pos_z as f32);
        self.pos_x = f32::max(self.pos_x, 50.0);
    }

    pub fn move_right(&mut self, speed : f32) {
        self.pos_x += speed * (0.8 + 0.2 * self.pos_z as f32);
        self.pos_x = f32::min(self.pos_x, 800.0 - 50.0);
    }

    pub fn move_upper_lane(&mut self) {
        if !self.on_the_air {
            self.pos_z = cmp::max(self.pos_z - 1, -1);    
        }
    }

    pub fn move_lower_lane(&mut self) {
        if !self.on_the_air {
            self.pos_z = cmp::min(self.pos_z + 1, 1);
        }
    }

    pub fn jump(&mut self) {
        if !self.on_the_air {
            self.velocity_y = 10.0;
            self.on_the_air = true;
        }
    }

    pub fn get_2d_coordinate(self, window_height : i32) -> mint::Point2<f32> {
        let base_y = (window_height / 2) + self.pos_z * 200;
        
        mint::Point2{
            x: self.pos_x, 
            y: (base_y as f32) - (1.0 + 0.2 * self.pos_z as f32) * self.pos_y
        }
    }
}