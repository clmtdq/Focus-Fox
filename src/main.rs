use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*, style::Color};
use std::time::{Duration, Instant};

mod app;
mod ui;
mod parsing;
mod executor;

use app::*;
use ui::*;
use parsing::parse_command;
use executor::execute_cmd;

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = app::App::new();

	let tick_rate = Duration::from_millis(100); // On vérifie 10 fois par seconde
	let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

		let timeout = tick_rate
			.checked_sub(last_tick.elapsed()) // check si on peut faire tickrate - last_tick.elapsed(), si oui unwrap, sinon on met 0 et on rattrape le retard
			.unwrap_or_else(|| Duration::from_secs(0));

        // GESTION DU CLAVIER
        
		// permet de check pendant les 100ms si l'user a appuyé sur une touche, sinon on continue le loop pour faire le rendu du chrono
		// c'est pour éviter de bloquer le rendu du chrono en attendant une entrée clavier.
		if event::poll(timeout)? {
			/*
			enum Event {
				Key(KeyEvent),   // Transporte les détails de la touche
				Mouse(MouseEvent), // Transporte les détails de la souris
				Resize(u16, u16),  // Transporte la nouvelle taille
			}
			*/
			if let Event::Key(key) = event::read()? { // event::read()? c pour dire, j'ai plus rien à afficher tant que l'user n'écrit rien.
				if key.kind == KeyEventKind::Press { // KeyEventKind::Press permet d'éviter l'envoi de 2 signaux (touche appuyé et touche relâchée)
					match key.code { // check ce qu'on a appuyé
						KeyCode::Char(c) => app.input.push(c),
						KeyCode::Backspace => { app.input.pop(); },
						KeyCode::Up => app.scroll_up(),
						KeyCode::Down => app.scroll_down(),
						KeyCode::Enter => {
							let user_input = app.input.trim().to_string();
							if !user_input.is_empty() {
								app.messages.push(format!("> {}", user_input));
                                let args: Vec<&str> = parse_command(&user_input);
                                if !args.is_empty() && args[0].starts_with('/') {
                                    execute_cmd(args[0], args, &mut app);
                                }
								app.scroll_to_bottom();
							}
							app.input.clear();
						}
						KeyCode::Esc => break,
						_ => {}
					}
				}
			}
		}
		if last_tick.elapsed() >= tick_rate {
			last_tick = Instant::now();
		}
	}

    // CLEANUP
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}