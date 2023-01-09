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
    Pod,
    Alleyway2,
    DeepDive,
}

struct IntroState {
    current_text: String,
    story_texts: Vec<String>,
    current_story_line: usize,
}

struct TechShopState {
    talking_entity: Entity,
    current_talking: u32,
    has_played_msg: bool,
    has_moved: bool,
    current_msg: String,
    talking: bool,
    dialog_line: usize,
    dialog_state: u32,
    dialog_texts: Vec<String>,

    location_string: String,
    alleyway_door: String,
    ask_robot: String,
    ask_pip: String,
}

struct AlleywayState {
    talking_entity: Entity,
    current_talking: u32,
    has_played_msg: bool,
    has_moved: bool,
    current_msg: String,
    talking: bool,
    dialog_line: usize,
    dialog_state: u32,
    dialog_texts: Vec<String>,

    location_string: String,
    tech_shop_door: String,
    ask_figure: String,
}

struct CyberwayState {
    talking_entity: Entity,
    current_talking: u32,
    has_played_msg: bool,
    has_moved: bool,
    current_msg: String,
    talking: bool,
    dialog_line: usize,
    dialog_state: u32,
    dialog_texts: Vec<String>,

    location_string: String,
    parts_shop_door: String,
    cafe_door: String,
    spawn_x: f32,
}

struct CafeState {
    talking_entity: Entity,
    current_talking: u32,
    has_played_msg: bool,
    has_moved: bool,
    current_msg: String,
    should_clear: bool,

    location_string: String,

    spawn_x: f32,
}

struct PodState {
    talking_entity: Entity,
    current_talking: u32,
    has_played_msg: bool,
    has_moved: bool,
    current_msg: String,
    should_clear: bool,

    location_string: String,
}

struct Alleyway2State {
    talking_entity: Entity,
    current_talking: u32,
    has_played_msg: bool,
    has_moved: bool,
    current_msg: String,
    should_clear: bool,

    location_string: String,
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
            talking_entity: Entity::from_raw(0),
            current_talking: 0,
            has_played_msg: false,
            has_moved: false,
            current_msg: String::from(""),
            talking: false,
            dialog_line: 0,
            dialog_state: 0,
            dialog_texts: Vec::new(),

            location_string: String::from("Location: Tech Workshop."),
            alleyway_door: String::from("Press [W] to enter the alleyway."),
            ask_robot: String::from("Floating Robot: Press [Space] to talk."),
            ask_pip: String::from("Pip: Press [Space] to talk."),
        }
    }
}

impl AlleywayState {
    fn new() -> Self {
        AlleywayState {
            talking_entity: Entity::from_raw(0),
            current_talking: 0,
            has_played_msg: false,
            has_moved: false,
            current_msg: String::from(""),
            talking: false,
            dialog_line: 0,
            dialog_state: 0,
            dialog_texts: Vec::new(),

            location_string: String::from("Location: Tech Workshop Back Alley."),
            tech_shop_door: String::from("Press [W] to enter the Tech Workshop."),
            ask_figure: String::from("Mysterious Figure: Press [Space] to talk."),
        }
    }
}

impl CyberwayState {
    fn new() -> Self {
        CyberwayState {
            talking_entity: Entity::from_raw(0),
            current_talking: 0,
            has_played_msg: false,
            has_moved: false,
            current_msg: String::from(""),
            talking: false,
            dialog_line: 0,
            dialog_state: 0,
            dialog_texts: Vec::new(),

            location_string: String::from("Location: Cyberway."),
            parts_shop_door: String::from("Press [W] to enter the Cyber Parts Shop."),
            cafe_door: String::from("Press [W] to enter Lycia Cafe."),
            spawn_x: 0.0,
        }
    }
}

impl CafeState {
    fn new() -> Self {
        CafeState {
            talking_entity: Entity::from_raw(0),
            current_talking: 0,
            has_played_msg: false,
            has_moved: false,
            current_msg: String::from(""),
            should_clear: false,

            location_string: String::from("Location: Lycia Cafe."),
            spawn_x: 0.0,
        }
    }
}

impl PodState {
    fn new() -> Self {
        PodState {
            talking_entity: Entity::from_raw(0),
            current_talking: 0,
            has_played_msg: false,
            has_moved: false,
            current_msg: String::from(""),
            should_clear: false,

            location_string: String::from("Location: Deep Dive Pod Room."),
        }
    }
}

