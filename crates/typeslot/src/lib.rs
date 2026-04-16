#![doc = include_str!("../README.md")]
#![no_std]

use core::any::TypeId;
use core::sync::atomic::{AtomicUsize, Ordering};

pub use typeslot_macros::{SlotGroup, TypeSlot};

#[doc(hidden)]
pub use inventory;

pub mod prelude {
    pub use crate::{SlotGroup, TypeSlot, init_slot};
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
    /// The [`TypeId`] of the registered type.
    pub type_id: TypeId,
    /// The [`TypeId`] of the group.
    pub group_id: TypeId,
    /// The slot where the assigned index will be written.
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
/// #[derive(SlotGroup)]
/// struct EnemyGroup;
///
/// #[derive(TypeSlot)]
/// #[slot(EnemyGroup)]
/// struct Orc;
/// ```
pub trait TypeSlot<G: 'static>: 'static {
    /// Returns the slot index, or `None` if [`init_slot`] or
    /// [`SlotGroup::init`] has not been called for `G` yet.
    fn try_slot() -> Option<usize>
    where
        Self: Sized;

    /// Returns the slot index.
    ///
    /// # Panics
    ///
    /// Panics if [`init_slot`] or [`SlotGroup::init`] has not been
    /// called for `G` yet.
    #[inline]
    fn slot() -> usize
    where
        Self: Sized,
    {
        Self::try_slot().expect("slot not initialized; call `init_slot` or `SlotGroup::init` first")
    }

    /// Returns the slot index via a trait object, or `None` if
    /// [`init_slot`] or [`SlotGroup::init`] has not been called for
    /// `G` yet.
    fn dyn_try_slot(&self) -> Option<usize>;

    /// Returns the slot index via a trait object.
    ///
    /// # Panics
    ///
    /// Panics if [`init_slot`] or [`SlotGroup::init`] has not been
    /// called for `G` yet.
    #[inline]
    fn dyn_slot(&self) -> usize {
        self.dyn_try_slot()
            .expect("slot not initialized; call `init_slot` or `SlotGroup::init` first")
    }
}

/// A group of types with statically assigned slot indices.
///
/// Always use the derive macro to generate the correct
/// implementation:
///
/// ```
/// use typeslot::prelude::*;
///
/// #[derive(SlotGroup)]
/// struct EnemyGroup;
/// ```
pub trait SlotGroup: 'static {
    /// Assigns a unique index to each type registered in this group
    /// and records the total count.
    ///
    /// Must be called once before any call to [`SlotGroup::slot`] or
    /// [`SlotGroup::len`].
    ///
    /// Returns the number of slots assigned.
    ///
    /// # Panics
    ///
    /// Panics if called more than once for the same group.
    fn init() -> usize;

    /// Returns the number of slots assigned to this group, or `None`
    /// if [`SlotGroup::init`] has not been called yet.
    fn try_len() -> Option<usize>;

    /// Returns the number of slots assigned to this group.
    ///
    /// # Panics
    ///
    /// Panics if [`SlotGroup::init`] has not been called yet.
    fn len() -> usize;

    /// Returns the slot index of type `T` in this group, or `None` if
    /// [`SlotGroup::init`] has not been called yet.
    #[inline]
    fn try_slot<T: TypeSlot<Self>>() -> Option<usize>
    where
        Self: Sized,
    {
        T::try_slot()
    }

    /// Returns the slot index of type `T` in this group.
    ///
    /// # Panics
    ///
    /// Panics if [`SlotGroup::init`] has not been called yet.
    #[inline]
    fn slot<T: TypeSlot<Self>>() -> usize
    where
        Self: Sized,
    {
        T::slot()
    }
}

/// Assigns a unique index to each type registered in
/// group `G`.
///
/// Must be called once per group before any call to
/// [`TypeSlot::slot`] or [`TypeSlot::try_slot`].
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
