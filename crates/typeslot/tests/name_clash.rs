use typeslot::prelude::*;

mod enemy {
    use super::*;

    #[derive(SlotGroup)]
    pub struct Group;
}
mod boss {
    use super::*;

    #[derive(SlotGroup)]
    pub struct Group;
}

#[derive(TypeSlot)]
#[slot(enemy::Group, boss::Group)]
struct Dragon;

#[test]
fn same_name_groups_are_independent() {
    assert_eq!(enemy::Group::try_slot::<Dragon>(), None);
    assert_eq!(boss::Group::try_slot::<Dragon>(), None);

    let enemy_count = enemy::Group::init();
    let boss_count = boss::Group::init();

    assert_eq!(enemy_count, 1);
    assert_eq!(boss_count, 1);

    assert!(enemy::Group::try_slot::<Dragon>().is_some());
    assert!(boss::Group::try_slot::<Dragon>().is_some());
}
