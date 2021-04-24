use std::time::{Duration, Instant};

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use tetra::graphics::{Rectangle, Texture};
use tetra::math::Vec2;
use tetra::Context;

use crate::{fireball::Fireball, humanoid::{Humanoid, HumanoidType}};
use crate::sprites::BASIC_GRUNTS;

pub struct EnemyManager {
    enemies: Vec<Humanoid>,
    last_spawn_time: Instant,
    avg_enemy_vel: f32
}

impl EnemyManager {
    pub fn new() -> Self {
        Self {
            enemies: vec![],
            last_spawn_time: Instant::now(),
            avg_enemy_vel: 1.0
        }
    }

    pub fn spawn_enemy(&mut self, ctx: &mut Context, kind: HumanoidType) {
        let mut rng = StdRng::from_entropy();

        self.last_spawn_time = Instant::now();
        let sprite = match kind {
            HumanoidType::Player => panic!("An enemy cannot have the player's sprite"),
            HumanoidType::BasicEnemy => BASIC_GRUNTS
                .choose(&mut rng)
                .expect("BASIC_GRUNTS should not be empty"),
            HumanoidType::StrongerEnemy => todo!(),
            HumanoidType::BadassEnemy => todo!(),
            HumanoidType::Boss => todo!(),
        };

        let texture = Texture::from_file_data(ctx, sprite).expect("failed to load built-in sprite");

        // rng.gen_range(0.0..10.0)
        let enemy_vel = Vec2::new(
            rng.gen_range(0.5 .. 1.0) + self.avg_enemy_vel,
            rng.gen_range(0.5 .. 1.0) + self.avg_enemy_vel
        );

        self.avg_enemy_vel += (enemy_vel.x + enemy_vel.y)/16.0;

        let enemy = Humanoid::new(texture, Vec2::new(0., 0.), enemy_vel, kind);
        self.enemies.push(enemy);
    }

    pub fn can_spawn(&self) -> bool {
        let time_since_last_spawn = self.last_spawn_time.elapsed();

        time_since_last_spawn > Duration::from_secs_f64(1.5) 
    }

    pub fn update(&mut self, ctx: &mut Context, heading_to: Vec2<f32>) {
        for enemy in &mut self.enemies {
            enemy.advance_animation(ctx);
            enemy.head_to(heading_to);
        }
    }


    // Currently O(n²) :C
    pub fn check_for_fireball_collisions(&mut self, enemy_rects: &[Rectangle], fireballs: &[Fireball]) {
        
        let fireball_rects: Vec<_> = fireballs
            .iter()
            .map(|x| x.get_position())
            .map(Vec2::into_tuple)
            .map(|(x, y)| Rectangle::new(x+5.0, y+5.0, 32.0, 32.0))
            .collect();

        // Enemies that get hit with a fireball will be internally teleported somewhere far away so that our out-of-bounds system removes them  
        let thrown_away_pos = Vec2::new(5000.0, 5000.0);

        for (enemy, enemy_rect) in self.enemies.iter_mut().zip(enemy_rects) {
            for fireball in &fireball_rects {
                if enemy_rect.intersects(fireball) {
                    enemy.set_position(thrown_away_pos);
                }
            }
        }
    }

    pub fn enemies_ref(&self) -> &[Humanoid] {
        &*self.enemies
    }

    pub fn draw(& self, ctx: &mut Context) {
        for enemy in &self.enemies {
            enemy.draw(ctx);
        }
    }
}
