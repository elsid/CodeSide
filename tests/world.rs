mod helpers;

use model::JumpState;

use helpers::{
    updated_world,
    with_my_jump_state,
};

use aicup2019::{
    examples::{
        example_properties,
        example_world,
    },
    my_strategy::{
        Location,
        change_jump_state,
    }
};

#[test]
fn test_find_tiles_path() {
    let properties = example_properties();
    let world = updated_world(with_my_jump_state(example_world(), JumpState {
        can_jump: true,
        speed: properties.unit_jump_speed,
        max_time: properties.unit_jump_time,
        can_cancel: true,
    }));
    assert_eq!(
        world.find_reversed_tiles_path(Location::new(37, 1), Location::new(29, 5)),
        vec![
            Location::new(29, 5), Location::new(30, 5), Location::new(31, 5), Location::new(32, 4),
            Location::new(33, 3), Location::new(34, 2), Location::new(35, 1), Location::new(36, 1),
        ]
    );
}

#[test]
fn test_find_shortcut_tiles_path_with_obstacles() {
    let properties = example_properties();
    let world = updated_world(with_my_jump_state(example_world(), JumpState {
        can_jump: true,
        speed: properties.unit_jump_speed,
        max_time: properties.unit_jump_time,
        can_cancel: true,
    }));
    assert_eq!(
        world.find_shortcut_tiles_path(Location::new(37, 1), Location::new(29, 5)),
        vec![Location::new(31, 5), Location::new(29, 5)]
    );
}

#[test]
fn test_find_shortcut_tiles_path_direct() {
    let properties = example_properties();
    let world = updated_world(with_my_jump_state(example_world(), JumpState {
        can_jump: true,
        speed: properties.unit_jump_speed,
        max_time: properties.unit_jump_time,
        can_cancel: true,
    }));
    assert_eq!(
        world.find_shortcut_tiles_path(Location::new(37, 1), Location::new(37, 5)),
        vec![Location::new(37, 5)]
    );
}

#[test]
fn test_find_shortcut_tiles_path_to_neighbor() {
    let properties = example_properties();
    let world = updated_world(with_my_jump_state(example_world(), JumpState {
        can_jump: true,
        speed: properties.unit_jump_speed,
        max_time: properties.unit_jump_time,
        can_cancel: true,
    }));
    assert_eq!(
        world.find_shortcut_tiles_path(Location::new(37, 1), Location::new(37, 2)),
        vec![Location::new(37, 2)]
    );
}

#[test]
fn test_find_shortcut_tiles_path_to_the_same_tile() {
    let properties = example_properties();
    let world = updated_world(with_my_jump_state(example_world(), JumpState {
        can_jump: true,
        speed: properties.unit_jump_speed,
        max_time: properties.unit_jump_time,
        can_cancel: true,
    }));
    assert_eq!(
        world.find_shortcut_tiles_path(Location::new(37, 1), Location::new(37, 1)),
        vec![]
    );
}

#[test]
fn test_find_shortcut_tiles_path_over_jump_pad() {
    let properties = example_properties();
    let world = updated_world(with_my_jump_state(example_world(), JumpState {
        can_jump: true,
        speed: properties.unit_jump_speed,
        max_time: properties.unit_jump_time,
        can_cancel: true,
    }));
    assert_eq!(
        world.find_shortcut_tiles_path(Location::new(37, 1), Location::new(29, 1)),
        vec![Location::new(33, 1), Location::new(29, 1)]
    );
}

#[test]
fn test_change_jump_state_fall_to_fall() {
    {
        let result = change_jump_state(
            &JumpState {can_jump: false, speed: 0.0, max_time: 0.0, can_cancel: false},
            &JumpState {can_jump: false, speed: 0.0, max_time: 0.0, can_cancel: false},
            0.1
        );
        assert_eq!((result.can_jump, result.speed, result.max_time, result.can_cancel), (false, 0.0, 0.0, false));
    }
}

#[test]
fn test_change_jump_state_fall_to_jump() {
    {
        let result = change_jump_state(
            &JumpState {can_jump: false, speed: 0.0, max_time: 0.0, can_cancel: false},
            &JumpState {can_jump: true, speed: 1.0, max_time: 1.0, can_cancel: true},
            0.1
        );
        assert_eq!((result.can_jump, result.speed, result.max_time, result.can_cancel), (true, 1.0, 1.0, true));
    }
}

