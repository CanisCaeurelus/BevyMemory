
use bevy::{prelude::*, ui::FocusPolicy};
use serde::{Serialize,Deserialize};

pub struct HighScorePlugin;

use crate::{AppState,MainCamera,NumberOfPlayers,BoardSize, GameScore};
use super::macros;

#[cfg(not(target_arch = "wasm32"))]
use ureq::{Agent, AgentBuilder,json,Error};
use std::time::Duration;
#[derive(Resource)]
struct UiAssets{
    font:Handle<Font>,
    button:Handle<Image>,
    button_pressed:Handle<Image>
}

#[derive(Serialize,Deserialize,Clone)]
pub struct Score{
    id:i32,
    date:i64,
    game_time:i32,
    player_name:String,
    card_num:i32,
    moves:i32,
}

#[derive(Serialize,Deserialize)]
pub struct CreateEntryData{
    pub player_name:String,
    pub game_time:i32,
    pub card_num:i32,
    pub moves:i32,
}

#[derive(Component)]
struct OnHighScoreScreen;

#[derive(Component)]
struct HighScoreTime;

#[derive(Component)]
struct HighScoreMoves;

impl Plugin for HighScorePlugin {
    fn build(&self, app: &mut App){
        app.add_system_set(SystemSet::on_update(AppState::HighScore).with_system(handle_menu_button))
        .add_system_set(SystemSet::on_update(AppState::HighScore).with_system(handle_high_scores))

        .add_system_set(SystemSet::on_enter(AppState::HighScore).with_system(setup_high_score))
        
        .add_system_set(SystemSet::on_exit(AppState::HighScore).with_system(despawn_high_score));
    }
}

