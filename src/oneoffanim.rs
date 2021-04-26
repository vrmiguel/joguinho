use std::{
    time::{Duration, Instant},
    usize,
};

use tetra::{
    graphics::{animation::Animation, DrawParams, Rectangle, Texture},
    math::Vec2,
    Context,
};

use crate::resources::EXPLOSION;

struct OneOffAnimation {
    current_frame: u8,
    position: Vec2<f32>,
}

pub struct OneOffAnimationManager {
    last_updated_time: Instant,
    explosion_anim: Animation,
    explosion_anim_frames: u8,
    explosions: Vec<OneOffAnimation>,
}

impl OneOffAnimationManager {
    pub fn new(ctx: &mut Context) -> Self {
        let explosion_sprite = Texture::from_file_data(ctx, EXPLOSION)
            .expect("Failed to load built-in explosion sprite");

        let explosion_anim = Animation::new(
            explosion_sprite,
            Rectangle::row(0.0, 0.0, 64.0, 64.0).take(10).collect(),
            Duration::from_secs_f32(0.05),
        );

        let explosion_anim_frames = explosion_anim.frames().len() as u8;

        Self {
            last_updated_time: Instant::now(),
            explosion_anim,
            explosion_anim_frames,
            explosions: vec![],
        }
    }

    pub fn add_explosion(&mut self, position: Vec2<f32>) {
        let explosion_anim = OneOffAnimation {
            position,
            current_frame: 0,
        };

        self.explosions.push(explosion_anim);
    }

    fn clean_up_finished_animations(&mut self) {
        let explosion_final_frame = self.explosion_anim_frames - 1;
        self.explosions
            .retain(|x| x.current_frame != explosion_final_frame);
    }

    pub fn update(&mut self) {
        self.clean_up_finished_animations();

        if !self.can_update_frames() {
            return;
        }

        self.last_updated_time = Instant::now();

        for explosion in &mut self.explosions {
            explosion.current_frame += 1;
        }
    }

    fn can_update_frames(&self) -> bool {
        let elapsed = self.last_updated_time.elapsed();

        elapsed > Duration::from_secs_f32(0.10)
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        for explosion in &self.explosions {
            self.explosion_anim
                .set_current_frame_index(explosion.current_frame as usize);
            self.explosion_anim
                .draw(ctx, DrawParams::new().position(explosion.position));
        }
    }
}
