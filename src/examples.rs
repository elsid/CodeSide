use model::{
    Game,
    Level,
    LootBox,
    Player,
    Properties,
    Unit,
    Vec2F64,
};
use crate::my_strategy::{
    Config,
    SeedableRng,
    World,
    XorShiftRng,
};

pub const EXAMPLE_MY_PLAYER_ID: i32 = 11;
pub const EXAMPLE_OPPONENT_PLAYER_ID: i32 = 12;
pub const EXAMPLE_MY_UNIT_ID: i32 = 101;
pub const EXAMPLE_MY_UNIT_ID_1: i32 = 102;
pub const EXAMPLE_OPPONENT_UNIT_ID: i32 = 111;
pub const EXAMPLE_OPPONENT_UNIT_ID_1: i32 = 112;

pub fn example_rng(seed: u64) -> XorShiftRng {
    XorShiftRng::from_seed([
        seed as u32,
        (seed as u64 >> 32) as u32,
        1841971383,
        1904458926,
    ])
}

pub fn example_world() -> World {
    example_world_with_team_size(1)
}

pub fn example_world_with_team_size(team_size: i32) -> World {
    let properties = example_properties_with_team_size(team_size);
    let my_player = example_my_player();
    let opponent = example_opponent();
    let player_id = my_player.id;
    let config = Config::new().adjusted(properties.team_size);
    let game = example_game(my_player, opponent, properties);
    World::new(config, player_id, game)
}

pub fn example_properties() -> Properties {
    example_properties_with_team_size(1)
}

pub fn example_properties_with_team_size(team_size: i32) -> Properties {
    use model::{
        BulletParams,
        ExplosionParams,
        WeaponParams,
        WeaponType::*,
    };

    Properties {
        max_tick_count: 3600,
        team_size,
        ticks_per_second: 60.0,
        updates_per_tick: 100,
        loot_box_size: Vec2F64 { x: 0.5, y: 0.5 },
        unit_size: Vec2F64 { x: 0.9, y: 1.8 },
        unit_max_horizontal_speed: 10.0,
        unit_fall_speed: 10.0,
        unit_jump_time: 0.55,
        unit_jump_speed: 10.0,
        jump_pad_jump_time: 0.525,
        jump_pad_jump_speed: 20.0,
        unit_max_health: 100,
        health_pack_health: 50,
        weapon_params: [
            (
                RocketLauncher,
                WeaponParams {
                    magazine_size: 1,
                    fire_rate: 1.0,
                    reload_time: 1.0,
                    min_spread: 0.1,
                    max_spread: 0.5,
                    recoil: 1.0,
                    aim_speed: 1.0,
                    bullet: BulletParams {
                        speed: 20.0,
                        size: 0.4,
                        damage: 30,
                    },
                    explosion: Some(ExplosionParams {
                        radius: 3.0,
                        damage: 50,
                    }),
                },
            ),
            (
                Pistol,
                WeaponParams {
                    magazine_size: 8,
                    fire_rate: 0.4,
                    reload_time: 1.0,
                    min_spread: 0.05,
                    max_spread: 0.5,
                    recoil: 0.5,
                    aim_speed: 1.0,
                    bullet: BulletParams {
                        speed: 50.0,
                        size: 0.2,
                        damage: 20,
                    },
                    explosion: None,
                },
            ),
            (
                AssaultRifle,
                WeaponParams {
                    magazine_size: 20,
                    fire_rate: 0.1,
                    reload_time: 1.0,
                    min_spread: 0.1,
                    max_spread: 0.5,
                    recoil: 0.2,
                    aim_speed: 1.9,
                    bullet: BulletParams {
                        speed: 50.0,
                        size: 0.2,
                        damage: 5,
                    },
                    explosion: None,
                },
            ),
        ].iter().cloned().collect(),
        mine_size: Vec2F64 { x: 0.5, y: 0.5 },
        mine_explosion_params: ExplosionParams {
            radius: 3.0,
            damage: 50,
        },
        mine_prepare_time: 1.0,
        mine_trigger_time: 0.5,
        mine_trigger_radius: 1.0,
        kill_score: 1000,
    }
}

