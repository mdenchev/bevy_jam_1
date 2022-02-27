use bevy::prelude::*;

#[derive(Component)]
pub struct Disabled;

pub const NORMAL_COLOR: UiColor = UiColor(Color::rgb(0.15, 0.15, 0.15));
pub const HOVERED_COLOR: UiColor = UiColor(Color::rgb(0.25, 0.25, 0.25));
pub const PRESSED_COLOR: UiColor = UiColor(Color::rgb(0.35, 0.75, 0.35));

pub fn button_style() -> Style {
    Style {
        size: Size::new(Val::Px(150.0), Val::Px(35.0)),
        align_items: AlignItems::Center,
        align_self: AlignSelf::Center,
        justify_content: JustifyContent::Center,
        ..Default::default()
    }
}

pub fn text_style() -> Style {
    Style {
        align_self: AlignSelf::Center,
        position_type: PositionType::Relative,
        position: Rect::default(),
        ..Default::default()
    }
}

pub fn text_textstyle(asset_server: &AssetServer) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::WHITE,
    }
}

pub fn button_text_alignment() -> TextAlignment {
    TextAlignment {
        horizontal: HorizontalAlign::Center,
        vertical: VerticalAlign::Center,
    }
}

pub fn cleanup(mut commands: Commands, query: Query<Entity>) {
    for ent in query.iter() {
        commands.entity(ent).despawn();
    }
}
