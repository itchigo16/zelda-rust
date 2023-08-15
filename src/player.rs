use bevy::prelude::*;
use crate::collisions;
use crate::constants::*;
use crate::collisions::*;
use crate::ennemies::Ennemy;
use crate::structures;
use crate::structures::*;
use crate::setup::*;

#[derive(PartialEq)]
pub enum PlayerFacingDirection {
    Left,
    TopLeft,
    Up,
    TopRight,
    Right,
    BottomRight,
    Down,
    BottomLeft,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_move, 
                                                    update_player_pos, 
                                                    player_facing_direction, 
                                                    update_player_sprite_moving,
                                                    tower_detection,
                                                    sanctuary_detection,
                                                    tree_transparency,
                                                    update_hitbox_pos,
                                                    update_hitbox_visibility));
    }
}


#[derive(Component)]
pub struct Player {
    x: f32,
    y: f32,
    x: f32,
    y: f32,
    facing_direction: PlayerFacingDirection,
    sprinting: bool,
    health: i32,
    health: i32,
}

impl Player {
    pub fn new() -> Self {
        Self { x: 0., y: 0., facing_direction: PlayerFacingDirection::Right, sprinting:false, health: 20 }
    }

    fn is_facing_down(&self) -> bool {
        self.facing_direction == PlayerFacingDirection::Down || self.facing_direction == PlayerFacingDirection::BottomLeft || self.facing_direction == PlayerFacingDirection::BottomRight
    }

    fn is_facing_up(&self) -> bool {
        self.facing_direction == PlayerFacingDirection::Up || self.facing_direction == PlayerFacingDirection::TopLeft || self.facing_direction == PlayerFacingDirection::TopRight
    }

    fn is_facing_left(&self) -> bool {
        self.facing_direction == PlayerFacingDirection::Left || self.facing_direction == PlayerFacingDirection::TopLeft || self.facing_direction == PlayerFacingDirection::BottomLeft
    }

    fn is_facing_right(&self) -> bool {
        self.facing_direction == PlayerFacingDirection::Right || self.facing_direction == PlayerFacingDirection::TopRight || self.facing_direction == PlayerFacingDirection::BottomRight
    }
}

impl Collisionable for Player {
    fn get_pos(&self) -> (f32, f32) {
    fn get_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, PLAYER_HITBOX_WIDTH * 0.8, PLAYER_HITBOX_HEIGHT*0.8)
    fn get_hitbox(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, PLAYER_HITBOX_WIDTH * 0.8, PLAYER_HITBOX_HEIGHT*0.8)
    }
    
}


fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
    collisionable_query: Query<&CollisionComponent, Without<Ennemy>>,
    mut ennemy_query: Query<(&CollisionComponent, &mut Ennemy), With<Ennemy>>
) {

    let player_speed: f32;
    let player_speed: f32;
    let mut player = player_query.single_mut();

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        player.sprinting = true;
        player_speed = PLAYER_SPRINT_SPEED;
        player_speed = PLAYER_SPRINT_SPEED;

    } else {
        player.sprinting = false;
        player_speed = PLAYER_NORMAL_SPEED;
        player_speed = PLAYER_NORMAL_SPEED;
    }


    let left_boundary = -((MAP_SIZE / 2.0) - (PLAYER_HITBOX_WIDTH / 2.));
    let left_boundary = -((MAP_SIZE / 2.0) - (PLAYER_HITBOX_WIDTH / 2.));
    let right_boundary = -left_boundary;
    let top_boundary = right_boundary;
    let bottom_boundary = left_boundary;

    let actual_x = player.x;
    let actual_y = player.y;

    let mut new_x = if keyboard_input.pressed(KeyCode::Left) { player.x - player_speed }
                     else if keyboard_input.pressed(KeyCode::Right) { player.x + player_speed }
                     else { player.x };

    for collidable in collisionable_query.iter() {
        if player.would_collide(new_x, player.y, collidable){ 
            new_x = actual_x;
        }
    }

    for (collidable, mut ennemy) in ennemy_query.iter_mut() {
        if player.would_collide(new_x, player.y, &collidable) {
            if player.is_facing_right() {
                if !ennemy.move_in_direction(&PlayerFacingDirection::Right, player_speed, &collisionable_query) {
                    new_x = actual_x;
                }
            } else if player.is_facing_left() {
                if !ennemy.move_in_direction(&PlayerFacingDirection::Left, player_speed, &collisionable_query) {
                    new_x = actual_x;
                }
            } else {
                new_x = actual_x;
            }
        }
    }

    if new_x > right_boundary {
        new_x = actual_x;
    } else if new_x < left_boundary {
        new_x = actual_x;
    }

    let mut new_y = if keyboard_input.pressed(KeyCode::Down) { player.y - player_speed }
                     else if keyboard_input.pressed(KeyCode::Up) { player.y + player_speed }
                     else { player.y };

    for collidable in collisionable_query.iter() {
        if player.would_collide(player.x, new_y, collidable){ 
            new_y = actual_y;
        }
    }

    for (collidable, mut ennemy) in ennemy_query.iter_mut() {
        if player.would_collide(player.x, new_y, &collidable) {
            if player.is_facing_up() {
                if !ennemy.move_in_direction(&PlayerFacingDirection::Up, player_speed, &collisionable_query) {
                    new_y = actual_y;
                }
            } else if player.is_facing_down() {
                if !ennemy.move_in_direction(&PlayerFacingDirection::Down, player_speed, &collisionable_query) {
                    new_y = actual_y;
                }
            } else {
                new_y = actual_y;
            }
        }
    }

    if new_y > top_boundary {
        new_y = actual_y;
    } else if new_y < bottom_boundary {
        new_y = actual_y;
    }

    player.x = new_x;
    player.y = new_y;
}
    

