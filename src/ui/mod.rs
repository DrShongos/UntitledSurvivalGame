use bevy::{app::AppExit, prelude::*};

use crate::{asset::TitleImage, state::GameState, world::ClearWorldEvent};

pub struct UiPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                (
                    button_hover,
                    start_button_click,
                    exit_button_click,
                    continue_button_click,
                    exit_to_menu_button_click,
                    resume_button_click,
                ),
            )
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
            .add_systems(
                OnExit(GameState::Paused),
                despawn_container::<PauseMenuContainer>,
            )
            .add_systems(OnEnter(GameState::Dead), spawn_death_menu)
            .add_systems(
                OnExit(GameState::Dead),
                despawn_container::<DeathMenuContainer>,
            )
            .add_systems(OnExit(GameState::MainMenu), exit_main_menu);
    }
}

#[derive(Component)]
struct MainMenuContainer;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct PauseMenuContainer;

#[derive(Component)]
struct ResumeButton;

#[derive(Component)]
struct ExitToMenuButton;

#[derive(Component)]
struct DeathMenuContainer;

#[derive(Component)]
struct ContinueButton;

fn button_hover(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, mut border_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
                *border_color = Color::BLACK.into();
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
                *border_color = Color::WHITE.into();
            }
            _ => {}
        }
    }
}

fn start_button_click(
    button_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<StartButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::LoadingAssets);
        }
    }
}

fn exit_button_click(
    button_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ExitButton>)>,
    mut app_exit: EventWriter<AppExit>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            app_exit.send(AppExit);
        }
    }
}

fn continue_button_click(
    button_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ContinueButton>)>,
    mut clear_world_event: EventWriter<ClearWorldEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            clear_world_event.send(ClearWorldEvent);
            game_state.set(GameState::MainMenu);
        }
    }
}

fn resume_button_click(
    button_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ResumeButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::InGame);
        }
    }
}

fn exit_to_menu_button_click(
    button_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ExitToMenuButton>)>,
    mut clear_world_event: EventWriter<ClearWorldEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            clear_world_event.send(ClearWorldEvent);
            game_state.set(GameState::MainMenu);
        }
    }
}

fn setup_main_menu(mut commands: Commands, title_image: Res<TitleImage>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    grid_auto_flow: GridAutoFlow::Row,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenuContainer,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(384.0),
                    height: Val::Px(384.0),
                    justify_content: JustifyContent::Center,
                    justify_items: JustifyItems::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                image: UiImage::new(title_image.0.clone()),
                ..Default::default()
            });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            justify_items: JustifyItems::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),

                        ..Default::default()
                    },
                    StartButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font_size: 40.0,
                            ..Default::default()
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            justify_items: JustifyItems::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    ExitButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font_size: 40.0,
                            ..Default::default()
                        },
                    ));
                });
        });
}

fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(25.0),
                    height: Val::Percent(45.0),
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    justify_items: JustifyItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    grid_auto_flow: GridAutoFlow::Row,
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                background_color: Color::DARK_GRAY.into(),
                border_color: Color::BLACK.into(),
                ..Default::default()
            },
            PauseMenuContainer,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Paused",
                TextStyle {
                    font_size: 40.0,
                    ..Default::default()
                },
            ));

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(160.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            justify_items: JustifyItems::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    ResumeButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Resume",
                        TextStyle {
                            font_size: 40.0,
                            ..Default::default()
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(160.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            justify_items: JustifyItems::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    ExitToMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit to Menu",
                        TextStyle {
                            font_size: 40.0,
                            ..Default::default()
                        },
                    ));
                });
        });
}

fn spawn_death_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(25.0),
                    height: Val::Percent(45.0),
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    justify_items: JustifyItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    grid_auto_flow: GridAutoFlow::Row,
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                background_color: Color::DARK_GRAY.into(),
                border_color: Color::BLACK.into(),
                ..Default::default()
            },
            DeathMenuContainer,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Game Over",
                TextStyle {
                    font_size: 40.0,
                    ..Default::default()
                },
            ));

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(160.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            justify_items: JustifyItems::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    },
                    ContinueButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Continue",
                        TextStyle {
                            font_size: 40.0,
                            ..Default::default()
                        },
                    ));
                });
        });
}

fn exit_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenuContainer>>) {
    let menu_container = menu_query.single();

    commands.entity(menu_container).despawn_recursive();
}

fn despawn_container<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}
