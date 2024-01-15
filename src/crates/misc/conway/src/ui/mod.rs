use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::BLACK;
const CELL_COLOR: Color = Color::GREEN;
const CELL_BORDER_COLOR: Color = Color::DARK_GREEN;

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
        app
            .add_systems(Startup, setup)
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
                text.sections[0].value = "Press".to_string();
                text.sections[0].style.color = PRESSED_BUTTON_TEXT_COLOR;
                *bg_color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = PRESSED_BUTTON_BORDER_COLOR;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                text.sections[0].style.color = HOVERED_BUTTON_TEXT_COLOR;
                *bg_color = HOVERED_BUTTON_COLOR.into();
                border_color.0 = HOVERED_BUTTON_BORDER_COLOR;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                text.sections[0].style.color = NORMAL_BUTTON_TEXT_COLOR;
                *bg_color = NORMAL_BUTTON_COLOR.into();
                border_color.0 = NORMAL_BUTTON_BORDER_COLOR;
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(Camera2dBundle::default());
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
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("./fonts/mainframe/mainframe-opto.ttf"),
                            font_size: 20.0,
                            color: NORMAL_BUTTON_TEXT_COLOR,
                        },
                    ));
                });
        });
}