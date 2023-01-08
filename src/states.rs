use std::fs;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

const TIME_STEP: f32 = 1.0 / 60.0;

const DEEP_DIVE_TILE_SCALE: f32 = 16.0;

const PLAYER_SPEED: f32 = 6.0;

pub struct StatesPlugin;

enum State {
    Intro,
    TechShop,
    Alleyway,
    Cyberway,
    Cafe,
    DeepDive,
}

struct IntroState {
    current_text: String,
    story_texts: Vec<String>,
    current_story_line: usize,
}

struct TechShopState {
    has_played_location: bool,
}

struct AlleywayState {
    has_played_location: bool
}

struct CyberwayState {
    has_played_location: bool,
    spawn_x: f32,
}

struct CafeState {
    has_played_location: bool,
    spawn_x: f32,
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
            has_played_location: false
        }
    }
}

impl AlleywayState {
    fn new() -> Self {
        AlleywayState {
            has_played_location: false
        }
    }
}

impl CyberwayState {
    fn new() -> Self {
        CyberwayState {
            has_played_location: false,
            spawn_x: 0.0,
        }
    }
}

impl CafeState {
    fn new() -> Self {
        CafeState {
            has_played_location: false,
            spawn_x: 0.0,
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

struct StateCollection {
    intro_state: IntroState,
    tech_shop_state: TechShopState,
    alleyway_state: AlleywayState,
    cyberway_state: CyberwayState,
    cafe_state: CafeState,
    deep_dive_state: DeepDiveState,
}

impl StateCollection {
    fn new() -> Self {
        StateCollection {
            intro_state: IntroState::new(),
            tech_shop_state: TechShopState::new(),
            alleyway_state: AlleywayState::new(),
            cyberway_state: CyberwayState::new(),
            cafe_state: CafeState::new(),
            deep_dive_state: DeepDiveState::new(),
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
struct StateData(StateCollection);

#[derive(Resource)]
struct DeepDiveDataBank(u32);

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentState(State::Cafe))
        .insert_resource(NextState(State::Intro))
        .insert_resource(StateData(StateCollection::new()))
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
        println!("Start techshop");
        self.has_played_location = false;

        let texture_handle = asset_server.load("textures/player_walk.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 8, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut player_transform = Transform::from_scale(Vec3::splat(5.0));
        player_transform.translation.x = 320.0;
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
                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                ..default()
            },
            texture: asset_server.load("textures/tech_shop.png"),
            transform: background_transform,
            ..default()
        });

        let font = asset_server.load("fonts/PressStart2P-Regular.ttf");
        let text_style = TextStyle {
            font,
            font_size: 12.0,
            color: Color::WHITE,
        };
        
        commands.spawn((
            TextBundle::from_section(
                "",
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

    }

    fn run(
        &mut self,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
        mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
        mut story_query: Query<&mut Text, With<StoryText>>,
    ) {
        for (mut sprite, mut timer, mut player_transform, mut player) in player_query.iter_mut() {

            if keyboard_input.pressed(KeyCode::A) {
                if timer.paused() {
                    timer.unpause();
                }
                player.velocity.x = -PLAYER_SPEED;

                sprite.flip_x = true;

            } else if keyboard_input.pressed(KeyCode::D) {
                if timer.paused() {
                    timer.unpause();
                }

                sprite.flip_x = false;

                player.velocity.x = PLAYER_SPEED;
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

            if target.x > 350.0 {
                for mut text in &mut story_query {
                    let msg = String::from("Press [W] to enter the back alley.");
                    if text.sections[0].value.len() < msg.len() {
                        text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
                    }
                }

                if keyboard_input.just_pressed(KeyCode::W) {
                    next_state.0 = State::Alleyway;
                } 

            } else if target.x > 300.0 {
                if !self.has_played_location {
                    for mut text in &mut story_query {
                        let msg = String::from("Location: The cyber-tech workshop.");
                        if text.sections[0].value.len() < msg.len() {
                            text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
                        } else {
                            self.has_played_location = true;
                        }
                    }
                }
            } else {
                for mut text in &mut story_query {
                    if text.sections[0].value.len() != 0 {
                        text.sections[0].value = "".to_string();
                    }
                }
            }
    
            player_transform.translation = target;
    
            if keyboard_input.just_pressed(KeyCode::Escape) {
                println!("MENU")
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

impl AlleywayState {
    fn start(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) {
        self.has_played_location = false;
        println!("Start alleyway");

        let texture_handle = asset_server.load("textures/player_walk.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 8, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut player_transform = Transform::from_scale(Vec3::splat(5.0));
        player_transform.translation.x = 210.0;
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
        background_transform.translation.y = 78.0;

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                ..default()
            },
            texture: asset_server.load("textures/alleyway.png"),
            transform: background_transform,
            ..default()
        });

        let font = asset_server.load("fonts/PressStart2P-Regular.ttf");
        let text_style = TextStyle {
            font,
            font_size: 12.0,
            color: Color::WHITE,
        };
        
        commands.spawn((
            TextBundle::from_section(
                "",
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

    }

    fn run(
        &mut self,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
        mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
        mut story_query: Query<&mut Text, With<StoryText>>,
    ) {
        for (mut sprite, mut timer, mut player_transform, mut player) in player_query.iter_mut() {

            if keyboard_input.pressed(KeyCode::A) {
                if timer.paused() {
                    timer.unpause();
                }
                player.velocity.x = -PLAYER_SPEED;

                sprite.flip_x = true;

            } else if keyboard_input.pressed(KeyCode::D) {
                if timer.paused() {
                    timer.unpause();
                }

                sprite.flip_x = false;

                player.velocity.x = PLAYER_SPEED;
            } else {
                player.velocity.x = 0.0;
                if timer.just_finished() {
                    timer.pause();
                }
            }
    
            let mut target = Vec3::new(player_transform.translation.x + player.velocity.x, 
                                  player_transform.translation.y, 
                                  player_transform.translation.z);

            target.x = target.x.clamp(-610.0, 620.0);

            if target.x > 615.0 {
                next_state.0 = State::Cyberway;
            }

            if target.x > 75.0 && target.x < 150.0 {
                for mut text in &mut story_query {
                    let msg = String::from("Press [W] to enter the back alley.");
                    if text.sections[0].value.len() < msg.len() {
                        text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
                    }
                }

                if keyboard_input.just_pressed(KeyCode::W) {
                    next_state.0 = State::TechShop;
                } 
            } else if target.x > 200.0 {
                if !self.has_played_location {
                    for mut text in &mut story_query {
                        let msg = String::from("Location: The back alleys.");
                        if text.sections[0].value.len() < msg.len() {
                            text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
                        } else {
                            self.has_played_location = true;
                        }
                    }
                }
            } else {
                for mut text in &mut story_query {
                    if text.sections[0].value.len() != 0 {
                        text.sections[0].value = "".to_string();
                    }
                }
            }
    
            player_transform.translation = target;
    
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

impl CyberwayState {
    fn start(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) {
        self.has_played_location = false;
        println!("Start cyberway");

        let texture_handle = asset_server.load("textures/player_walk.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 8, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut player_transform = Transform::from_scale(Vec3::splat(5.0));
        player_transform.translation.x = self.spawn_x;
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
        background_transform.translation.y = 92.0;

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                ..default()
            },
            texture: asset_server.load("textures/cyberway.png"),
            transform: background_transform,
            ..default()
        });

        let font = asset_server.load("fonts/PressStart2P-Regular.ttf");
        let text_style = TextStyle {
            font,
            font_size: 12.0,
            color: Color::WHITE,
        };
        
        commands.spawn((
            TextBundle::from_section(
                "",
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

    }

    fn run(
        &mut self,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
        mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
        mut story_query: Query<&mut Text, With<StoryText>>,
    ) {
        for (mut sprite, mut timer, mut player_transform, mut player) in player_query.iter_mut() {

            if keyboard_input.pressed(KeyCode::A) {
                if timer.paused() {
                    timer.unpause();
                }
                player.velocity.x = -PLAYER_SPEED;

                sprite.flip_x = true;

            } else if keyboard_input.pressed(KeyCode::D) {
                if timer.paused() {
                    timer.unpause();
                }

                sprite.flip_x = false;

                player.velocity.x = PLAYER_SPEED;
            } else {
                player.velocity.x = 0.0;
                if timer.just_finished() {
                    timer.pause();
                }
            }
    
            let mut target = Vec3::new(player_transform.translation.x + player.velocity.x, 
                                  player_transform.translation.y, 
                                  player_transform.translation.z);

            target.x = target.x.clamp(-605.0, 610.0);

            if target.x < -600.0 {
                next_state.0 = State::Alleyway;
            }

            if target.x > -480.0 && target.x < -380.0 {
                for mut text in &mut story_query {
                    let msg = String::from("Press [W] to enter the cyber-parts shop.");
                    if text.sections[0].value.len() < msg.len() {
                        text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
                    }
                }

                if keyboard_input.just_pressed(KeyCode::W) {
                   // next_state.0 = State::CyberShop;
                } 
            } else if target.x > 430.0 && target.x < 520.0 {
                for mut text in &mut story_query {
                    let msg = String::from("Press [W] to enter Lycia Cafe.");
                    if text.sections[0].value.len() < msg.len() {
                        text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
                    }
                }

                if keyboard_input.just_pressed(KeyCode::W) {
                    next_state.0 = State::Cafe;
                } 
            } else if target.x < -550.0 {
                if !self.has_played_location {
                    for mut text in &mut story_query {
                        let msg = String::from("Location: The Cyberway.");
                        if text.sections[0].value.len() < msg.len() {
                            text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
                        } else {
                            self.has_played_location = true;
                        }
                    }
                }
            } else {
                for mut text in &mut story_query {
                    if text.sections[0].value.len() != 0 {
                        text.sections[0].value = "".to_string();
                    }
                }
            }
    
            player_transform.translation = target;
    
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

impl CafeState {
    fn start(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) {
        self.has_played_location = false;
        println!("Start cafe");

        let texture_handle = asset_server.load("textures/player_walk.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 8, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut player_transform = Transform::from_scale(Vec3::splat(5.0));
        player_transform.translation.x = 320.0;
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
        background_transform.translation.y = 23.0;

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                ..default()
            },
            texture: asset_server.load("textures/cafe.png"),
            transform: background_transform,
            ..default()
        });

        let font = asset_server.load("fonts/PressStart2P-Regular.ttf");
        let text_style = TextStyle {
            font,
            font_size: 12.0,
            color: Color::WHITE,
        };
        
        commands.spawn((
            TextBundle::from_section(
                "",
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

    }

    fn run(
        &mut self,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
        mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
        mut story_query: Query<&mut Text, With<StoryText>>,
    ) {
        for (mut sprite, mut timer, mut player_transform, mut player) in player_query.iter_mut() {

            if keyboard_input.pressed(KeyCode::A) {
                if timer.paused() {
                    timer.unpause();
                }
                player.velocity.x = -PLAYER_SPEED;

                sprite.flip_x = true;

            } else if keyboard_input.pressed(KeyCode::D) {
                if timer.paused() {
                    timer.unpause();
                }

                sprite.flip_x = false;

                player.velocity.x = PLAYER_SPEED;
            } else {
                player.velocity.x = 0.0;
                if timer.just_finished() {
                    timer.pause();
                }
            }
    
            let mut target = Vec3::new(player_transform.translation.x + player.velocity.x, 
                                  player_transform.translation.y, 
                                  player_transform.translation.z);

            target.x = target.x.clamp(-605.0, 610.0);

            // if target.x > 615.0 {
            //     next_state.0 = State::Cyberway;
            // }

            println!("{}", target.x);

            if target.x < -520.0 {
                for mut text in &mut story_query {
                    let msg = String::from("Press [W] to enter the deep-dive room.");
                    if text.sections[0].value.len() < msg.len() {
                        text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
                    }
                }

                if keyboard_input.just_pressed(KeyCode::W) {
                   // next_state.0 = State::CyberShop;
                } 
            } else if target.x > 380.0 && target.x < 480.0 {
                for mut text in &mut story_query {
                    let msg = String::from("Press [W] to enter the Cyberway.");
                    if text.sections[0].value.len() < msg.len() {
                        text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
                    }
                }

                if keyboard_input.just_pressed(KeyCode::W) {
                    next_state.0 = State::Cyberway;
                } 
            } else if target.x > -520.0 && target.x < 380.0 {
                if !self.has_played_location {
                    for mut text in &mut story_query {
                        let msg = String::from("Location: Lycia Cafe.");
                        if text.sections[0].value.len() < msg.len() {
                            text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
                        } else {
                            self.has_played_location = true;
                        }
                    }
                }
            } else {
                for mut text in &mut story_query {
                    if text.sections[0].value.len() != 0 {
                        text.sections[0].value = "".to_string();
                    }
                }
            }
    
            player_transform.translation = target;
    
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
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
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

        let texture_handle = asset_server.load("textures/player_walk.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 8, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut player_transform = Transform::from_scale(Vec3::splat(0.5));
        player_transform.translation.z = 100.0;
        let mut player_anim_timer = Timer::from_seconds(0.1, TimerMode::Repeating);

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
    mut states: ResMut<StateData>,
) {
    match current_state.0 {
        State::Intro => {
            states.0.intro_state.run(keyboard_input, next_state, story_query);
        },
        State::TechShop => {
            states.0.tech_shop_state.run(keyboard_input, next_state, player_query, story_query);
        },
        State::Alleyway => {
            states.0.alleyway_state.run(keyboard_input, next_state, player_query, story_query);
        },
        State::Cyberway => {
            states.0.cyberway_state.run(keyboard_input, next_state, player_query, story_query);
        },
        State::Cafe => {
            states.0.cafe_state.run(keyboard_input, next_state, player_query, story_query);
        },
        State::DeepDive => {
            states.0.deep_dive_state.run(keyboard_input, next_state, player_query, wall_query, lava_query, data_query, portal_query, deep_dive_data_bank);
        }
    }
}

fn start_initial_state(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_state: Res<CurrentState>,
    mut states: ResMut<StateData>,

    mut texture_atlases: ResMut<Assets<TextureAtlas>>,

    mut deep_dive_data_bank: ResMut<DeepDiveDataBank>,
) {
    match current_state.0 {
        State::Intro => {
            states.0.intro_state.start(&mut commands, &asset_server);
        },
        State::TechShop => {
            states.0.tech_shop_state.start(&mut commands, &asset_server, &mut texture_atlases);
        },
        State::Alleyway => {
            states.0.alleyway_state.start(&mut commands, &asset_server, &mut texture_atlases);
        },
        State::Cyberway => {
            states.0.cyberway_state.start(&mut commands, &asset_server, &mut texture_atlases);
        },
        State::Cafe => {
            states.0.cafe_state.start(&mut commands, &asset_server, &mut texture_atlases);
        },
        State::DeepDive => {
            states.0.deep_dive_state.start(&mut commands, &asset_server, &mut texture_atlases, deep_dive_data_bank);
        }
    }
}

fn manage_state_changes(
    next_state: ResMut<NextState>,
    mut current_state: ResMut<CurrentState>,
    mut states: ResMut<StateData>,

    mut deep_dive_data_bank: ResMut<DeepDiveDataBank>,

    mut texture_atlases: ResMut<Assets<TextureAtlas>>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut entity_query: Query<Entity, Without<Camera>>,
) {
    if next_state.is_changed() && !next_state.is_added() {

        match (&current_state.0, &next_state.0) {
            (State::Intro, State::TechShop) => {
                states.0.intro_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::TechShop;
                states.0.tech_shop_state.start(&mut commands, &asset_server, &mut texture_atlases);
            },
            (State::TechShop, State::Alleyway) => {
                states.0.tech_shop_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::Alleyway;
                states.0.alleyway_state.start(&mut commands, &asset_server, &mut texture_atlases);
            },
            (State::Alleyway, State::TechShop) => {
                states.0.alleyway_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::TechShop;
                states.0.tech_shop_state.start(&mut commands, &asset_server, &mut texture_atlases);
            },
            (State::Alleyway, State::Cyberway) => {
                states.0.alleyway_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::Cyberway;
                states.0.cyberway_state.spawn_x = -580.0;
                states.0.cyberway_state.start(&mut commands, &asset_server, &mut texture_atlases);
            },
            (State::Cyberway, State::Cafe) => {
                states.0.cyberway_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::Cafe;
                println!("Here");
                states.0.cafe_state.start(&mut commands, &asset_server, &mut texture_atlases);
            },
            (State::Cyberway, State::Alleyway) => {
                states.0.cyberway_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::Alleyway;
                // cyberway_state.0.spawn_x = 400.0;
                states.0.alleyway_state.start(&mut commands, &asset_server, &mut texture_atlases);
            },
            (State::Cafe, State::Cyberway) => {
                states.0.cafe_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::Cyberway;
                states.0.cyberway_state.spawn_x = 400.0;
                states.0.cyberway_state.start(&mut commands, &asset_server, &mut texture_atlases);
            },

            (State::TechShop, State::DeepDive) => {
                states.0.tech_shop_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::DeepDive;
                states.0.deep_dive_state.start(&mut commands, &asset_server, &mut texture_atlases, deep_dive_data_bank);
            }
            (State::DeepDive, State::TechShop) => {
                states.0.deep_dive_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::TechShop;
                states.0.tech_shop_state.start(&mut commands, &asset_server, &mut texture_atlases);
            },
            (State::DeepDive, State::DeepDive) => {
                states.0.deep_dive_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::DeepDive;
                states.0.deep_dive_state.start(&mut commands, &asset_server, &mut texture_atlases, deep_dive_data_bank);
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

