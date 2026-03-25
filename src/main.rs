use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*, style::Color};

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
}

impl App {
    fn new() -> App {
        App {
            input: String::new(),
            chrono_state: Chrono::Stopped, // On démarre à l'arrêt
            messages: Vec::new(),
            list_state: ListState::default(),
        }
    }

    fn start_chrono(&mut self) {
        self.chrono_state = Chrono::Started;
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

            // 2. CRÉATION DES WIDGETS (On les crée ICI pour qu'ils existent)
            let header = Paragraph::new("🦊 FOCUS-FOX CLI | Mode: Travail")
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

        // GESTION DU CLAVIER
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
                                    app.chrono_state = Chrono::Stopped;
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

    // CLEANUP
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}