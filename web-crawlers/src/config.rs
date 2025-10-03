// use std::fs;
use serde::Deserialize;
use anyhow::Result;
use config;
// use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub target: Target
}

#[derive(Deserialize, Debug)]
pub struct Target {
    pub url: String,
    // pub cookie: String,
    pub page: usize,
    pub task_num: usize
}

pub fn parse_config() -> Result<Config>
{
    // let content = fs::read_to_string("Config.toml")?;

    // let config: Config = toml::from_str(&content)?;

    let config: Config = config::Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()?
        .try_deserialize()?;

    // let mut builder = config::Config::builder();
    //
    // for item in dotenvy::from_path_iter(".env")? {
    //     let (key, val) = item?;
    //     builder = builder.set_override(key, val)?;
    // }

    // let config: Config = builder.build()?.try_deserialize()?;

    Ok(config)
}