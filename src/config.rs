use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    process::{Command, Stdio},
};

use ascii_forge::window::Window;
use serde::Deserialize;

use crate::matching::rank;

#[derive(Deserialize)]
#[serde(default)]
pub struct Style {
    pub icon: String,
    pub icon_color: crossterm::style::Color,
    pub text_color: crossterm::style::Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            icon: "󱃷".to_string(),
            icon_color: crossterm::style::Color::Reset,
            text_color: crossterm::style::Color::Reset,
        }
    }
}

#[derive(Deserialize, Clone)]
pub enum AppEvent {
    Cmd(String),
    Exec(String),
    Exit,
}

pub fn run(command: AppEvent, window: &mut Window) -> anyhow::Result<bool> {
    let output = match command {
        AppEvent::Cmd(cmd) => Command::new("zsh")
            .arg("-c")
            .arg(cmd)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .output()?,
        AppEvent::Exec(arg) => Command::new("zsh")
            .args(["-c", &format!("hyprctl dispatch exec {}", arg)])
            .output()?,
        AppEvent::Exit => return Ok(true),
    };

    if !output.status.success() {
        window.restore()?;
        panic!("{:?}", output);
    }

    Ok(false)
}

#[derive(Deserialize)]
pub struct AppInfo {
    #[serde(default)]
    pub style: Style,

    pub event: Vec<AppEvent>,
}

#[derive(Deserialize)]
pub struct Config {
    pub each: Vec<AppEvent>,
    #[serde(rename = "app")]
    pub apps: HashMap<String, AppInfo>,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let directory = dirs::home_dir()
            .unwrap()
            .join(".config/launcher/config.toml");
        let mut file = File::open(directory)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;
        let code = toml::from_str::<Self>(&text)?;
        Ok(code)
    }

    pub fn matching(&self, key: &str, count: usize) -> Vec<&str> {
        let mut results = vec![];
        for item in &self.apps {
            let rank = rank(key, item.0);
            results.push((rank, item.0.as_str()));
        }
        let mut results = results
            .iter()
            .filter(|x| x.0.is_some())
            .map(|x| (x.0.unwrap(), x.1))
            .collect::<Vec<_>>();
        results.sort();

        results.iter().map(|x| x.1).take(count).collect::<Vec<_>>()
    }
}
