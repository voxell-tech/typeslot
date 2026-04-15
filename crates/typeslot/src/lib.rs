#![doc = include_str!("../README.md")]
#![no_std]

use core::any::TypeId;
use core::marker::PhantomData;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;

pub use typeslot_macros::TypeSlot;

#[doc(hidden)]
pub use inventory;

pub mod prelude {
    pub use crate::{SlotGroup, TypeSlot, init_slot, slot, try_slot};
}

/// A write-once slot for a `usize` index.
///
/// Wraps an `AtomicUsize` with `usize::MAX` as the
/// uninitialized sentinel.
pub struct AtomicSlot(AtomicUsize);

impl AtomicSlot {
    /// Creates a new, uninitialized slot.
    pub const fn new() -> Self {
        Self(AtomicUsize::new(usize::MAX))
    }

    /// Returns the slot index, or `None` if not yet set.
    pub fn get(&self) -> Option<usize> {
        let val = self.0.load(Ordering::Acquire);
        (val != usize::MAX).then_some(val)
    }

    /// Sets the slot index.
    ///
    /// # Panics
    ///
    /// Panics if called more than once.
    pub fn set(&self, index: usize) {
        self.0
            .compare_exchange(
                usize::MAX,
                index,
                Ordering::Release,
                Ordering::Relaxed,
            )
            .expect("`AtomicSlot::set` called twice");
    }
}

impl Default for AtomicSlot {
    fn default() -> Self {
        Self::new()
    }
}

/// Registration entry for a type that implements
/// [`TypeSlot<G>`] within group `G`.
///
/// Submitted to the [`inventory`] collection at link time
/// via `#[derive(TypeSlot)]`.
pub struct TypeSlotEntry {
    pub type_id: TypeId,
    pub group_id: TypeId,
    pub slot: &'static AtomicSlot,
}

inventory::collect!(TypeSlotEntry);

/// A type with a statically assigned slot index within group `G`.
///
/// Always use the derive macro to generate the correct
/// implementation:
///
/// ```
/// use typeslot::prelude::*;
///
/// // Define group markers.
/// struct ResourceGroup;
///
/// // Derive `TypeSlot` on your types.
/// #[derive(TypeSlot)]
/// #[slot(ResourceGroup)]
/// struct Health;
/// ```
pub trait TypeSlot<G: 'static>: 'static {
    /// Returns the slot index, or `None` if [`init_slot`] has not
    /// been called for `G` yet.
    fn slot() -> Option<usize>
    where
        Self: Sized;

    fn dyn_slot(&self) -> Option<usize>;
}

/// A zero-sized handle for querying slot indices within group `G`.
///
/// Can be instantiated anywhere and used to query slots without
/// repeating the group type parameter.
pub struct SlotGroup<G>(PhantomData<G>);

impl<G: 'static> SlotGroup<G> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }

    /// Returns the slot index of type `T`, or `None` if [`init_slot`]
    /// has not been called for `G` yet.
    pub fn try_get<T: TypeSlot<G>>(&self) -> Option<usize> {
        try_slot::<T, G>()
    }

    /// Returns the slot index of type `T`.
    ///
    /// # Panics
    ///
    /// Panics if [`init_slot`] has not been called for `G` yet.
    pub fn get<T: TypeSlot<G>>(&self) -> usize {
        slot::<T, G>()
    }
}

impl<G: 'static> Default for SlotGroup<G> {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns the slot index of type `T` in group `G`.
///
/// # Panics
///
/// Panics if [`init_slot`] has not been called for `G` yet.
pub fn slot<T: TypeSlot<G>, G: 'static>() -> usize {
    T::slot().expect("slot not initialized; call `init_slot` first")
}

/// Returns the slot index of type `T` in group `G`, or `None` if
/// [`init_slot`] has not been called for `G` yet.
pub fn try_slot<T: TypeSlot<G>, G: 'static>() -> Option<usize> {
    T::slot()
}

/// Assigns a unique index to each type registered in
/// group `G`.
///
/// Must be called once per group before any call to
/// [`TypeSlot<G>::slot`].
///
/// Returns the number of slots assigned.
///
/// # Panics
///
/// Panics if called more than once for the same group.
pub fn init_slot<G: 'static>() -> usize {
    let group_id = TypeId::of::<G>();
    let mut index = 0usize;
    for entry in inventory::iter::<TypeSlotEntry>() {
        if entry.group_id == group_id {
            entry.slot.set(index);
            index += 1;
        }
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_slot_starts_unset() {
        let slot = AtomicSlot::new();
        assert_eq!(slot.get(), None);
    }

    #[test]
    fn atomic_slot_set_and_get() {
        let slot = AtomicSlot::new();
        slot.set(42);
        assert_eq!(slot.get(), Some(42));
    }

    #[test]
    #[should_panic(expected = "`AtomicSlot::set` called twice")]
    fn atomic_slot_panics_on_double_set() {
        let slot = AtomicSlot::new();
        slot.set(1);
        slot.set(2);
    }

    #[test]
    fn dyn_compatibility() {
        let _: Option<&dyn TypeSlot<()>> = None;
    }
}
