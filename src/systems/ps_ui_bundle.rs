use amethyst::{
    core::bundle::SystemBundle, 
    ecs::prelude::DispatcherBuilder, 
    error::Error,
};
use crate::systems::ui_flashing_system::UiFlashingSystem;
use crate::systems::ui_swinging_system::UiSwingingSystem;
use crate::systems::ui_cursor_system::UiCursorSystem;

pub struct PsUiBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PsUiBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(UiFlashingSystem, "ui_flashing_system", &[]);
        builder.add(UiSwingingSystem, "ui_swinging_system", &[]);
        builder.add(UiCursorSystem, "ui_cursor_system", &[]);
        Ok(())
    }
}