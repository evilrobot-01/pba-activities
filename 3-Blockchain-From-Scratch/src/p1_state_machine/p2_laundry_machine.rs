//! When you wear clothes they get dirty. When you wash them they get wet. When you dry them, they're
//! ready to be worn again. Of course washing and wearing clothes takes its toll on the clothes, and
//! eventually they get tattered.
//!
use super::StateMachine;

/// The rules are:
/// TODO
pub struct ClothesMachine;

/// Models a piece of clothing throughout its lifecycle.
#[derive(PartialEq, Eq, Debug)]
pub enum ClothesState {
    /// Clean clothes ready to be worn. With some given life left.
    Clean(u64),
    /// Dirty clothes. With some given life left.
    Dirty(u64),
    /// Wet clothes. With some given life left.
    Wet(u64),
    /// Tattered clothes beyond their useful life. Cannot be used anymore and will always be tattered.
    Tattered,
}

/// Something you can do with clothes
pub enum ClothesAction {
    Wear,
    Wash,
    Dry,
}

impl StateMachine for ClothesMachine {
    type State = ClothesState;
    type Transition = ClothesAction;

    fn next_state(starting_state: &ClothesState, t: &ClothesAction) -> ClothesState {
        match starting_state {
            ClothesState::Clean(life) => match t {
                ClothesAction::Wear => ClothesState::Dirty(life - 1),
                ClothesAction::Wash => ClothesState::Wet(life - 2),
                ClothesAction::Dry => ClothesState::Clean(life - 2),
            },
            ClothesState::Dirty(life) => match t {
                ClothesAction::Wear => ClothesState::Dirty(life - 2),
                ClothesAction::Wash => ClothesState::Wet(life - 1),
                ClothesAction::Dry => ClothesState::Dirty(life - 3),
            },
            ClothesState::Wet(life) => match t {
                ClothesAction::Wear => ClothesState::Dirty(life - 2),
                ClothesAction::Wash => ClothesState::Wet(life - 2),
                ClothesAction::Dry => ClothesState::Clean(life - 1),
            },
            ClothesState::Tattered => ClothesState::Tattered,
        }
    }
}

#[test]
fn sm_2_wear_clean_clothes() {
    let starting_state = ClothesState::Clean(3);
    assert_eq!(
        ClothesMachine::next_state(&starting_state, &ClothesAction::Wear),
        ClothesState::Dirty(2)
    )
}

#[test]
fn sm_2_wear_dirty_clothes() {
    let starting_state = ClothesState::Dirty(3);
    assert_eq!(
        ClothesMachine::next_state(&starting_state, &ClothesAction::Wear),
        ClothesState::Dirty(1)
    )
}

#[test]
fn sm_2_wear_wet_clothes() {
    let starting_state = ClothesState::Dirty(3);
    assert_eq!(
        ClothesMachine::next_state(&starting_state, &ClothesAction::Wear),
        ClothesState::Dirty(1)
    )
}

#[test]
fn sm_2_wear_tattered_clothes() {
    let starting_state = ClothesState::Tattered;
    assert_eq!(
        ClothesMachine::next_state(&starting_state, &ClothesAction::Wear),
        ClothesState::Tattered
    )
}

#[test]
fn sm_2_wash_clean_clothes() {
    let starting_state = ClothesState::Clean(5);
    assert_eq!(
        ClothesMachine::next_state(&starting_state, &ClothesAction::Wash),
        ClothesState::Wet(3)
    )
}

#[test]
fn sm_2_wash_dirty_clothes() {
    let starting_state = ClothesState::Dirty(5);
    assert_eq!(
        ClothesMachine::next_state(&starting_state, &ClothesAction::Wash),
        ClothesState::Wet(4)
    )
}

#[test]
fn sm_2_wash_wet_clothes() {
    let starting_state = ClothesState::Wet(5);
    assert_eq!(
        ClothesMachine::next_state(&starting_state, &ClothesAction::Wash),
        ClothesState::Wet(3)
    )
}

#[test]
fn sm_2_wash_tattered_clothes() {
    let starting_state = ClothesState::Tattered;
    assert_eq!(
        ClothesMachine::next_state(&starting_state, &ClothesAction::Wash),
        ClothesState::Tattered
    )
}

#[test]
fn sm_2_dry_clean_clothes() {
    let starting_state = ClothesState::Clean(3);
    assert_eq!(
        ClothesMachine::next_state(&starting_state, &ClothesAction::Dry),
        ClothesState::Clean(1)
    )
}

#[test]
fn sm_2_dry_dirty_clothes() {
    let starting_state = ClothesState::Dirty(3);
    assert_eq!(
        ClothesMachine::next_state(&starting_state, &ClothesAction::Dry),
        ClothesState::Dirty(0)
    )
}

#[test]
fn sm_2_dry_wet_clothes() {
    let starting_state = ClothesState::Wet(3);
    assert_eq!(
        ClothesMachine::next_state(&starting_state, &ClothesAction::Dry),
        ClothesState::Clean(2)
    )
}

#[test]
fn sm_2_dry_tattered_clothes() {
    let starting_state = ClothesState::Tattered;
    assert_eq!(
        ClothesMachine::next_state(&starting_state, &ClothesAction::Dry),
        ClothesState::Tattered
    )
}