fn player_facing_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Player>,
) {
    let mut player = query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) && keyboard_input.pressed(KeyCode::Up) {
        player.facing_direction = PlayerFacingDirection::TopLeft;
    }
    else if keyboard_input.pressed(KeyCode::Right) && keyboard_input.pressed(KeyCode::Up) {
        player.facing_direction = PlayerFacingDirection::TopRight;
    }
    else if keyboard_input.pressed(KeyCode::Left) && keyboard_input.pressed(KeyCode::Down) {
        player.facing_direction = PlayerFacingDirection::BottomLeft;
    }
    else if keyboard_input.pressed(KeyCode::Right) && keyboard_input.pressed(KeyCode::Down) {
        player.facing_direction = PlayerFacingDirection::BottomRight;
    }
    else if keyboard_input.pressed(KeyCode::Left) {
        player.facing_direction = PlayerFacingDirection::Left;
    }
    else if keyboard_input.pressed(KeyCode::Right) {
        player.facing_direction = PlayerFacingDirection::Right;
    }
    else if keyboard_input.pressed(KeyCode::Up) {
        player.facing_direction = PlayerFacingDirection::Up;
    }
    else if keyboard_input.pressed(KeyCode::Down) {
        player.facing_direction = PlayerFacingDirection::Down;
    }
}
   


fn update_player_sprite_moving(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut TextureAtlasSprite)>,
) {
    let mut player = query.single_mut();
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::Down) {
        if player.0.sprinting {
            player.1.index = 24;
        }
        else {
            player.1.index = 26;
        }
    }
    else {
        player.1.index = 0;
    }
}

fn update_player_pos(
    mut query: Query<(&mut Player, &mut Transform)>,
) {

    let (player, mut sprite) = query.single_mut();
    let x = player.x as f32;
    let y = player.y as f32;
    sprite.translation.x = x;
    sprite.translation.y = y;
    match player.facing_direction {
        PlayerFacingDirection::Left => sprite.scale.x = -PLAYER_SPRITE_SCALE,
        PlayerFacingDirection::Right => sprite.scale.x = PLAYER_SPRITE_SCALE,
        _ => {}
    }
}

#[derive(Component)]
pub struct HitBox;

