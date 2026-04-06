use typeslot::prelude::*;

struct ElementGroup;
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
    let elements = SlotGroup::<ElementGroup>::new();
    let resources = SlotGroup::<ResourceGroup>::new();

    assert_eq!(elements.try_get::<Button>(), None);
    assert_eq!(resources.try_get::<Texture>(), None);
    assert_eq!(elements.try_get::<Image>(), None);
    assert_eq!(resources.try_get::<Image>(), None);

    let element_count = init_slot::<ElementGroup>();
    let resource_count = init_slot::<ResourceGroup>();

    assert_eq!(element_count, 2); // Button, Image
    assert_eq!(resource_count, 2); // Texture, Image

    assert!(elements.try_get::<Button>().is_some());
    assert!(resources.try_get::<Texture>().is_some());
    assert!(elements.try_get::<Image>().is_some());
    assert!(resources.try_get::<Image>().is_some());

    assert_ne!(elements.get::<Button>(), elements.get::<Image>());
    assert_ne!(resources.get::<Texture>(), resources.get::<Image>());
}
