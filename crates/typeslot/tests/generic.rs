use typeslot::prelude::*;
use typeslot::register;

#[derive(SlotGroup)]
struct Group;

struct Wrapper<T>(T);

register!(Group, [Wrapper<u8>, Wrapper<u16>, Wrapper<u32>]);

#[test]
fn generic_monomorphizations_get_unique_slots() {
    assert_eq!(Group::try_slot::<Wrapper<u8>>(), None);
    assert_eq!(Group::try_slot::<Wrapper<u16>>(), None);
    assert_eq!(Group::try_slot::<Wrapper<u32>>(), None);

    let count = Group::init();

    assert_eq!(count, 3);
    assert!(Group::try_slot::<Wrapper<u8>>().is_some());
    assert!(Group::try_slot::<Wrapper<u16>>().is_some());
    assert!(Group::try_slot::<Wrapper<u32>>().is_some());

    assert_ne!(
        Group::slot::<Wrapper<u8>>(),
        Group::slot::<Wrapper<u16>>()
    );
    assert_ne!(
        Group::slot::<Wrapper<u16>>(),
        Group::slot::<Wrapper<u32>>()
    );
    assert_ne!(
        Group::slot::<Wrapper<u8>>(),
        Group::slot::<Wrapper<u32>>()
    );
}
