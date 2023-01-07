use std::fs;

use bevy::{
    prelude::*,
    time::FixedTimestep,
};

const TIME_STEP: f32 = 1.0 / 60.0;

pub struct StatesPlugin;

enum State {
    Intro,
    World,
}

struct IntroState {
    current_text: String,
    story_texts: Vec<String>,
    current_story_line: usize,
}
struct WorldState {
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

#[derive(Component)]
struct StoryText;

#[derive(Resource)]
struct CurrentState(State);

#[derive(Resource)]
struct NextState(State);

#[derive(Resource)]
struct IntroData(IntroState);

#[derive(Resource)]
struct WorldData(WorldState);

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentState(State::Intro))
        .insert_resource(NextState(State::Intro))
        .insert_resource(IntroData(IntroState::new()))
        .insert_resource(WorldData(WorldState::new()))
        .add_startup_system(start_initial_state)
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            .with_system(run_current_game_state)
            .with_system(manage_state_changes)
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

    current_state: Res<CurrentState>,
    mut intro_state: ResMut<IntroData>,
    mut world_state: ResMut<WorldData>,
) {
    match current_state.0 {
        State::Intro => {
            intro_state.0.run(keyboard_input, next_state, story_query);
        },
        State::World => {
            world_state.0.run(keyboard_input, next_state);
        }
    }
}

fn start_initial_state(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_state: Res<CurrentState>,
    mut intro_state: ResMut<IntroData>,
    mut world_state: ResMut<WorldData>,
) {
    match current_state.0 {
        State::Intro => {
            intro_state.0.start(&mut commands, &asset_server);
        },
        State::World => {
            world_state.0.start(&mut commands, &asset_server);
        }
    }
}

fn manage_state_changes(
    next_state: ResMut<NextState>,
    mut current_state: ResMut<CurrentState>,
    mut intro_state: ResMut<IntroData>,
    mut world_state: ResMut<WorldData>,

    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut entity_query: Query<Entity, Without<Camera>>,
) {
    if next_state.is_changed() && !next_state.is_added() {
        intro_state.0.close(&mut commands, &mut entity_query);
        current_state.0 = State::World;
        world_state.0.start(&mut commands, &asset_server);
    }
}