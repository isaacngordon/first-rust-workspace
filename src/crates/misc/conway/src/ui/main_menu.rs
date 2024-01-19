use bevy::{prelude::*, ui::FocusPolicy};
use bevy_inspector_egui::egui::Checkbox;

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

#[derive(Event)]
pub enum MenuButtonEvent {
    NextStepEvent,
    PreviousStepEvent,
    RandomizeGameEvent,
    ToggleContinuousEvent,
}

#[derive(Component)]
struct MenuButton(ButtonAction);

#[derive(PartialEq, Copy, Clone)]
enum ButtonAction {
    Next,
    Prev,
    Randomize,
    ToggleContinuous,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuButtonEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, button_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // Set up the main container spanning the entire screen...
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Bottom UP
            // 2nd row of buttons
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
                    spawn_button(
                        parent,
                        &asset_server,
                        "Toggle",
                        ButtonAction::ToggleContinuous,
                    );
                });

            // first row of buttons
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
                    spawn_button(parent, &asset_server, "PREV", ButtonAction::Prev);
                    // "PLAY" button
                    spawn_button(parent, &asset_server, "RANDOM", ButtonAction::Randomize);
                    // "NEXT" button
                    spawn_button(parent, &asset_server, "NEXT", ButtonAction::Next);
                });
        });
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &MenuButton,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut button_event_writer: EventWriter<MenuButtonEvent>,
) {
    for (interaction, menu_btn, mut bg_color, mut border_color, children) in &mut interaction_query
    {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].style.color = PRESSED_BUTTON_TEXT_COLOR;
                *bg_color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = PRESSED_BUTTON_BORDER_COLOR;

                match menu_btn.0 {
                    ButtonAction::Next => {
                        println!("Next");
                        button_event_writer.send(MenuButtonEvent::NextStepEvent);
                    }
                    ButtonAction::Prev => {
                        println!("Prev");
                        button_event_writer.send(MenuButtonEvent::PreviousStepEvent);
                    }
                    ButtonAction::Randomize => {
                        println!("Randomize");
                        button_event_writer.send(MenuButtonEvent::RandomizeGameEvent);
                    }
                    ButtonAction::ToggleContinuous => {
                        println!("Toggle Continuous");
                        button_event_writer.send(MenuButtonEvent::ToggleContinuousEvent);
                    }
                }
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

fn spawn_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    button_action: ButtonAction,
) {
    parent
        .spawn(build_button_bundle())
        .with_children(|parent| {
            parent.spawn(build_button_text(asset_server, text));
        })
        .insert(MenuButton(button_action));
}

fn build_button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            height: Val::Px(40.0),
            width: Val::Px(100.0),
            margin: UiRect::all(Val::Px(5.0)), // Space between the buttons
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: NORMAL_BUTTON_COLOR.into(),
        // Make sure to handle focus so the button can be clicked
        focus_policy: FocusPolicy::Pass,
        ..default()
    }
}

fn build_button_text(asset_server: &Res<AssetServer>, text: &str) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/mainframe/mainframe-spore.ttf"),
                font_size: 20.0,
                color: NORMAL_BUTTON_TEXT_COLOR,
            },
        ),
        ..default()
    }
}
