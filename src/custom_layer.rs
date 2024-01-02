use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct GameSettings {
    pub maps: Vec<String>,
    pub global_items: Vec<crate::GameItem>,
    pub player: GameSettingsPlayer,
    pub start_pos: [usize; 2],
}

#[derive(Deserialize, Debug)]
pub struct RpProperties {
    pub at: i32,
    pub aw: i32,
    pub rs: i32,
}

#[derive(Deserialize, Debug)]
pub struct GameSettingsPlayer {
    pub name: String,
    pub character_type: String,
    pub total_health: i32,
    pub rp_properties: RpProperties,
}

pub fn parse_game_settings(file_name: &str) -> Result<GameSettings, serde_yaml::Error> {
    let contents = fs::read_to_string(file_name).expect("to be able to open the file");
    let game_settings: Result<GameSettings, serde_yaml::Error> = serde_yaml::from_str(&contents);
    return game_settings;
}

#[cfg(test)]
mod tests {
    use super::parse_game_settings;

    #[test]
    fn test_parse_game_settings() {
        let game_settings = parse_game_settings("test.yaml").unwrap();
        println!("{:?}", game_settings);
    }
}
