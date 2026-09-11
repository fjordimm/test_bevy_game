use bevy::{
    platform::collections::HashMap,
    prelude::*,
    window::{CursorIcon, PrimaryWindow, SystemCursorIcon},
};

use crate::game::{
    core::{resources::GlobalGuiRoot, sets::GlobalStartupOrdering},
    gui::{
        make_global_gui_root,
        resources::{
            CursorIconHandler, FontHandles, GuiScale, GuiTheme, GuiThemeComputed,
            GuiThemeUncomputed, UiIconHandles,
        },
        sets::GUI_SYSTEMS_ORDERING_ORDER,
        widgets::{
            button::GuiButtonPlugin, checkbox::GuiCheckboxPlugin, div::GuiDivPlugin,
            floating_panel::GuiFloatingPanelPlugin, icon::GuiIconPlugin,
            screen_div::GuiScreenDivPlugin, text::GuiTextPlugin,
        },
    },
    util::alrms,
};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .configure_sets(Update, GUI_SYSTEMS_ORDERING_ORDER.chain())
            .add_systems(Startup,
                (startup1, startup2)
                    .chain()
                    .in_set(GlobalStartupOrdering::GuiUseOnly)
            )
            .add_systems(Update,
                update_gui_theme_computed
                    .run_if(resource_changed::<GuiScale>.or_else(resource_changed::<GuiThemeUncomputed>))
            )
            .add_systems(Update, update_cursor_icon)
            .add_plugins(GuiDivPlugin)
            .add_plugins(GuiScreenDivPlugin)
            .add_plugins(GuiTextPlugin)
            .add_plugins(GuiIconPlugin)
            .add_plugins(GuiButtonPlugin)
            .add_plugins(GuiCheckboxPlugin)
            .add_plugins(GuiFloatingPanelPlugin)
        ;
    }
}

fn startup1(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FontHandles::make(&asset_server));
    commands.insert_resource(UiIconHandles::make(&asset_server));

    let gui_root = commands.spawn(make_global_gui_root()).id();
    commands.insert_resource(GlobalGuiRoot(gui_root));
}

fn startup2(mut commands: Commands, font_handles: Res<FontHandles>) {
    let gui_theme_uncomputed = GuiThemeUncomputed(GuiTheme::make(&font_handles));
    let gui_scale = GuiScale::default();

    commands.insert_resource(GuiThemeComputed::compute_from(
        &gui_theme_uncomputed.0,
        &gui_scale,
    ));
    commands.insert_resource(gui_theme_uncomputed);
    commands.insert_resource(gui_scale);

    commands.insert_resource(CursorIconHandler {
        candidates: HashMap::new(),
    });
}

fn update_gui_theme_computed(
    theme_uncomputed: Res<GuiThemeUncomputed>,
    scale: Res<GuiScale>,
    mut theme: ResMut<GuiThemeComputed>,
) {
    *theme = GuiThemeComputed::compute_from(&theme_uncomputed.0, &scale);
}

fn update_cursor_icon(
    mut commands: Commands,
    cursor_icon_handler: Res<CursorIconHandler>,
    window_q: Option<Single<Entity, With<PrimaryWindow>>>,
) {
    if let Some(window) = alrms!(window_q) {
        if cursor_icon_handler.candidates.is_empty() {
            commands
                .entity(*window)
                .insert(CursorIcon::from(SystemCursorIcon::Default));
        } else {
            if let Some(((_, icon), _)) = cursor_icon_handler
                .candidates
                .iter()
                .max_by(|(_, x), (_, y)| x.cmp(y))
            {
                commands.entity(*window).insert(CursorIcon::from(*icon));
            }
        }
    }
}
