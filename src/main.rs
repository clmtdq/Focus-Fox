use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*, style::Color};
use std::time::{Duration, Instant};

#[derive(PartialEq)]
enum Chrono {
    Started,
    Paused,
    Stopped,
}

struct App {
    input: String,
    chrono_state: Chrono,
	messages: Vec<String>,
    list_state: ListState,
    start_time: Option<Instant>,
}

impl App {
    fn new() -> App {
        App {
            input: String::new(),
            chrono_state: Chrono::Stopped, // On démarre à l'arrêt
            messages: Vec::new(),
            list_state: ListState::default(),
            start_time: None,
        }
    }

	fn start_chrono(&mut self) {
		self.chrono_state = Chrono::Started;
		self.start_time = Some(Instant::now());
	}

	fn stop_chrono(&mut self) {
		self.chrono_state = Chrono::Stopped;
		self.start_time = None;
	}

    fn scroll_to_bottom(&mut self) {
        if !self.messages.is_empty() {
            self.list_state.select(Some(self.messages.len() - 1));
        }
    }

    fn scroll_up(&mut self) {
        if !self.messages.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) => if i == 0 { 0 } else { i - 1 },
                None => 0,
            };
            self.list_state.select(Some(i)); // on va changer la variable sélectionné afin de pointer vers un autre message
        }
    }

    fn scroll_down(&mut self) {
        if !self.messages.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i >= self.messages.len() - 1 {
                        self.messages.len() - 1
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();

	let tick_rate = Duration::from_millis(100); // On vérifie 10 fois par seconde
	let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| {
            // 1. DÉFINITION DU LAYOUT
            let mut constraints = vec![
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Body
                Constraint::Length(3), // Input
            ];

            // Si le chrono est lancé, on insère la zone du renard
            if app.chrono_state == Chrono::Started {
                constraints.insert(2, Constraint::Length(10)); // on ajoute une zone pour le petit renard si on lance le chrono
            }                                                  // Comment le rendu se refresh auto, constraints n'aura plus
                                                               // le renard si on stop le chrono

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(f.size());

			// le if let ici nous dis, est-ce que app.start_time est une variante de Some ? Si oui on sort l'élément dedans et on le nomme start
			// ici l'élément dedans est un Instant
			let time_display = if let Some(start) = app.start_time {
				let elapsed = start.elapsed().as_secs();
				let mins = elapsed / 60;
				let secs = elapsed % 60;
				format!("{:02}:{:02}", mins, secs)
			} else {
				"00:00".to_string()
			};

            // 2. CRÉATION DES WIDGETS (On les crée ICI pour qu'ils existent)
            let header = Paragraph::new(format!("🦊 FOCUS-FOX | Temps : {}", time_display))
				.style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
				.block(Block::default().borders(Borders::ALL).title(" Statut "));

            let items = [
                ListItem::new("1. Démarrer Focus"),
                ListItem::new("2. Voir les stats"),
                ListItem::new("3. Paramètres"),
            ];

            let history: Vec<ListItem> = app.messages
                .iter()
                .map(|msg| ListItem::new(msg.as_str()))
                .collect();

            let body = List::new(history)
                .block(Block::default().title(" Console Focus-Fox ").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_symbol(">> ")
                .highlight_style(Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
                );
			
            // ATTENTION : On utilise render_stateful_widget ici !
            /*
            fn render_stateful_widget<W, S>(
                &mut self, 
                widget: W,      // Le widget (ex: List)
                area: Rect,    // La zone (ex: chunks[1])
                state: &mut S  // L'état (ex: &mut ListState)
            ) 
            where 
                W: StatefulWidget<State = S>
            */
            f.render_stateful_widget(body, chunks[1], &mut app.list_state);

            let input_widget = Paragraph::new(app.input.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title(" Commande "));

            // 3. RENDU (Affichage)
            f.render_widget(header, chunks[0]);

            if app.chrono_state == Chrono::Started {
                let fox_art = Paragraph::new(" (\\_/) \n (='.'=) \n (\")_(\") \n Je lance le chrono ! ") // Dessin du renard
                    .style(Style::default().fg(Color::Rgb(255, 165, 0))) // fg = Couleur du texte 
                    .block(Block::default().borders(Borders::ALL).title(" Assistant "));
                
                f.render_widget(fox_art, chunks[2]);
                f.render_widget(input_widget, chunks[3]); // Input poussé en 3
            } else {
                f.render_widget(input_widget, chunks[2]); // Input normal en 2
            }
        })?;

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
							let user_input = app.input.trim();
							if !user_input.is_empty() {
								app.messages.push(format!("> {}", user_input));
								match user_input {
									"start" => {
										app.start_chrono();
										app.messages.push("🦊 Focus-Fox: C'est parti ! Travaille bien.".to_string());
									},
									"stop" => {
										app.stop_chrono();
										app.messages.push("🦊 Focus-Fox: Repos mérité !".to_string());
									},
									_ => {
										app.messages.push(format!("⚠️ Commande inconnue: {}", user_input));
									}
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