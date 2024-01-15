use bevy::{prelude::*, ui::FocusPolicy};

const GRID_BACKGROUND_COLOR: Color = Color::DARK_GRAY;
const CELL_COLOR: Color = Color::GREEN;
const CELL_BORDER_COLOR: Color = Color::DARK_GREEN;

const BUTTON_SECTION_BACKGROUND_COLOR: Color = Color::CRIMSON;

const NORMAL_BUTTON_COLOR: Color = Color::WHITE;
const NORMAL_BUTTON_TEXT_COLOR: Color = Color::BLACK;
const NORMAL_BUTTON_BORDER_COLOR: Color = Color::BLACK;

const HOVERED_BUTTON_COLOR: Color = Color::GRAY;
const HOVERED_BUTTON_TEXT_COLOR: Color = Color::WHITE;
const HOVERED_BUTTON_BORDER_COLOR: Color = Color::WHITE;

const PRESSED_BUTTON_COLOR: Color = Color::DARK_GRAY;
const PRESSED_BUTTON_TEXT_COLOR: Color = Color::YELLOW;
const PRESSED_BUTTON_BORDER_COLOR: Color = Color::YELLOW;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, button_system);
    }
}

fn button_system(
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
    for (interaction, mut bg_color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].style.color = PRESSED_BUTTON_TEXT_COLOR;
                *bg_color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = PRESSED_BUTTON_BORDER_COLOR;
            }
            Interaction::Hovered => {
                text.sections[0].style.color = HOVERED_BUTTON_TEXT_COLOR;
                *bg_color = HOVERED_BUTTON_COLOR.into();
                border_color.0 = HOVERED_BUTTON_BORDER_COLOR;
            }
            Interaction::None => {
                text.sections[0].style.color = NORMAL_BUTTON_TEXT_COLOR;
                *bg_color = NORMAL_BUTTON_COLOR.into();
                border_color.0 = NORMAL_BUTTON_BORDER_COLOR;
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                ..default()
            },
            background_color: GRID_BACKGROUND_COLOR.into(),
            ..default()
        })
        .with_children(|parent| {
            // Set up the bottom bar...
            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Px(50.0),
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    background_color: BUTTON_SECTION_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // "PREV" button
                    spawn_button(parent, &asset_server, "PREV");
                    // "PLAY" button
                    spawn_button(parent, &asset_server, "PLAY");
                    // "NEXT" button
                    spawn_button(parent, &asset_server, "NEXT");
                });
        });
}

fn spawn_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, text: &str) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                height: Val::Px(40.0),
                width: Val::Px(100.0),
                margin: UiRect::all(Val::Px(5.0)),              // Space between the buttons
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON_COLOR.into(),
            // Make sure to handle focus so the button can be clicked
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .insert(ButtonActions::default()) // This will track the button state (hovered, pressed, etc.)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/mainframe/mainframe-spore.ttf"),
                        font_size: 20.0,
                        color: NORMAL_BUTTON_TEXT_COLOR,
                    },
                ),
                ..default()
            });
        });
}

// Define a component to track button actions
#[derive(Default, Component)]
struct ButtonActions {
    hovered: bool,
    pressed: bool,
}
