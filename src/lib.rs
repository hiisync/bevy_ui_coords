use bevy::{
    app::{App, Plugin, Startup, Update},
    color::Color,
    prelude::{default, Camera, Commands, Component, Query, TextBundle, Transform, With},
    text::{Text, TextStyle},
};

pub struct PositionUiTextPlugin;

impl Plugin for PositionUiTextPlugin {
    fn build(&self, app: &mut App) {
        // Add the systems to spawn text on startup and to update the text on each frame
        app.add_systems(Startup, spawn_text)
            .add_systems(Update, change_text);
    }
}

#[derive(Component)]
struct TextChanges;

fn spawn_text(mut commands: Commands) {
    // Spawn the text entity with initial text and style
    commands.spawn((
        TextBundle::from_section(
            "Position", // Initial text
            TextStyle {
                font_size: 20.0,                   // Font size of the text
                color: Color::srgb(0.9, 0.9, 0.9), // Color of the text
                ..default()                        // Use default values for other fields
            },
        ),
        TextChanges, // Marker component for text that changes
    ));
}

fn change_text(
    camera_query: Query<&Transform, With<Camera>>, // Query to get the camera's transform
    mut text_query: Query<&mut Text, With<TextChanges>>, // Query to get the text components that need updating
) {
    // Get the camera position
    if let Ok(transform) = camera_query.get_single() {
        let pos = transform.translation; // Extract the camera's translation vector

        // Update the text with the new camera position
        for mut text in &mut text_query {
            // Format the text to display the camera's current position
            text.sections[0].value = format!("Position: {:.2}, {:.2}, {:.2}", pos.x, pos.y, pos.z);
        }
    }
}
