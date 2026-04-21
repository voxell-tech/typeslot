use typeslot::prelude::*;
use typeslot::register_typeslot;

#[derive(SlotGroup)]
struct Group;

struct Wrapper<T>(T);

register_typeslot!(Wrapper<u8>, Group);
register_typeslot!(Wrapper<u16>, Group);
register_typeslot!(Wrapper<u32>, Group);

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

    assert_ne!(Group::slot::<Wrapper<u8>>(), Group::slot::<Wrapper<u16>>());
    assert_ne!(Group::slot::<Wrapper<u16>>(), Group::slot::<Wrapper<u32>>());
    assert_ne!(Group::slot::<Wrapper<u8>>(), Group::slot::<Wrapper<u32>>());
}
