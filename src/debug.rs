use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins((
                WorldInspectorPlugin::new(),
                RapierDebugRenderPlugin::default(),
            ))
            .add_systems(Update, debug_render_toggle);
        }
    }
}

fn debug_render_toggle(mut contexts: EguiContexts, mut render_context: ResMut<DebugRenderContext>) {
    egui::Window::new("Debug Render").show(contexts.ctx_mut(), |ui| {
        ui.checkbox(&mut render_context.enabled, "Enable collider rendering");
    });
}
