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
    let mut selected: usize = 0;

    let mut should_run = false;

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
                            selected = selected.saturating_sub(1usize);
                        }
                        KeyCode::Down => {
                            if selected < 10 {
                                selected += 1;
                            }
                        }
                        KeyCode::Enter => {
                            should_run = true;
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

        if !found.is_empty() {
            selected = selected.min(found.len() - 1);

            for (i, item) in found.into_iter().enumerate() {
                let info = config.apps.get(item).unwrap();

                if i == selected {
                    if should_run {
                        run(info.event_type, &info.args, &mut window)?;
                        should_run = false;
                    }
                    render!(window, vec2(list_pos.x + 1, i as u16 + 9) => [ info.style.icon.clone().with(info.style.icon_color), "  ", item.with(info.style.text_color), " < Enter > ".blue() ]);
                } else {
                    render!(window, vec2(list_pos.x + 1, i as u16 + 9)=> [ info.style.icon.clone().with(info.style.icon_color), "  ", item.with(info.style.text_color) ]);
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
