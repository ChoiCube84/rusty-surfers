use std::cmp;
use ggez::mint;

#[derive(Copy, Clone, PartialEq, Eq)]
enum PlayerStatus {
    Idle,
    Aerial,
    LaneSwitching(i32, bool)
}

#[derive(Copy, Clone)]
pub struct Player {
    pub pos_x : f32,
    pub pos_y : f32,
    pub pos_z: i32,

    status : PlayerStatus,

    horizontal_speed : f32,

    gravity_constant : f32,
    initial_jump_velocity : f32,

    velocity_y : f32,
    acceleration_y : f32,

    lane_switching_frames : i32,
}

impl Player {
    pub fn new(window_width : i32) -> Player {
        Player {
            pos_x : (window_width / 2) as f32,
            pos_y : 0.0,
            pos_z: 0,

            status : PlayerStatus::Idle,

            horizontal_speed : 3.0,

            gravity_constant : 0.3,
            initial_jump_velocity : 10.0,

            velocity_y : 0.0,
            acceleration_y : 0.0,

            lane_switching_frames : 30,
        }
    }

    pub fn jump_customize(mut self, maximum_jump_height : f32, jump_frames : i32) -> Self {
        self.initial_jump_velocity = (4.0 / (jump_frames + 2) as f32) * maximum_jump_height;
        self.gravity_constant = (2.0 / jump_frames as f32) * self.initial_jump_velocity;

        self
    }

    pub fn update(&mut self) {
        match self.status {
            PlayerStatus::Aerial => {
                self.acceleration_y = -self.gravity_constant;
                self.velocity_y += self.acceleration_y;
                self.pos_y += self.velocity_y;

                if self.pos_y <= 0.0 {
                    self.acceleration_y = 0.0;
                    self.velocity_y = 0.0;
                    self.pos_y = 0.0;

                    self.status = PlayerStatus::Idle;
            }
            },
            PlayerStatus::LaneSwitching(ref mut frames_left, is_up_direction) => {
                *frames_left -= 1;
                if *frames_left <= 0 {
                    self.status = PlayerStatus::Idle;

                    if is_up_direction {
                        self.pos_z -= 1;
                    }
                    else {
                        self.pos_z += 1;
                    }
                }
            },
            _ => {}
        }
    }

    pub fn move_left(&mut self) {
        self.pos_x -= self.horizontal_speed * (0.8 + 0.2 * self.pos_z as f32);
        self.pos_x = f32::max(self.pos_x, 50.0);
    }

    pub fn move_right(&mut self) {
        self.pos_x += self.horizontal_speed * (0.8 + 0.2 * self.pos_z as f32);
        self.pos_x = f32::min(self.pos_x, 800.0 - 50.0);
    }

    pub fn move_upper_lane(&mut self) {
        if self.status == PlayerStatus::Idle {
            if self.pos_z > -1 {
                self.status = PlayerStatus::LaneSwitching(self.lane_switching_frames, true);
            }
        }
    }

    pub fn move_lower_lane(&mut self) {
        if self.status == PlayerStatus::Idle {
            if self.pos_z < 1 {
                self.status = PlayerStatus::LaneSwitching(self.lane_switching_frames, false);
            }
        }
    }

    pub fn jump(&mut self) {
        if self.status == PlayerStatus::Idle {
            self.velocity_y = self.initial_jump_velocity;
            self.status = PlayerStatus::Aerial;
        }
    }

    pub fn get_2d_coordinate(self, window_height : i32, perspective : f32) -> mint::Point2<f32> {
        let base_y = window_height * 3 / 4;

        match self.status {
            PlayerStatus::LaneSwitching(frames_left, is_up_direction) => {
                let mut actual_pos_z: f32 = self.pos_z as f32;
                
                if is_up_direction {
                    actual_pos_z -= (self.lane_switching_frames - frames_left) as f32 / self.lane_switching_frames as f32;
                }
                else {
                    actual_pos_z += (self.lane_switching_frames - frames_left) as f32 / self.lane_switching_frames as f32;
                }

                mint::Point2{
                    x: self.pos_x,
                    y: (base_y as f32) + (actual_pos_z * 100.0) - (1.0 + perspective * actual_pos_z) * self.pos_y
                }
            }

            _ => {
                mint::Point2{
                    x: self.pos_x, 
                    y: (base_y as f32) + (self.pos_z as f32 * 100.0) - (1.0 + perspective * self.pos_z as f32) * self.pos_y
                }
            }
        }
        
    }
}