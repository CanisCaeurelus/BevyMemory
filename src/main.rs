//! This example displays each contributor to the bevy source code as a bouncing bevy-ball.

use std::f32::consts::E;

use bevy::{prelude::*};
use rand::{prelude::SliceRandom};

use {core::f32::consts::PI};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use reqwest;

mod game_menu;

use game_menu::GameMenuPlugin;
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Restart,
}

fn main() {
    let mut app = App::new();

        app.add_plugins(DefaultPlugins)
        .add_plugin(GameMenuPlugin)
        .add_startup_system(state_system)
        // add the app state type
        .add_state(AppState::MainMenu)
        // systems to run only in the main menu
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(select_card_system)
                .with_system(cursor_state_system)
                .with_system(clickable_card)
                .with_system(flip_card_system)
                .with_system(destroy_cards_system)
                .with_system(hoverable)
                .with_system(move_card_system)
                .with_system(select_player_system)
                .with_system(check_board_state)
        )

        // setup when entering the state
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(setup)
                .with_system(setup_cards)
                .with_system(setup_players)
        )

        // cleanup when exiting the state
        .add_system_set(
            SystemSet::on_exit(AppState::InGame)
            .with_system(despawn_scene)
                
        )
        .run();
}

#[derive(Resource)]
struct CardSelection {
    order: Vec<Entity>,
}
#[derive(Resource)]
struct CardCount {
    count: u8,
}

// impl Default for SelectionState {
//     fn default() -> Self {
//         Self {
//             timer: Timer::from_seconds(SHOWCASE_TIMER_SECS, TimerMode::Repeating),
//             has_triggered: false,
//         }
//     }
// }

#[derive(Component)]
struct RotateCard
{
    rotation: f32,
    duration: f32,
}

#[derive(Component)]
struct MainCamera;
#[derive(Component)]
struct Hoverable;
#[derive(Component)]
struct Hovered;
#[derive(Component)]
struct Clickable;
#[derive(Component)]
struct Clicked;
#[derive(Component)]
struct Card {
    hju:f32,
    index:usize,
}

#[derive(Component)]
struct MemoryPlayer {
    colour:Color,
    index:usize,
    collected_cards:usize,
}

impl Clone for MemoryPlayer {
    fn clone(&self) -> Self {
        MemoryPlayer {
            colour: self.colour.clone(),
            index: self.index.clone(),
            collected_cards:self.collected_cards.clone(),
        }
    }
}

#[derive(Component)]
struct CursorState {
    cursor_world: Vec2,
    cursor_moved: bool,
}

#[derive(Component)]
struct MovedCard 
{
    init_transform: Transform,
    target_transl: Vec3,
    target_rot:f32,
    progression: f32,
    speed: f32,
}

#[derive(Component)]
struct Collected;

#[derive(Component)]
struct Rewers;
#[derive(Component)]
struct Awers;
const SPRITE_SIZE: f32 = 95.0;

const SATURATION_DESELECTED: f32 = 0.0;
const LIGHTNESS_DESELECTED: f32 = 0.9;
const SATURATION_SELECTED: f32 = 0.0;
const LIGHTNESS_SELECTED: f32 = 1.0;
const ALPHA: f32 = 1.0;

const SHOWCASE_TIMER_SECS: f32 = 3.0;

const PLAYER_DISTANCE: f32 = 400.0;
const PLAYER__HAND_DISTANCE: f32 = 80.0;

#[derive(Resource)]
struct ImportedImagesFront {
    handles: Vec<Handle<Image>>,
}

#[derive(Resource)]
struct BoardSize {
    size: Vec2
}

#[derive(Resource)]
struct NumberOfPlayers {
    num: usize
}

#[derive(Resource)]
struct ImportedImageBack {
    handle: Handle<Image>,
}

#[derive(Resource)]
struct ClickedCardIndex {
    index: usize,
}

#[derive(Resource)]
struct CurrentPlayerIndex {
    index: usize,
    last_index: usize,
    transition_time: f32,
    transition_speed: f32
}

