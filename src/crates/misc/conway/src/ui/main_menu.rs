use bevy::{prelude::*, ui::FocusPolicy};

const CONTAINER_BACKGROUND_COLOR: Color = Color::DARK_GRAY;
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
pub struct NextStepEvent;

#[derive(Event)]
pub struct PreviousStepEvent;

#[derive(Event)]
pub struct RandomizeGameEvent;

#[derive(Component)]
struct MenuButton(ButtonAction);

#[derive(PartialEq, Copy, Clone)]
enum ButtonAction {
    Next,
    Prev,
    Randomize,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NextStepEvent>()
            .add_event::<PreviousStepEvent>()
            .add_event::<RandomizeGameEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, button_system);
    }
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
    mut next_writer: EventWriter<NextStepEvent>,
    mut prev_writer: EventWriter<PreviousStepEvent>,
    mut random_writer: EventWriter<RandomizeGameEvent>,
) {
    for (interaction, menu_btn, mut bg_color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].style.color = PRESSED_BUTTON_TEXT_COLOR;
                *bg_color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = PRESSED_BUTTON_BORDER_COLOR;

                match menu_btn.0 {
                    ButtonAction::Next => {
                        println!("Next");
                        next_writer.send(NextStepEvent);
                    }
                    ButtonAction::Prev => {
                        println!("Prev");
                        prev_writer.send(PreviousStepEvent);
                    }
                    ButtonAction::Randomize => {
                        println!("Randomize");
                        random_writer.send(RandomizeGameEvent);
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn(Camera2dBundle::default());
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
            background_color: CONTAINER_BACKGROUND_COLOR.into(),
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
                    spawn_button(parent, &asset_server, "PREV", ButtonAction::Prev);
                    // "PLAY" button
                    spawn_button(parent, &asset_server, "RANDOM", ButtonAction::Randomize);
                    // "NEXT" button
                    spawn_button(parent, &asset_server, "NEXT", ButtonAction::Next);
                });
        });
}

fn spawn_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, text: &str, button_action: ButtonAction) {
    parent
        .spawn(build_button(asset_server, text))
        .with_children(|parent| {
            parent.spawn(build_button_text(asset_server, text));
        })
        .insert(MenuButton(button_action));
}

fn build_button(asset_server: &Res<AssetServer>, text: &str) -> ButtonBundle {
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
