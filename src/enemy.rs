use bracket_lib::prelude::*;
use crate::entity::Entity;
use crate::entity::Visibility;
use rand::Rng;

pub struct Enemy {
    pub entity: Entity,
    pub score: i32,
}

impl Visibility for Enemy {
    fn create(pos_x: i32, pos_y: i32, w: i32, h: i32, num_sprites: usize, frame_duration_ms: f32) -> Enemy {
        Enemy {
            entity : Entity {
                x: pos_x,
                y: pos_y,
                vel_x: rand::thread_rng().gen_range(1.0, 5.0),
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
        /*
         * Does not translate along y-axis
         */
    }

    fn move_x(&mut self) {
        let w = self.entity.outline.width();
        self.entity.x -= 7 +  (self.entity.vel_x) as i32;
        self.entity.outline.x1 = self.entity.x;
        self.entity.outline.x2 = self.entity.x + w;
        //self.entity.outline = Rect::with_size(self.entity.x, self.entity.y, h, w);
    }

    fn render(&mut self, ctx: &mut BTerm, _show_box: bool) {
        self.entity.frame_timer += ctx.frame_time_ms;
        if self.entity.frame_timer > self.entity.frame_duration {
            self.entity.frame_timer = 0.0;
            self.entity.frames = (self.entity.frames + 1) % self.entity.sprite_vec.len() as i32;
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
        println!("Destroying enemy"); 
        /* Player goes out of scope */
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

