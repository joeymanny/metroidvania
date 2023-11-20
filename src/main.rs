use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update,(
            handle_input,
            update_player_rotation
        ))
        .init_resource::<PlayerData>()
        .run();
}
#[derive(Resource)]
struct PlayerData{
    speed: f32,
}

impl Default for PlayerData{
    fn default() -> Self {
        Self {
            speed: 100.0
        }
    }
}


#[derive(Bundle, Default)]
struct PlayerBundle{
    player_data: PlayerMarker,
    sprite_sheet: SpriteSheetBundle,
}

#[derive(Component, Default)]
struct PlayerMarker;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    commands.spawn(Camera2dBundle::default());
    let texture_atlas = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_atlas, Vec2::new(48.0, 48.0), 6, 10, None, None);
    let texture_atlas = texture_atlases.add(texture_atlas);
    commands.spawn(
        PlayerBundle{
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 12,
                    custom_size: Some(Vec2::splat(300.0)),
                    ..default()
                },
            texture_atlas: texture_atlas,
            ..default()
            },
            ..default()
        }
    );
}

fn handle_input(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<PlayerMarker>>,
    time: Res<Time>,
    player: Res<PlayerData>
){
    let mut transform = query.single_mut();
    let mut delta = Vec2::ZERO;
    if keyboard.pressed(KeyCode::W){
        delta.y += player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S){
        delta.y -= player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A){
        delta.x -= player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D){
        delta.x += player.speed * time.delta_seconds();
    }
    if let Some(delta) = delta.try_normalize(){
        transform.translation += delta.extend(0.0);
    }
}

fn update_player_rotation(
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut q_player_transform: Query<&mut Transform, With<PlayerMarker>>,
){
    let window = q_windows.single();
    if let Some(mouse) = window.cursor_position(){
        let mut player_transform = q_player_transform.single_mut();
        let target = Vec2{
            x: mouse.x - (window.width() / 2.0) - player_transform.translation.x,
            y: (window.height() / 2.0) - mouse.y - player_transform.translation.y,
        }.extend(0.0);

        // if (player_translation.translation - target).length() > 50.0{
            *player_transform = player_transform.looking_at(Vec3{ z: -1.0, ..player_transform.translation }, target);
        // }
    }
}




// {
//     let (mut player, mut sprite_sheet) = query.single_mut();
//     if let Some(_previous_index) = player.previous_animation_index {
//         // player.animation_timer.tick(time.delta());
//         // if player.animation_timer.just_finished(){
//         //     {
//         //         if let 0..=35 | 48.. = sprite_sheet.index{
//         //             sprite_sheet.index =
//         //                 if let 18..=23 = sprite_sheet.index{ // facing us
//         //                     36
//         //                 }else if let 24..=29 = sprite_sheet.index{// left/right
//         //                     40
//         //                 }else{// 'up'
//         //                     44
//         //                 };
//         //         }
//         //         match sprite_sheet.index{
//         //             36..=39 => {
//         //                 sprite_sheet.index += 1;
//         //                 if sprite_sheet.index > 39{
//         //                     player.previous_animation_index = None;
//         //                 }
//         //             },
//         //             40..=43 => {
//         //                 sprite_sheet += 1;
//         //                 if sprite_sheet.index > 43{
//         //                     player.
//         //                 }
//         //             }
//                
//         //             _ =>()
//         //         }
//         //     }
//         // }
//     }else{
//         player.animation_timer.tick(time.delta());
//         if player.animation_timer.just_finished(){
//             match player.moving{
//                 Some(dir)=>{
//                         match dir{
//                             Vec2{ x, y:_ } if x.abs() > 0.0 => {
//                                 sprite_sheet.index = ((sprite_sheet.index + 1) % 6) + 24;
//                                 sprite_sheet.flip_x = if dir.x.is_sign_positive(){ false }else{ true };
//                             },
//
//                             Vec2{ x:_, y } if y > 0.0 => {
//                                 sprite_sheet.index = ((sprite_sheet.index + 1) % 6) + 30;
//
//                             },
//                             Vec2{ x:_, y } if y < 0.0 => {
//                                 sprite_sheet.index = ((sprite_sheet.index + 1) % 6) + 18;
//                             }
//                             _=>()
//                         } // moving at all
//                 },
//                 None => {
//                     match sprite_sheet.index{
//                         0..=5 | 18..=23 =>{ // facing us
//                             sprite_sheet.index = (sprite_sheet.index + 1) % 6;
//                             // dbg!(sprite_sheet.index);
//                         },
//                         6..=11 | 24..=29 =>{ // right/left
//                             sprite_sheet.index = ((sprite_sheet.index + 1) % 6) + 6;
//                         },
//                         30..=35 | 12..=17 =>{ // looking 'up'
//                             sprite_sheet.index = ((sprite_sheet.index + 1) % 6) + 12;
//                         }
//                         _ => ()
//                     }
//                 }
//             }
//         }
//     }
//
// }