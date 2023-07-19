use bevy::app::{Startup, Update};
use bevy::prelude::*;
use bevy::DefaultPlugins;

const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (bevy::window::close_on_esc, update))
        .run();
}

fn update(mut query: Query<(&mut Transform, &mut Sprite)>, time_step: Res<FixedTime>) {
    let size = query.iter().len();
    info!(
        "size: {} , fixed time: {}",
        size,
        time_step.period.as_secs_f32()
    );

    for (mut transform, _) in &mut query {
        transform.translation.x += 100.0 * time_step.period.as_secs_f32();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(200.0, 200.0, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: PADDLE_COLOR,
            ..default()
        },
        ..default()
    });
}
