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
use typeslot::TypeSlot;

// Define group markers.
struct ElementGroup;
struct ResourceGroup;

// Derive `TypeSlot` on your types.
#[derive(TypeSlot)]
#[slot(ElementGroup)]
struct Horizontal;

#[derive(TypeSlot)]
#[slot(ElementGroup)]
struct Vertical;

#[derive(TypeSlot)]
#[slot(ResourceGroup)]
struct Health;

// A type can belong to multiple groups.
#[derive(TypeSlot)]
#[slot(ElementGroup, ResourceGroup)]
struct Label;

// Call init once per group before accessing slots.
// It returns the number of slots assigned in the group.
let element_count = typeslot::init::<ElementGroup>();
let resource_count = typeslot::init::<ResourceGroup>();

assert_eq!(element_count, 3); // Horizontal, Vertical, Label
assert_eq!(resource_count, 2); // Health, Label

// Each group has its own independent index space.
println!("{:?}", <Horizontal as TypeSlot<ElementGroup>>::slot());
println!("{:?}", <Health as TypeSlot<ResourceGroup>>::slot());
```

## Join the community!

You can join us on the [Voxell discord server](https://discord.gg/Mhnyp6VYEQ).

## License

`typeslot` is dual-licensed under either:

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
