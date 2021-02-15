use std::mem::take;

use bevy::{math::vec2, prelude::*, render::render_graph::SlotLabel};

#[derive(Default)]
struct Player;

struct Physics {
    velocity: Vec2,
    acceleration: Vec2,
    dampening: f32,
    mass: f32,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            velocity: Vec2::zero(),
            acceleration: Vec2::zero(),
            dampening: 0.9,
            mass: 1.0,
        }
    }
}

impl Physics {
    fn apply_force(&mut self, force: Vec2) {
        assert!(force.is_finite());
        if self.mass > 0.0 {
            self.acceleration += force / self.mass;
        }
    }

    fn step(&mut self, dt: f32) {
        assert!(self.velocity.is_finite());
        assert!(self.acceleration.is_finite());
        self.velocity *= self.dampening;
        self.velocity += take(&mut self.acceleration) * dt;
    }

    fn is_moving(&self) -> bool {
        self.velocity != Vec2::zero()
    }
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    let player_size = 10.0;
    commands
        .spawn(SpriteBundle {
            sprite: Sprite::new(vec2(player_size, player_size)),
            ..Default::default()
        })
        .with(Physics::default())
        .with(Player::default());
}

fn player_move(keys: Res<Input<KeyCode>>, mut player: Query<&mut Physics, With<Player>>) {
    let mut force = Vec2::zero();
    if keys.pressed(KeyCode::Up) {
        force.y += 1.0;
    }
    if keys.pressed(KeyCode::Down) {
        force.y -= 1.0;
    }
    if keys.pressed(KeyCode::Left) {
        force.x -= 1.0;
    }
    if keys.pressed(KeyCode::Right) {
        force.x += 1.0;
    }

    let speedup = 40.0;
    let slowdown = 20.0;
    for mut p in player.iter_mut() {
        if force == Vec2::zero() {
            // no input force, try to slow down
            if p.velocity.length() > 1.0 {
                let stop = p.velocity.normalize() * -slowdown;
                p.apply_force(stop);
            }
        } else {
            p.apply_force(force.normalize() * speedup);
        }
    }
}

fn physics(time: Res<Time>, mut q: Query<(&mut Transform, &mut Physics)>) {
    for (mut tf, mut ph) in q.iter_mut() {
        tf.translation.x += ph.velocity.x;
        tf.translation.y += ph.velocity.y;

        ph.step(time.delta_seconds())
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_move.system())
        .add_system(physics.system())
        .run()
}
