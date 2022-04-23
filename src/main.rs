use bevy::prelude::*;

fn setup(
    mut commands: Commands,
) {

}

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "empty game".to_string(),
            ..default()
        })    
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
