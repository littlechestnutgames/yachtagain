use crate::shared::{self, YachtAgainStageEvent, AppState};

use bevy::prelude::*;
use derive_more::{Deref, DerefMut};
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};

const PRESS_A_Y_POS: f32 = -228.0;
const PRESS_A_FLOAT_STRENGTH: f32 = 10.0;

pub struct SplashScreenPlugin;
impl Plugin for SplashScreenPlugin {
    fn build (&self, app: &mut App) {
        app.add_system(event_listener)
            .add_event::<YachtAgainStageEvent>()
            .add_system_set(SystemSet::on_enter(AppState::TitleScreen)
                .with_system(spawn_logo)
                .with_system(spawn_splash)
                .with_system(spawn_press_a)
            )
            .add_system_set(SystemSet::on_update(AppState::TitleScreen)
                .with_system(animate_press_a)
                .with_system(press_a_input)
                .with_system(animate_fade_out_title)
            )
            .add_system_set(SystemSet::on_exit(AppState::TitleScreen)
                .with_system(despawn_title_screen)
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(Component)]
pub struct Logo;

#[derive(Component)]
pub struct Splash;

#[derive(Component, Inspectable, Default)]
pub struct PressA;

#[derive(Component, Deref, DerefMut)]
pub struct PressATimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct FadeOutTimer(Timer);

pub fn spawn_logo(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("branding/yachtagain.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Logo);
}

pub fn spawn_splash(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let x_pos = window.width() * 0.5 - 120.0;
    let y_pos = window.height() * 0.5 - 120.0;
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("branding/splash.png"),
            transform: Transform::from_xyz(x_pos, y_pos, 0.0),
            ..Default::default()
        })
        .insert(Splash);
}

pub fn spawn_press_a(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("ui/press_a.png"),
            transform: Transform {
                translation: Vec3::new(0.0, PRESS_A_Y_POS, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PressA)
        .insert(PressATimer(Timer::from_seconds(0.001, true)));
}

pub fn despawn_title_screen(
    mut commands: Commands,
    mut query: Query<Entity, Or<(With<Logo>, With<Splash>, With<PressA>, With<PressATimer>)>>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

pub fn animate_fade_out_title(
    time: Res<Time>,
    mut timer_query: Query<&mut FadeOutTimer>,
    mut sprite_query: Query<&mut Sprite>,
    mut event_writer: EventWriter<YachtAgainStageEvent>,
) {
    for mut timer in timer_query.iter_mut() {
        timer.tick(time.delta());

        if (timer.just_finished()) {
            for mut sprite in sprite_query.iter_mut() {
                let a = sprite.color.a();
                if a > 0.0 {
                    sprite.color.set_a(a-0.1);
                } else {
                    event_writer.send(YachtAgainStageEvent { message: "MainMenu".to_string() });
                }
            }
        }
    }
}

pub fn animate_press_a(time: Res<Time>, mut query: Query<(&mut PressATimer, &mut Transform)>) {
    for (mut timer, mut transform) in query.iter_mut() {
        timer.tick(time.delta());
        if (timer.finished()) {
            let f: f32 = PRESS_A_Y_POS + (time.time_since_startup().as_secs_f32().sin() * -15.0);
            transform.rotation =
                Quat::from_rotation_z((time.time_since_startup().as_secs_f32() * 2.0).cos() * 0.25);
            transform.scale =
                Vec3::splat(((time.time_since_startup().as_secs_f32() * 2.0).cos().abs()).max(0.5));
            transform.translation.y = f;
        }
    }
}

pub fn press_a_input(
    mut event_writer: EventWriter<YachtAgainStageEvent>,
    keys: Res<Input<KeyCode>>,
    query: Query<(Entity, With<PressA>, Without<FadeOutTimer>)>,
) {
    for press_a in query.iter() {
        if keys.just_pressed(KeyCode::A) {
            event_writer.send(YachtAgainStageEvent {
                message: "TitleScreenPressed".to_string(),
            });
        }
    }
}

pub fn event_listener(
    mut commands: Commands,
    mut events: EventReader<YachtAgainStageEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut press_a_query: Query<Entity, With<PressA>>,
    mut press_a_timer_query: Query<Entity, With<PressATimer>>,
    mut app_state: ResMut<State<AppState>>,
) {
    for e in events.iter() {
        if (e.message == "TitleScreenPressed") {
            audio.play(asset_server.load("sounds/ring.mp3"));
            for press_a in press_a_query.iter_mut() {
                for press_a_timer in press_a_timer_query.iter_mut() {
                    commands.entity(press_a).insert(FadeOutTimer(Timer::from_seconds(0.1, true)));
                }
            }
        }
        if (e.message == "MainMenu") {            
            app_state.set(AppState::MainMenu);
        }
    }
}
