use typeslot::prelude::*;

struct ElementGroup;
struct ResourceGroup;

#[derive(TypeSlot)]
#[slot(ElementGroup)]
struct Horizontal;

#[derive(TypeSlot)]
#[slot(ElementGroup)]
struct Vertical;

#[derive(TypeSlot)]
#[slot(ResourceGroup)]
struct Health;

#[derive(TypeSlot)]
#[slot(ResourceGroup)]
struct Mana;

// A type can belong to multiple groups.
#[derive(TypeSlot)]
#[slot(ElementGroup, ResourceGroup)]
struct Label;

fn main() {
    init_slot::<ElementGroup>();
    init_slot::<ResourceGroup>();

    let elements = SlotGroup::<ElementGroup>::new();
    let resources = SlotGroup::<ResourceGroup>::new();

    println!("Elements:");
    println!("  Horizontal: {}", elements.get::<Horizontal>());
    println!("  Vertical:   {}", elements.get::<Vertical>());
    println!("  Label:      {}", elements.get::<Label>());

    println!("Resources:");
    println!("  Health: {}", resources.get::<Health>());
    println!("  Mana:   {}", resources.get::<Mana>());
    println!("  Label:  {}", resources.get::<Label>());
}
