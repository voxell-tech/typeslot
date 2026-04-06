use typeslot::TypeSlot;

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
    typeslot::init_slot::<ElementGroup>();
    typeslot::init_slot::<ResourceGroup>();

    println!("Elements:");
    println!(
        "  Horizontal: {:?}",
        <Horizontal as TypeSlot<ElementGroup>>::slot()
    );
    println!(
        "  Vertical:   {:?}",
        <Vertical as TypeSlot<ElementGroup>>::slot()
    );
    println!(
        "  Label:      {:?}",
        <Label as TypeSlot<ElementGroup>>::slot()
    );

    println!("Resources:");
    println!(
        "  Health: {:?}",
        <Health as TypeSlot<ResourceGroup>>::slot()
    );
    println!(
        "  Mana:   {:?}",
        <Mana as TypeSlot<ResourceGroup>>::slot()
    );
    println!(
        "  Label:  {:?}",
        <Label as TypeSlot<ResourceGroup>>::slot()
    );
}