fn setup_high_score(mut commands: Commands, assets: Res<AssetServer>, board_size: Res<BoardSize>, player_num: Res<NumberOfPlayers>,
game_score: Res<GameScore>)
{
    commands.spawn((Camera2dBundle::default(),MainCamera));
    let ui_assets = UiAssets{
        font:assets.load("fonts/FiraMono-Medium.ttf"),
        button:assets.load("buton.png"),
        button_pressed:assets.load("buton_presd.png")
    };
    if player_num.num == 1
    {
        // let mut scores_moves: Vec<Score> = Vec::new();
        // if !scores_times.is_empty()
        // {
        //     scores_moves = get_high_scores(&agent,(board_size.size.x * board_size.size.y) as i32, "moves".to_string());
        //     if game_score.locked 
        //     {
        //     post_score(&agent,CreateEntryData{
        //         player_name: game_score.player_name.to_string(),
        //         game_time: game_score.game_time,
        //         card_num:(board_size.size.x * board_size.size.y) as i32,
        //         moves: game_score.moves});
        //     }
        // }
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
                },OnHighScoreScreen
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
                        macros::spawn_label!(parent,ui_assets,40.0,"BevyMemory",40.0);
                        let label = format!("HighScores for {} cards", board_size.size.x * board_size.size.y);
                        macros::spawn_label!(parent,ui_assets,32.0,label,40.0);
                        parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            size: Size::new(Val::Percent(100.0),Val::Percent(120.0)),
                            margin: UiRect::all(Val::Auto),
                            ..default()
                        },
                        background_color: Color::rgb(0.7,0.4,0.2).into(),
                        ..default()
                    })
                    .with_children(|parent| 
                    {
                        parent
                        .spawn((NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                size: Size::new(Val::Percent(100.0),Val::Percent(100.0)),
                                ..default()
                            },
                            background_color: Color::rgb(0.7,0.4,0.2).into(),
                            ..default()
                        },HighScoreTime));
                        // .with_children(|parent| {
                        //     let scores = scores_times;
                        //     let mut lp:i32 = 0;
                        //     for score in scores.iter()
                        //     {
                        //         lp += 1;
                        //         let entry = format!("{}. {}: {}s ({} moves)",lp,score.player_name,(score.game_time as f32)/1000.0, score.moves);
                        //         macros::spawn_label!(parent,ui_assets,20.0,entry);
                        //     }

                        // });

                        parent
                    .spawn((NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            size: Size::new(Val::Percent(100.0),Val::Percent(100.0)),
                            ..default()
                        },
                        background_color: Color::rgb(0.7,0.4,0.2).into(),
                        ..default()
                    },HighScoreMoves));
                    // .with_children(|parent| {
                    //     let scores = scores_moves;
                    //     let mut lp:i32 = 0;
                    //     for score in scores.iter()
                    //     {
                    //         lp += 1;
                    //         let entry = format!("{}. {}: {} moves ({}s)",lp,score.player_name,score.moves,(score.game_time as f32)/1000.0);
                    //         macros::spawn_label!(parent,ui_assets,20.0,entry);
                    //     }

                    // });
                    });
                    parent.spawn(ButtonBundle{
                        style: Style{
                            align_self: AlignSelf::Center,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            size: Size::new(Val::Percent(20.0),Val::Percent(40.0)),
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
                                    "Main menu",
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
                }
    commands.insert_resource(ui_assets);
}

fn handle_menu_button(
    interaction_query: Query<(&Children,&Interaction),Changed<Interaction>>,
    ui_assets: Res<UiAssets>,
    mut image_query: Query<&mut UiImage>,
    mut app_state: ResMut<State<AppState>>,
    player_num: Res<NumberOfPlayers>,
)
{
    if player_num.num == 1
    {
        for (children,interaction) in interaction_query.iter()
        {
            let child = children.iter().next().unwrap();
            let mut image = image_query.get_mut(*child).unwrap();
            match interaction{
                Interaction::Clicked => {
                        app_state.set(AppState::MainMenu).unwrap();
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
    else 
    {
        app_state.set(AppState::MainMenu).unwrap();
    }
}

fn handle_high_scores(
    mut commands: Commands, assets: Res<UiAssets>, board_size: Res<BoardSize>, player_num: Res<NumberOfPlayers>,
game_score: Res<GameScore>, label_time: Query<Entity,With<HighScoreTime>>, label_moves: Query<Entity,With<HighScoreMoves>>, 
)
{
    
    for ent in label_time.iter()
    {
        if game_score.locked 
            {
            post_score(CreateEntryData{
                player_name: game_score.player_name.to_string(),
                game_time: game_score.game_time,
                card_num:(board_size.size.x * board_size.size.y) as i32,
                moves: game_score.moves});
            }


    let scores_times = get_high_scores((board_size.size.x * board_size.size.y) as i32, "time".to_string());

        commands.entity(ent).add_children(|parent| {
            let scores = scores_times.clone();
            let mut lp:i32 = 0;
            for score in scores.iter()
            {
                lp += 1;
                let entry = format!("{}. {}: {}s ({} moves)",lp,score.player_name,(score.game_time as f32)/1000.0, score.moves);
                macros::spawn_label!(parent,assets,20.0,entry);
            }
            //macros::spawn_label!(parent,assets,20.0,"lololo");

        });
        commands.entity(ent).remove::<HighScoreTime>();
        
    }
    for ent in label_moves.iter()
    {
        let scores_moves = get_high_scores((board_size.size.x * board_size.size.y) as i32, "moves".to_string());

        commands.entity(ent).add_children(|parent| {
            let moves = scores_moves.clone();
            let mut lp = 0;
            for score in moves.iter()
            {
                lp += 1;
                let entry = format!("{}. {}: {} moves ({}s)",lp,score.player_name,score.moves,(score.game_time as f32)/1000.0);
                macros::spawn_label!(parent,assets,20.0,entry);
            }

        });
        commands.entity(ent).remove::<HighScoreMoves>();
        
    }
    
    
    
}

fn despawn_high_score(
    mut commands: Commands,
    cam_query: Query<Entity, With<MainCamera>>,
    butt_query: Query<Entity, With<Button>>,
    men_query: Query<Entity, With<OnHighScoreScreen>>,
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

#[cfg(not(target_arch = "wasm32"))]
fn get_high_scores(number_of_cards:i32, sort_by:String) -> Vec<Score>
{
    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_millis(1000))
        .timeout_read(Duration::from_millis(1000))
        .timeout_write(Duration::from_millis(1000))
        .build();
    let body = get_body(&agent,number_of_cards,sort_by).unwrap_or("".to_string());
    
    println!("body: {:?}", body);
    let scores : Vec<Score>= serde_json::from_str(&body).unwrap_or_default();
    return scores;
}

#[cfg(target_arch = "wasm32")]
fn get_high_scores(_number_of_cards:i32, _sort_by:String) -> Vec<Score>
{
    Vec::new()
}

#[cfg(not(target_arch = "wasm32"))]
fn get_body(agent:&Agent, number_of_cards:i32, sort_by:String) -> Result<String,ureq::Error>
{    
    let s: String = format!("http://bevymemory.fly.dev/highscores/{}/{}",number_of_cards,sort_by).to_owned();
    let s_slice: &str = &s[..];
    let body: String = agent.get(s_slice)
    .set("Example-Header", "header value")
    .call()?
    .into_string()?;
    Ok(body)
}

fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
) {
    for ev in char_evr.iter() {
        println!("Got char: '{}'", ev.char);
        string.push(ev.char);
    }

    if keys.just_pressed(KeyCode::Return) {
        println!("Text input: {}", *string);
        string.clear();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn post_score(score: CreateEntryData) 
{
    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_millis(10000))
        .timeout_read(Duration::from_millis(10000))
        .timeout_write(Duration::from_millis(10000))
        .build();
    
    match agent.post("https://bevymemory.fly.dev/score")
    .set("Accept", "*/*")
    .set("User-Agent", "Thunder Client (https://www.thunderclient.com)")
    .send_json(ureq::json!({
        "player_name": score.player_name,
        "game_time": score.game_time,
        "card_num": score.card_num,
        "moves": score.moves
    })){
    Ok(response) => { println!("Score could be post"); /* it worked */},
    Err(Error::Status(code, response)) => {
        /* the server returned an unexpected status
        code (such as 400, 500 etc) */
        println!("Cannot post score {}", code);
    }
        Err(_) => { /* some kind of io/transport error */ }
    }
    
}

#[cfg(target_arch = "wasm32")]
fn post_score(_score: CreateEntryData) 
{
    // HTTP not supported in WASM build
}