#[derive(Resource)]
struct CurrentPlayer{
    order: Vec<Entity>,
}


#[derive(Resource)]
struct CardToBeDestroyed {
    index: usize,
}

impl Clone for ImportedImageBack {
    fn clone(&self) -> Self {
        ImportedImageBack {
            handle: self.handle.clone(),
        }
    }
}

fn state_system(mut commands:Commands,mut app_state: ResMut<State<AppState>>) {
    let num_of_players = NumberOfPlayers{num:1};
    let board_size = BoardSize { size: Vec2::new(4.0,5.0) };
    commands.insert_resource(num_of_players);
    commands.insert_resource(board_size);
    // match app_state.current()
    // {
    //     AppState::MainMenu =>
    //     {
    //         //app_state.pop();
    //         //app_state.set(AppState::InGame).unwrap();
    //     }
    //     AppState::InGame =>
    //     {
            
    //     }
    //     AppState::Restart =>
    //     {
            
    //     }
    // }
    // // ^ this can fail if we are already in the target state
    // // or if another state change is already queued
}


fn setup_players(mut commands: Commands,asset_server:Res<AssetServer>, player_num:Res<NumberOfPlayers>)
{
    let texture_handle=asset_server.load("icon1.png");
    let mut rng = rand::thread_rng();
    let player_array:[MemoryPlayer; 5] = 
    [
        MemoryPlayer{colour:Color::rgb(0.8,0.4,0.2),
            index:0,collected_cards:0},
        MemoryPlayer{colour:Color::rgb(0.4,0.8,0.2),
            index:1,collected_cards:0},
        MemoryPlayer{colour:Color::rgb(0.4,0.2,0.8),
            index:2,collected_cards:0},
        MemoryPlayer{colour:Color::rgb(0.6,0.6,0.2),
            index:3,collected_cards:0},
        MemoryPlayer{colour:Color::rgb(0.2,0.7,0.7),
            index:4,collected_cards:0},
    ];
    let mut player_vector : Vec<MemoryPlayer> =  player_array[0..player_num.num].to_vec();
    player_vector.shuffle(&mut rng);
    let player_radial_distance: f32 = 2.0*PI / (player_num.num as f32);
    let mut player_selection : CurrentPlayer = CurrentPlayer { order:Vec::with_capacity(player_num.num) } ;
    for player in player_vector.iter_mut()
    {
        let rot = player_radial_distance * player.index as f32;
        let pos = (rot.cos() * PLAYER_DISTANCE,rot.sin() * PLAYER_DISTANCE);
        let transform = Transform{translation:Vec3::new(pos.0, pos.1, 0.0),rotation:Quat::from_rotation_z(PI/2.0 +rot),scale:Vec3::new(1f32,1f32,1f32)};
        let entity = commands.spawn(
            (
                (*player).clone(),
                SpriteBundle
            {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0) * SPRITE_SIZE),
                    color: Color::rgb(player.colour.r(),player.colour.g(),player.colour.b()),
                    ..default()
                },
                texture: texture_handle.clone(),
                transform,
                ..default()
            }
        )).id();
        player_selection.order.push(entity);
    }
    
    commands.insert_resource(player_selection);
    commands.insert_resource(CurrentPlayerIndex{index:0,last_index:0,transition_time:0.0,transition_speed:0.4});
}

fn despawn_scene(mut commands: Commands,    
    mut player_query: Query<Entity, With<MemoryPlayer>>, 
    mut card_query: Query<Entity, With<Card>>,
    mut cam_query: Query<Entity, With<MainCamera>>,
    mut cur_query: Query<Entity, With<CursorState>>)
{
    for (ent) in player_query.iter()
    {
        commands.entity(ent).despawn_recursive();
    }
    for (ent) in card_query.iter()
    {
        commands.entity(ent).despawn_recursive();
    }
    for (ent) in cam_query.iter()
    {
        commands.entity(ent).despawn_recursive();
    }
    for (ent) in cur_query.iter()
    {
        commands.entity(ent).despawn_recursive();
    }
    commands.remove_resource::<CurrentPlayerIndex>();
    commands.remove_resource::<ClickedCardIndex>();
    commands.remove_resource::<CardToBeDestroyed>();
    commands.remove_resource::<CardCount>();
    //commands.remove_resource::<ImportedImagesFront>();
    //commands.remove_resource::<ImportedImageBack>();
    commands.remove_resource::<CurrentPlayer>();
}

