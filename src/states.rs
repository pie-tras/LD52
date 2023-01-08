use std::fs;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

const TIME_STEP: f32 = 1.0 / 60.0;

const DEEP_DIVE_TILE_SCALE: f32 = 16.0;
const DEEP_DIVE_TILE_COUNT: i32 = 16;

pub struct StatesPlugin;

enum State {
    Intro,
    World,
    DeepDive,
}

struct IntroState {
    current_text: String,
    story_texts: Vec<String>,
    current_story_line: usize,
}
struct WorldState {
    test: u32
}

struct DeepDiveState {
    test: u32
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

impl WorldState {
    fn new() -> Self {
        WorldState {
            test: 0
        }
    }
}

impl DeepDiveState {
    fn new() -> Self {
        DeepDiveState {
            test: 0
        }
    }
}

#[derive(Component)]
struct StoryText;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Lava;

#[derive(Component)]
struct DataPort;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Resource)]
struct CurrentState(State);

#[derive(Resource)]
struct NextState(State);

#[derive(Resource)]
struct IntroData(IntroState);

#[derive(Resource)]
struct WorldData(WorldState);

#[derive(Resource)]
struct DeepDiveData(DeepDiveState);

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentState(State::DeepDive))
        .insert_resource(NextState(State::Intro))
        .insert_resource(IntroData(IntroState::new()))
        .insert_resource(WorldData(WorldState::new()))
        .insert_resource(DeepDiveData(DeepDiveState::new()))
        .add_startup_system(start_initial_state)
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            .with_system(manage_state_changes)
            .with_system(run_current_game_state)
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

        let raw_text = fs::read_to_string("texts/intro.txt")
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
                next_state.0 = State::World;
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

