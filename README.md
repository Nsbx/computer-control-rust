# computer-control-rust

## Description
computer-control-rust est un serveur web écrit en Rust utilisant la crate Rocket. 
Il offre des fonctionnalités pour contrôler le volume, activer/désactiver le mode muet et éteindre le système via des routes HTTP.
Tel que je l'ai imaginé, le projet a pour but d'être "cablé" à un home-assistant puis à homekit bridge par exemple.

## Fonctionnalités
- Contrôle du volume par pourcentage.
- Activation/désactivation du mode muet.
- Commande d'extinction du système.

### Build
cargo build --release
strip target/x86_64-pc-windows-gnu/release/computer-control-rust.exe
