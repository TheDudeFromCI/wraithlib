use bevy::prelude::*;
use bevy_wh_elements::components::TextInput;

use super::*;
use crate::common::uuid::Uuid;

pub(super) fn init_main_menu(mut next_state: ResMut<NextState<MainMenuState>>) {
    next_state.set(MainMenuState::TitleScreen);
}

pub(super) fn build_ui(
    asset_server: Res<AssetServer>,
    mut properties: ResMut<MainMenuProperties>,
    mut commands: Commands,
) {
    let mut canvas = None;
    std::mem::swap(&mut canvas, &mut properties.canvas);

    if let Some(canvas) = canvas {
        canvas.build(&mut commands, &asset_server);
    }
}

pub(super) fn add_server_entry(
    properties: Res<MainMenuProperties>,
    server_list: Query<Entity, With<ServerListPane>>,
    asset_server: Res<AssetServer>,
    mut add_server_evs: EventReader<AddServerEntry>,
    mut commands: Commands,
) {
    let Ok(server_list) = server_list.get_single() else {
        return;
    };

    for ev in add_server_evs.read() {
        let Some(builder) = &properties.server_entry else {
            continue;
        };

        let elem = builder(ev.uuid.clone(), &ev.name, &ev.address);
        elem.build_child(&mut commands, &asset_server, Some(server_list));
    }
}

pub(super) fn cleanup(ui: Query<Entity, With<MainMenuCanvas>>, mut commands: Commands) {
    for entity in ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub(super) fn confirm_edit_server(
    button: Query<&Interaction, (Changed<Interaction>, With<ConfirmEditServerButton>)>,
    server_name_text: Query<&TextInput, With<ServerNameTextInput>>,
    server_address_text: Query<&TextInput, With<ServerAddressTextInput>>,
    mut add_server_evs: EventWriter<AddServerEntry>,
) {
    for interaction in button.iter() {
        if let Interaction::Pressed = *interaction {
            let name = server_name_text.single().current_text();
            let address = server_address_text.single().current_text();
            add_server_evs.send(AddServerEntry {
                uuid: Uuid::from_random(),
                name: name.into(),
                address: address.into(),
            });
        }
    }
}
