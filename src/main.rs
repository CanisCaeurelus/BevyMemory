//! This example displays each contributor to the bevy source code as a bouncing bevy-ball.

use bevy::{prelude::*};
use rand::{prelude::SliceRandom};

use {core::f32::consts::PI};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(setup_cards)
        //.add_system(velocity_system)
        //.add_system(move_system)
        .add_system(select_card_system)
        .add_system(cursor_state_system)
        .add_system(clickable)
        //.add_system(click_card_system)
        .add_system(flip_card_system)
        .add_system(destroy_cards_system)
        //.add_system(display_cards)
        .add_system(hoverable)
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
struct CursorState {
    cursor_world: Vec2,
    cursor_moved: bool,
}

#[derive(Component)]
struct Rewers;
#[derive(Component)]
struct Awers;
const SPRITE_SIZE: f32 = 95.0;

const SATURATION_DESELECTED: f32 = 0.0;
const LIGHTNESS_DESELECTED: f32 = 0.6;
const SATURATION_SELECTED: f32 = 0.0;
const LIGHTNESS_SELECTED: f32 = 0.7;
const ALPHA: f32 = 0.92;

const SHOWCASE_TIMER_SECS: f32 = 3.0;

const CARD_ROWS: usize = 3;
const CARD_COLS: usize = 4;

const CARD_OFFSET_X: f32 = -450.0 + 100.0 * (10.0 - (CARD_COLS as f32)) * 0.5;
const CARD_OFFSET_Y: f32 = -450.0 + 100.0 * (10.0 - (CARD_ROWS as f32)) * 0.5;

#[derive(Resource)]
struct ImportedImagesFront {
    handles: Vec<Handle<Image>>,
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

fn setup_cards(mut commands: Commands, asset_server: Res<AssetServer>)
{
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
    let used_images:Vec<Handle<Image>> = available_images[0..(CARD_ROWS*CARD_COLS/2)].to_vec();
    let mut texture_handle_a:ImportedImagesFront = ImportedImagesFront{
        handles: used_images};
    let mut card_selection = CardSelection {
        order: Vec::with_capacity(CARD_ROWS*CARD_COLS),
    };
    let clicked_card_index: ClickedCardIndex = ClickedCardIndex { index: CARD_ROWS*CARD_COLS };
    let card_to_be_destroyed: CardToBeDestroyed = CardToBeDestroyed { index: CARD_ROWS*CARD_COLS };
    let selected_cnt: CardCount = CardCount {count:0,};
    let mut indexes_array: [usize; CARD_ROWS*CARD_COLS] = core::array::from_fn(|i| i);
    indexes_array.shuffle(&mut rng);
    let mut iter= 0;
    for i in 0..CARD_ROWS 
    {
        for j in 0..CARD_COLS
        {
            
            let pos = (CARD_OFFSET_X + 100.0 * (i as f32), CARD_OFFSET_Y + 100.0 * (j as f32));
            let transform = Transform::from_xyz(pos.0, pos.1, 0.0);
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(CursorState{cursor_world:Vec2{x:0.0,y:0.0},cursor_moved:false});
}

fn destroy_cards_system(mut commands: Commands, 
    mut query: Query<(Entity,&mut Card, Option<&Clicked>, &mut Transform,)>,
    mut destroy_card_index:ResMut<CardToBeDestroyed>,
    mut clicked_card_index:ResMut<ClickedCardIndex>,
    mut card_cnt: ResMut<CardCount>,) {
    for (entity,card,_,mut trans) in query.iter_mut()
    {
        if card.index == destroy_card_index.index
        {
            hide_card(&mut trans);
            card_cnt.count = 0;
            commands.entity(entity).despawn();
            clicked_card_index.index = CARD_ROWS*CARD_COLS;
        }
    }
    destroy_card_index.index = CARD_ROWS*CARD_COLS;

    if card_cnt.count > 2
    {
        for (entity,_,clicked,_) in query.iter()
        {
            if clicked.is_some()
            {
                commands.entity(entity).remove::<Clicked>();
            }
        
        }
        card_cnt.count = 0;
    }

    
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
    mut query: Query<(Entity, Option<&Clicked>, &mut RotateCard, &mut Sprite, &mut Card, &mut Handle<Image>)>,
    front_images : Res<ImportedImagesFront>,
    back_image : Res<ImportedImageBack>,
    time: Res<Time>
    )
{
    for (_, clicked, mut rotate, mut sprite, card, mut handle) in query.iter_mut()
    {
        let delta = time.delta_seconds();
        if clicked.is_some() 
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
    q_camera: Query<&Transform, With<Camera>>
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
                && destroy_card_index.index == CARD_ROWS * CARD_COLS
            {
                commands.entity(entity).insert(Hovered);
            } else {
                commands.entity(entity).remove::<Hovered>();
            }
        }
    }
}

fn clickable(
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

            println!(
            "Clicked!"
            );
            // Left button was pressed
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
