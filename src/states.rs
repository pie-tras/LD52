use std::fs;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

const TIME_STEP: f32 = 1.0 / 60.0;

const DEEP_DIVE_TILE_SCALE: f32 = 16.0;

pub struct StatesPlugin;

enum State {
    Intro,
    TechShop,
    DeepDive,
}

struct IntroState {
    current_text: String,
    story_texts: Vec<String>,
    current_story_line: usize,
}
struct TechShopState {
    test: u32
}

struct DeepDiveState {
    level: u32
}

impl IntroState {
    fn new() -> Self {
        IntroState {
            current_text: String::from(""),
            story_texts: Vec::new(),
            current_story_line: 0,
        }
    }
}

impl TechShopState {
    fn new() -> Self {
        TechShopState {
            test: 0
        }
    }
}

impl DeepDiveState {
    fn new() -> Self {
        DeepDiveState {
            level: 0
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct StoryText;

#[derive(Component)]
struct Player {
    velocity: Vec2
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Lava;

#[derive(Component)]
struct DataPort;

#[derive(Component)]
struct Portal;

#[derive(Resource)]
struct CurrentState(State);

#[derive(Resource)]
struct NextState(State);

#[derive(Resource)]
struct IntroData(IntroState);

#[derive(Resource)]
struct TechShopData(TechShopState);

#[derive(Resource)]
struct DeepDiveData(DeepDiveState);

#[derive(Resource)]
struct DeepDiveDataBank(u32);

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentState(State::TechShop))
        .insert_resource(NextState(State::Intro))
        .insert_resource(IntroData(IntroState::new()))
        .insert_resource(TechShopData(TechShopState::new()))
        .insert_resource(DeepDiveData(DeepDiveState::new()))
        .insert_resource(DeepDiveDataBank(0))
        .add_startup_system(start_initial_state)
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            .with_system(run_current_game_state)
            .with_system(animate_sprite)
            .with_system(manage_state_changes.before(run_current_game_state))
        );
    }
}

impl IntroState {
    fn start(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>
    ) {
        self.current_story_line = 0;

        println!("Start intro");

        let raw_text = fs::read_to_string("assets/texts/intro.txt")
            .expect("Cannot open texts/intro.txt");
        let lines = raw_text.split('\n');
        for l in lines {
            self.story_texts.push(l.to_string());
        }

        let font = asset_server.load("fonts/PressStart2P-Regular.ttf");
        let text_style = TextStyle {
            font,
            font_size: 12.0,
            color: Color::WHITE,
        };
        
        commands.spawn((
            TextBundle::from_section(
                self.current_text.clone(),
                text_style,
            ).with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Percent(5.0),
                    bottom: Val::Percent(10.0),
                    ..default()
                },
                flex_wrap: FlexWrap::Wrap,
                max_size: Size{
                    width: Val::Px(1000.0),
                    height: Val::Px(800.0),
                },
                ..default()
            }),
            StoryText
        ));

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 1.0), //fade in later?
                ..default()
            },
            texture: asset_server.load("textures/fusion_cell.png"),
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..default()
        });
    }

    fn run(
        &mut self,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,

        mut story_query: Query<&mut Text, With<StoryText>>,
    ) {

        if self.current_text.len() < self.story_texts[self.current_story_line].len() {
            let slice = self.story_texts[self.current_story_line].clone();

            self.current_text = slice[..self.current_text.len() + 1].to_string();

            for mut text in &mut story_query {
                text.sections[0].value = self.current_text.clone();
            }
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            if self.current_story_line < self.story_texts.len() - 2 {
                self.current_story_line += 1;
                self.current_text = String::from("");
                println!("{}", self.story_texts[self.current_story_line])
            } else {
                next_state.0 = State::TechShop;
            }
        }
    }

    fn close(
        &mut self,
        commands: &mut Commands,
        entity_query: &mut Query<Entity, Without<Camera>>,
    ) {
        println!("close intro");
        for entity in entity_query.iter() {
            commands.entity(entity).despawn();
        }

        self.story_texts.clear();
    }
}

