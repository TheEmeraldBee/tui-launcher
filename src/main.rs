use std::time::Duration;

use anyhow::Result;
use ascii_forge::prelude::*;
use forge_widgets::{border::Border, utils::CenteredOver};

use config::*;

mod config;
mod matching;

fn main() -> Result<()> {
    let mut window = Window::init()?;
    handle_panics();
    let config = Config::load()?;

    let mut text = "".to_string();
    let mut selected = 0;

    loop {
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
        let list_border = Border::box_style(vec2(size.x / 2, size.y - 10));

        let found = config.matching(&text, 10);

        let mut list_pos = list_border.centered_on(vec2(center.x, 100));
        list_pos.y = 8;

        if found.len() != 0 {
            selected = selected.min(found.len() - 1);

            for (i, item) in found.into_iter().enumerate() {
                let info = config.apps.get(item).unwrap();

                if i == selected {
                    render!(window, vec2(list_pos.x + 1, i as u16 + 9) => [ info.icon, " - ", item, " < ".blue() ]);
                } else {
                    render!(window, vec2(list_pos.x + 1, i as u16 + 9)=> [ info.icon, " - ", item ]);
                }
            }

            if event!(window, Event::Key(k) => k.code == KeyCode::Enter) {}
        }

        let border_pos = window_border.centered_on(vec2(center.x, 5));

        render!(window, border_pos => [ window_border ], vec2(border_pos.x + 1, border_pos.y + 1) => [ text ]);
        render!(window, list_pos => [ list_border ]);

        window.update(Duration::from_millis(100))?;
    }
}
