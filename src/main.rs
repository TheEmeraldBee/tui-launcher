use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

use anyhow::Result;
use ascii_forge::prelude::*;
use forge_widgets::{border::Border, utils::CenteredOver};
use serde::Deserialize;

#[derive(Deserialize)]
struct AppInfo {
    command: String,
    args: Vec<String>,
}

#[derive(Deserialize)]
struct Config {
    #[serde(rename = "app")]
    apps: HashMap<String, AppInfo>,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let mut file = File::open(dirs::config_dir().unwrap().join("launcher/config.toml"))?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;
        let code = toml::from_str::<Self>(&text)?;
        Ok(code)
    }

    pub fn matching(&self, key: &str, mut count: usize) -> Vec<&str> {
        let mut results = vec![];
        for item in &self.apps {
            if item.0.contains(key) {
                results.push(item.0.as_str());
                count -= 1;
            }

            if count == 0 {
                break;
            }
        }
        results
    }
}

fn main() -> Result<()> {
    let mut window = Window::init()?;
    handle_panics();
    let config = Config::load()?;

    let mut text = "".to_string();
    let mut selected = 0;

    loop {
        // Handle input
        for event in window.events() {
            if let Event::Key(k) = event {
                if k.modifiers == KeyModifiers::CONTROL && k.code == KeyCode::Char('c') {
                    return Ok(());
                } else if k.code == KeyCode::Backspace {
                    selected = 0;
                    text.pop();
                } else if let KeyCode::Char(c) = k.code {
                    selected = 0;
                    text.push(c);
                } else {
                    match k.code {
                        KeyCode::Up => {
                            if selected != 0 {
                                selected -= 1
                            }
                        }
                        KeyCode::Down => {
                            if selected < 10 {
                                selected += 1
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        let size = window.size();
        let center = vec2(size.x / 2, size.y / 2);

        let window_border = Border::box_style(vec2(size.x / 2, 3));

        let found = config.matching(&text, 10);

        if found.len() != 0 {
            selected = selected.min(found.len() - 1);

            for (i, item) in found.into_iter().enumerate() {
                if i == selected {
                    let pos = vec2(center.x - item.len() as u16 / 2 - 3, i as u16 + 8);
                    render!(window, pos => [ " > ", item ]);
                } else {
                    let pos = vec2(center.x - item.len() as u16 / 2, i as u16 + 8);
                    render!(window, pos => [ item ]);
                }
            }

            if event!(window, Event::Key(k) => k.code == KeyCode::Enter) {}
        }

        let border_pos = window_border.centered_on(vec2(center.x, 5));

        render!(window, border_pos => [ window_border ], vec2(border_pos.x + 1, border_pos.y + 1) => [ text ]);

        window.update(Duration::from_millis(100))?;
    }
}
