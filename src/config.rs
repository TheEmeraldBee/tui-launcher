use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use ascii_forge::window::Window;
use hyprland::{
    ctl::{notify, Color},
    dispatch,
    dispatch::{Dispatch, DispatchType},
};
use serde::Deserialize;

use crate::matching::rank;

#[derive(Deserialize)]
pub enum AppType {
    /// Drop to shell and run command.
    DropSh,
    Sh,
    Exec,
}

impl AppType {
    pub fn run(&self, command: &str, window: &mut Window) -> anyhow::Result<()> {
        match self {
            Self::DropSh => {
                window.restore()?;
                let status = Command::new("sh").arg(command).status()?;
                if !status.success() {
                    notify::call(
                        notify::Icon::NoIcon,
                        Duration::from_secs(5),
                        Color::new(255, 255, 255, 255),
                        "Latest DropSh Event From App Launcher Failed To Execute Without Errors"
                            .to_string(),
                    )?;

                    // Allow time to see the error.
                    thread::sleep(Duration::from_secs(2));
                }
            }
            Self::Sh => {
                let status = Command::new("sh")
                    .arg(command)
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .status()?;

                if !status.success() {
                    notify::call(
                        notify::Icon::NoIcon,
                        Duration::from_secs(5),
                        Color::new(255, 255, 255, 255),
                        "Latest Sh Event From App Launcher Failed To Execute Without Errors"
                            .to_string(),
                    )?;
                }
            }
            Self::Exec => {
                dispatch!(Exec, command)?;
            }
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct AppInfo {
    #[serde(default = "default_icon")]
    pub icon: String,

    pub command: String,

    #[serde(rename = "type")]
    pub event_type: AppType,
}

fn default_icon() -> String {
    "󰘔".to_string()
}

#[derive(Deserialize)]
pub struct Config {
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
