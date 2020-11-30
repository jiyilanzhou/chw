/*
0. 配置

 */
/* 此时编写无代码提示则先将其于 main.rs 中暴露
   (或点击此文件右上方的" Attach file to main.rs "[基于 CLion ])
 */
use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServeConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServeConfig,
    pub pg:deadpool_postgres::Config
}

impl Config{
    pub fn from_env()->Result<Self,ConfigError>{
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}