fn setup_cards(mut commands: Commands, asset_server: Res<AssetServer>,board_size:Res<BoardSize>)
{
    let card_cols = board_size.size.x as usize;
    let card_rows = board_size.size.y as usize;
    let mut rng = rand::thread_rng();
    let texture_handle = asset_server.load("rewersu.png");
    let mut available_images:[Handle<Image>; 21] = 
        [
            asset_server.load("icon0.png"),
            asset_server.load("icon1.png"),
            asset_server.load("icon2.png"),
            asset_server.load("icon3.png"),
            asset_server.load("icon4.png"),
            asset_server.load("icon5.png"),
            asset_server.load("icon6.png"),
            asset_server.load("icon7.png"),
            asset_server.load("icon8.png"),
            asset_server.load("icon9.png"),
            asset_server.load("icon10.png"),
            asset_server.load("icon11.png"),
            asset_server.load("icon12.png"),
            asset_server.load("icon13.png"),
            asset_server.load("icon14.png"),
            asset_server.load("icon15.png"),
            asset_server.load("icon16.png"),
            asset_server.load("icon17.png"),
            asset_server.load("icon18.png"),
            asset_server.load("icon19.png"),
            asset_server.load("icon20.png"),
            //asset_server.load("icon21.png"),
            //asset_server.load("icon22.png"),
            //asset_server.load("icon23.png"),
            //asset_server.load("icon24.png"),
        ];
        available_images.shuffle(&mut rng);
    let used_images:Vec<Handle<Image>> = available_images[0..(card_rows*card_cols/2)].to_vec();
    let mut texture_handle_a:ImportedImagesFront = ImportedImagesFront{
        handles: used_images};
    let mut card_selection = CardSelection {
        order: Vec::with_capacity(card_rows*card_cols),
    };
    let clicked_card_index: ClickedCardIndex = ClickedCardIndex { index: card_rows*card_cols };
    let card_to_be_destroyed: CardToBeDestroyed = CardToBeDestroyed { index: card_rows*card_cols };
    let selected_cnt: CardCount = CardCount {count:0,};
    let mut indexes_array: Vec<usize> = (0..card_rows*card_cols).collect();
    //let mut indexes_array: [usize; card_rows*card_cols] = core::array::from_fn(|i| i);
    indexes_array.shuffle(&mut rng);
    let mut iter= 0;
    for i in 0..card_cols 
    {
        for j in 0..card_rows
        {
            let card_offset_x: f32 = -450.0 + 100.0 * (10.0 - (card_cols as f32)) * 0.5;
            let card_offset_y: f32 = -450.0 + 100.0 * (10.0 - (card_rows as f32)) * 0.5;
            let pos = (card_offset_x + 100.0 * (i as f32), card_offset_y + 100.0 * (j as f32));
            let transform = Transform::from_xyz(pos.0, pos.1, 1.0);
            let hue = 0.0;//rng.gen_range(0.0..=360.0);
            
            let entity = commands.spawn((
                Card{
                    hju: hue,
                    index: indexes_array[iter]/2},
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1.0, 1.0) * SPRITE_SIZE),
                        color: Color::hsla(hue, SATURATION_DESELECTED, LIGHTNESS_DESELECTED, ALPHA),
                        ..default()
                    },
                    texture: texture_handle.clone(),
                    transform,
                    ..default()
                },
                Hoverable,
                Clickable,
                RotateCard
                {
                    rotation:0.0,
                    duration:0.7,
                }
            )).id();
            card_selection.order.push(entity);
            iter += 1;
        }
    }
    card_selection.order.shuffle(&mut rng);
    commands.insert_resource(clicked_card_index);
    commands.insert_resource(card_to_be_destroyed);
    commands.insert_resource(selected_cnt);
    commands.insert_resource(ImportedImagesFront{handles:texture_handle_a.handles});
    commands.insert_resource(ImportedImageBack {
        handle: texture_handle,
    });
    
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>,) {
    let background_image: Handle<Image> = asset_server.load("bakgrund.png");
    let transform = Transform::from_xyz(0.0, 0.0, 0.0);

    commands.spawn((Camera2dBundle::default(),MainCamera));
    commands.spawn(CursorState{cursor_world:Vec2{x:0.0,y:0.0},cursor_moved:false});
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(10.0, 10.0) * SPRITE_SIZE),
            ..default()
        },
        texture: background_image.clone(),
        transform: transform,

        ..Default::default()
    });
}

