pub mod entity;
pub mod player;
pub mod enemy;
use crate::entity::Visibility;
use crate::player::Player;
use crate::enemy::Enemy;
use bracket_terminal::prelude::*;
use rand::Rng;

const SCREEN_WIDTH : i32 = 640;
const SCREEN_HEIGHT : i32 = 480;
const ENEMY_NO : i32 = 7;

fn get_rnd_x()-> i32 {
    rand::thread_rng().gen_range(SCREEN_WIDTH, SCREEN_WIDTH+200)
}
fn get_rnd_y()-> i32 {
    rand::thread_rng().gen_range(1, SCREEN_HEIGHT - 1)
}

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    player: Player,
    enemy : Vec<Enemy>,
    active_enemies : i32,
    bckgrnd: Bckgrnd,
    mode: GameMode,
}

struct Bckgrnd {
    frame: usize,
    timer: f32,
    sprites: Vec<usize>,
}

impl State {
    fn new(p: Player, b: Bckgrnd) -> Self {
        State {
            player: p,
            enemy: Vec::new(),
            active_enemies : 0,
            bckgrnd: b,
            mode: GameMode::Menu,
        }
    }

    fn is_player_hit(&mut self) -> bool {
        for enemy in self.enemy.iter() {
            if enemy.entity.active && enemy.entity.outline.intersect(&self.player.entity.outline) {
                return true;
            };
        }
        return false;
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.print(1,1, "Jai Hanumanji ki");
        ctx.print(40,1, &format!("SCORE: {}", self.player.score));
        ctx.set_active_console(0);
        ctx.cls();
        self.bckgrnd.timer += ctx.frame_time_ms;
        if self.bckgrnd.timer > 750.0 {
            self.bckgrnd.timer = 0.0;
            self.bckgrnd.frame = (self.bckgrnd.frame + 1) % self.bckgrnd.sprites.len();
        }
        ctx.add_sprite(
            Rect::with_size(0, 0, 640, 480),
            1,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            self.bckgrnd.sprites[self.bckgrnd.frame],
            );

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx, false);
        if self.active_enemies == 0 {
            self.enemy.clear();
            let active_enemies = rand::thread_rng().gen_range(ENEMY_NO / 2, ENEMY_NO);
            for _ in 0..active_enemies {
                let mut e = Enemy::create(get_rnd_x(), get_rnd_y(), 47,42, 2, 100.0);
                e.entity.sprite_vec = vec![0,1];
                e.entity.active = true;
                self.enemy.push(e);
                self.active_enemies += 1;
            }
        }
        
        for enemy in self.enemy.iter_mut() {
            enemy.render(ctx, false);
            if enemy.entity.active && enemy.entity.x <= 0 {
                enemy.entity.active = false;
                self.active_enemies -= 1;
                /* Enemy dodged! Go have 10 pts */
                self.player.score += 10;
            }
        }


        if self.player.entity.y >= SCREEN_HEIGHT || self.is_player_hit() {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self, _ctx:&mut BTerm) {
        self.player = Player::create(50, 100, 91, 46, 6, 100.0);
        self.player.entity.sprite_vec = vec![2,2,2,3,3,3];
        self.active_enemies = 0;
        self.enemy.clear();
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.print_centered(5, "Jai Hanuman!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.print_centered(5, "Game-Over!");
        ctx.print_centered(8, " (P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

}


impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.game_over(ctx),
            GameMode::Playing => self.play(ctx),
        }

    }
}

bracket_terminal::embedded_resource!(HANUMAN_JI, "../resources/sprite_1.png");

fn main() -> BError {

    bracket_terminal::link_resource!(HANUMAN_JI, "resources/sprite_1.png");

    let context = BTermBuilder::new()
        .with_sprite_console(SCREEN_WIDTH, SCREEN_HEIGHT, 0)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console_no_bg(80, 50, "terminal8x8.png")
        .with_title("Jai Hanuman")
        .with_sprite_sheet(
            SpriteSheet::new("resources/sprite_1.png")
            .add_sprite(Rect::with_size(47,39,47,42))
            .add_sprite(Rect::with_size(108,39,47,42))
            .add_sprite(Rect::with_size(5,131,91,46))
            .add_sprite(Rect::with_size(105,131,91,46))
            .add_sprite(Rect::with_size(0,200,200,200))
            .add_sprite(Rect::with_size(0,400,200,200)),
            )
        .with_vsync(false)
        .build()?;
    let mut p = Player::create(50, 100, 91, 46, 6, 100.0);
    p.entity.sprite_vec = vec![2,2,2,3,3,3];

    let mut b = Bckgrnd {
        frame : 0,
        timer : 0.0,
        sprites : Vec::new(),
    };
    b.sprites = vec![4,5];
    let gs = State::new(p, b);
    main_loop(context, gs)
}
