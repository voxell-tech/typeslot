mod group_a {
    pub struct Group;
}
mod group_b {
    pub struct Group;
}

#[derive(typeslot::TypeSlot)]
#[slot(group_a::Group, group_b::Group)]
struct Shared;

#[test]
fn same_name_groups_are_independent() {
    use typeslot::TypeSlot;
    assert_eq!(<Shared as TypeSlot<group_a::Group>>::slot(), None);
    assert_eq!(<Shared as TypeSlot<group_b::Group>>::slot(), None);
    let count_a = typeslot::init::<group_a::Group>();
    let count_b = typeslot::init::<group_b::Group>();

    assert_eq!(count_a, 1);
    assert_eq!(count_b, 1);

    assert!(<Shared as TypeSlot<group_a::Group>>::slot().is_some());
    assert!(<Shared as TypeSlot<group_b::Group>>::slot().is_some());
}