pub fn example_my_player() -> Player {
    Player {
        id: EXAMPLE_MY_PLAYER_ID,
        score: 0
    }
}

pub fn example_opponent() -> Player {
    Player {
        id: EXAMPLE_OPPONENT_PLAYER_ID,
        score: 0
    }
}

pub fn example_game(my_player: Player, opponent: Player, properties: Properties) -> Game {
    use model::{
        WeaponType::*,
        Item::*,
    };

    Game {
        current_tick: 0,
        level: example_level(),
        units: match properties.team_size {
            1 => vec![
                example_my_unit(&my_player, &properties),
                example_opponent_unit(&opponent, &properties),
            ],
            2 => vec![
                example_my_unit(&my_player, &properties),
                example_my_unit_1(&my_player, &properties),
                example_opponent_unit(&opponent, &properties),
                example_opponent_unit_1(&opponent, &properties),
            ],
            _ => Vec::new(),
        },
        players: vec![my_player, opponent],
        bullets: vec![],
        mines: vec![],
        loot_boxes: vec![
            LootBox {
                position: Vec2F64 { x: 10.5, y: 22.0 },
                size: properties.loot_box_size.clone(),
                item: Weapon {
                    weapon_type: Pistol,
                },
            },
            LootBox {
                position: Vec2F64 { x: 10.5, y: 1.0 },
                size: properties.loot_box_size.clone(),
                item: Weapon {
                    weapon_type: Pistol,
                },
            },
            LootBox {
                position: Vec2F64 { x: 29.5, y: 1.0 },
                size: properties.loot_box_size.clone(),
                item: Weapon {
                    weapon_type: Pistol,
                },
            },
            LootBox {
                position: Vec2F64 { x: 16.5, y: 22.0 },
                size: properties.loot_box_size.clone(),
                item: Weapon {
                    weapon_type: AssaultRifle,
                },
            },
            LootBox {
                position: Vec2F64 { x: 23.5, y: 22.0 },
                size: properties.loot_box_size.clone(),
                item: Weapon {
                    weapon_type: AssaultRifle,
                },
            },
            LootBox {
                position: Vec2F64 { x: 17.5, y: 9.0 },
                size: properties.loot_box_size.clone(),
                item: Weapon {
                    weapon_type: RocketLauncher,
                },
            },
            LootBox {
                position: Vec2F64 { x: 22.5, y: 9.0 },
                size: properties.loot_box_size.clone(),
                item: Weapon {
                    weapon_type: RocketLauncher,
                },
            },
            LootBox {
                position: Vec2F64 { x: 16.5, y: 26.0 },
                size: properties.loot_box_size.clone(),
                item: HealthPack { health: properties.health_pack_health },
            },
            LootBox {
                position: Vec2F64 { x: 23.5, y: 26.0 },
                size: properties.loot_box_size.clone(),
                item: HealthPack { health: properties.health_pack_health },
            },
            LootBox {
                position: Vec2F64 { x: 9.5, y: 22.0 },
                size: properties.loot_box_size.clone(),
                item: HealthPack { health: properties.health_pack_health },
            },
            LootBox {
                position: Vec2F64 { x: 30.5, y: 22.0 },
                size: properties.loot_box_size.clone(),
                item: HealthPack { health: properties.health_pack_health },
            },
            LootBox {
                position: Vec2F64 { x: 17.5, y: 22.0 },
                size: properties.loot_box_size.clone(),
                item: Mine {},
            },
            LootBox {
                position: Vec2F64 { x: 22.5, y: 22.0 },
                size: properties.loot_box_size.clone(),
                item: Mine {},
            },
            LootBox {
                position: Vec2F64 { x: 18.5, y: 9.0 },
                size: properties.loot_box_size.clone(),
                item: HealthPack { health: properties.health_pack_health },
            },
            LootBox {
                position: Vec2F64 { x: 21.5, y: 9.0 },
                size: properties.loot_box_size.clone(),
                item: HealthPack { health: properties.health_pack_health },
            },
            LootBox {
                position: Vec2F64 { x: 16.5, y: 5.0 },
                size: properties.loot_box_size.clone(),
                item: HealthPack { health: properties.health_pack_health },
            },
            LootBox {
                position: Vec2F64 { x: 23.5, y: 5.0 },
                size: properties.loot_box_size.clone(),
                item: HealthPack { health: properties.health_pack_health },
            },
            LootBox {
                position: Vec2F64 { x: 14.5, y: 1.0 },
                size: properties.loot_box_size.clone(),
                item: Mine {},
            },
            LootBox {
                position: Vec2F64 { x: 25.5, y: 1.0 },
                size: properties.loot_box_size.clone(),
                item: Mine {},
            },
        ],
        properties: properties,
    }
}

