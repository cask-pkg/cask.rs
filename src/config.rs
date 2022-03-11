use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Deserialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

fn main() {
    let config: Config = toml::from_str(r#"
        ip = '127.0.0.1'

        [keys]
        github = 'xxxxxxxxxxxxxxxxx'
        travis = 'yyyyyyyyyyyyyyyyy'
    "#).unwrap();

    assert_eq!(config.ip, "127.0.0.1");
    assert_eq!(config.port, None);
    assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
    assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
}