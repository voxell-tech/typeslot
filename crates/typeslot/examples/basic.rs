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

#[derive(HasSlot)]
#[slot(ResourceGroup)]
struct Mana;

// A type can belong to multiple groups.
#[derive(HasSlot)]
#[slot(ElementGroup, ResourceGroup)]
struct Label;

fn main() {
    typeslot::init::<ElementGroup>();
    typeslot::init::<ResourceGroup>();

    println!("Elements:");
    println!(
        "  Horizontal: {:?}",
        <Horizontal as HasSlot<ElementGroup>>::slot()
    );
    println!(
        "  Vertical:   {:?}",
        <Vertical as HasSlot<ElementGroup>>::slot()
    );
    println!(
        "  Label:      {:?}",
        <Label as HasSlot<ElementGroup>>::slot()
    );

    println!("Resources:");
    println!(
        "  Health: {:?}",
        <Health as HasSlot<ResourceGroup>>::slot()
    );
    println!(
        "  Mana:   {:?}",
        <Mana as HasSlot<ResourceGroup>>::slot()
    );
    println!(
        "  Label:  {:?}",
        <Label as HasSlot<ResourceGroup>>::slot()
    );
}
