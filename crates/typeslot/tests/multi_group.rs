use typeslot::prelude::*;

#[derive(SlotGroup)]
struct ElementGroup;

#[derive(SlotGroup)]
struct ResourceGroup;

#[derive(TypeSlot)]
#[slot(ElementGroup)]
struct Button;

#[derive(TypeSlot)]
#[slot(ResourceGroup)]
struct Texture;

#[derive(TypeSlot)]
#[slot(ElementGroup, ResourceGroup)]
struct Image;

#[test]
fn unique_slot_indices_per_group() {
    assert_eq!(ElementGroup::try_slot::<Button>(), None);
    assert_eq!(ResourceGroup::try_slot::<Texture>(), None);
    assert_eq!(ElementGroup::try_slot::<Image>(), None);
    assert_eq!(ResourceGroup::try_slot::<Image>(), None);

    let element_count = ElementGroup::init();
    let resource_count = ResourceGroup::init();

    assert_eq!(element_count, 2); // Button, Image
    assert_eq!(resource_count, 2); // Texture, Image

    assert!(ElementGroup::try_slot::<Button>().is_some());
    assert!(ResourceGroup::try_slot::<Texture>().is_some());
    assert!(ElementGroup::try_slot::<Image>().is_some());
    assert!(ResourceGroup::try_slot::<Image>().is_some());

    assert_ne!(
        ElementGroup::slot::<Button>(),
        ElementGroup::slot::<Image>()
    );
    assert_ne!(
        ResourceGroup::slot::<Texture>(),
        ResourceGroup::slot::<Image>()
    );
}