impl TechShopState {
    fn start(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) {
        println!("Start world");

        let texture_handle = asset_server.load("textures/player_walk.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 8, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut player_transform = Transform::from_scale(Vec3::splat(5.0));
        player_transform.translation.z = 100.0;
        let mut player_anim_timer = Timer::from_seconds(0.1, TimerMode::Repeating);
        player_anim_timer.pause();

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: player_transform,
                ..default()
            },
            AnimationTimer(player_anim_timer),
            Player {
                velocity: Vec2::new(0.0, 0.0)
            },
        ));

        let mut background_transform = Transform::from_scale(Vec3::splat(5.0));
        background_transform.translation.z = 0.0;
        background_transform.translation.y = 24.0;

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 1.0), //fade in later?
                ..default()
            },
            texture: asset_server.load("textures/tech_shop.png"),
            transform: background_transform,
            ..default()
        });

     
        // commands.spawn(PointLightBundle {
        //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
        //     ..default()
        // });
    }

    fn run(
        &mut self,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
        mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
    ) {
        for (mut sprite, mut timer, mut player_transform, mut player) in player_query.iter_mut() {

            if keyboard_input.pressed(KeyCode::A) {
                if timer.paused() {
                    timer.unpause();
                }
                player.velocity.x = -12.0;

                sprite.flip_x = true;

            } else if keyboard_input.pressed(KeyCode::D) {
                if timer.paused() {
                    timer.unpause();
                }

                sprite.flip_x = false;

                player.velocity.x = 12.0;
            } else {
                player.velocity.x = 0.0;
                if timer.just_finished() {
                    timer.pause();
                }
            }
    
            let mut target = Vec3::new(player_transform.translation.x + player.velocity.x, 
                                  player_transform.translation.y, 
                                  player_transform.translation.z);

            target.x = target.x.clamp(-430.0, 430.0);
    
            // if dive_collision_check(target, &wall_query, &lava_query, &data_query, &portal_query, &mut self.level, &mut deep_dive_data_bank, &mut next_state) {
            player_transform.translation = target;
            // } else {
            //     player.velocity.x = 0.0;
            //     player.velocity.y = 0.0;
            // }
    
            if keyboard_input.just_pressed(KeyCode::Escape) {
                println!("MENU")
            }
    
            if keyboard_input.just_pressed(KeyCode::P) {
                next_state.0 = State::DeepDive;
            }
        }
    }

    fn close(
        &mut self,
        commands: &mut Commands,
        entity_query: &mut Query<Entity, Without<Camera>>,
    ) {
        println!("close world");
        for entity in entity_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

impl DeepDiveState {
    fn start(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        mut deep_dive_data_bank: ResMut<DeepDiveDataBank>,
    ) {
        println!("Start deep dive");

        let mut tiles: Vec<Entity> = Vec::new();
        let mut map_lines: Vec<String> = Vec::new();

        let mut path = String::from("assets/dives/map");
        path.push_str(&deep_dive_data_bank.0.to_string()[..]);
        path.push_str("-");
        path.push_str(&self.level.to_string()[..]);
        path.push_str(".txt");

        let raw_text = fs::read_to_string(path)
            .expect("Cannot open map!");
        let lines = raw_text.split('\n');
        for l in lines {
            map_lines.push(l.to_string());
        }

        let half_map: i32 = map_lines.len() as i32 / 2;

        for y in 0..map_lines.len() {
            let line = map_lines[y as usize].clone();
            for x in 0..line.len() {
                let tile_char = line.chars().nth(x as usize).unwrap();

                let mut transform = Transform::from_scale(Vec3::splat(DEEP_DIVE_TILE_SCALE));
                transform.translation.x = (x as f32 - half_map as f32) * DEEP_DIVE_TILE_SCALE;
                transform.translation.y = (y as f32 - half_map as f32) * DEEP_DIVE_TILE_SCALE;

                let mut color = Color::rgba(1.0, 1.0, 1.0, 1.0);

                if tile_char == '#' {
                    color = Color::rgba(1.0, 1.0, 1.0, 1.0);
                }
                if tile_char == '~' {
                    color = Color::rgba(1.0, 0.0, 0.0, 1.0);
                }
                if tile_char == '@' {
                    color = Color::rgba(0.0, 1.0, 1.0, 1.0);
                }
                if tile_char == '$' {
                    color = Color::rgba(0.0, 1.0, 0.0, 1.0);
                }

                if tile_char == '#' || tile_char == '~' || tile_char == '@' || tile_char == '$' {
                    let tile = commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color,
                            ..default()
                        },
                        transform,
                        ..default()
                    }).id();

                    if tile_char == '#' {
                        commands.entity(tile).insert(Collider);
                    }
                    if tile_char == '~' {
                        commands.entity(tile).insert(Lava);
                    }
                    if tile_char == '@' {
                        commands.entity(tile).insert(DataPort);
                    }
                    if tile_char == '$' {
                        commands.entity(tile).insert(Portal);
                    }
                }
            }
        }

        let mut transform = Transform::from_scale(Vec3::splat(DEEP_DIVE_TILE_SCALE));
        transform.translation.x = 0.0 as f32 * DEEP_DIVE_TILE_SCALE;
        transform.translation.y = 0.0 as f32 * DEEP_DIVE_TILE_SCALE;

        commands.spawn((SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 0.3, 1.0, 1.0),
                    ..default()
                },
                transform,
                ..default()
            }, 
            Player {
                velocity: Vec2::new(0.0, 0.0),
            }
        ));

        println!("Created player")
    }

    fn run(
        &mut self,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
        mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
        wall_query: Query<&Transform, (Without<Player>, With<Collider>)>,
        lava_query: Query<&Transform, (Without<Player>, With<Lava>)>,
        data_query: Query<&Transform, (Without<Player>, With<DataPort>)>,
        portal_query: Query<&Transform, (Without<Player>, With<Portal>)>,

        mut deep_dive_data_bank: ResMut<DeepDiveDataBank>,
    ) {
        for (mut sprite, mut timer, mut player_transform, mut player) in player_query.iter_mut() {
            
            if player.velocity.x == 0.0 && player.velocity.y == 0.0 {
                if keyboard_input.just_pressed(KeyCode::A) {
                    player.velocity.x = -DEEP_DIVE_TILE_SCALE;
                } else if keyboard_input.just_pressed(KeyCode::W) {
                    player.velocity.y = DEEP_DIVE_TILE_SCALE;
                } else if keyboard_input.just_pressed(KeyCode::S) {
                    player.velocity.y = -DEEP_DIVE_TILE_SCALE;
                } else if keyboard_input.just_pressed(KeyCode::D) {
                    player.velocity.x = DEEP_DIVE_TILE_SCALE;
                }
            }
    
            let target = Vec3::new(player_transform.translation.x + player.velocity.x,
                player_transform.translation.y + player.velocity.y, 0.0);
    
            if dive_collision_check(target, &wall_query, &lava_query, &data_query, &portal_query, &mut self.level, &mut deep_dive_data_bank, &mut next_state) {
                player_transform.translation = target;
            } else {
                player.velocity.x = 0.0;
                player.velocity.y = 0.0;
            }
    
            if keyboard_input.just_pressed(KeyCode::Escape) {
                println!("MENU")
            }
    
            if keyboard_input.just_pressed(KeyCode::P) {
                next_state.0 = State::TechShop;
            }
        }
    }

    fn close(
        &mut self,
        commands: &mut Commands,
        entity_query: &mut Query<Entity, Without<Camera>>,
    ) {
        println!("close world");
        for entity in entity_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn run_current_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState>,

    story_query: Query<&mut Text, With<StoryText>>,
  
    mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
    wall_query: Query<&Transform, (Without<Player>, With<Collider>)>,
    lava_query: Query<&Transform, (Without<Player>, With<Lava>)>,
    data_query: Query<&Transform, (Without<Player>, With<DataPort>)>,
    portal_query: Query<&Transform, (Without<Player>, With<Portal>)>,

    mut deep_dive_data_bank: ResMut<DeepDiveDataBank>,

    current_state: Res<CurrentState>,
    mut intro_state: ResMut<IntroData>,
    mut tech_shop_state: ResMut<TechShopData>,
    mut deep_dive_state: ResMut<DeepDiveData>,
) {
    match current_state.0 {
        State::Intro => {
            intro_state.0.run(keyboard_input, next_state, story_query);
        },
        State::TechShop => {
            tech_shop_state.0.run(keyboard_input, next_state, player_query);
        },
        State::DeepDive => {
            deep_dive_state.0.run(keyboard_input, next_state, player_query, wall_query, lava_query, data_query, portal_query, deep_dive_data_bank);
        }
    }
}

