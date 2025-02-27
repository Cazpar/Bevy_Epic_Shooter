# Bevy Slasher RPG

A top-down slasher RPG game built with the Bevy game engine.

## Project Overview

This project is a top-down slasher RPG game that focuses on melee combat, character progression, and exploration. Originally started as a shooter game, it has been pivoted to create a more immersive RPG experience with melee combat mechanics.

The game leverages Bevy's Entity Component System (ECS) architecture to create a maintainable and performant game while providing engaging gameplay mechanics.

## Game Features

- Top-down slasher RPG gameplay
- Player movement and melee combat
- Character progression and stats
- Enemy AI and behavior systems
- Melee weapon systems with various attack patterns
- Map generation and exploration
- Collision detection and resolution
- UI overlays and game state management
- Item pickup and interaction systems
- Inventory and equipment management

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
- [x] Animation system
- [x] Enemy systems (basic implementation)
- [x] Collision detection
- [x] Health and damage systems
- [%] Map generation
- [%] UI systems
- [ ] Melee combat system
- [ ] Character progression
- [ ] Inventory system
- [ ] Equipment system
- [ ] Game state management
- [ ] Sound effects and music

## Getting Started

### Prerequisites

- Rust and Cargo (install via [rustup](https://rustup.rs/))

### Building and Running

```bash
# Clone the repository
git clone https://github.com/yourusername/bevy_slasher_rpg.git
cd bevy_slasher_rpg

# Build and run the game
cargo run
```

## Architecture

The game uses Bevy's ECS paradigm:

- **Components** define the data structures for game entities
- **Systems** process entities with specific components
- **Resources** provide global state
- **Plugins** organize related components and systems
- **Events** facilitate communication between systems

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- The Bevy community for their excellent documentation and examples 