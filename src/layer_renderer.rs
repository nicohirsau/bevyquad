use bevy_ecs::prelude::*;
use glam::{vec2, Vec2};

use crate::{
    camera::{set_camera, set_default_camera, Camera2D},
    color::Color,
    input::mouse_position,
    prelude::Material,
    texture::{draw_texture_ex, DrawTextureParams},
    window::{clear_background, screen_height, screen_width},
    WHITE,
};

pub trait RenderLayerIdentifier: Component {
    fn new() -> Self;
    fn get_name() -> String;
}

#[derive(Default, Debug, Component)]
pub struct MousePosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Bundle)]
pub struct RenderLayerBundle<T>
where
    T: RenderLayerIdentifier,
{
    pub camera: Camera2D,
    pub clear_color: Color,
    pub mouse_position: MousePosition,
    pub layer_identifier: T,
}

#[derive(Default)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(Default)]
pub struct RenderLayerConfig {
    pub resolution: Resolution,
    pub clear_color: Color,
    pub display: bool,
    pub post_processing_shader: Option<Material>,
}

pub fn create_render_layer_schedule<T>(config: &RenderLayerConfig, world: &mut World) -> Schedule
where
T: RenderLayerIdentifier
{
    world.spawn(
        RenderLayerBundle {
            camera: crate::camera::Camera2D {
                render_target: Some(crate::texture::render_target_nearest(
                    config.resolution.width,
                    config.resolution.height,
                )),
                ..crate::camera::Camera2D::from_display_rect(crate::math::Rect::new(
                    0.,
                    0.,
                    config.resolution.width as f32,
                    config.resolution.height as f32,
                ))
            },
            clear_color: config.clear_color,
            mouse_position: MousePosition { x: 0, y: 0 },
            layer_identifier: T::new(),
        }
    );

    let mut schedule = Schedule::default();
    schedule.add_systems((
        prepare_layer_rendering::<T>,
        end_layer_rendering::<T>
            .after(prepare_layer_rendering::<T>),
    ));
    if config.display {
        schedule.add_systems(
            display_render_layer::<T>.after(end_layer_rendering::<T>)
        );
    }
    schedule
}

fn prepare_layer_rendering<T>(
    layer_camera_query: Query<&Camera2D, With<T>>,
    layer_clear_color_query: Query<&Color, With<T>>,
    mut layer_mouse_position_query: Query<&mut MousePosition, With<T>>,
) where
    T: RenderLayerIdentifier,
{
    let Ok(layer_camera) = layer_camera_query.get_single() else {
        panic!("Amount of cameras for layer {:?} is != 1", T::get_name());
    };
    let Ok(layer_clear_color) = layer_clear_color_query.get_single() else {
        panic!(
            "Amount of clear colors for layer {:?} is != 1!",
            T::get_name()
        );
    };
    let Ok(mut layer_mouse_position) = layer_mouse_position_query.get_single_mut() else {
        panic!(
            "Amount of mouse positions for layer {:?} is != 1!",
            T::get_name()
        );
    };

    set_camera(layer_camera);
    clear_background(*layer_clear_color);

    let rendertarget = layer_camera.render_target.as_ref().unwrap();

    let rendertarget_dimensions = Vec2 {
        x: rendertarget.texture.width(),
        y: rendertarget.texture.height(),
    };

    let scale = f32::min(
        screen_width() / rendertarget_dimensions.x,
        screen_height() / rendertarget_dimensions.y,
    );

    layer_mouse_position.x = ((mouse_position().0
        - (screen_width() - (rendertarget_dimensions.x * scale)) * 0.5)
        / scale) as i32;
    layer_mouse_position.y = ((mouse_position().1
        - (screen_height() - (rendertarget_dimensions.y * scale)) * 0.5)
        / scale) as i32;
}

fn end_layer_rendering<T>()
where
    T: RenderLayerIdentifier,
{
    set_default_camera();
}

fn display_render_layer<T>(layer_camera_query: Query<&Camera2D, With<T>>)
where
    T: RenderLayerIdentifier,
{
    let Ok(layer_camera) = layer_camera_query.get_single() else {
        panic!("Amount of cameras for layer {:?} is != 1!", T::get_name());
    };

    let rendertarget = layer_camera.render_target.as_ref().unwrap();

    let rendertarget_dimensions = Vec2 {
        x: rendertarget.texture.width(),
        y: rendertarget.texture.height(),
    };

    let scale = f32::min(
        screen_width() / rendertarget_dimensions.x,
        screen_height() / rendertarget_dimensions.y,
    );

    draw_texture_ex(
        &rendertarget.texture,
        (screen_width() - rendertarget_dimensions.x * scale) * 0.5,
        (screen_height() - rendertarget_dimensions.y * scale) * 0.5,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                rendertarget_dimensions.x * scale,
                rendertarget_dimensions.y * scale,
            )),
            flip_y: true,
            ..Default::default()
        },
    );
}
