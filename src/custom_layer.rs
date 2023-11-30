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

pub fn parse_game_settings(file_name: &str) -> Result<GameSettings, toml::de::Error> {
    let contents = fs::read_to_string(file_name).expect("to be able to open the file");
    let game_settings: Result<GameSettings, toml::de::Error> = toml::de::from_str(&contents);
    return game_settings;
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::parse_game_settings;

    #[derive(Deserialize, Debug)]
    struct Person {
        name: String,
        age: u32,
        test: Test,
    }

    #[derive(Deserialize, Debug)]
    struct Test {
        foo: String,
        bar: i32,
    }

    #[test]
    fn test_toml_serde() {
        let toml = r#"
name = "John Doe"
age = 30

[test]
foo = "aoeu"
bar = 234
"#;
        let person: Person = toml::de::from_str(toml).unwrap();
        println!("person: {:?}", person)
    }

    #[test]
    fn test_parse_game_settings() {
        let game_settings = parse_game_settings("test.toml").unwrap();
        println!("{:?}", game_settings);
    }
}
