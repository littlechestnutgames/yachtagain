use serde::{Serialize, Deserialize};
use bevy::prelude::*;

const DEFAULT_WINDOW_WIDTH : f32 = 1280.0;
const DEFAULT_WINDOW_HEIGHT : f32 = 720.0;
const ZERO : i32 = 0;

#[derive(Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    pub size: Vec2,
    pub position: IVec2,
    pub vsync: bool
}

impl ::std::default::Default for WindowConfig {
    fn default() -> Self { Self { size: Vec2::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT), position: IVec2::splat(ZERO), vsync: true } }
}

pub fn get_config() -> WindowConfig {
    let exe_path = match std::env::current_exe() {
        Ok(e) => e.parent().unwrap().display().to_string(),
        Err(_) => ".".to_string()
    };

    let path = exe_path + std::path::MAIN_SEPARATOR_STR + "yacht-config";

    let config = match confy::load_path(path.clone()) {
        Ok(conf) => conf,
        Err(e) => WindowConfig { size: Vec2::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT), position: IVec2::splat(ZERO), vsync: true }
    };
    confy::store_path(path.clone(), config.clone());

    return config;
}

pub fn set_config(config: WindowConfig) {
    let exe_path = match std::env::current_exe() {
        Ok(e) => e.parent().unwrap().display().to_string(),
        Err(_) => ".".to_string()
    };

    let path = exe_path + std::path::MAIN_SEPARATOR_STR + "yacht-config";
    confy::store_path(path.clone(), config.clone());
}