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

For generic structs, use `register_typeslot!` directly, each concrete
monomorphization gets its own slot:

```rust
use typeslot::prelude::*;
use typeslot::register_typeslot;

#[derive(SlotGroup)]
struct EnemyGroup;

#[derive(SlotGroup)]
struct BossGroup;

struct Fire;
struct Ice;
struct Lightning;

struct Elemental<T>(core::marker::PhantomData<T>);

register_typeslot!(Elemental<Fire>, EnemyGroup);
register_typeslot!(Elemental<Ice>,  EnemyGroup);
register_typeslot!(Elemental<Lightning>, BossGroup);

let enemy_count = EnemyGroup::init();
let boss_count = BossGroup::init();

assert_eq!(enemy_count, 2); // Elemental<Fire>, Elemental<Ice>
assert_eq!(boss_count, 1);  // Elemental<Lightning>

assert_ne!(
    EnemyGroup::slot::<Elemental<Fire>>(),
    EnemyGroup::slot::<Elemental<Ice>>(),
);
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
