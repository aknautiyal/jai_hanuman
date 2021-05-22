use bracket_lib::prelude::*;
use crate::entity::Entity;
use crate::entity::Visibility;

pub struct Player {
    pub entity: Entity,
    pub score: i32,
}

impl Visibility for Player {
    fn create(pos_x: i32, pos_y: i32, w: i32, h: i32, num_sprites: usize, frame_duration_ms: f32) -> Player {
        Player {
            entity : Entity {
                x: pos_x,
                y: pos_y,
                vel_x: 0.0,
                vel_y: 0.0,
                outline: Rect::with_size(pos_x, pos_y, w, h),
                frames : 0,
                frame_timer : 0.0,
                frame_duration: frame_duration_ms,
                active : false,
                sprite_vec : Vec::with_capacity(num_sprites),
            },
            score : 0,
        }
    }

    fn move_y(&mut self) {
        let h = self.entity.outline.height();
        if self.entity.vel_y < 8.0 {
            self.entity.vel_y += 1.0;
        }
        self.entity.y += self.entity.vel_y as i32;

        self.entity.outline.y1 = self.entity.y;
        self.entity.outline.y2 = self.entity.y + h;

        if self.entity.y < 0 {
            self.entity.y = 0;
            self.entity.outline.y1 = 0;
            self.entity.outline.y2 = h;
        }
    }

    fn move_x(&mut self) {
        /*
         * Does not translate along X-axis
         */
    }

    fn render(&mut self, ctx: &mut BTerm, _show_box: bool) {
        self.entity.frame_timer += ctx.frame_time_ms;
        if self.entity.frame_timer > self.entity.frame_duration {
            self.entity.frame_timer = 0.0;
            self.entity.frames = (self.entity.frames + 1 )  % self.entity.sprite_vec.len() as i32;
            self.move_x();
            self.move_y();
        }
        let index = self.entity.frames as usize;
        ctx.add_sprite(
            self.entity.outline,
            2,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            self.entity.sprite_vec[index],
            );
    }
        

    fn destroy(self) {
        println!("Destroying Player"); 
        /* Player goes out of scope */
    }
}

impl Player {
    pub fn flap(&mut self) {
        if self.entity.y > 0 {
            self.entity.vel_y -= 5.0;
        }
    }

    pub fn incr_score(&mut self) {
        self.score += 100;
    }
}

/*
impl Drop for Player {
    fn drop(&mut self)
    {
        println!("Dropping Player"); 
    }
}
*/