fn destroy_cards_system(mut commands: Commands, 
    mut query: Query<(Entity,&mut Card, Option<&Clicked>, &mut Transform)>,
    mut destroy_card_index:ResMut<CardToBeDestroyed>,
    mut clicked_card_index:ResMut<ClickedCardIndex>,
    mut card_cnt: ResMut<CardCount>,
    mut player_index: ResMut<CurrentPlayerIndex>,
    mut player_query: Query<(&mut MemoryPlayer)>,
    mut app_state: ResMut<State<AppState>>,
    board_size: Res<BoardSize>,
    player_num:Res<NumberOfPlayers>
) {
    let board_size = board_size.size.x as usize * board_size.size.y as usize;
    for (entity,card,_,mut trans) in query.iter_mut()
    {
        if card.index == destroy_card_index.index
        {
            //hide_card(&mut trans);
            card_cnt.count = 0;
            //commands.entity(entity).despawn();
            commands.entity(entity).remove::<Clicked>();
            commands.entity(entity).remove::<Clickable>();
            commands.entity(entity).remove::<Hoverable>();
            commands.entity(entity).insert(Collected);
            let player_radial_distance: f32 = 2.0*PI / (player_num.num as f32);
            for (mut player) in player_query.iter_mut()
            {
                if player_index.index == player.index
                {
                    let mut helper_rot = player_radial_distance * player.index as f32;
                    let mut target_pos = Vec3::new(helper_rot.cos() * (PLAYER_DISTANCE),helper_rot.sin() * PLAYER_DISTANCE,player.collected_cards as f32);
                    let rotation_offset = -(player.collected_cards as f32) * 0.18 + 1.0;
                    player.collected_cards += 1;
                    let target_rot = helper_rot + rotation_offset;
                    helper_rot += PI + rotation_offset;
                    let card_offset = Vec3::new(helper_rot.cos() * PLAYER__HAND_DISTANCE,helper_rot.sin() * PLAYER__HAND_DISTANCE,player.collected_cards as f32);
                    target_pos+=card_offset;
                    //target_trans.rotate(Quat::from_rotation_z(2.0));
                    commands.entity(entity).insert(MovedCard{target_transl: target_pos,target_rot:target_rot + PI/2.0,
                        init_transform:trans.clone(),speed:1.0,progression:0.0});
                }
            }
            
            
            //commands.entity(entity).remove::<Card>();
            clicked_card_index.index = board_size;
        }
    }
    destroy_card_index.index = board_size;

    if card_cnt.count > 2
    {
        for (entity,_,clicked,_) in query.iter()
        {
            if clicked.is_some()
            {
                commands.entity(entity).remove::<Clicked>();
            }
        
        }
        switch_player(&mut player_index,player_num.num);
        card_cnt.count = 0;
    }
}

