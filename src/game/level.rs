use super::*;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            // .init_resource::<Level>()
            .init_resource::<Levels>()
            // .init_resource::<CurrentLevelIndex>()
            .add_systems(Update, (print_level, update_level));
    }
}

#[derive(Resource)]
pub struct Levels {
    pub levels: Vec<&'static str>,
}

macro_rules! load_levels {
    ($($path:expr),*) => { {
        vec![$(include_str!($path)), *]
    } };
}

impl Default for Levels {
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        let levels = load_levels!(
            "..\\..\\assets\\levels\\level1.txt"
        );
        Self { levels }
    }
}

#[derive(Resource, Default)]
pub struct Level(pub Vec<Vec<char>>);

// #[derive(Resource)]
// pub struct CurrentLevelIndex(pub usize);

// impl Default for CurrentLevelIndex {
//     fn default() -> Self {
//         CurrentLevelIndex(1)
//     }
// }

#[allow(dead_code)]
pub fn print_level(level: Res<Level>) {
    // todo!()
    info!("BreadCount: {}", 11);
}

fn spawn_level(
    mut commands: Commands,
    levels: Res<Levels>,
) {
    // if let Ok(level) = load_level(0, levels) {

    // }
}

pub fn update_level() {
    print!("update level...");
}

// #[derive(Error, Debug)]
// pub enum GameError {
//     #[error("Fail to load level!")]
//     FailToLoadLevels,
// }


// fn load_level(level_index: usize, levels: Res<Levels>) -> anyhow::Result<Level> {
//     levels
//         .levels
//         .get(level_index - 1)
//         .map(|&level_content| {
//             let level_data: Vec<Vec<char>> = level_content
//                 .lines()
//                 .map(|line| line.chars().collect())
//                 .collect();
//             Level(level_data)
//         })
//         .ok_or_else(|| GameError::FailToLoadLevels.into())
// }