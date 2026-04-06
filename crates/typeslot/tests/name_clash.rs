use typeslot::prelude::*;

mod group_a {
    pub struct Group;
}
mod group_b {
    pub struct Group;
}

#[derive(TypeSlot)]
#[slot(group_a::Group, group_b::Group)]
struct Shared;

#[test]
fn same_name_groups_are_independent() {
    let a = SlotGroup::<group_a::Group>::new();
    let b = SlotGroup::<group_b::Group>::new();

    assert_eq!(a.try_get::<Shared>(), None);
    assert_eq!(b.try_get::<Shared>(), None);

    let count_a = init_slot::<group_a::Group>();
    let count_b = init_slot::<group_b::Group>();

    assert_eq!(count_a, 1);
    assert_eq!(count_b, 1);

    assert!(a.try_get::<Shared>().is_some());
    assert!(b.try_get::<Shared>().is_some());
}
