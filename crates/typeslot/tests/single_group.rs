use typeslot::prelude::*;

struct ComponentGroup;

#[derive(TypeSlot)]
#[slot(ComponentGroup)]
struct Position;

#[derive(TypeSlot)]
#[slot(ComponentGroup)]
struct Velocity;

#[derive(TypeSlot)]
#[slot(ComponentGroup)]
struct Health;

#[test]
fn unique_slot_indices() {
    let group = SlotGroup::<ComponentGroup>::new();

    assert_eq!(group.try_get::<Position>(), None);
    assert_eq!(group.try_get::<Velocity>(), None);
    assert_eq!(group.try_get::<Health>(), None);

    let count = init_slot::<ComponentGroup>();

    assert_eq!(count, 3);
    assert!(group.try_get::<Position>().is_some());
    assert!(group.try_get::<Velocity>().is_some());
    assert!(group.try_get::<Health>().is_some());

    assert_ne!(group.get::<Position>(), group.get::<Velocity>());
    assert_ne!(group.get::<Velocity>(), group.get::<Health>());
    assert_ne!(group.get::<Position>(), group.get::<Health>());
}
