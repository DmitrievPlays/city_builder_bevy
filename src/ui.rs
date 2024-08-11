use bevy::{
    color::palettes::css::RED,
    prelude::*,
    ui::{widget::UiImageSize, ContentSize},
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, create_build_menu, create_topbar))
            .add_systems(Update, (button_system, open_build_menu));
    }
}

#[derive(Component)]
pub struct BuildingUI;

pub fn create_build_menu(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                bottom: Val::Px(10.0),
                width: Val::Percent(20.0),
                height: Val::Px(120.0),
                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            border_radius: BorderRadius::all(Val::Px(16.0)),
            ..Default::default()
        },
        BuildingUI,
    ));
}

#[derive(Component)]
pub struct TopBarUI;

pub fn create_topbar(mut commands: Commands) {
    let a = commands
        .spawn(TextBundle::from_section("000", default()))
        .id();
    let b = commands
        .spawn(TextBundle::from_section("001", default()))
        .id();
    let c = commands
        .spawn(TextBundle::from_section("002", default()))
        .id();

    commands
        .spawn((TopBarUI,))
        .push_children(&[a, b, c])
        //.insert(TextBundle::from_section("00", default()))
        .insert(TextBundle::from_section("01", default()))
        .insert(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                ..Default::default()
            },
            background_color: Color::srgb_u8(48, 57, 82).into(),
            ..Default::default()
        });
}

pub fn open_build_menu(
    //ui_state: Res<GameUIStates>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    // if input.just_pressed(KeyCode::KeyB) {
    //     if ui_state.build_menu_opened {
    //         commands.insert_resource(GameUIStates {
    //             build_menu_opened: true,
    //         });
    //     }
    // }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn setup(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    _materials: ResMut<Assets<StandardMaterial>>,
) {
    let text_parent = NodeBundle {
        node: NodeBundle {
            ..Default::default()
        }
        .node,
        ..Default::default()
    };

    let icon_parent = NodeBundle {
        node: NodeBundle {
            ..Default::default()
        }
        .node,
        ..Default::default()
    };

    let money_text_entity = commands.spawn(text_parent.clone()).id();
    let gold_text_entity = commands.spawn(text_parent.clone()).id();
    let money_icon_entity = commands.spawn(icon_parent.clone()).id();

    // commands
    //     .entity(money_text_entity)
    //     .insert(TextBundle::from_section("00", default()));
    // commands
    //     .entity(gold_text_entity)
    //     .insert(TextBundle::from_section("01", default()));
    commands.entity(money_icon_entity).insert(ImageBundle {
        image: _asset_server.load("icons/money.png").into(),
        style: Style {
            width: Val::Px(36.0),
            height: Val::Px(36.0), // Set the size of the image
            ..Default::default()
        },
        ..Default::default()
    });

    return;
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}
