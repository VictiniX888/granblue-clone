use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use granblue_clone::{
    battle::BattleState,
    character::{katalina_grand::KATALINA_GRAND, Character},
    enemy::Enemy,
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Spans, Text},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame, Terminal,
};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    run(&mut terminal)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // return
    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    // setup app state
    let mut app = App {
        selector_inner_x: 0,
        selector_outer_y: 0,
        selector_outer_x: 0,
        is_inner: false,
        is_skill_description: false,
    };

    // setup game state
    let mut state = BattleState {
        enemy: Enemy {
            name: "Tiamat",
            hp: 400_000_f64,
            max_hp: 400_000_f64,
            def: 10_f64,
        },

        characters: vec![
            Character::new(&KATALINA_GRAND),
            Character::new(&KATALINA_GRAND),
            Character::new(&KATALINA_GRAND),
        ],

        log_text: vec![],
    };

    loop {
        terminal.draw(|f| ui(f, &app, &state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Right => {
                    if app.is_inner {
                        if app.selector_inner_x
                            < state.characters[app.selector_outer_x].data.skill_count - 1
                        {
                            app.selector_inner_x += 1;
                        }
                    } else if app.selector_outer_y == 0
                        && app.selector_outer_x < state.characters.len() - 1
                    {
                        app.selector_outer_x += 1;
                    };
                }
                KeyCode::Left => {
                    if app.is_inner {
                        if app.selector_inner_x > 0 {
                            app.selector_inner_x -= 1;
                        }
                    } else if app.selector_outer_y == 0 && app.selector_outer_x > 0 {
                        app.selector_outer_x -= 1;
                    };
                }
                KeyCode::Down => {
                    if !app.is_inner && app.selector_outer_y < 1 {
                        app.selector_outer_y += 1;
                    };
                }
                KeyCode::Up => {
                    if !app.is_inner && app.selector_outer_y > 0 {
                        app.selector_outer_y -= 1;
                    };
                }
                KeyCode::Enter => {
                    if app.is_inner {
                        state.skill(app.selector_inner_x, app.selector_outer_x);
                    } else if app.selector_outer_y == 1 {
                        state.attack();
                    } else {
                        app.is_inner = true;
                    };
                }
                KeyCode::Esc => {
                    if app.is_inner {
                        app.is_inner = false;
                        app.is_skill_description = false;
                        app.selector_inner_x = 0;
                    };
                }
                KeyCode::Char('\'') => {
                    if app.is_inner {
                        if !app.is_skill_description {
                            app.is_skill_description = true;
                        } else {
                            app.is_skill_description = false;
                        }
                    }
                }
                KeyCode::Char('q') => return Ok(()),
                _ => (),
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App, state: &BattleState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(8),
                Constraint::Length(3),
                Constraint::Min(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let enemy_healthbar = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(state.enemy.name),
        )
        .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
        .ratio(state.enemy.hp / state.enemy.max_hp)
        .label(format!("{} / {}", state.enemy.hp, state.enemy.max_hp));
    f.render_widget(enemy_healthbar, chunks[0]);

    let character_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(25); 4].as_ref())
        .split(chunks[1]);

    for (i, char) in state.characters.iter().enumerate() {
        if i >= 4 {
            break;
        }

        let mut character_block = Block::default().borders(Borders::ALL).title(char.data.name);
        if !app.is_inner && app.selector_outer_y == 0 && i == app.selector_outer_x {
            character_block = character_block.border_style(Style::default().fg(Color::LightCyan));
        }
        f.render_widget(character_block, character_chunks[i]);

        let character_inner_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(character_chunks[i]);

        let character_healthbar = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("HP"))
            .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
            .ratio(char.hp / char.max_hp)
            .label(format!("{} / {}", char.hp, char.max_hp));
        f.render_widget(character_healthbar, character_inner_chunks[0]);

        let character_skill_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Percentage(25); 4].as_ref())
            .split(character_inner_chunks[1]);

        for j in 0..4_usize {
            if let Some(skill) = char.get_skill(j) {
                let inner_text = if skill.curr_cooldown != 0 {
                    skill.curr_cooldown.to_string()
                } else {
                    "".to_string()
                };

                let mut skill_block = Paragraph::new(inner_text)
                    .block(Block::default().borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Center);
                if app.is_inner && i == app.selector_outer_x && j == app.selector_inner_x {
                    skill_block =
                        skill_block.style(Style::default().fg(Color::LightCyan).bg(Color::Black));
                }
                f.render_widget(skill_block, character_skill_chunks[j]);
            }
        }
    }

    let mut attack_block = Paragraph::new("Attack")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center);
    if !app.is_inner && app.selector_outer_y == 1 {
        attack_block = attack_block.style(Style::default().fg(Color::LightCyan).bg(Color::Black));
    }
    f.render_widget(attack_block, chunks[2]);

    let log: Paragraph;
    if app.is_inner && app.is_skill_description {
        let text = state.characters[app.selector_outer_x]
            .get_skill(app.selector_inner_x)
            .and_then(|skill| Some(format!("{}: {}", skill.data.name, skill.data.description)))
            .unwrap_or(String::from(""));
        log = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Skill"))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
    } else {
        let log_length = state.log_text.len().try_into().unwrap_or(0);
        let scroll;
        if log_length >= chunks[3].height - 2 {
            scroll = log_length + 2 - chunks[3].height;
        } else {
            scroll = 0;
        }
        log = Paragraph::new(to_tui_text(&state.log_text))
            .block(Block::default().borders(Borders::ALL).title("Log"))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .scroll((scroll, 0));
    }
    f.render_widget(log, chunks[3]);
}

struct App {
    selector_outer_x: usize,
    selector_outer_y: usize,
    selector_inner_x: usize,
    is_inner: bool,
    is_skill_description: bool,
}

fn to_tui_text(text: &Vec<String>) -> Text {
    Text {
        lines: text.iter().map(|str| Spans::from(str.as_str())).collect(),
    }
}
