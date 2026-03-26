use ratatui::{prelude::*, widgets::*};
use crate::app::{App, Chrono};

pub fn draw(f: &mut Frame, app: &mut App) {
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
        let elapsed = start.elapsed();
        
        if elapsed >= app.focus_duration {
            "00:00 - FINI ! 🦊".to_string()
        } else {
            let remaining = app.focus_duration - elapsed;
            let total_secs = remaining.as_secs();
            format!("{:02}:{:02}", total_secs / 60, total_secs % 60)
        }
    } else {
        // On calcule l'affichage basé sur la durée réglée, même à l'arrêt
        let total_secs = app.focus_duration.as_secs();
        format!("{:02}:{:02}", total_secs / 60, total_secs % 60)
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
}