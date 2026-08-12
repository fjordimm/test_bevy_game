use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::game::diagnosis::resources::LagSpikeDiag;

pub struct DiagnosisPlugin;

impl Plugin for DiagnosisPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(FrameTimeDiagnosticsPlugin::new(300))
            .add_plugins(EntityCountDiagnosticsPlugin::default())
            .insert_resource(LagSpikeDiag(0))
            .add_systems(Update, update_largest_recent_lag_spike)
        ;
    }
}

const HISTORY_LEN: usize = 60;

fn update_largest_recent_lag_spike(
    mut lag_spike_diag: ResMut<LagSpikeDiag>,
    mut history: Local<Option<Vec<u32>>>,
    mut history_index: Local<usize>,
    time: Res<Time>,
) {
    if let Some(ref mut history) = *history {
        history[*history_index] = time.delta().as_millis() as u32;

        *history_index += 1;
        if *history_index >= HISTORY_LEN {
            *history_index = 0;
        }

        lag_spike_diag.0 = *history.iter().max().unwrap_or(&0);
    } else {
        *history = Some(vec![0; HISTORY_LEN]);
        *history_index = 0;
    }
}
