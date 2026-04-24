use core::marker::PhantomData;
use typeslot::prelude::*;
use typeslot::register_generic;

#[derive(SlotGroup)]
#[generic]
struct Group;

#[derive(TypeSlot)]
#[slot(Group)]
struct Wrapper<T>(PhantomData<T>);

#[derive(TypeSlot)]
#[slot(Group)]
struct Pair<A, B>(PhantomData<(A, B)>);

#[derive(TypeSlot)]
#[slot(Group)]
struct Plain;

#[test]
fn generic_derive_gets_unique_slots() {
    let count = Group::init();

    // Plain is the only non-generic; it gets slot 0 via inventory.
    assert_eq!(count, 1);
    assert_eq!(Group::slot::<Plain>(), 0);

    // Generic monomorphizations are assigned lazily starting at 1.
    let a = Group::slot::<Wrapper<u8>>();
    let b = Group::slot::<Wrapper<u16>>();
    let c = Group::slot::<Pair<u8, u16>>();

    assert_ne!(a, b);
    assert_ne!(b, c);
    assert_ne!(a, c);

    // All indices are >= 1 (after the non-generic slot).
    assert!(a >= 1);
    assert!(b >= 1);
    assert!(c >= 1);

    // Repeated calls return the same slot.
    assert_eq!(Group::slot::<Wrapper<u8>>(), a);
    assert_eq!(Group::slot::<Wrapper<u16>>(), b);
}

#[derive(SlotGroup)]
#[generic]
struct ForeignGroup;

struct ForeignType<T>(PhantomData<T>);
struct ForeignPair<A, B>(PhantomData<(A, B)>);
// A different and more dynamic way to register generic types.
register_generic!(ForeignGroup, [ForeignType<T>, ForeignPair<A, B>]);

#[test]
fn register_generic_gets_unique_slots() {
    ForeignGroup::init();

    let a = ForeignGroup::slot::<ForeignType<u8>>();
    let b = ForeignGroup::slot::<ForeignType<u16>>();
    let c = ForeignGroup::slot::<ForeignPair<u8, u16>>();

    assert_ne!(a, b);
    assert_ne!(b, c);
    assert_ne!(a, c);
    assert_eq!(ForeignGroup::slot::<ForeignType<u8>>(), a);
}

#[derive(SlotGroup)]
#[generic]
struct LenGroup;

#[derive(TypeSlot)]
#[slot(LenGroup)]
struct LenType<T>(PhantomData<T>);

#[test]
fn len_grows_as_generic_slots_are_first_accessed() {
    let static_count = LenGroup::init();

    // No non-generic types registered, so static count is 0.
    assert_eq!(static_count, 0);
    assert_eq!(LenGroup::len(), 0);

    LenGroup::slot::<LenType<u8>>();
    assert_eq!(LenGroup::len(), 1);

    LenGroup::slot::<LenType<u16>>();
    assert_eq!(LenGroup::len(), 2);

    // Re-accessing an already-seen monomorphization does not grow len.
    LenGroup::slot::<LenType<u8>>();
    assert_eq!(LenGroup::len(), 2);
}

#[test]
fn try_slot_returns_none_before_init() {
    // A separate group so init hasn't been called.
    #[derive(SlotGroup)]
    #[generic]
    struct UninitGroup;

    #[derive(TypeSlot)]
    #[slot(UninitGroup)]
    struct Gen<T: Default>(PhantomData<T>);

    assert_eq!(UninitGroup::try_slot::<Gen<u8>>(), None);

    UninitGroup::init();

    // Order is determine by "first caller" order.
    assert_eq!(UninitGroup::try_slot::<Gen<u8>>(), Some(0));
    assert_eq!(UninitGroup::try_slot::<Gen<u16>>(), Some(1));
    assert_eq!(UninitGroup::try_slot::<Gen<u32>>(), Some(2));
    assert_eq!(UninitGroup::try_slot::<Gen<u8>>(), Some(0));
}
