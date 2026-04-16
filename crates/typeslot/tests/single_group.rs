use typeslot::prelude::*;

#[derive(SlotGroup)]
struct EnemyGroup;

#[derive(TypeSlot)]
#[slot(EnemyGroup)]
struct Orc;

#[derive(TypeSlot)]
#[slot(EnemyGroup)]
struct Goblin;

#[derive(TypeSlot)]
#[slot(EnemyGroup)]
struct Troll;

#[test]
fn unique_slot_indices() {
    assert_eq!(EnemyGroup::try_slot::<Orc>(), None);
    assert_eq!(EnemyGroup::try_slot::<Goblin>(), None);
    assert_eq!(EnemyGroup::try_slot::<Troll>(), None);

    let count = EnemyGroup::init();

    assert_eq!(count, 3);
    assert!(EnemyGroup::try_slot::<Orc>().is_some());
    assert!(EnemyGroup::try_slot::<Goblin>().is_some());
    assert!(EnemyGroup::try_slot::<Troll>().is_some());

    assert_ne!(
        EnemyGroup::slot::<Orc>(),
        EnemyGroup::slot::<Goblin>()
    );
    assert_ne!(
        EnemyGroup::slot::<Goblin>(),
        EnemyGroup::slot::<Troll>()
    );
    assert_ne!(
        EnemyGroup::slot::<Orc>(),
        EnemyGroup::slot::<Troll>()
    );
}
