# Bevy Position Text Plugin ✨

Bevy plugin for displaying camera position UI text.

## Features 🌟

- Spawns a text entity with initial text.
- Updates the text to display the camera's position in real-time.

## Requirements ⚙️

- Rust
- Bevy 0.14+

## Installation 📦

1. Add the Bevy dependency to your `Cargo.toml`:

   ```toml
   [dependencies]
   bevy_ui_coords = "0.1.0" # or the latest version
   ```
2. Include the PositionTextPlugin in your Bevy app:
   ```rust
    use bevy::prelude::*;
    use bevy_ui_coords::PositionUiTextPlugin;

    fn main() {
        App::new()
            ...
            .add_plugins(PositionTextPlugin)
            ...
            .run();
    }
    ```

# License
This project is licensed under the MIT License - see the LICENSE file for details.