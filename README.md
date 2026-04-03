# Typeslot

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/voxell-tech/typeslot#license)
[![Crates.io](https://img.shields.io/crates/v/typeslot.svg)](https://crates.io/crates/typeslot)
[![Downloads](https://img.shields.io/crates/d/typeslot.svg)](https://crates.io/crates/typeslot)
[![Docs](https://docs.rs/typeslot/badge.svg)](https://docs.rs/typeslot/latest/typeslot/)
[![CI](https://github.com/voxell-tech/typeslot/workflows/CI/badge.svg)](https://github.com/voxell-tech/typeslot/actions)
[![Discord](https://img.shields.io/discord/442334985471655946.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/Mhnyp6VYEQ)

Assigns each type a unique `usize` index at startup, with optional group compartmentalization.

## Usage

Define group markers and derive `HasSlot` on your types:

```rust
use typeslot::HasSlot;

struct ElementGroup;
struct ResourceGroup;

#[derive(HasSlot)]
#[slot(ElementGroup)]
struct Horizontal;

#[derive(HasSlot)]
#[slot(ElementGroup)]
struct Vertical;

#[derive(HasSlot)]
#[slot(ResourceGroup)]
struct Health;

// A type can belong to multiple groups.
#[derive(HasSlot)]
#[slot(ElementGroup, ResourceGroup)]
struct Label;
```

Call `init` once per group before accessing slots:

```rust
typeslot::init::<ElementGroup>();
typeslot::init::<ResourceGroup>();

// Each group has its own independent index space.
println!("{:?}", <Horizontal as HasSlot<ElementGroup>>::slot());
println!("{:?}", <Health as HasSlot<ResourceGroup>>::slot());
```

## Join the community!

You can join us on the [Voxell discord server](https://discord.gg/Mhnyp6VYEQ).

## License

`typeslot` is dual-licensed under either:

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
