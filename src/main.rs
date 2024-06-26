use bevy::prelude::*;
use bevy::window::*;
mod gamestate;
mod hud;
mod sonar;
mod torpedo;
mod player;
mod menu;
mod loading;
mod pause;
mod enemy;
mod hitbox;
mod gameover;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Miles Below Darkness".to_string(),
                resolution: WindowResolution::new(1920., 1080.).with_scale_factor_override(1.0),
                ..default()
            }),
            //exit_condition: ExitCondition::OnAllClosed, //it tries to exit app, but bugs out
            ..default()
        }))
        .init_state::<gamestate::GameState>()
        .add_systems(Startup, setup_cam)
        .add_plugins(hud::HudPlugin)
        .add_plugins(menu::MenuPlugin)
        .add_plugins(loading::LoadingPlugin)
        .add_plugins(pause::PausePlugin)
        .add_plugins(sonar::SonarPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(torpedo::TorpedoPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(hitbox::HitboxPlugin)
        .add_plugins(gameover::GameOverPlugin)
        .run()
}

fn setup_cam(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
