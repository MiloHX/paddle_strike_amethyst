use amethyst::{
    core::bundle::SystemBundle, 
    ecs::prelude::DispatcherBuilder, 
    error::Error,
};
use crate::systems::ui_glowing_system::UiGlowingSystem;
use crate::systems::ui_swinging_system::UiSwingingSystem;
use crate::systems::ui_cursor_system::UiCursorSystem;
use crate::systems::ui_jumping_system::UiJumpingSystem;

pub struct PsUiBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PsUiBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(UiGlowingSystem, "ui_glowing_system", &[]);
        builder.add(UiSwingingSystem, "ui_swinging_system", &[]);
        builder.add(UiCursorSystem, "ui_cursor_system", &[]);
        builder.add(UiJumpingSystem, "ui_jumping_system", &[]);
        Ok(())
    }
}