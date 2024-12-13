use super::{*};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Level>()
            .init_resource::<CurrentLevelIndex>()
            .add_systems(Update, (
                print_level,
                update_level,
            ));
    }
}




#[derive(Resource, Default)]
pub struct Level(pub Vec<Vec<char>>);

#[derive(Resource)]
pub struct CurrentLevelIndex(pub usize);

impl Default for CurrentLevelIndex {
    fn default() -> Self {
        CurrentLevelIndex(1)
    }
}

#[allow(dead_code)]
pub fn print_level(
    level: Res<Level>,
) {
    // todo!()
    info!("BreadCount: {}", 11);
}

pub fn update_level() {
    print!("update level...");
}