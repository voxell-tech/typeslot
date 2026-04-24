# Typeslot

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/voxell-tech/typeslot#license)
[![Crates.io](https://img.shields.io/crates/v/typeslot.svg)](https://crates.io/crates/typeslot)
[![Downloads](https://img.shields.io/crates/d/typeslot.svg)](https://crates.io/crates/typeslot)
[![Docs](https://docs.rs/typeslot/badge.svg)](https://docs.rs/typeslot/latest/typeslot/)
[![CI](https://github.com/voxell-tech/typeslot/workflows/CI/badge.svg)](https://github.com/voxell-tech/typeslot/actions)
[![Discord](https://img.shields.io/discord/442334985471655946.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/Mhnyp6VYEQ)

Assigns each type a unique `usize` index at startup, with optional group compartmentalization.

## Usage

```rust
use typeslot::prelude::*;

#[derive(SlotGroup)]
struct EnemyGroup;

#[derive(SlotGroup)]
struct AllyGroup;

#[derive(SlotGroup)]
struct BossGroup;

#[derive(TypeSlot)]
#[slot(EnemyGroup)]
struct Orc;

// A type can belong to multiple groups.
// Dragon is both a regular enemy and a boss.
#[derive(TypeSlot)]
#[slot(EnemyGroup, BossGroup)]
struct Dragon;

// Knight is both an ally and a boss-tier ally.
#[derive(TypeSlot)]
#[slot(AllyGroup, BossGroup)]
struct Knight;

let enemy_count = EnemyGroup::init();
let ally_count = AllyGroup::init();
let boss_count = BossGroup::init();

assert_eq!(enemy_count, 2); // 2: Orc, Dragon
assert_eq!(ally_count, 1);  // 1: Knight
assert_eq!(boss_count, 2);  // 2: Dragon, Knight

// Use `SlotGroup::len()` anytime to get the total count.
assert_eq!(enemy_count, EnemyGroup::len());
assert_eq!(ally_count, AllyGroup::len());
assert_eq!(boss_count, BossGroup::len());

println!("{}", EnemyGroup::slot::<Dragon>());
println!("{}", BossGroup::slot::<Knight>());
```

## Generic types

Rust does not support [per-monomorphization statics in blanket impls](https://github.com/rust-lang/rfcs/issues/2130).
There are two ways to work around this.

**Static registration** - use `register!` to explicitly register each concrete
monomorphization you need. Slots are assigned at startup by `init()`, and `len()`
reflects exactly the number registered:

```rust
use typeslot::prelude::*;
use typeslot::register;

#[derive(SlotGroup)]
struct EnemyGroup;

#[derive(SlotGroup)]
struct BossGroup;

struct Fire;
struct Ice;
struct Lightning;

struct Elemental<T>(core::marker::PhantomData<T>);

register!(EnemyGroup, [Elemental<Fire>, Elemental<Ice>]);
register!(BossGroup,  Elemental<Lightning>);

let enemy_count = EnemyGroup::init();
let boss_count = BossGroup::init();

assert_eq!(enemy_count, 2); // Elemental<Fire>, Elemental<Ice>
assert_eq!(boss_count, 1);  // Elemental<Lightning>

assert_ne!(
    EnemyGroup::slot::<Elemental<Fire>>(),
    EnemyGroup::slot::<Elemental<Ice>>(),
);
```

**Lazy registration, `generic` feature** - enable the `generic` feature
(enabled by default) for open-ended registration. Slots for generic
types are assigned on first access via a per-group `HashMap<TypeId, usize>`,
starting after the statically registered slots. `len()` grows each time a
new monomorphization is first queried.

```toml
[dependencies]
typeslot = { version = "*", features = ["generic"] }
```

Use `#[derive(TypeSlot)]` on a generic struct - any `T` gets a slot automatically:

```rust
use typeslot::prelude::*;
use core::marker::PhantomData;

#[derive(SlotGroup)]
// This attribute is essential, otherwise, #[derive(TypeSlot)] will fail on generics.
#[generic]
struct EnemyGroup;

#[derive(TypeSlot)]
#[slot(EnemyGroup)]
struct Elemental<T>(PhantomData<T>);

#[derive(TypeSlot)]
#[slot(EnemyGroup)]
struct Fire;

#[derive(TypeSlot)]
#[slot(EnemyGroup)]
struct Ice;

EnemyGroup::init();

// Fire & Ice will be registered first since they are concrete types.
assert_eq!(EnemyGroup::len(), 2);

let fire_slot = EnemyGroup::slot::<Elemental<Fire>>();
assert_eq!(EnemyGroup::len(), 3); // grew on first access

let ice_slot  = EnemyGroup::slot::<Elemental<Ice>>();
assert_eq!(EnemyGroup::len(), 4); // grew again

assert_ne!(fire_slot, ice_slot);
assert_eq!(EnemyGroup::slot::<Elemental<Fire>>(), fire_slot); // stable across calls
assert_eq!(EnemyGroup::len(), 4); // re-access does not grow len
```

For foreign generic types, use `register_generic!` instead:

```rust
use typeslot::prelude::*;
use typeslot::register_generic;
use core::marker::PhantomData;

#[derive(SlotGroup)]
#[generic]
struct EnemyGroup;

struct Elemental<T>(PhantomData<T>);
struct Duo<A, B>(PhantomData<(A, B)>);
struct Fire;
struct Ice;

// Single type or multiple types with []:
register_generic!(EnemyGroup, [Elemental<T>, Duo<A, B>]);

EnemyGroup::init();
println!("{}", EnemyGroup::slot::<Elemental<Fire>>());
println!("{}", EnemyGroup::slot::<Duo<Fire, Ice>>());
```

## Dynamic dispatch

`TypeSlot` is dyn-compatible. Use `dyn_slot` to retrieve a type's slot
index through a trait object:

```rust
use typeslot::prelude::*;

#[derive(SlotGroup)]
struct EnemyGroup;

#[derive(TypeSlot)]
#[slot(EnemyGroup)]
struct Goblin;

EnemyGroup::init();

let enemy: &dyn TypeSlot<EnemyGroup> = &Goblin;
println!("{}", enemy.dyn_slot());
```

## Join the community!

You can join us on the [Voxell discord server](https://discord.gg/Mhnyp6VYEQ).

## License

`typeslot` is dual-licensed under either:

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
