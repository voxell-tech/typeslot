use typeslot::prelude::*;

mod group_a {
    use super::*;

    #[derive(SlotGroup)]
    pub struct Group;
}
mod group_b {
    use super::*;

    #[derive(SlotGroup)]
    pub struct Group;
}

#[derive(TypeSlot)]
#[slot(group_a::Group, group_b::Group)]
struct Shared;

#[test]
fn same_name_groups_are_independent() {
    assert_eq!(group_a::Group::try_slot::<Shared>(), None);
    assert_eq!(group_b::Group::try_slot::<Shared>(), None);

    let count_a = group_a::Group::init();
    let count_b = group_b::Group::init();

    assert_eq!(count_a, 1);
    assert_eq!(count_b, 1);

    assert!(group_a::Group::try_slot::<Shared>().is_some());
    assert!(group_b::Group::try_slot::<Shared>().is_some());
}
