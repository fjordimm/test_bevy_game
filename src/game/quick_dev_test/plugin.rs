use std::time::Duration;

use bevy::{input::mouse::MouseWheel, prelude::*, time::common_conditions::on_timer};

use crate::game::playing_state::{sets::DuringPlayingUnpaused, world::TimeOfDay};

pub struct QuickDevTestPlugin;

impl Plugin for QuickDevTestPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                rotate_sun
                    .in_set(DuringPlayingUnpaused::General)
            )
            .add_systems(Update,
                after_a_sec
                    .run_if(on_timer(Duration::from_secs(1)))
            )
        ;
    }
}

fn after_a_sec(/* mut gui_scale: ResMut<GuiScale> */) {
    // gui_scale.0 = 5.;
}

fn rotate_sun(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mut time_of_day: ResMut<TimeOfDay>,
) {
    for mouse_wheel_msg in mouse_wheel_reader.read() {
        time_of_day.0 += -0.03 * mouse_wheel_msg.y;
    }
}
