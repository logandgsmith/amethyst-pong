use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    input::{InputBundle, StringBindings};
};

mod pong;
use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    // Setup logging
    amethyst::start_logger(Default::default());

    // Setup the assets directory
    let assets_dir = app_root.join("assets");

    // Prepare the display config
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // Prepare the input config
    let binding_path = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    // Application setup
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;

    // Run the game
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();

    Ok(())
}