fn check_board_state(query_collected: Query<&Collected>,
    board_size : Res<BoardSize>,
    mut app_state: ResMut<State<AppState>>,
    buttons: Res<Input<MouseButton>>)
{
    if buttons.just_pressed(MouseButton::Left)
    {
        let mut board_size = board_size.size.x as usize*board_size.size.y as usize;
        for collected in query_collected.iter()
        {
            board_size -= 1;
        }
        if board_size == 0
        {   
            app_state.set(AppState::MainMenu).unwrap();
        }
    }
}

fn move_card_system(mut commands: Commands, 
    mut query: Query<(Entity,&mut MovedCard, &mut Transform,)>,
    timer: Res<Time>) {
        let mut delta = timer.delta_seconds();
        for (entity,mut moved_card,mut trans) in query.iter_mut()
        {
            moved_card.progression += delta;
            let mut delta = moved_card.progression/moved_card.speed;
            if delta < 1.0
            {
                
            }
            else 
            {
                delta = 1.0;
                commands.entity(entity).remove::<MovedCard>();
            }
            let target_pos = moved_card.target_transl * smoothstep(delta);
            
            let transl = (1.0 - smoothstep(delta)) * moved_card.init_transform.translation + target_pos;
            let rot = Quat::from_rotation_z(moved_card.target_rot * smoothstep(delta) );

            trans.translation = transl;
            trans.rotation = rot;
            //trans.rotation = rot;
            
        }
}

fn smoothstep(x:f32) -> f32
{
    if x<0f32
    {
        return 0f32;
    }
    if x>1f32
    {
        return 1f32;
    }
    return 3f32*x*x- 2f32*x*x*x;

}
fn select_card_system(mut query: Query<(&Card, &mut Sprite,Option<&Hovered>,Option<&Clicked>)>,
)
{
    for(card, mut sprite, hovered, clicked) in query.iter_mut()
    {
        if hovered.is_some() || clicked.is_some()
        {
            select_card(&mut sprite, card);
        }
        else
        {
            deselect_card(&mut sprite, card);
        }
    }
}

fn select_player_system(mut query: Query<&mut Camera2d, (With<MainCamera>)>,
                        mut player_query:Query<(Entity, &MemoryPlayer)>,
                        mut player_index: ResMut<CurrentPlayerIndex>,
                        time: Res<Time>,
                        mut current_player: ResMut<CurrentPlayer>,
)
{
    let delta = time.delta_seconds();
    player_index.transition_time += delta;
    let mut delta = player_index.transition_time/player_index.transition_speed;
    let mut colour = Color::rgb(0f32,0f32,0f32);

    if delta<1.0
    {
        for (_,player) in player_query.iter()
        {
            if player.index == player_index.last_index
            {
                colour = Color::rgb((player.colour.r() * (1.0-delta)),(player.colour.g() * (1.0-delta)),(player.colour.b() * (1.0-delta))) ;
            }
        }
    }
    else
    {
        delta = 1.0;
    }

    for(mut camera) in query.iter_mut()
    {
        for (entity,player) in player_query.iter()
        {
            if player.index == player_index.index
            {

                camera.clear_color = ClearColorConfig::Custom(Color::rgb(
                    player.colour.r() * delta +colour.r(),
                    player.colour.g() * delta +colour.g(),
                    player.colour.b() * delta +colour.b(),
                ));
            }
        }
        
    }
    for(mut camera) in query.iter_mut()
    {
    //camera.clear_color = ClearColorConfig::Custom(Color::rgb(0.1,0.8,0.1));
}
}

// fn click_card_system(    mut commands: Commands,
// mut query: Query<(Entity, &Card, &mut Sprite,Option<&Clicked>)>,
// mut card_cnt: ResMut<CardCount>,
// )
// {
//     for(entity, card, mut sprite, clicked) in query.iter_mut()
//     {
//         if clicked.is_some()
//         {
//             card_cnt.count = card_cnt.count + 1;
//             if card_cnt.count > 2
//             {
//                 card_cnt.count = card_cnt.count - 1;
//                 commands.entity(entity).remove::<Clicked>();
//                 deselect_card(&mut sprite, card);
//             }
//         }
//     }
// }

