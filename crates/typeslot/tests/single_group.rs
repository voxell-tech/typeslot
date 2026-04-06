use typeslot::TypeSlot;

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
    assert_eq!(<Position as TypeSlot<ComponentGroup>>::slot(), None);
    assert_eq!(<Velocity as TypeSlot<ComponentGroup>>::slot(), None);
    assert_eq!(<Health as TypeSlot<ComponentGroup>>::slot(), None);

    let count = typeslot::init::<ComponentGroup>();

    assert_eq!(count, 3);
    assert!(<Position as TypeSlot<ComponentGroup>>::slot().is_some());
    assert!(<Velocity as TypeSlot<ComponentGroup>>::slot().is_some());
    assert!(<Health as TypeSlot<ComponentGroup>>::slot().is_some());

    assert_ne!(
        <Position as TypeSlot<ComponentGroup>>::slot(),
        <Velocity as TypeSlot<ComponentGroup>>::slot()
    );
    assert_ne!(
        <Velocity as TypeSlot<ComponentGroup>>::slot(),
        <Health as TypeSlot<ComponentGroup>>::slot()
    );
    assert_ne!(
        <Position as TypeSlot<ComponentGroup>>::slot(),
        <Health as TypeSlot<ComponentGroup>>::slot()
    );
}