impl Alleyway2State {
    fn new() -> Self {
        Alleyway2State {
            talking_entity: Entity::from_raw(0),
            current_talking: 0,
            has_played_msg: false,
            has_moved: false,
            current_msg: String::from(""),
            should_clear: false,

            location_string: String::from("Location: Lycia Cafe Back Alley."),
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
    pod_state: PodState,
    alleyway2_state: Alleyway2State,
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
            pod_state: PodState::new(),
            alleyway2_state: Alleyway2State::new(),
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

#[derive(Component)]
struct NPC {
    talking_id: i32
}

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
        app.insert_resource(CurrentState(State::TechShop))
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
        self.talking_entity = Entity::from_raw(0);
        self.current_talking = 0;
        self.has_played_msg = false;
        self.has_moved = false;
        self.current_msg = self.location_string.clone();
        self.talking = false;
        self.dialog_line = 0;

        spawn_player(commands, &asset_server, texture_atlases, 320.0);
        spawn_robot(commands, &asset_server, texture_atlases, -200.0);
        spawn_background(commands, &asset_server, State::TechShop, 23.0);
        self.talking_entity = spawn_text_box(commands, &asset_server);
        spawn_story_text(commands, &asset_server);
    }

    fn run(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
        mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
        mut story_query: Query<&mut Text, With<StoryText>>,
        npc_query: Query<(&NPC, &Transform), (Without<Player>, With<NPC>)>,
    ) {
        for (mut sprite, mut timer, mut player_transform, mut player) in player_query.iter_mut() {

            let (mut target, mut moved) = move_player(&keyboard_input, &mut sprite, &mut timer, &mut player_transform, &mut player);
            if moved {
                self.has_moved = true;
                self.talking = false;
                commands.entity(self.talking_entity).despawn();
                self.talking_entity = spawn_talking_entity(commands, &asset_server, 0);
                self.dialog_line = 0;
                self.dialog_texts.clear();
            }

            let mut queue_clear = false;

            let npc = npc_collision_check(target, &npc_query);

            if self.has_moved {
                if npc != -1 && !moved {
                    if !self.talking {
                        if self.dialog_state == 0 {
                            if !self.current_msg.eq(&self.ask_robot) {
                                self.current_msg = self.ask_robot.clone();
                            }
                        } else {
                             if !self.current_msg.eq(&self.ask_pip) {
                                self.current_msg = self.ask_pip.clone();
                            }
                        }
                    }

                    if keyboard_input.just_pressed(KeyCode::Space) {
                        self.talking = true;

                        (self.dialog_state, self.dialog_line, self.talking_entity) =
                        manage_dialog(commands, &asset_server,
                                    State::TechShop,
                                    npc as u32,
                                    self.talking_entity,
                                    self.dialog_state,
                                    self.dialog_line,
                                    &mut self.dialog_texts,
                                    &mut self.current_msg,
                        );

                        queue_clear = true;
                    }
                } else {
                    if target.x > 350.0 && !moved {
                        commands.entity(self.talking_entity).despawn();
                        self.talking_entity = spawn_talking_entity(commands, &asset_server, 1);

                        if !self.current_msg.eq(&self.alleyway_door) {
                            self.current_msg = self.alleyway_door.clone();
                        }
                        if keyboard_input.just_pressed(KeyCode::W) {
                            next_state.0 = State::Alleyway;
                        } 
                    } else if self.has_played_msg {
                        queue_clear = true;
                    }
                }
            }

            for mut text in &mut story_query {
                if queue_clear && (self.talking || self.has_played_msg) {
                    clear_msg(&self.current_msg, &mut text);
                } else {
                    self.has_played_msg = update_msg(&self.current_msg, &mut text);
                }
            }

            target.x = target.x.clamp(-430.0, 430.0);
            player_transform.translation = target;
        }
    }

    fn close(
        &mut self,
        commands: &mut Commands,
        entity_query: &mut Query<Entity, Without<Camera>>,
    ) {
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
        spawn_x: f32,
    ) {
        self.talking_entity = Entity::from_raw(0);
        self.current_talking = 0;
        self.has_played_msg = false;
        self.has_moved = false;
        self.current_msg = self.location_string.clone();
        self.talking = false;
        self.dialog_line = 0;

        spawn_player(commands, &asset_server, texture_atlases, spawn_x);
        if self.dialog_state == 0 {
            spawn_figure(commands, &asset_server, texture_atlases, -310.0);
        }
        spawn_background(commands, &asset_server, State::Alleyway, 78.0);
        self.talking_entity = spawn_text_box(commands, &asset_server);
        spawn_story_text(commands, &asset_server);
    }

    fn run(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
        mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
        mut story_query: Query<&mut Text, With<StoryText>>,
        npc_query: Query<(&NPC, &Transform), (Without<Player>, With<NPC>)>,
    ) {
        for (mut sprite, mut timer, mut player_transform, mut player) in player_query.iter_mut() {

            let (mut target, mut moved) = move_player(&keyboard_input, &mut sprite, &mut timer, &mut player_transform, &mut player);
            if moved {
                self.has_moved = true;
                self.talking = false;
                commands.entity(self.talking_entity).despawn();
                self.talking_entity = spawn_talking_entity(commands, &asset_server, 0);
                self.dialog_line = 0;
                self.dialog_texts.clear();
            }

            let mut queue_clear = false;

            let npc = npc_collision_check(target, &npc_query);

            if self.has_moved {
                if npc != -1 && !moved {
                    if !self.talking && !self.current_msg.eq(&self.ask_figure) {
                        self.current_msg = self.ask_figure.clone();
                    }

                    if keyboard_input.just_pressed(KeyCode::Space) {
                        self.talking = true;

                        (self.dialog_state, self.dialog_line, self.talking_entity) =
                        manage_dialog(commands, &asset_server,
                                    State::Alleyway,
                                    npc as u32,
                                    self.talking_entity,
                                    self.dialog_state,
                                    self.dialog_line,
                                    &mut self.dialog_texts,
                                    &mut self.current_msg,
                        );

                        queue_clear = true;
                    }
                } else {
                    if target.x > 50.0 && target.x < 170.0 && !moved {
                        commands.entity(self.talking_entity).despawn();
                        self.talking_entity = spawn_talking_entity(commands, &asset_server, 1);

                        if !self.current_msg.eq(&self.tech_shop_door) {
                            self.current_msg = self.tech_shop_door.clone();
                        }
                        if keyboard_input.just_pressed(KeyCode::W) {
                            next_state.0 = State::TechShop;
                        } 
                    } else if self.has_played_msg {
                        queue_clear = true;
                    }
                }
            }

            for mut text in &mut story_query {
                if queue_clear && (self.talking || self.has_played_msg) {
                    clear_msg(&self.current_msg, &mut text);
                } else {
                    self.has_played_msg = update_msg(&self.current_msg, &mut text);
                }
            }

            target.x = target.x.clamp(-610.0, 620.0);

            if target.x > 615.0 {
                next_state.0 = State::Cyberway;
            }
    
            player_transform.translation = target;
        }
    }

    fn close(
        &mut self,
        commands: &mut Commands,
        entity_query: &mut Query<Entity, Without<Camera>>,
    ) {
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
        spawn_x: f32,
    ) {
        println!("Start cyberway");

        self.talking_entity = Entity::from_raw(0);
        self.current_talking = 0;
        self.has_played_msg = false;
        self.has_moved = false;
        self.current_msg = self.location_string.clone();
        self.talking = false;
        self.dialog_line = 0;

        spawn_player(commands, &asset_server, texture_atlases, spawn_x);
        spawn_background(commands, &asset_server, State::Cyberway, 92.0);
        self.talking_entity = spawn_text_box(commands, &asset_server);
        spawn_story_text(commands, &asset_server);
    }

    fn run(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
        mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
        mut story_query: Query<&mut Text, With<StoryText>>,
        npc_query: Query<(&NPC, &Transform), (Without<Player>, With<NPC>)>,
    ) {
        for (mut sprite, mut timer, mut player_transform, mut player) in player_query.iter_mut() {

            let (mut target, mut moved) = move_player(&keyboard_input, &mut sprite, &mut timer, &mut player_transform, &mut player);
            if target.x > 610.0 {
                target.x = 0.0;
            }

            if moved {
                self.has_moved = true;
                self.talking = false;
                commands.entity(self.talking_entity).despawn();
                self.talking_entity = spawn_talking_entity(commands, &asset_server, 0);
                self.dialog_line = 0;
                self.dialog_texts.clear();
            }

            let mut queue_clear = false;

            let npc = npc_collision_check(target, &npc_query);

            if self.has_moved {
                if npc != -1 && !moved {
                    if keyboard_input.just_pressed(KeyCode::Space) {
                        self.talking = true;

                        (self.dialog_state, self.dialog_line, self.talking_entity) =
                        manage_dialog(commands, &asset_server,
                                    State::Alleyway,
                                    npc as u32,
                                    self.talking_entity,
                                    self.dialog_state,
                                    self.dialog_line,
                                    &mut self.dialog_texts,
                                    &mut self.current_msg,
                        );

                        queue_clear = true;
                    }
                } else {
                    if target.x > -450.0 && target.x < -380.0 && !moved {
                        commands.entity(self.talking_entity).despawn();
                        self.talking_entity = spawn_talking_entity(commands, &asset_server, 1);

                        if !self.current_msg.eq(&self.parts_shop_door) {
                            self.current_msg = self.parts_shop_door.clone();
                        }
                        if keyboard_input.just_pressed(KeyCode::W) {
                           // next_state.0 = State::PartsShop;
                        } 
                    } else  if target.x > 440.0 && target.x < 520.0 && !moved {
                        commands.entity(self.talking_entity).despawn();
                        self.talking_entity = spawn_talking_entity(commands, &asset_server, 1);

                        if !self.current_msg.eq(&self.cafe_door) {
                            self.current_msg = self.cafe_door.clone();
                        }
                        if keyboard_input.just_pressed(KeyCode::W) {
                            next_state.0 = State::Cafe;
                        } 
                    } else if self.has_played_msg {
                        queue_clear = true;
                    }
                }
            }

            for mut text in &mut story_query {
                if queue_clear && (self.talking || self.has_played_msg) {
                    clear_msg(&self.current_msg, &mut text);
                } else {
                    self.has_played_msg = update_msg(&self.current_msg, &mut text);
                }
            }

            target.x = target.x.clamp(-605.0, 610.0);

            if target.x < -600.0 {
                next_state.0 = State::Alleyway;
            }

            if target.x > 605.0 {
                next_state.0 = State::Alleyway2;
            }
    
            player_transform.translation = target;
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
        println!("Start cafe");

        self.talking_entity = Entity::from_raw(0);
        self.current_talking = 0;
        self.has_played_msg = false;
        self.has_moved = false;
        self.current_msg = self.location_string.clone();
        self.should_clear = false;

        self.spawn_x = 320.0;

        spawn_player(commands, &asset_server, texture_atlases, self.spawn_x);
        spawn_background(commands, &asset_server, State::Cafe, 23.0);
        self.talking_entity = spawn_text_box(commands, &asset_server);
        spawn_story_text(commands, &asset_server);
    }

    fn run(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
        mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
        mut story_query: Query<&mut Text, With<StoryText>>,
        npc_query: Query<(&NPC, &Transform), (Without<Player>, With<NPC>)>,
    ) {
        for (mut sprite, mut timer, mut player_transform, mut player) in player_query.iter_mut() {

            let (mut target, moved) = move_player(&keyboard_input, &mut sprite, &mut timer, &mut player_transform, &mut player);
            if moved {
                self.has_moved = true;
            }

            if self.has_moved {
                if target.x > 350.0 {
                    self.should_clear = false;
                    // if !self.current_msg.eq(&self.tech_shop_door) {
                    //     self.current_msg = self.tech_shop_door.clone();
                    // }
                    if keyboard_input.just_pressed(KeyCode::W) {
                        next_state.0 = State::TechShop;
                    } 
                } else if self.has_played_msg {
                    self.should_clear = true;
                }
            }

            for mut text in &mut story_query {
                self.has_played_msg = update_msg(&self.current_msg, &mut text);
            }

            target.x = target.x.clamp(-605.0, 610.0);

            // if target.x > 615.0 {
            //     next_state.0 = State::Cyberway;
            // }

            // println!("{}", target.x);

            // if target.x < -520.0 {
            //     for mut text in &mut story_query {
            //         let msg = String::from("Press [W] to enter the deep-dive room.");
            //         if text.sections[0].value.len() < msg.len() {
            //             text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
            //         }
            //     }

            //     if keyboard_input.just_pressed(KeyCode::W) {
            //         next_state.0 = State::Pod;
            //     } 
            // } else if target.x > 380.0 && target.x < 480.0 {
            //     for mut text in &mut story_query {
            //         let msg = String::from("Press [W] to enter the Cyberway.");
            //         if text.sections[0].value.len() < msg.len() {
            //             text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
            //         }
            //     }

            //     if keyboard_input.just_pressed(KeyCode::W) {
            //         next_state.0 = State::Cyberway;
            //     } 
            // } else if target.x > -520.0 && target.x < 380.0 {
            //     if !self.has_played_location {
            //         for mut text in &mut story_query {
            //             let msg = String::from("Location: Lycia Cafe.");
            //             if text.sections[0].value.len() < msg.len() {
            //                 text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
            //             } else {
            //                 self.has_played_location = true;
            //             }
            //         }
            //     }
            // } else {
            //     for mut text in &mut story_query {
            //         if text.sections[0].value.len() != 0 {
            //             text.sections[0].value = "".to_string();
            //         }
            //     }
            // }
    
            player_transform.translation = target;
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

impl PodState {
    fn start(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) {
        println!("Start cafe");
        
        self.talking_entity = Entity::from_raw(0);
        self.current_talking = 0;
        self.has_played_msg = false;
        self.has_moved = false;
        self.current_msg = self.location_string.clone();
        self.should_clear = false;

        spawn_player(commands, &asset_server, texture_atlases, 0.0);
        spawn_background(commands, &asset_server, State::Pod, 23.0);
        self.talking_entity = spawn_text_box(commands, &asset_server);
        spawn_story_text(commands, &asset_server);
    }

    fn run(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
        mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
        mut story_query: Query<&mut Text, With<StoryText>>,
        npc_query: Query<(&NPC, &Transform), (Without<Player>, With<NPC>)>,
    ) {
        for (mut sprite, mut timer, mut player_transform, mut player) in player_query.iter_mut() {

            let (mut target, moved) = move_player(&keyboard_input, &mut sprite, &mut timer, &mut player_transform, &mut player);
            if moved {
                self.has_moved = true;
            }

            if self.has_moved {
                if target.x > 350.0 {
                    self.should_clear = false;
                    // if !self.current_msg.eq(&self.tech_shop_door) {
                    //     self.current_msg = self.tech_shop_door.clone();
                    // }
                    if keyboard_input.just_pressed(KeyCode::W) {
                        next_state.0 = State::TechShop;
                    } 
                } else if self.has_played_msg {
                    self.should_clear = true;
                }
            }

            for mut text in &mut story_query {
                self.has_played_msg = update_msg(&self.current_msg, &mut text);
            }

            target.x = target.x.clamp(-605.0, 610.0);

            // if target.x > 615.0 {
            //     next_state.0 = State::Cyberway;
            // }

            println!("{}", target.x);

            // if target.x < -520.0 {
            //     for mut text in &mut story_query {
            //         let msg = String::from("Press [W] to enter the deep-dive room.");
            //         if text.sections[0].value.len() < msg.len() {
            //             text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
            //         }
            //     }

            //     if keyboard_input.just_pressed(KeyCode::W) {
            //        // next_state.0 = State::CyberShop;
            //     } 
            // } else if target.x > 380.0 && target.x < 480.0 {
            //     for mut text in &mut story_query {
            //         let msg = String::from("Press [W] to enter the Cyberway.");
            //         if text.sections[0].value.len() < msg.len() {
            //             text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
            //         }
            //     }

            //     if keyboard_input.just_pressed(KeyCode::W) {
            //         next_state.0 = State::Cyberway;
            //     } 
            // } else if target.x > -520.0 && target.x < 380.0 {
            //     if !self.has_played_location {
            //         for mut text in &mut story_query {
            //             let msg = String::from("Location: Lycia Cafe.");
            //             if text.sections[0].value.len() < msg.len() {
            //                 text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
            //             } else {
            //                 self.has_played_location = true;
            //             }
            //         }
            //     }
            // } else {
            //     for mut text in &mut story_query {
            //         if text.sections[0].value.len() != 0 {
            //             text.sections[0].value = "".to_string();
            //         }
            //     }
            // }
    
            player_transform.translation = target;
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

impl Alleyway2State {
    fn start(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) {
        println!("Start alleyway");

        self.talking_entity = Entity::from_raw(0);
        self.current_talking = 0;
        self.has_played_msg = false;
        self.has_moved = false;
        self.current_msg = self.location_string.clone();
        self.should_clear = false;

        spawn_player(commands, &asset_server, texture_atlases, 210.0);
        spawn_figure(commands, &asset_server, texture_atlases, -310.0);
        spawn_background(commands, &asset_server, State::Alleyway2, 78.0);
        self.talking_entity = spawn_text_box(commands, &asset_server);
        spawn_story_text(commands, &asset_server);
    }

    fn run(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState>,
        mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
        mut story_query: Query<&mut Text, With<StoryText>>,
        npc_query: Query<(&NPC, &Transform), (Without<Player>, With<NPC>)>,
    ) {
        for (mut sprite, mut timer, mut player_transform, mut player) in player_query.iter_mut() {

            let (mut target, moved) = move_player(&keyboard_input, &mut sprite, &mut timer, &mut player_transform, &mut player);
            if moved {
                self.has_moved = true;
            }

            if self.has_moved {
                if target.x > 350.0 {
                    self.should_clear = false;
                    // if !self.current_msg.eq(&self.tech_shop_door) {
                    //     self.current_msg = self.tech_shop_door.clone();
                    // }
                    if keyboard_input.just_pressed(KeyCode::W) {
                        next_state.0 = State::TechShop;
                    } 
                } else if self.has_played_msg {
                    self.should_clear = true;
                }
            }

            for mut text in &mut story_query {
                self.has_played_msg = update_msg(&self.current_msg, &mut text);
            }

            target.x = target.x.clamp(-610.0, 620.0);

            if target.x > 615.0 {
                next_state.0 = State::Cyberway;
            }

            // if target.x > 75.0 && target.x < 150.0 {
            //     for mut text in &mut story_query {
            //         let msg = String::from("Press [W] to enter the back alley.");
            //         if text.sections[0].value.len() < msg.len() {
            //             text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
            //         }
            //     }

            //     if keyboard_input.just_pressed(KeyCode::W) {
            //         next_state.0 = State::TechShop;
            //     } 
            // } else if target.x > 200.0 {
            //     if !self.has_played_location {
            //         for mut text in &mut story_query {
            //             let msg = String::from("Location: The back alleys.");
            //             if text.sections[0].value.len() < msg.len() {
            //                 text.sections[0].value = msg[..text.sections[0].value.len() + 1].to_string();
            //             } else {
            //                 self.has_played_location = true;
            //             }
            //         }
            //     }
            // } else {
            //     for mut text in &mut story_query {
            //         if text.sections[0].value.len() != 0 {
            //             text.sections[0].value = "".to_string();
            //         }
            //     }
            // }
    
            player_transform.translation = target;
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState>,
  
    mut player_query: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Transform, &mut Player), With<Player>>,
    story_query: Query<&mut Text, With<StoryText>>,
    npc_query: Query<(&NPC, &Transform), (Without<Player>, With<NPC>)>,

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
            states.0.tech_shop_state.run(&mut commands, &asset_server, keyboard_input, next_state, player_query, story_query, npc_query);
        },
        State::Alleyway => {
            states.0.alleyway_state.run(&mut commands, &asset_server, keyboard_input, next_state, player_query, story_query, npc_query);
        },
        State::Cyberway => {
            states.0.cyberway_state.run(&mut commands, &asset_server, keyboard_input, next_state, player_query, story_query, npc_query);
        },
        State::Cafe => {
            states.0.cafe_state.run(&mut commands, &asset_server, keyboard_input, next_state, player_query, story_query, npc_query);
        },
        State::Pod => {
            states.0.pod_state.run(&mut commands, &asset_server, keyboard_input, next_state, player_query, story_query, npc_query);
        },
        State::Alleyway2 => {
            states.0.alleyway2_state.run(&mut commands, &asset_server, keyboard_input, next_state, player_query, story_query, npc_query);
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
            states.0.alleyway_state.start(&mut commands, &asset_server, &mut texture_atlases, 0.0);
        },
        State::Cyberway => {
            states.0.cyberway_state.start(&mut commands, &asset_server, &mut texture_atlases, 0.0);
        },
        State::Cafe => {
            states.0.cafe_state.start(&mut commands, &asset_server, &mut texture_atlases);
        },
        State::Pod => {
            states.0.pod_state.start(&mut commands, &asset_server, &mut texture_atlases);
        },
        State::Alleyway2 => {
            states.0.alleyway2_state.start(&mut commands, &asset_server, &mut texture_atlases);
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
                states.0.alleyway_state.start(&mut commands, &asset_server, &mut texture_atlases, 210.0);
            },
            (State::Alleyway, State::TechShop) => {
                states.0.alleyway_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::TechShop;
                states.0.tech_shop_state.start(&mut commands, &asset_server, &mut texture_atlases);
            },
            (State::Alleyway, State::Cyberway) => {
                states.0.alleyway_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::Cyberway;
                states.0.cyberway_state.start(&mut commands, &asset_server, &mut texture_atlases, -580.0);
            },
            (State::Cyberway, State::Cafe) => {
                states.0.cyberway_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::Cafe;
                states.0.cafe_state.start(&mut commands, &asset_server, &mut texture_atlases);
            },
            (State::Cyberway, State::Alleyway) => {
                states.0.cyberway_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::Alleyway;
                states.0.alleyway_state.start(&mut commands, &asset_server, &mut texture_atlases, 510.0);
            },
            (State::Cyberway, State::Alleyway2) => {
                states.0.cyberway_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::Alleyway2;
                states.0.alleyway2_state.start(&mut commands, &asset_server, &mut texture_atlases);
            },
            (State::Cafe, State::Cyberway) => {
                states.0.cafe_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::Cyberway;
                states.0.cyberway_state.spawn_x = 400.0;
                states.0.cyberway_state.start(&mut commands, &asset_server, &mut texture_atlases, -300.0);
            },
            (State::Cafe, State::Pod) => {
                states.0.cafe_state.close(&mut commands, &mut entity_query);
                current_state.0 = State::Pod;
                states.0.pod_state.start(&mut commands, &asset_server, &mut texture_atlases);
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

fn npc_collision_check(
    target_player_pos: Vec3,
    npc_query: &Query<(&NPC, &Transform), (Without<Player>, With<NPC>)>,
) -> i32 {

    for (npc, npc_trans) in npc_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(64.0),
            npc_trans.translation,
            Vec2::splat(128.0),
        );
        if collision.is_some() {
            return npc.talking_id;
        }
    }

    -1
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

fn move_player(
    keyboard_input: &Res<Input<KeyCode>>,
    sprite: &mut TextureAtlasSprite,
    timer: &mut AnimationTimer,
    player_transform: &mut Transform,
    player: &mut Player,
) -> (Vec3, bool) {
    let mut moved = false;

    if keyboard_input.pressed(KeyCode::A) {
        if timer.paused() {
            timer.unpause();
        }
        player.velocity.x = -PLAYER_SPEED;
        sprite.flip_x = true;
        moved = true;
    } else if keyboard_input.pressed(KeyCode::D) {
        if timer.paused() {
            timer.unpause();
        }
        player.velocity.x = PLAYER_SPEED;
        sprite.flip_x = false;
        moved = true;
    } else {
        player.velocity.x = 0.0;
        if timer.just_finished() {
            timer.pause();
        }
    }

    let target = Vec3::new(player_transform.translation.x + player.velocity.x, 
        player_transform.translation.y, 
        player_transform.translation.z);

    (target, moved)
}

fn spawn_story_text(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let font = asset_server.load("fonts/PressStart2P-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 16.0,
        color: Color::WHITE,
    };
    
    commands.spawn((
        TextBundle::from_section(
            "",
            text_style,
        ).with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Percent(17.0),
                bottom: Val::Percent(14.0),
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

fn spawn_player(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    x_offset: f32,
) {
    let player_texture_handle = asset_server.load("textures/player_walk.png");
    let player_texture_atlas =
        TextureAtlas::from_grid(player_texture_handle, Vec2::new(32.0, 32.0), 1, 8, None, None);
    let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);

    let mut player_transform = Transform::from_scale(Vec3::splat(5.0));
    player_transform.translation.x = x_offset;
    player_transform.translation.z = 100.0;
    let mut player_anim_timer = Timer::from_seconds(0.1, TimerMode::Repeating);
    player_anim_timer.pause();

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: player_texture_atlas_handle,
            transform: player_transform,
            ..default()
        },
        AnimationTimer(player_anim_timer),
        Player {
            velocity: Vec2::new(0.0, 0.0)
        },
    ));
}

fn spawn_robot(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    x_offset: f32,
) {
    let robot_texture_handle = asset_server.load("textures/robot.png");
    let robot_texture_atlas =
        TextureAtlas::from_grid(robot_texture_handle, Vec2::new(8.0, 8.0), 1, 4, None, None);
    let robot_texture_atlas_handle = texture_atlases.add(robot_texture_atlas);

    let mut robot_transform = Transform::from_scale(Vec3::splat(5.0));
    robot_transform.translation.x = x_offset;
    robot_transform.translation.y = 10.0;
    robot_transform.translation.z = 100.0;
    let mut robot_anim_timer = Timer::from_seconds(0.1, TimerMode::Repeating);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: robot_texture_atlas_handle,
            transform: robot_transform,
            ..default()
        },
        AnimationTimer(robot_anim_timer),
        NPC {
            talking_id: 1
        }
    ));
}

fn spawn_figure(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    x_offset: f32,
) {
    let figure_texture_handle = asset_server.load("textures/figure.png");
    let figure_texture_atlas =
        TextureAtlas::from_grid(figure_texture_handle, Vec2::new(15.0, 32.0), 1, 24, None, None);
    let figure_texture_atlas_handle = texture_atlases.add(figure_texture_atlas);

    let mut figure_transform = Transform::from_scale(Vec3::splat(5.0));
    figure_transform.translation.x = x_offset;
    figure_transform.translation.z = 80.0;
    let mut figure_anim_timer = Timer::from_seconds(0.1, TimerMode::Repeating);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: figure_texture_atlas_handle,
            transform: figure_transform,
            ..default()
        },
        AnimationTimer(figure_anim_timer),
        NPC {
            talking_id: 2
        }
    ));
}

fn spawn_background(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    state: State,
    y_offset: f32,
) {
    let mut background_transform = Transform::from_scale(Vec3::splat(5.0));
    background_transform.translation.z = 0.0;
    background_transform.translation.y = y_offset;

    let backgound_texture = match state {
        State::TechShop => asset_server.load("textures/tech_shop.png"),
        State::Alleyway => asset_server.load("textures/alleyway.png"),
        State::Cyberway => asset_server.load("textures/cyberway.png"),
        State::Cafe => asset_server.load("textures/cafe.png"),
        State::Pod => asset_server.load("textures/pods.png"),
        State::Alleyway2 => asset_server.load("textures/alleyway2.png"),
        _ => return,
    };

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            ..default()
        },
        texture: backgound_texture,
        transform: background_transform,
        ..default()
    });
}