fn flip_card_system(
    mut query: Query<(Entity, Option<&Clicked>, &mut RotateCard, &mut Sprite, &mut Card, &mut Handle<Image>, Option<&Collected>)>,
    front_images : Res<ImportedImagesFront>,
    back_image : Res<ImportedImageBack>,
    time: Res<Time>
    )
{
    for (_, clicked, mut rotate, mut sprite, card, mut handle,collected) in query.iter_mut()
    {
        let delta = time.delta_seconds();
        if clicked.is_some() || collected.is_some()
        {
            if rotate.rotation < 1.0
            {
                rotate.rotation = rotate.rotation + delta / rotate.duration;
            }
        }
        else
        {
            if rotate.rotation > 0.0
            {
                rotate.rotation = rotate.rotation - delta / rotate.duration;
            }
        }
        
        let transformed_rotation = (PI * rotate.rotation).cos();
        if transformed_rotation > 0.0
        {
            sprite.custom_size= Some(Vec2::new( transformed_rotation, 1.0)*SPRITE_SIZE);
            *handle = back_image.handle.clone();

        }
        else
        {
            sprite.custom_size= Some(Vec2::new( -transformed_rotation, 1.0)*SPRITE_SIZE); 
            *handle = front_images.handles[card.index].clone();
        }
    }
}

fn cursor_state_system(mut q_cursor_state: Query<&mut CursorState>,
    mut cursor_evr: EventReader<CursorMoved>,
    windows: Res<Windows>,
    q_camera: Query<&Transform, With<MainCamera>>
) 
{
    let window = windows.get_primary().unwrap();
    let cam_transform = q_camera.iter().last().unwrap();
    let mut world_pos =Vec2 {x:0.0,y:0.0};
    let mut cursor_moved = false;

    for cursor_state in q_cursor_state.iter_mut() 
    {
        world_pos = cursor_state.cursor_world;
    }

    for ev in cursor_evr.iter() {
        cursor_moved = true;
        world_pos = cursor_to_world(window, cam_transform, ev.position);

    }

    for mut cursor_state in q_cursor_state.iter_mut() 
    {
        cursor_state.cursor_world = world_pos;
        cursor_state.cursor_moved = cursor_moved;
    }
}

fn hoverable(
    mut commands: Commands,
    q_cursor_state: Query<& CursorState>,
    q_hoverable: Query<(Entity, &Transform, &Sprite, &Hoverable)>,
    destroy_card_index:Res<CardToBeDestroyed>,
    board_size:Res<BoardSize>
) 
{
    
    for cursor_state in q_cursor_state.iter() {

        for (entity, transform, sprite, _) in q_hoverable.iter() {
            let wek:Vec2 = sprite.custom_size.unwrap();
            let half_width = wek.x * 0.5;
            let half_height = wek.y * 0.5;

            if transform.translation.x - half_width < cursor_state.cursor_world.x
                && transform.translation.x + half_width > cursor_state.cursor_world.x
                && transform.translation.y - half_height < cursor_state.cursor_world.y
                && transform.translation.y + half_height > cursor_state.cursor_world.y
                && destroy_card_index.index == (board_size.size.x as usize * board_size.size.y as usize)
            {
                commands.entity(entity).insert(Hovered);
            } else {
                commands.entity(entity).remove::<Hovered>();
            }
        }
    }
}

