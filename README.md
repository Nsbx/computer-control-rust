# computer-control-rust

> [!CAUTION]
> Ceci est un projet expérimental. De plus, pour la sécurité de votre machine, veillez à bien configurer votre firewall pour ne pas exposer votre serveur à un réseau autre que local.

## Description
`computer-control-rust` est un serveur web écrit en Rust permettant de prendre le controle de sa machine Windows 
à travert différente route.

## Fonctionnalités
- **Contrôle du volume** : Ajustez le volume du système.
- **Mode muet** : Activez ou désactivez le mode muet.
- **Extinction du système** : Éteignez le système.

## Routes
- /mute/<true|false> : Activez (true) ou désactivez (false) le mode muet.
- /volume/<0-100> : Réglez le volume en pourcentage (de 0 à 100).
- /shutdown : Éteignez le système.

### Build
cargo build --release
strip target/x86_64-pc-windows-gnu/release/computer-control-rust.exe
