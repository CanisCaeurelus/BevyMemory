use bevy::{prelude::*, ui::FocusPolicy};

pub struct GameMenuPlugin;

use crate::{AppState,MainCamera,NumberOfPlayers,BoardSize,GameScore};
use super::macros;

#[derive(Component)]
struct OnMainMenuScreen;
#[derive(Component)]
struct Button1Player;
#[derive(Component)]
struct Button2Player;
#[derive(Component)]
struct Button3Player;
#[derive(Component)]
struct Button4Player;
#[derive(Component)]
struct Button5Player;

#[derive(Component)]
struct Button12Cards;
#[derive(Component)]
struct Button16Cards;
#[derive(Component)]
struct Button20Cards;
#[derive(Component)]
struct Button24Cards;
#[derive(Component)]
struct Button30Cards;

#[derive(Component)]
struct PlayerNumberLabel;
#[derive(Component)]
struct CardNumberLabel;



#[derive(Resource)]
struct UiAssets{
    font:Handle<Font>,
    button:Handle<Image>,
    button_pressed:Handle<Image>
}
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App){
        app.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(handle_start_button)
    )

        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_menu))
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(despawn_menu));
    }
}

fn setup_menu(mut commands: Commands, assets: Res<AssetServer>)
{
    commands.spawn((Camera2dBundle::default(),MainCamera));
    let ui_assets = UiAssets{
        font:assets.load("fonts/FiraMono-Medium.ttf"),
        button:assets.load("buton.png"),
        button_pressed:assets.load("buton_presd.png")
    };

    commands
            .spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnMainMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            size: Size::new(Val::Percent(100.0),Val::Percent(100.0)),
                            ..default()
                        },
                        background_color: Color::rgb(0.7,0.4,0.2).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Display the game name
                        macros::spawn_label!(parent,ui_assets,40.0,"BevyMemory");
                        macros::spawn_editable_label!(parent,ui_assets,PlayerNumberLabel,"Number of players");
                        parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            size: Size::new(Val::Percent(60.0),Val::Percent(100.0)),
                            margin: UiRect::all(Val::Auto),
                            ..default()
                        },
                        background_color: Color::rgb(0.7,0.4,0.2).into(),
                        ..default()
                    })
                    .with_children(|parent| 
                    {
                        macros::spawn_button!(parent,ui_assets,Button1Player,"1");
                        macros::spawn_button!(parent,ui_assets,Button2Player,"2");
                        macros::spawn_button!(parent,ui_assets,Button3Player,"3");
                        macros::spawn_button!(parent,ui_assets,Button4Player,"4");
                        macros::spawn_button!(parent,ui_assets,Button5Player,"5");
                    });
                    macros::spawn_editable_label!(parent,ui_assets,CardNumberLabel,"Number of cards");

                        parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            size: Size::new(Val::Percent(60.0),Val::Percent(100.0)),
                            margin: UiRect::all(Val::Auto),
                            ..default()
                        },
                        background_color: Color::rgb(0.7,0.4,0.2).into(),
                        ..default()
                    })
                    .with_children(|parent| 
                    {
                        macros::spawn_button!(parent,ui_assets,Button12Cards,"12");
                        macros::spawn_button!(parent,ui_assets,Button16Cards,"16");
                        macros::spawn_button!(parent,ui_assets,Button20Cards,"20");
                        macros::spawn_button!(parent,ui_assets,Button24Cards,"24");
                        macros::spawn_button!(parent,ui_assets,Button30Cards,"30");
                    });



                        parent.spawn(ButtonBundle{
                            style: Style{
                                align_self: AlignSelf::Center,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                size: Size::new(Val::Percent(20.0),Val::Percent(100.0)),
                                margin: UiRect::all(Val::Px(50.0)),
                                
                                ..default()
                            },
                            background_color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|parent|{
                            parent.spawn(ImageBundle{
                                style: Style{
                                    size: Size::new(Val::Percent(100.0),Val::Percent(100.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                image: ui_assets.button.clone().into(),
                                ..default()
                            }).insert(FocusPolicy::Pass).with_children(|parent|{
                                parent.spawn(TextBundle{
                                    text: Text::from_section(
                                        "Start game",
                                        TextStyle{
                                            font: ui_assets.font.clone(),
                                            font_size: 40.0,
                                            color: Color::rgb(0.9,0.9,0.9),
                                        }),
                                        focus_policy: FocusPolicy::Pass,
                                        ..Default::default()
                                    });
                                    
                                });
                            });
                        });
                    });
    commands.insert_resource(ui_assets);
}

