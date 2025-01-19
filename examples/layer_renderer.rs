use bevy_ecs::prelude::*;
use bevyquad::{
    layer_renderer::{RenderLayerConfig, RenderLayerIdentifier, Resolution, create_render_layer_schedule},
    prelude::*,
};

#[derive(Component)]
struct Layer1;
impl RenderLayerIdentifier for Layer1 {
    fn new() -> Self {
        Self {}
    }
    fn get_name() -> String {
        String::from("Layer1")
    }
}

#[bevyquad::main("Layer Renderer")]
async fn main() {
    let mut world = World::new();
    let mut tick_schedule = Schedule::default();

    let layer1_config = RenderLayerConfig {
        resolution: Resolution {
            width: 320,
            height: 240,
        },
        clear_color: PURPLE,
        display: true,
        ..Default::default()
    };
    let mut layer1_render_schedule = create_render_layer_schedule::<Layer1>(&layer1_config, &mut world);

    loop {
        tick_schedule.run(&mut world);
        layer1_render_schedule.run(&mut world);
        next_frame().await;
    }
}
