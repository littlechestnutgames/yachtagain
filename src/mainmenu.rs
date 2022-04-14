use bevy::prelude::*;
use crate::shared::{self, YachtAgainStageEvent, AppState};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build (&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu)
                .with_system(spawn_player_count_text))
            .add_system_set(SystemSet::on_update(AppState::MainMenu))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu))
            ;
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(Component)]
pub struct HowManyPlayers;

pub fn spawn_player_count_text(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let height = windows.get_primary().unwrap().height();

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("ui/howmanyplayers.png"),
            transform: Transform {
                translation: Vec3::new(0.0, (height / 2.0) - 110.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HowManyPlayers);
}

pub fn event_listener(
    mut events: EventReader<YachtAgainStageEvent>,
    mut app_state: ResMut<State<AppState>>,
) {
    for e in events.iter() {
    }
}