fn handle_start_button(
    interaction_query: Query<(&Children,&Interaction, 
        Option<&Button1Player>,
        Option<&Button2Player>,
        Option<&Button3Player>,
        Option<&Button4Player>,
        Option<&Button5Player>,
        Option<&Button12Cards>,
        Option<&Button16Cards>,
        Option<&Button20Cards>,
        Option<&Button24Cards>,
        Option<&Button30Cards>
    ),Changed<Interaction>>,
    mut image_query: Query<&mut UiImage>,
    ui_assets: Res<UiAssets>,
    mut app_state: ResMut<State<AppState>>,
    mut num_of_players: ResMut<NumberOfPlayers>,
    mut board_size: ResMut<BoardSize>,
    mut player_label_query: Query<&mut Text,(With<PlayerNumberLabel>, Without<CardNumberLabel>)>,
    mut card_label_query: Query<&mut Text,(With<CardNumberLabel>, Without<PlayerNumberLabel>)>,
    //ascii: Res<AsciiSheet>
)
{
    for (children,interaction,
        btn1p,
        btn2p,
        btn3p,
        btn4p,
        btn5p,
        btn12c,
        btn16c,
        btn20c,
        btn24c,
        btn30c
    
    ) in interaction_query.iter()
    {
        let child = children.iter().next().unwrap();
        let mut image = image_query.get_mut(*child).unwrap();
        match interaction{
            Interaction::Clicked => {
                if btn1p.is_some()
                {
                    num_of_players.num = 1;
                }
                else
                if btn2p.is_some()
                {
                    num_of_players.num = 2;
                }
                else
                if btn3p.is_some()
                {
                    num_of_players.num = 3;
                }
                else
                if btn4p.is_some()
                {
                    num_of_players.num = 4;
                }
                else
                if btn5p.is_some()
                {
                    num_of_players.num = 5;
                }
                else
                if btn12c.is_some()
                {
                    board_size.size = Vec2::new(4.0,3.0);
                }
                else
                if btn16c.is_some()
                {
                    board_size.size = Vec2::new(4.0,4.0);
                }
                else
                if btn20c.is_some()
                {
                    board_size.size = Vec2::new(5.0,4.0);
                }
                else
                if btn24c.is_some()
                {
                    board_size.size = Vec2::new(6.0,4.0);
                }
                else
                if btn30c.is_some()
                {
                    board_size.size = Vec2::new(6.0,5.0);
                }
                else
                {
                    app_state.set(AppState::InGame).unwrap();
                }
                image.0 = ui_assets.button_pressed.clone();
                for mut text in player_label_query.iter_mut()
                {
                    let some_value = num_of_players.num;
                    text.sections[0].value = format!("Number of players: {some_value}");
                }

                for mut text in card_label_query.iter_mut()
                {
                    let some_value = board_size.size.x * board_size.size.y;
                    text.sections[0].value = format!("Number of cards: {some_value}");
                }
            }
            Interaction::Hovered =>
            {
                image.0 = ui_assets.button_pressed.clone();
            } 
            Interaction::None =>
            {
                image.0 = ui_assets.button.clone();
            }
        }
    }
}


fn despawn_menu(
    mut commands: Commands,
    cam_query: Query<Entity, With<MainCamera>>,
    butt_query: Query<Entity, With<Button>>,
    men_query: Query<Entity, With<OnMainMenuScreen>>,
)
{
    for ent in cam_query.iter()
    {
        commands.entity(ent).despawn_recursive();
    }
    for ent in butt_query.iter()
    {
        commands.entity(ent).despawn_recursive();
    }
    commands.remove_resource::<UiAssets>();
    for ent in men_query.iter()
    {
        commands.entity(ent).despawn_recursive();
    }
}