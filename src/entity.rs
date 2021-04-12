use bracket_lib::prelude::*;

pub struct Entity {
	pub x : i32,
    pub y : i32,
    pub vel_x : f32,
    pub vel_y : f32,
	pub outline: Rect,
	pub frames : i32,
    pub frame_timer: f32,
    pub frame_duration: f32,
	pub active : bool,
    pub sprite_vec: Vec<usize>,
}

pub trait Visibility {
	fn create(pos_x: i32, pos_y: i32, w:i32, h:i32, num_sprites:usize, frame_duration_ms:f32) -> Self;
	fn move_x(&mut self);
	fn move_y(&mut self);
    fn destroy(self);
	fn render(&mut self, ctx: &mut BTerm, show_box: bool);
}
