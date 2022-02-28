use bevy::{prelude::*, render::render_resource::TextureUsages};

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, set_texture_filters_to_nearest)
            .add_startup_system(load_common_handles);
    }
}

// Taken from https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/helpers/texture.rs
pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}

/// Generic error handler system that can be chained into
/// to allow using ? for error checking and logging
pub fn log_error(In(result): In<anyhow::Result<()>>) {
    if let Err(e) = result {
        error!("{:?}", e);
    }
}

/// Where we can store commonly used handles
/// instead of always using asset server
pub struct CommonHandles {
    pub player_sprites: Handle<TextureAtlas>,
}

pub fn load_common_handles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("Loading common handles!");
    let player_sprites_tex = asset_server.load("images/images.png");
    let player_sprites_atlas =
        TextureAtlas::from_grid(player_sprites_tex, Vec2::new(32.0, 32.0), 8, 8);
    let player_sprites = texture_atlases.add(player_sprites_atlas);
    commands.insert_resource(CommonHandles { player_sprites });
    info!("Common handles loaded!");
}
