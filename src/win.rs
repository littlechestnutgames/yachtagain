use bevy::prelude::*;
use crate::config::*;
use bevy::{ecs::event::Events, render::render_resource::FilterMode};
use bevy::window::WindowResized;

pub struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn build (&self, app: &mut App) {
        let config = get_config();

        app.insert_resource(ClearColor(Color::rgb(0.2, 0.42, 0.2)))
            .insert_resource(WindowDescriptor {
                title: "yacht again".to_string(),
                vsync: config.vsync,
                width: config.size.x,
                height: config.size.y,
                resizable: true,
                position: Some(Vec2::new(
                    config.position.x as f32,
                    config.position.y as f32,
                )),
                ..Default::default()
            })
            .add_system(set_img_sampler_filter) 
            .add_system(resize_notificator)
            .add_system(move_notificator);
    }
}

pub fn resize_notificator(resize_event: Res<Events<WindowResized>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        let mut config = get_config();
        config.size = Vec2::new(e.width, e.height);
        set_config(config);
    }
}

pub fn move_notificator(move_event: Res<Events<WindowMoved>>) {
    let mut reader = move_event.get_reader();
    for e in reader.iter(&move_event) {
        let mut config = get_config();
        config.position = e.position.clone();
        set_config(config);
    }
}

pub fn set_img_sampler_filter(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                if let Some(mut texture) = assets.get_mut(handle) {
                    texture.sampler_descriptor.mag_filter = FilterMode::Linear;
                    texture.sampler_descriptor.min_filter = FilterMode::Linear;
                }
            }
            _ => {}
        }
    }
}