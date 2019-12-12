use model::{
    Bullet,
    Game,
    Level,
    LootBox,
    Mine,
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

pub fn example_rng(seed: u64) -> XorShiftRng {
    XorShiftRng::from_seed([
        seed as u32,
        (seed as u64 >> 32) as u32,
        1841971383,
        1904458926,
    ])
}

pub fn example_world() -> World {
    let properties = example_properties();
    World::new(Config::new(), example_me(&properties), example_game(properties))
}

pub fn example_properties() -> Properties {
    use model::{
        BulletParams,
        ExplosionParams,
        WeaponParams,
        WeaponType::*,
    };

    Properties {
        max_tick_count: 3600,
        team_size: 1,
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

pub fn example_game(properties: Properties) -> Game {
    use model::{
        WeaponType::*,
        Item::*,
    };

    Game {
        current_tick: 0,
        players: vec![
            Player {
                id: 1,
                score: 0
            },
            Player {
                id: 3,
                score: 0
            },
        ],
        level: example_level(),
        units: vec![
            example_me(&properties),
            example_opponent_1(&properties),
        ],
        bullets: vec![
            example_bullet(&properties),
        ],
        mines: vec![
            example_mine(&properties),
        ],
        loot_boxes: vec![
            example_loot_box_weapon(&properties),
            LootBox {
                position: Vec2F64 { x: 10.5, y: 1.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: Weapon {
                    weapon_type: Pistol,
                },
            },
            LootBox {
                position: Vec2F64 { x: 29.5, y: 1.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: Weapon {
                    weapon_type: Pistol,
                },
            },
            LootBox {
                position: Vec2F64 { x: 16.5, y: 22.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: Weapon {
                    weapon_type: AssaultRifle,
                },
            },
            LootBox {
                position: Vec2F64 { x: 23.5, y: 22.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: Weapon {
                    weapon_type: AssaultRifle,
                },
            },
            LootBox {
                position: Vec2F64 { x: 17.5, y: 9.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: Weapon {
                    weapon_type: RocketLauncher,
                },
            },
            LootBox {
                position: Vec2F64 { x: 22.5, y: 9.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: Weapon {
                    weapon_type: RocketLauncher,
                },
            },
            LootBox {
                position: Vec2F64 { x: 16.5, y: 26.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: HealthPack { health: 50 },
            },
            LootBox {
                position: Vec2F64 { x: 23.5, y: 26.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: HealthPack { health: 50 },
            },
            LootBox {
                position: Vec2F64 { x: 9.5, y: 22.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: HealthPack { health: 50 },
            },
            LootBox {
                position: Vec2F64 { x: 30.5, y: 22.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: HealthPack { health: 50 },
            },
            LootBox {
                position: Vec2F64 { x: 17.5, y: 22.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: Mine {},
            },
            LootBox {
                position: Vec2F64 { x: 22.5, y: 22.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: Mine {},
            },
            LootBox {
                position: Vec2F64 { x: 18.5, y: 9.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: HealthPack { health: 50 },
            },
            LootBox {
                position: Vec2F64 { x: 21.5, y: 9.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: HealthPack { health: 50 },
            },
            LootBox {
                position: Vec2F64 { x: 16.5, y: 5.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: HealthPack { health: 50 },
            },
            LootBox {
                position: Vec2F64 { x: 23.5, y: 5.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: HealthPack { health: 50 },
            },
            LootBox {
                position: Vec2F64 { x: 14.5, y: 1.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: Mine {},
            },
            LootBox {
                position: Vec2F64 { x: 25.5, y: 1.0 },
                size: Vec2F64 { x: 0.5, y: 0.5 },
                item: Mine {},
            },
        ],
        properties: properties,
    }
}

pub fn example_me(properties: &Properties) -> Unit {
    use model::JumpState;

    Unit {
        player_id: 3,
        id: 4,
        health: 100,
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

pub fn example_opponent_1(properties: &Properties) -> Unit {
    use model::JumpState;

    Unit {
        player_id: 1,
        id: 2,
        health: 100,
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

pub fn example_bullet(properties: &Properties) -> Bullet {
    use model::WeaponType::AssaultRifle;
    let params = &properties.weapon_params.get(&AssaultRifle).unwrap();

    Bullet {
        weapon_type: AssaultRifle,
        unit_id: 2,
        player_id: 1,
        position: Vec2F64 {
            x: 15.832623548153254,
            y: 5.93438708445076,
        },
        velocity: Vec2F64 {
            x: params.bullet.speed,
            y: 0.0,
        },
        damage: params.bullet.damage,
        size: params.bullet.size,
        explosion_params: params.explosion.clone(),
    }
}

pub fn example_mine(properties: &Properties) -> Mine {
    use model::MineState::Preparing;

    Mine {
        player_id: 1,
        position: Vec2F64 {
            x: 25.716666665660146,
            y: 9.000000000999998,
        },
        size: properties.mine_size.clone(),
        state: Preparing,
        timer: Some(properties.mine_prepare_time),
        trigger_radius: properties.mine_trigger_radius,
        explosion_params: properties.mine_explosion_params.clone(),
    }
}

pub fn example_loot_box_weapon(properties: &Properties) -> LootBox {
    use model::Item::Weapon;
    use model::WeaponType::Pistol;

    LootBox {
        position: Vec2F64 {
            x: 10.5,
            y: 22.0,
        },
        size: properties.loot_box_size.clone(),
        item: Weapon {
            weapon_type: Pistol,
        },
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
