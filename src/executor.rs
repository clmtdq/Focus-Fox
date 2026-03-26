use crate::parsing::{parse_command};
use crate::app::*;

pub fn execute_cmd(cmd: &str, args: Vec<&str>, app: &mut App) {
    match cmd {
            "/start" => {
                app.start_chrono();
				app.messages.push("🦊 Focus-Fox: C'est parti ! Travaille bien.".to_string());
            },
            "/stop" => {
                app.stop_chrono();
                app.messages.push("🦊 Focus-Fox: Repos mérité !".to_string());
            },
            "/task" => {
                app.messages.push("🦊 Focus-Fox: Tâche ajoutée !".to_string());
            },
            "/change" => {
                if args.len() == 1 {
                    app.messages.push("⚠️ Donnez une durée !".to_string());
                } else {
                    match args[1].parse::<u64>() {
						Ok(mins) => {
							app.change_duration(mins);
							app.messages.push(format!("🦊 Focus-Fox: Durée modifiée à {} min !", mins));
						}
						Err(_) => {
							app.messages.push("⚠️ Focus-Fox: Il me faut un nombre de minutes valide !".to_string());
						}
					}
                }
            }
            _ => (),
    }
}