fn start_initial_state(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_state: Res<CurrentState>,
    mut intro_state: ResMut<IntroData>,
    mut tech_shop_state: ResMut<TechShopData>,
    mut deep_dive_state: ResMut<DeepDiveData>,

    mut texture_atlases: ResMut<Assets<TextureAtlas>>,

    mut deep_dive_data_bank: ResMut<DeepDiveDataBank>,
) {
    match current_state.0 {
        State::Intro => {
            intro_state.0.start(&mut commands, &asset_server);
        },
        State::TechShop => {
            tech_shop_state.0.start(&mut commands, &asset_server, &mut texture_atlases);
        },
        State::DeepDive => {
            deep_dive_state.0.start(&mut commands, &asset_server, deep_dive_data_bank);
        }
    }
}

fn manage_state_changes(
    next_state: ResMut<NextState>,
    mut current_state: ResMut<CurrentState>,
    mut intro_state: ResMut<IntroData>,
    mut tech_shop_state: ResMut<TechShopData>,
    mut deep_dive_state: ResMut<DeepDiveData>,

    mut deep_dive_data_bank: ResMut<DeepDiveDataBank>,

    mut texture_atlases: ResMut<Assets<TextureAtlas>>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut entity_query: Query<Entity, Without<Camera>>,
) {
    if next_state.is_changed() && !next_state.is_added() {

        match (&current_state.0, &next_state.0) {
            (State::Intro, State::TechShop) => {
                intro_state.0.close(&mut commands, &mut entity_query);
                current_state.0 = State::TechShop;
                tech_shop_state.0.start(&mut commands, &asset_server, &mut texture_atlases);
            },
            (State::TechShop, State::DeepDive) => {
                tech_shop_state.0.close(&mut commands, &mut entity_query);
                current_state.0 = State::DeepDive;
                deep_dive_state.0.start(&mut commands, &asset_server, deep_dive_data_bank);
            }
            (State::DeepDive, State::TechShop) => {
                deep_dive_state.0.close(&mut commands, &mut entity_query);
                current_state.0 = State::TechShop;
                tech_shop_state.0.start(&mut commands, &asset_server, &mut texture_atlases);
            },
            (State::DeepDive, State::DeepDive) => {
                deep_dive_state.0.close(&mut commands, &mut entity_query);
                current_state.0 = State::DeepDive;
                deep_dive_state.0.start(&mut commands, &asset_server, deep_dive_data_bank);
            },
            _ => ()
        }

    }
}