impl WorldState {
    fn start(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        println!("Start world");

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 1.0), //fade in later?
                ..default()
            },
            texture: asset_server.load("textures/fusiogenic_logo.png"),
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..default()
        });
    }

    fn run(
        &mut self,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
    ) {

        if keyboard_input.just_pressed(KeyCode::Escape) {
            println!("MENU")
        }

        if keyboard_input.just_pressed(KeyCode::P) {
            next_state.0 = State::DeepDive;
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
    ) {
        println!("Start deep dive");

        let file = File::open("assets/dives/map.txt").expect("No map file found");
        let mut tiles: Vec<Entity> = Vec::new();

        for (y, line) in BufReader::new(file).lines().enumerate() {
            if let Ok(line) = line {
                for (x, char) in line.chars().enumerate() {

                    let mut transform = Transform::from_scale(Vec3::splat(DEEP_DIVE_TILE_SCALE));
                    transform.translation.x = (x as f32 * DEEP_DIVE_TILE_SCALE) - (DEEP_DIVE_TILE_SCALE * DEEP_DIVE_TILE_COUNT as f32);
                    transform.translation.y = (y as f32 * DEEP_DIVE_TILE_SCALE) - (DEEP_DIVE_TILE_SCALE * DEEP_DIVE_TILE_COUNT as f32);

                    let mut color = Color::rgba(1.0, 1.0, 1.0, 1.0);

                    if char == '#' {
                        color = Color::rgba(1.0, 1.0, 1.0, 1.0);
                    }
                    if char == '~' {
                        color = Color::rgba(1.0, 0.0, 0.0, 1.0);
                    }
                    if char == '@' {
                        color = Color::rgba(0.0, 1.0, 0.0, 1.0);
                    }

                    if char == '#' || char == '~' || char == '@' {
                        let tile = commands.spawn(SpriteBundle {
                            sprite: Sprite {
                                color,
                                ..default()
                            },
                            transform,
                            ..default()
                        }).id();

                        if char == '#' {
                            commands.entity(tile).insert(Collider);
                        }
                        if char == '~' {
                            commands.entity(tile).insert(Lava);
                        }
                        if char == '@' {
                            commands.entity(tile).insert(DataPort);
                        }
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
            Player,
            Velocity(Vec2::new(0.0, 0.0)),
        ));
    }

    fn run(
        &mut self,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
        mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
        wall_query: Query<&Transform, (Without<Player>, With<Collider>)>,
        lava_query: Query<&Transform, (Without<Player>, With<Lava>)>,
        data_query: Query<&Transform, (Without<Player>, With<DataPort>)>,
    ) {
        let (mut player_transform, mut player_velocity) = player_query.single_mut();

        if player_velocity.x == 0.0 && player_velocity.y == 0.0 {
            if keyboard_input.just_pressed(KeyCode::A) {
                player_velocity.x = -DEEP_DIVE_TILE_SCALE;
            } else if keyboard_input.just_pressed(KeyCode::W) {
                player_velocity.y = DEEP_DIVE_TILE_SCALE;
            } else if keyboard_input.just_pressed(KeyCode::S) {
                player_velocity.y = -DEEP_DIVE_TILE_SCALE;
            } else if keyboard_input.just_pressed(KeyCode::D) {
                player_velocity.x = DEEP_DIVE_TILE_SCALE;
            }
        }

        let target = Vec3::new(player_transform.translation.x + player_velocity.x,
            player_transform.translation.y + player_velocity.y, 0.0);

        if dive_collision_check(target, &wall_query, &lava_query, &data_query, &mut next_state) {
            player_transform.translation = target;
        } else {
            player_velocity.x = 0.0;
            player_velocity.y = 0.0;
        }

        if player_transform.translation.x > DEEP_DIVE_TILE_SCALE * DEEP_DIVE_TILE_COUNT as f32 {
            player_velocity.x = 0.0;
            player_transform.translation.x = DEEP_DIVE_TILE_SCALE * DEEP_DIVE_TILE_COUNT as f32;
        }

        if player_transform.translation.x < -DEEP_DIVE_TILE_SCALE * DEEP_DIVE_TILE_COUNT as f32 {
            player_velocity.x = 0.0;
            player_transform.translation.x = -DEEP_DIVE_TILE_SCALE * DEEP_DIVE_TILE_COUNT as f32;
        }

        if player_transform.translation.y > DEEP_DIVE_TILE_SCALE * DEEP_DIVE_TILE_COUNT as f32 {
            player_velocity.y = 0.0;
            player_transform.translation.y = DEEP_DIVE_TILE_SCALE * DEEP_DIVE_TILE_COUNT as f32;
        }

        if player_transform.translation.y < -DEEP_DIVE_TILE_SCALE * DEEP_DIVE_TILE_COUNT as f32 {
            player_velocity.y = 0.0;
            player_transform.translation.y = -DEEP_DIVE_TILE_SCALE * DEEP_DIVE_TILE_COUNT as f32;
        }

        if keyboard_input.just_pressed(KeyCode::Escape) {
            println!("MENU")
        }

        if keyboard_input.just_pressed(KeyCode::P) {
            next_state.0 = State::World;
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
  
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    wall_query: Query<&Transform, (Without<Player>, With<Collider>)>,
    lava_query: Query<&Transform, (Without<Player>, With<Lava>)>,
    data_query: Query<&Transform, (Without<Player>, With<DataPort>)>,

    current_state: Res<CurrentState>,
    mut intro_state: ResMut<IntroData>,
    mut world_state: ResMut<WorldData>,
    mut deep_dive_state: ResMut<DeepDiveData>,
) {
    match current_state.0 {
        State::Intro => {
            intro_state.0.run(keyboard_input, next_state, story_query);
        },
        State::World => {
            world_state.0.run(keyboard_input, next_state);
        },
        State::DeepDive => {
            deep_dive_state.0.run(keyboard_input, next_state, player_query, wall_query, lava_query, data_query);
        }
    }
}

fn start_initial_state(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_state: Res<CurrentState>,
    mut intro_state: ResMut<IntroData>,
    mut world_state: ResMut<WorldData>,
    mut deep_dive_state: ResMut<DeepDiveData>,
) {
    match current_state.0 {
        State::Intro => {
            intro_state.0.start(&mut commands, &asset_server);
        },
        State::World => {
            world_state.0.start(&mut commands, &asset_server);
        },
        State::DeepDive => {
            deep_dive_state.0.start(&mut commands, &asset_server);
        }
    }
}

fn manage_state_changes(
    next_state: ResMut<NextState>,
    mut current_state: ResMut<CurrentState>,
    mut intro_state: ResMut<IntroData>,
    mut world_state: ResMut<WorldData>,
    mut deep_dive_state: ResMut<DeepDiveData>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut entity_query: Query<Entity, Without<Camera>>,
) {
    if next_state.is_changed() && !next_state.is_added() {

        match (&current_state.0, &next_state.0) {
            (State::Intro, State::World) => {
                intro_state.0.close(&mut commands, &mut entity_query);
                current_state.0 = State::World;
                world_state.0.start(&mut commands, &asset_server);
            },
            (State::World, State::DeepDive) => {
                world_state.0.close(&mut commands, &mut entity_query);
                current_state.0 = State::DeepDive;
                deep_dive_state.0.start(&mut commands, &asset_server);
            }
            (State::DeepDive, State::World) => {
                deep_dive_state.0.close(&mut commands, &mut entity_query);
                current_state.0 = State::World;
                world_state.0.start(&mut commands, &asset_server);
            },
            (State::DeepDive, State::DeepDive) => {
                deep_dive_state.0.close(&mut commands, &mut entity_query);
                current_state.0 = State::DeepDive;
                deep_dive_state.0.start(&mut commands, &asset_server);
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
    next_state: &mut ResMut<NextState>,
) -> bool {

    for data_trans in data_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(DEEP_DIVE_TILE_SCALE),
            data_trans.translation,
            Vec2::splat(DEEP_DIVE_TILE_SCALE),
        );
        if collision.is_some() {
            next_state.0 = State::World;
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