fn spawn_text_box(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let mut text_box_transform =  Transform::from_scale(Vec3::splat(6.0));
    text_box_transform.translation.y = -250.0;
    text_box_transform.translation.z = 40.0;

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0.1, 0.0, 0.4, 1.0),
            ..default()
        },
        texture: asset_server.load("textures/text_box.png"),
        transform: text_box_transform,
        ..default()
    });

    let talking_entity = spawn_talking_entity(commands, &asset_server, 1);

    talking_entity
}

fn spawn_talking_entity (
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    current_talking: u32
) -> Entity {
    let mut talking_transform =  Transform::from_scale(Vec3::splat(6.0));
    talking_transform.translation.x = -525.0;
    talking_transform.translation.y = -250.0;
    talking_transform.translation.z = 250.0;

    let talking_texture;

    if current_talking == 0 {
        talking_texture = asset_server.load("textures/talking0.png")
    } else if current_talking == 1 {
        talking_texture = asset_server.load("textures/talking1.png")
    } else {
        talking_texture = asset_server.load("textures/talking2.png")
    }

    let talking_entity = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            ..default()
        },
        texture: talking_texture,
        transform: talking_transform,
        ..default()
    }).id();

    talking_entity
}

fn update_msg(
    current_msg: &String,
    text: &mut Text,
) -> bool {
    if text.sections[0].value.len() < current_msg.len() {
        text.sections[0].value = current_msg[..text.sections[0].value.len() + 1].to_string();
        return false;
    }

    true
}