fn dive_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (Without<Player>, With<Collider>)>,
    lava_query: &Query<&Transform, (Without<Player>, With<Lava>)>,
    data_query: &Query<&Transform, (Without<Player>, With<DataPort>)>,
    portal_query: &Query<&Transform, (Without<Player>, With<Portal>)>,
    level: &mut u32,
    deep_dive_data_bank: &mut ResMut<DeepDiveDataBank>,
    next_state: &mut ResMut<NextState>,
) -> bool {

    for portal_trans in portal_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(DEEP_DIVE_TILE_SCALE),
            portal_trans.translation,
            Vec2::splat(DEEP_DIVE_TILE_SCALE),
        );
        if collision.is_some() {
            if deep_dive_data_bank.0 < 1 {
                deep_dive_data_bank.0 += 1;
            }
            next_state.0 = State::TechShop;
        }
    }

    for data_trans in data_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(DEEP_DIVE_TILE_SCALE),
            data_trans.translation,
            Vec2::splat(DEEP_DIVE_TILE_SCALE),
        );
        if collision.is_some() {
            if *level < 2 {
                *level += 1;
            }
            next_state.0 = State::DeepDive;
        }
    }

    for lava_trans in lava_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(DEEP_DIVE_TILE_SCALE),
            lava_trans.translation,
            Vec2::splat(DEEP_DIVE_TILE_SCALE),
        );
        if collision.is_some() {
            next_state.0 = State::DeepDive;
        }
    }

    for wall_trans in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(DEEP_DIVE_TILE_SCALE),
            wall_trans.translation,
            Vec2::splat(DEEP_DIVE_TILE_SCALE),
        );
        if collision.is_some() {
            return false;
        }
    }

    true
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());

        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        if timer.paused() {
            sprite.index = 0;
        } else if timer.just_finished() {
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