pub fn example_my_unit(player: &Player, properties: &Properties) -> Unit {
    use model::JumpState;

    Unit {
        player_id: player.id,
        id: EXAMPLE_MY_UNIT_ID,
        health: properties.unit_max_health,
        position: Vec2F64 {
            x: 37.5,
            y: 1.0,
        },
        size: properties.unit_size.clone(),
        jump_state: JumpState {
            can_jump: false,
            speed: 0.0,
            max_time: 0.0,
            can_cancel: false,
        },
        walked_right: false,
        stand: true,
        on_ground: false,
        on_ladder: false,
        mines: 0,
        weapon: None,
    }
}

pub fn example_my_unit_1(player: &Player, properties: &Properties) -> Unit {
    use model::JumpState;

    Unit {
        player_id: player.id,
        id: EXAMPLE_MY_UNIT_ID_1,
        health: properties.unit_max_health,
        position: Vec2F64 {
            x: 36.5,
            y: 1.0,
        },
        size: properties.unit_size.clone(),
        jump_state: JumpState {
            can_jump: false,
            speed: 0.0,
            max_time: 0.0,
            can_cancel: false,
        },
        walked_right: false,
        stand: true,
        on_ground: false,
        on_ladder: false,
        mines: 0,
        weapon: None,
    }
}

pub fn example_opponent_unit(player: &Player, properties: &Properties) -> Unit {
    use model::JumpState;

    Unit {
        player_id: player.id,
        id: EXAMPLE_OPPONENT_UNIT_ID,
        health: properties.unit_max_health,
        position: Vec2F64 {
            x: 2.5,
            y: 1.0,
        },
        size: properties.unit_size.clone(),
        jump_state: JumpState {
            can_jump: false,
            speed: 0.0,
            max_time: 0.0,
            can_cancel: false,
        },
        walked_right: true,
        stand: true,
        on_ground: false,
        on_ladder: false,
        mines: 0,
        weapon: None,
    }
}

pub fn example_opponent_unit_1(player: &Player, properties: &Properties) -> Unit {
    use model::JumpState;

    Unit {
        player_id: player.id,
        id: EXAMPLE_OPPONENT_UNIT_ID_1,
        health: properties.unit_max_health,
        position: Vec2F64 {
            x: 3.5,
            y: 1.0,
        },
        size: properties.unit_size.clone(),
        jump_state: JumpState {
            can_jump: false,
            speed: 0.0,
            max_time: 0.0,
            can_cancel: false,
        },
        walked_right: true,
        stand: true,
        on_ground: false,
        on_ladder: false,
        mines: 0,
        weapon: None,
    }
}

pub fn example_level() -> Level {
    use model::Tile::*;

    Level {
        tiles: vec![
            vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
            vec![Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Wall, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Ladder, Ladder, Ladder, Ladder, Empty, Empty, Wall, Empty, Empty, Empty, Wall, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Wall, JumpPad, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Ladder, Ladder, Ladder, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Wall, Wall, Wall, Wall, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Wall, Wall, Wall, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall, Ladder, Ladder, Ladder, Ladder, Empty, Empty, Empty, Wall],
            vec![Wall, JumpPad, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall],
            vec![Wall, Wall, Empty, Empty, Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Wall, Empty, Wall, Wall],
            vec![Wall, Wall, Empty, Empty, Platform, Empty, Empty, Empty, Wall, JumpPad, Empty, Empty, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Wall, Wall, Wall, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall],
            vec![Wall, Ladder, Ladder, Ladder, Platform, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Wall, Wall, Wall, Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Platform, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Ladder, Ladder, Ladder, Ladder, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, JumpPad, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall],
            vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall],
        ]
    }
}
