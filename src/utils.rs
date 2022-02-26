use bevy::{prelude::*, render::render_resource::TextureUsages};

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