#[test]
fn test_change_jump_state_fall_to_pad() {
    {
        let result = change_jump_state(
            &JumpState {can_jump: false, speed: 0.0, max_time: 0.0, can_cancel: false},
            &JumpState {can_jump: true, speed: 2.0, max_time: 0.5, can_cancel: false},
            0.1
        );
        assert_eq!((result.can_jump, result.speed, result.max_time, result.can_cancel), (true, 2.0, 0.5, false));
    }
}

#[test]
fn test_change_jump_state_jump_to_jump() {
    {
        let result = change_jump_state(
            &JumpState {can_jump: true, speed: 1.0, max_time: 1.0, can_cancel: true},
            &JumpState {can_jump: true, speed: 1.0, max_time: 1.0, can_cancel: true},
            0.1
        );
        assert_eq!((result.can_jump, result.speed, result.max_time, result.can_cancel), (true, 1.0, 1.0, true));
    }
}

#[test]
fn test_change_jump_state_jump_to_fall_have_time() {
    {
        let result = change_jump_state(
            &JumpState {can_jump: true, speed: 1.0, max_time: 1.0, can_cancel: true},
            &JumpState {can_jump: false, speed: 0.0, max_time: 0.0, can_cancel: false},
            0.1
        );
        assert_eq!((result.can_jump, result.speed, result.max_time, result.can_cancel), (true, 1.0, 0.9, true));
    }
}

#[test]
fn test_change_jump_state_jump_to_fall_no_time() {
    {
        let result = change_jump_state(
            &JumpState {can_jump: true, speed: 1.0, max_time: 0.1, can_cancel: true},
            &JumpState {can_jump: false, speed: 0.0, max_time: 0.0, can_cancel: false},
            0.1
        );
        assert_eq!((result.can_jump, result.speed, result.max_time, result.can_cancel), (false, 0.0, 0.0, false));
    }
}

#[test]
fn test_change_jump_state_jump_to_pad() {
    {
        let result = change_jump_state(
            &JumpState {can_jump: true, speed: 1.0, max_time: 1.0, can_cancel: true},
            &JumpState {can_jump: true, speed: 2.0, max_time: 0.5, can_cancel: false},
            0.1
        );
        assert_eq!((result.can_jump, result.speed, result.max_time, result.can_cancel), (true, 2.0, 0.5, false));
    }
}

#[test]
fn test_change_jump_state_pad_to_pad() {
    {
        let result = change_jump_state(
            &JumpState {can_jump: true, speed: 2.0, max_time: 0.5, can_cancel: false},
            &JumpState {can_jump: true, speed: 2.0, max_time: 0.5, can_cancel: false},
            0.1
        );
        assert_eq!((result.can_jump, result.speed, result.max_time, result.can_cancel), (true, 2.0, 0.5, false));
    }
}

#[test]
fn test_change_jump_state_pad_to_jump_have_time() {
    {
        let result = change_jump_state(
            &JumpState {can_jump: true, speed: 2.0, max_time: 0.5, can_cancel: false},
            &JumpState {can_jump: true, speed: 1.0, max_time: 1.0, can_cancel: true},
            0.1
        );
        assert_eq!((result.can_jump, result.speed, result.max_time, result.can_cancel), (true, 2.0, 0.4, false));
    }
}

#[test]
fn test_change_jump_state_pad_to_jump_no_time() {
    {
        let result = change_jump_state(
            &JumpState {can_jump: true, speed: 2.0, max_time: 0.1, can_cancel: false},
            &JumpState {can_jump: true, speed: 1.0, max_time: 1.0, can_cancel: true},
            0.1
        );
        assert_eq!((result.can_jump, result.speed, result.max_time, result.can_cancel), (true, 1.0, 1.0, true));
    }
}

#[test]
fn test_change_jump_state_pad_to_fall_have_time() {
    {
        let result = change_jump_state(
            &JumpState {can_jump: true, speed: 2.0, max_time: 0.5, can_cancel: false},
            &JumpState {can_jump: false, speed: 0.0, max_time: 0.0, can_cancel: false},
            0.1
        );
        assert_eq!((result.can_jump, result.speed, result.max_time, result.can_cancel), (true, 2.0, 0.4, false));
    }
}

#[test]
fn test_change_jump_state_pad_to_fall_no_time() {
    {
        let result = change_jump_state(
            &JumpState {can_jump: true, speed: 2.0, max_time: 0.1, can_cancel: false},
            &JumpState {can_jump: false, speed: 0.0, max_time: 0.0, can_cancel: false},
            0.1
        );
        assert_eq!((result.can_jump, result.speed, result.max_time, result.can_cancel), (false, 0.0, 0.0, false));
    }
}
