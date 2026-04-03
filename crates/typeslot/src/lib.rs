use core::any::TypeId;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;

pub use typeslot_macros::HasSlot;

#[doc(hidden)]
pub use inventory;

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
/// [`HasSlot<G>`] within group `G`.
///
/// Submitted to the [`inventory`] collection at link time
/// via `#[derive(HasSlot)]`.
pub struct TypeSlotEntry {
    pub type_id: TypeId,
    pub group_id: TypeId,
    pub slot: &'static AtomicSlot,
}

inventory::collect!(TypeSlotEntry);

/// A type with a statically assigned slot index within
/// group `G`.
pub trait HasSlot<G: 'static>: 'static {
    /// Returns the slot index, or `None` if [`init`] has not
    /// been called for `G` yet.
    fn slot() -> Option<usize>;
}

/// Assigns a unique index to each type registered in
/// group `G`.
///
/// Must be called once per group before any call to
/// [`HasSlot<G>::slot`].
///
/// # Panics
///
/// Panics if called more than once for the same group.
pub fn init<G: 'static>() {
    let group_id = TypeId::of::<G>();
    let mut index = 0usize;
    for entry in inventory::iter::<TypeSlotEntry>() {
        if entry.group_id == group_id {
            entry.slot.set(index);
            index += 1;
        }
    }
}
