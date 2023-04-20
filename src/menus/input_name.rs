use bevy::{prelude::*, ui::FocusPolicy};
use crate::{AppState,MainCamera,NumberOfPlayers,BoardSize, GameScore};
#[derive(Resource)]
struct UiAssets{
    font:Handle<Font>,
    button:Handle<Image>,
    button_pressed:Handle<Image>
}
use super::macros;
pub struct NamePlugin;
impl Plugin for NamePlugin {
    fn build(&self, app: &mut App){
        app.add_system_set(SystemSet::on_update(AppState::NameMenu).with_system(text_input))
        .add_system_set(SystemSet::on_update(AppState::NameMenu).with_system(handle_button))

        .add_system_set(SystemSet::on_enter(AppState::NameMenu).with_system(setup_input_name))
        
        .add_system_set(SystemSet::on_exit(AppState::NameMenu).with_system(despawn_input_name));
    }
}

#[derive(Component)]
struct OnNameScreen;

#[derive(Component)]
struct PlayerNameLabel;

fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
    mut player_name_query: Query<&mut Text,(With<PlayerNameLabel>)>,
    mut game_score: ResMut<GameScore>,
    mut app_state: ResMut<State<AppState>>,
) {
    for ev in char_evr.iter() {
        if !keys.just_pressed(KeyCode::Return)
        {
            println!("Got char: '{}'", ev.char);
            string.push(ev.char);
            game_score.player_name = string.clone();
        }
    }

    if keys.just_pressed(KeyCode::Back) {
        println!("<");
        string.pop(); 
        string.pop();
        println!("Text input: {}", *string);
        game_score.player_name = string.clone();
    }

    if keys.just_pressed(KeyCode::Return) {
        println!("Text input: {}", *string);
        
        if game_score.player_name != ""
        {
            game_score.player_name = string.clone();
            string.clear();
            app_state.set(AppState::HighScore).unwrap();
        }
    }

    for mut name in player_name_query.iter_mut()
    {
        name.sections[0].value = string.clone();
    }
}

fn handle_button(
    interaction_query: Query<(&Children,&Interaction,
    ),Changed<Interaction>>,
    mut image_query: Query<&mut UiImage>,
    ui_assets: Res<UiAssets>,
    mut app_state: ResMut<State<AppState>>,
    mut game_score: ResMut<GameScore>,
    //ascii: Res<AsciiSheet>
)
{
    for (children,interaction) in interaction_query.iter()
    {
        let child = children.iter().next().unwrap();
        let mut image = image_query.get_mut(*child).unwrap();
        match interaction{
            Interaction::Clicked => {
                if game_score.player_name != ""
                {
                    app_state.set(AppState::HighScore).unwrap();
                }
                image.0 = ui_assets.button_pressed.clone();
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

fn setup_input_name(mut commands: Commands, assets: Res<AssetServer>, board_size: Res<BoardSize>, player_num: Res<NumberOfPlayers>,
    game_score: Res<GameScore>)
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
                        background_color: Color::rgb(0.7,0.4,0.2).into(),
                        ..default()
                    }
                , OnNameScreen))
                .with_children(|parent| {
                    parent
                        .spawn((NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                size: Size::new(Val::Percent(40.0),Val::Percent(40.0)),
                                ..default()
                            },
                            background_color: Color::rgb(0.7,0.4,0.2).into(),
                            ..default()
                        }))
                        .with_children(|parent| {
                            // Display the game name
                            macros::spawn_label!(parent,ui_assets,20.0,"What's your name?",100.0);
                            macros::spawn_editable_label!(parent,ui_assets,PlayerNameLabel,"");
                        
                        parent.spawn(ButtonBundle{
                            style: Style{
                                align_self: AlignSelf::Center,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                size: Size::new(Val::Percent(50.0),Val::Percent(200.0)),
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
                                        "Continue",
                                        TextStyle{
                                            font: ui_assets.font.clone(),
                                            font_size: 20.0,
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

    fn despawn_input_name(
        mut commands: Commands,
        cam_query: Query<Entity, With<MainCamera>>,
        butt_query: Query<Entity, With<Button>>,
        men_query: Query<Entity, With<OnNameScreen>>,
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