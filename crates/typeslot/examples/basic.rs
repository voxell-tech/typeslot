use typeslot::prelude::*;

#[derive(SlotGroup)]
struct ElementGroup;

#[derive(SlotGroup)]
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
    ElementGroup::init();
    ResourceGroup::init();

    println!("Elements:");
    println!("  Horizontal: {}", ElementGroup::slot::<Horizontal>());
    println!("  Vertical:   {}", ElementGroup::slot::<Vertical>());
    println!("  Label:      {}", ElementGroup::slot::<Label>());

    println!("Resources:");
    println!("  Health: {}", ResourceGroup::slot::<Health>());
    println!("  Mana:   {}", ResourceGroup::slot::<Mana>());
    println!("  Label:  {}", ResourceGroup::slot::<Label>());
}