fn clickable_card(
    mut commands: Commands,
    q_hoverable: Query<(Entity, &Clickable,Option<&Clicked>,&Card,&Transform, &Sprite)>,
    buttons: Res<Input<MouseButton>>,
    mut card_cnt: ResMut<CardCount>,
    mut clicked_card_index: ResMut<ClickedCardIndex>,
    mut cards_to_be_destroyed: ResMut<CardToBeDestroyed>,
    q_cursor_state: Query<& CursorState>,
) {
    if buttons.just_pressed(MouseButton::Left)
    {
        for cursor_state in q_cursor_state.iter() {
            // Left button was pressed
            if 2 == card_cnt.count
            {
                card_cnt.count = card_cnt.count + 1;
            }
            else
            {
                for (entity, _, clicked,card,transform,sprite) in q_hoverable.iter() 
                {
                    let wek:Vec2 = sprite.custom_size.unwrap();
                    let half_width = wek.x * 0.5;
                    let half_height = wek.y * 0.5;
                    if transform.translation.x - half_width < cursor_state.cursor_world.x
                    && transform.translation.x + half_width > cursor_state.cursor_world.x
                    && transform.translation.y - half_height < cursor_state.cursor_world.y
                    && transform.translation.y + half_height > cursor_state.cursor_world.y
                    {
                        if clicked.is_some()
                        {
                            // card_cnt.count = card_cnt.count - 1;
                            // commands.entity(entity).remove::<Clicked>();
                        }
                        else 
                        {
                            if 0 == card_cnt.count 
                            {
                                clicked_card_index.index = card.index;
                            }

                            if card_cnt.count < 2
                            {
                                card_cnt.count = card_cnt.count + 1;
                                commands.entity(entity).insert(Clicked);
                            }
                            else 
                            {
                                card_cnt.count = card_cnt.count + 1;
                            }

                            if 2 == card_cnt.count
                            {
                                if card.index == clicked_card_index.index
                                {
                                    cards_to_be_destroyed.index = card.index;
                                }
                            }
                        }
                        
                    }
                }
            }
        }
        
    }
}

fn select_card(sprite: &mut Sprite, card: &Card)
{
    sprite.color = Color::hsla(
        card.hju,
        SATURATION_SELECTED,
        LIGHTNESS_SELECTED,
        ALPHA,
    );
}

fn deselect_card(sprite: &mut Sprite, card: &Card)
{
    sprite.color = Color::hsla(
        card.hju,
        SATURATION_DESELECTED,
        LIGHTNESS_DESELECTED,
        ALPHA,
    );
}

fn hide_card(sprite: &mut Transform)
{
    sprite.translation.x += 1500.0;
}

fn cursor_to_world(window: &Window, cam_transform: &Transform, cursor_pos: Vec2) -> Vec2 {
    // get the size of the window
    let size = Vec2::new(window.width() as f32, window.height() as f32);

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let screen_pos = cursor_pos - size / 2.0;

    // apply the camera transform
    let out = cam_transform.compute_matrix() * screen_pos.extend(0.0).extend(1.0);
    Vec2::new(out.x, out.y)
}

fn switch_player(player_index:&mut CurrentPlayerIndex, player_num:usize)
{
    (*player_index).last_index = (*player_index).index;  
    (*player_index).index += 1;
    if (*player_index).index >= player_num
    {
        (*player_index).index = 0;
    } 
    (*player_index).transition_time = 0.0;
}

fn clickable_button(
    mut commands: Commands,
    q_hoverable: Query<(Entity, &Clickable,Option<&Clicked> ,&Transform, &Sprite)>,
    buttons: Res<Input<MouseButton>>,
    q_cursor_state: Query<& CursorState>,
) {
    if buttons.just_pressed(MouseButton::Left)
    {
        for cursor_state in q_cursor_state.iter() {

            println!(
            "Clicked!"
            );
            // Left button was pressed
            for (entity, _, clicked,transform,sprite) in q_hoverable.iter() 
            {
                let wek:Vec2 = sprite.custom_size.unwrap();
                let half_width = wek.x * 0.5;
                let half_height = wek.y * 0.5;
                if transform.translation.x - half_width < cursor_state.cursor_world.x
                && transform.translation.x + half_width > cursor_state.cursor_world.x
                && transform.translation.y - half_height < cursor_state.cursor_world.y
                && transform.translation.y + half_height > cursor_state.cursor_world.y
                {
                    
                }
            }
        }
    }
}