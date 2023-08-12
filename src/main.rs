use bevy::prelude::*;
use bevy::window::PrimaryWindow;


//our manin function adding the .add_plugin(DefaultPlugins)  creates a default window, we also reference camera and player
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_player)
    .run();
}

//create our Player struct Component by first deriving the Component
#[derive(Component)]
pub struct Player {

}

//we create a function to span a player
/* NOTE: spawn_player takes in a Command, A query for the window, access to asset server */
pub fn spawn_player( 
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // we get a single ref to our window so we have some place to place the Player
    let window: &Window = window_query.get_single().unwrap();

    // we spawn the player by transforming the position to the center of the screen and setting the texture. 
    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player {}
        )
    );
}

// we create our camera 
pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}
