# Bevy Epic Shooter

A modern Rust port of "TheEpicShooterGame" using the Bevy game engine.

## Project Overview

This project is a port of a Java-based shooter game originally developed with LibGDX. The original game was created as a 4th semester group project (group 10) and featured a modular architecture with a plugin-based system.

The port aims to leverage Bevy's Entity Component System (ECS) architecture to create a more maintainable and performant version of the game while preserving the core gameplay mechanics.

## Original Game Features

- Top-down shooter gameplay
- Player movement and combat
- Enemy AI and behavior systems
- Weapon systems with various projectile types
- Map generation and management
- Collision detection and resolution
- UI overlays and game state management
- Item pickup and interaction systems

## Technology Stack

- [Rust](https://www.rust-lang.org/) - A language empowering everyone to build reliable and efficient software
- [Bevy 0.15](https://bevyengine.org/) - A refreshingly simple data-driven game engine built in Rust
- Bevy's Entity Component System (ECS) - For efficient game object management

## Project Structure

The project follows Bevy's recommended architecture:

- `src/components/` - Contains component definitions (data attached to entities)
- `src/resources/` - Contains game resources (global state)
- `src/systems/` - Contains systems that operate on entities with specific components
- `src/plugins/` - Contains plugin modules that group related systems and components
- `src/events/` - Contains event definitions for communication between systems

## Current Implementation Status

- [x] Basic project setup with Bevy 0.15
- [x] Player movement system
- [x] Enemy systems (basic implementation)
- [x] Weapon systems (basic implementation)
- [x] Collision detection (player-obstacle, enemy-obstacle, projectile-obstacle)
- [ ] Bullet damage system
- [ ] Health and damage systems
- [%] Map generation
- [ ] UI systems
- [ ] Game state management
- [ ] Sound effects and music

## Getting Started

### Prerequisites

- Rust and Cargo (install via [rustup](https://rustup.rs/))

### Building and Running

```bash
# Clone the repository
git clone https://github.com/yourusername/bevy_epic_shooter.git
cd bevy_epic_shooter

# Build and run the game
cargo run
```

## Architecture

This port transforms the original Java plugin-based architecture into Bevy's ECS paradigm:

- **Components** replace the data structures from the original game
- **Systems** replace the processing services
- **Resources** provide global state similar to the original managers
- **Plugins** organize related components and systems, similar to the original plugin modules
- **Events** facilitate communication between systems

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Original "TheEpicShooterGame" development team (Group 10)
- The Bevy community for their excellent documentation and examples 