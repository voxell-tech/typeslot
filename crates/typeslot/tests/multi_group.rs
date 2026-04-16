use typeslot::prelude::*;

#[derive(SlotGroup)]
struct EnemyGroup;

#[derive(SlotGroup)]
struct BossGroup;

#[derive(TypeSlot)]
#[slot(EnemyGroup)]
struct Orc;

#[derive(TypeSlot)]
#[slot(BossGroup)]
struct Knight;

#[derive(TypeSlot)]
#[slot(EnemyGroup, BossGroup)]
struct Dragon;

#[test]
fn unique_slot_indices_per_group() {
    assert_eq!(EnemyGroup::try_slot::<Orc>(), None);
    assert_eq!(BossGroup::try_slot::<Knight>(), None);
    assert_eq!(EnemyGroup::try_slot::<Dragon>(), None);
    assert_eq!(BossGroup::try_slot::<Dragon>(), None);

    let enemy_count = EnemyGroup::init();
    let boss_count = BossGroup::init();

    assert_eq!(enemy_count, 2); // Orc, Dragon
    assert_eq!(boss_count, 2); // Knight, Dragon

    assert!(EnemyGroup::try_slot::<Orc>().is_some());
    assert!(BossGroup::try_slot::<Knight>().is_some());
    assert!(EnemyGroup::try_slot::<Dragon>().is_some());
    assert!(BossGroup::try_slot::<Dragon>().is_some());

    assert_ne!(
        EnemyGroup::slot::<Orc>(),
        EnemyGroup::slot::<Dragon>()
    );
    assert_ne!(
        BossGroup::slot::<Knight>(),
        BossGroup::slot::<Dragon>()
    );
}