fn clear_msg(
    current_msg: &String,
    text: &mut Text,
) {
    if text.sections[0].value.len() != 0 {
        text.sections[0].value = "".to_string();
    }
}

fn manage_dialog(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    state: State,
    current_talking: u32,

    mut talking_entity: Entity,

    mut dialog_state: u32,
    mut dialog_line: usize,

    dialog_texts: &mut Vec<String>,
    current_msg: &mut String,
) -> (u32, usize, Entity) {

    let state_str = match state {
        State::TechShop => "tech_shop",
        State::Alleyway => "alleyway",
        State::Cyberway => "cyberway",
        State::Cafe => "cafe",
        State::Pod => "pod",
        State::Alleyway2 => "alleyway2",
        _ => "PANIC",
    };
   
    if dialog_texts.len() == 0 {
        let file_path = format!("assets/texts/{}/dialog{}-{}.txt", state_str, current_talking, dialog_state);
        println!("{}", file_path);
        
        let raw_text = fs::read_to_string(file_path).expect("Cannot open dialog text!");
        let lines = raw_text.split('\n');
        for l in lines {
            dialog_texts.push(l.to_string());
        }
    }

    commands.entity(talking_entity).despawn();

    if dialog_line % 2 != 0 {
        talking_entity = spawn_talking_entity(commands, &asset_server, 0);
    } else {
        talking_entity = spawn_talking_entity(commands, &asset_server, current_talking);
    }

    if dialog_line < dialog_texts.len() {
        *current_msg = dialog_texts[dialog_line].clone();
        dialog_line += 1; 
    } else {
        *current_msg = "".to_string();
        if dialog_state < 1 {
            dialog_state += 1;
        }
    }

    (dialog_state, dialog_line, talking_entity)
}
