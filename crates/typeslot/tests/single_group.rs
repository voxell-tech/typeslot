use typeslot::prelude::*;

#[derive(SlotGroup)]
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
    assert_eq!(ComponentGroup::try_slot::<Position>(), None);
    assert_eq!(ComponentGroup::try_slot::<Velocity>(), None);
    assert_eq!(ComponentGroup::try_slot::<Health>(), None);

    let count = init_slot::<ComponentGroup>();

    assert_eq!(count, 3);
    assert!(ComponentGroup::try_slot::<Position>().is_some());
    assert!(ComponentGroup::try_slot::<Velocity>().is_some());
    assert!(ComponentGroup::try_slot::<Health>().is_some());

    assert_ne!(
        ComponentGroup::slot::<Position>(),
        ComponentGroup::slot::<Velocity>()
    );
    assert_ne!(
        ComponentGroup::slot::<Velocity>(),
        ComponentGroup::slot::<Health>()
    );
    assert_ne!(
        ComponentGroup::slot::<Position>(),
        ComponentGroup::slot::<Health>()
    );
}