fn spawn_player(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,) 
    {

    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(PLAYER_SPRITE_SIZE, PLAYER_SPRITE_SIZE), 8, 9, Some(Vec2::new(0., 0.)), Some(Vec2::new(0., 0.)));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player: Player = Player::new();
    let hitbox = player.get_hitbox();

    commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                scale: Vec3::new(PLAYER_SPRITE_SCALE, PLAYER_SPRITE_SCALE, Z_LAYER_PLAYER),
                ..Transform::default()
            },
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        })
        .insert(player);

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0., 0., Z_LAYER_GUI),
            ..Transform::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(hitbox.2, hitbox.3)),
            color: Color::rgb(0.0, 0.0, 1.0),
            ..Default::default()
        },
        visibility: Visibility::Hidden,
        ..Default::default()
    }).insert(HitBox);
}
enum InteractionType {
    Tower,
    Sanctuary,
}
fn can_interact_with(
    player: &Player,
    interaction_type: &InteractionType,
    x: f32,
    y: f32,
    x: f32,
    y: f32,
    tower: Option<&Tower>,
    sanctuary: Option<&Sanctuary>,
) -> bool {
    match interaction_type {
        InteractionType::Tower => {
            if let Some(t) = tower {
                player.would_collide(x, y, &CollisionComponent::new_from_component(t))
            } else {
                false
            }
        }
        InteractionType::Sanctuary => {
            if let Some(s) = sanctuary {
                player.would_collide(x, y, &CollisionComponent::new_from_component(s))
            } else {
                false
            }
        }
    }
}

fn tower_detection(
    mut player_query: Query<&mut Player>,
    tower_query: Query<&Tower>,
    keyboard_input: Res<Input<KeyCode>>,
    query_sanctuary: Query<&mut Sanctuary>,
) {
    let player = player_query.single_mut();
    for tower in tower_query.iter() {
        if can_interact_with(&player, &InteractionType::Tower, player.x, player.y + 1., Some(tower), None) {
        if can_interact_with(&player, &InteractionType::Tower, player.x, player.y + 1., Some(tower), None) {
            if keyboard_input.just_pressed(KeyCode::Space) {
                structures::show_one_sanctuary(query_sanctuary);
                break;
            }
        }
    }
}

fn sanctuary_detection(
    mut player_query: Query<&mut Player>,
    mut sanctuary_query: Query<&mut Sanctuary>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let player = player_query.single_mut();
    for mut sanctuary in sanctuary_query.iter_mut() {
        if can_interact_with(&player, &InteractionType::Sanctuary, player.x, player.y + 1., None, Some(&sanctuary)) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x, player.y - 1., None, Some(&sanctuary)) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x + 1., player.y, None, Some(&sanctuary)) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x - 1., player.y, None, Some(&sanctuary)) {
        if can_interact_with(&player, &InteractionType::Sanctuary, player.x, player.y + 1., None, Some(&sanctuary)) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x, player.y - 1., None, Some(&sanctuary)) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x + 1., player.y, None, Some(&sanctuary)) ||
            can_interact_with(&player, &InteractionType::Sanctuary, player.x - 1., player.y, None, Some(&sanctuary)) {
            if keyboard_input.just_pressed(KeyCode::Space) {
                sanctuary.unlock();
                break;
            }
        }
    }
}

fn tree_transparency(
    player_query: Query<&Player>,
    mut tree_query: Query<(&mut TextureAtlasSprite, &Transform), With<Tree>>,
) {
    let player = player_query.single();
    for (mut sprite, transform) in tree_query.iter_mut() {
        if collisions::are_overlapping(player.x, player.y, PLAYER_HITBOX_WIDTH, PLAYER_HITBOX_HEIGHT, transform.translation.x, transform.translation.y, TREE_WIDTH*0.6, TREE_HEIGHT*0.6) {
            sprite.color.set_a(0.50);
        } else {
            sprite.color.set_a(1.0);
        }
    }
}

fn update_hitbox_pos(
    player_query: Query<&Player>,
    mut hitbox_query: Query<&mut Transform, With<HitBox>>,
) {
    let player = player_query.single();
    for mut transform in hitbox_query.iter_mut() {
        transform.translation.x = player.x;
        transform.translation.y = player.y;
    }
}

fn update_hitbox_visibility(
    keyboard_input: Res<Input<KeyCode>>,
    mut hitbox_query: Query<&mut Visibility, With<HitBox>>,
) {
    for mut visibility in hitbox_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::L) {
            *visibility = Visibility::Visible;
        }
        if keyboard_input.just_pressed(KeyCode::K) {
            *visibility = Visibility::Hidden;
        }
    }
}
