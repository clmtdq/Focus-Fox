# focus-fox

focus-fox est une application CLI en Rust pour aider a rester concentre pendant une session de travail.

Le projet utilise une interface terminal interactive (TUI) avec Ratatui + Crossterm.

## Fonctionnalites actuelles

- Interface plein ecran dans le terminal (alternate screen)
- Zone de statut
- Console de messages
- Champ de commande
- Scroll de l'historique avec les fleches haut/bas
- Commandes:
  - `start`: lance le mode focus
  - `stop`: arrete le mode focus
  - `Esc`: quitte l'application

## Stack technique

- Rust (edition 2024)
- `ratatui` pour l'UI terminal
- `crossterm` pour la gestion des evenements clavier et du terminal

## Prerequis

- Rust installe (via rustup)
- Cargo
- Un terminal compatible (Windows Terminal recommande sous Windows)

## Installation

```bash
cargo build
```

## Lancement

```bash
cargo run
```

## Utilisation rapide

1. Lance l'application avec `cargo run`.
2. Tape une commande dans la zone "Commande".
3. Appuie sur `Entree` pour executer.
4. Utilise `Up` / `Down` pour naviguer dans l'historique visible.
5. Appuie sur `Esc` pour quitter.

## Structure du projet

```text
.
|- Cargo.toml
|- src/
|  |- main.rs
|- history.txt
```

## Etat actuel et suite

Le projet est en phase initiale. Des dependances sont deja presentes pour evoluer vers:

- gestion de sessions (chrono, stats)
- persistance JSON
- options CLI avec Clap

## Developpement

Verifier rapidement que le projet compile:

```bash
cargo check
```

Executer les tests (si ajoutes):

```bash
cargo test
```

## Licence

MIT
