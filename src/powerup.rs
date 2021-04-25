use std::time::{Duration, Instant};

use rand::{
    distributions::Standard,
    prelude::{Distribution, StdRng},
    Rng, SeedableRng,
};
use tetra::{Context, graphics::{DrawParams, NineSlice, Rectangle, Texture}, math::Vec2};

use crate::{humanoid::Humanoid, resources};

#[derive(Debug)]
pub enum PowerUpKind {
    AdditionalHeart,
    FasterShooting,
}

#[derive(Debug)]
struct PowerUp {
    kind: PowerUpKind,
    spawned_time: Instant,
    position: Vec2<f32>,
    was_consumed: bool,
    flickering: u8,
}

impl PowerUp {
    pub fn is_expired(&self) -> bool {
        let elapsed = self.spawned_time.elapsed();

        elapsed > Duration::from_secs_f32(10.0)
    }

    pub fn flicker_if_almost_expiring(&mut self) {
        let elapsed = self.spawned_time.elapsed();
        if elapsed > Duration::from_secs_f32(8.0) {
            self.flickering = 121;
        }
    }
}

impl Distribution<PowerUpKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PowerUpKind {
        match rng.gen_range(0..=1) {
            0 => PowerUpKind::AdditionalHeart,
            _ => PowerUpKind::FasterShooting,
        }
    }
}

pub struct PowerUpManager {
    fire_scroll_sprite: Texture,
    heart_sprite: Texture,
    powerups: Vec<PowerUp>,
    last_spawned_time: Instant,
}

impl PowerUpManager {
    pub fn new(ctx: &mut Context) -> Self {

        let fire_scroll_sprite = Texture::from_file_data(ctx, resources::FIRE_SCROLL)
            .expect("failed to load built-in strawberry sprite");
        let heart_sprite = Texture::from_file_data(ctx, resources::HEART_32X)
            .expect("failed to load built-in heart 32x32 sprite");
    
        Self {
            fire_scroll_sprite,
            heart_sprite,
            powerups: vec![],
            last_spawned_time: Instant::now(),
        }
    }

    pub fn check_for_collision(&mut self, player: &mut Humanoid) {
        let player_pos = player.get_position();
        let player_rect = Rectangle::new(player_pos.x, player_pos.y, 16.0, 16.0);
        for powerup in &mut self.powerups {
            let powerup_rect = Rectangle::new(
                powerup.position.x, 
                powerup.position.y,  
                32.0, 
                32.0);
            
            if powerup_rect.intersects(&player_rect) {
                powerup.was_consumed = true;
                match powerup.kind {
                    PowerUpKind::AdditionalHeart => player.hearts += 1,
                    PowerUpKind::FasterShooting => {} // TODO
                }
            }
        }
    }

    pub fn can_spawn(&self) -> bool {
        let time_since_last_throw = self.last_spawned_time.elapsed();
        time_since_last_throw > Duration::from_secs_f64(5.00)
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        
        for powerup in &mut self.powerups {
            if powerup.flickering > 0 {                
                powerup.flickering -= 1;
                if powerup.flickering % 2 == 0 {
                    continue;
                }
            } else {
                println!("{:?} not flickering", powerup);
            }
            match powerup.kind {
                PowerUpKind::AdditionalHeart => self
                    .heart_sprite
                    .draw(ctx, DrawParams::new().position(powerup.position)),
                PowerUpKind::FasterShooting => self
                    .fire_scroll_sprite
                    .draw(ctx, DrawParams::new().position(powerup.position).scale(Vec2::new(2.5, 2.5))),
            }
        }
    }

    pub fn update(&mut self) {
        self.powerups.iter_mut().for_each(|p| p.flicker_if_almost_expiring());
        self.powerups.retain(|p| !p.was_consumed && !p.is_expired());
    }

    pub fn spawn_power_up(&mut self) {
        self.last_spawned_time = Instant::now();
        let mut rng = StdRng::from_entropy();
        let position = Vec2 { x: rng.gen_range(0.0..800.0), y: rng.gen_range(0.0..800.0)};

        let power_up = PowerUp {
            kind: rng.gen(),
            spawned_time: self.last_spawned_time,
            position,
            was_consumed: false,
            flickering: 0,
        };

        self.powerups.push(power_up);
    }
}
