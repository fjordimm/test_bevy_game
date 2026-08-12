use std::time::Duration;

use bevy::{math::DVec3, prelude::*, time::common_conditions::on_timer};

use crate::game::{
    core::states::OverallState,
    playing_state::{
        coord_rebasing::{CoordRebasingOrigin, WorldSpaceEntity},
        player::tags::PlayerTransf,
        sets::{DuringPlaying, DuringPlayingUnpaused, OnEnterPlaying},
    },
    util::alrms,
};

pub struct CoordRebasingPlugin;

impl Plugin for CoordRebasingPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(OnEnterPlaying::ResourceSetup)
            )
            .add_systems(Update,
                on_new_transform
                    .in_set(DuringPlaying)
            )
            .add_systems(Update,
                perform_rebase
                    .in_set(DuringPlayingUnpaused::General)
                    .run_if(on_timer(Duration::from_millis(REBASE_INTERVAL)))
            )
        ;
    }
}

const REBASE_INTERVAL: u64 = 3000;

fn on_enter(mut commands: Commands) {
    commands.insert_resource(CoordRebasingOrigin(DVec3::new(0., 0., 0.)));
}

#[derive(Component)]
struct LastTransfPosition(Vec3);

#[derive(Component)]
struct WorldSpacePosition(DVec3);

fn on_new_transform(
    mut commands: Commands,
    new_wse_q: Query<(Entity, &Transform, Option<&ChildOf>), Added<WorldSpaceEntity>>,
    any_ws_q: Query<(), With<WorldSpaceEntity>>,
    rebase_origin: Res<CoordRebasingOrigin>,
) {
    new_wse_q.iter().for_each(|(entity, transf, parent)| {
        let mut doesnt_have_parent_wse = true;
        if let Some(parent) = parent {
            if let Ok(_) = any_ws_q.get(parent.0) {
                doesnt_have_parent_wse = false;
            }
        }

        if doesnt_have_parent_wse {
            commands
                .entity(entity)
                .insert(LastTransfPosition(transf.translation));
            commands.entity(entity).insert(WorldSpacePosition(
                DVec3::from(transf.translation) + rebase_origin.0,
            ));
        }
    });
}

// TODO: entities that exist but aren't visible should not be rebased.
//   Although, maybe non-active entities should just be despawned.

fn perform_rebase(
    player_q: Option<Single<&mut Transform, With<PlayerTransf>>>,
    mut rebase_origin: ResMut<CoordRebasingOrigin>,
    mut wse_q: Query<
        (
            &mut Transform,
            &mut LastTransfPosition,
            &mut WorldSpacePosition,
        ),
        (With<WorldSpaceEntity>, Without<PlayerTransf>),
    >,
) {
    if let Some(mut player_transf) = alrms!(player_q) {
        let new_origin = rebase_origin.0 + DVec3::from(player_transf.translation);

        wse_q
            .iter_mut()
            .for_each(|(mut transf, mut last_transf_pos, mut world_space_pos)| {
                world_space_pos.0 += DVec3::from(transf.translation - last_transf_pos.0);
                transf.translation = (world_space_pos.0 - new_origin).as_vec3();
                last_transf_pos.0 = transf.translation;
            });

        rebase_origin.0 = new_origin;

        player_transf.translation = Vec3::ZERO;
    }
}
