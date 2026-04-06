use typeslot::TypeSlot;

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
    assert_eq!(<Button as TypeSlot<ElementGroup>>::slot(), None);
    assert_eq!(<Texture as TypeSlot<ResourceGroup>>::slot(), None);
    assert_eq!(<Image as TypeSlot<ElementGroup>>::slot(), None);
    assert_eq!(<Image as TypeSlot<ResourceGroup>>::slot(), None);

    let element_count = typeslot::init::<ElementGroup>();
    let resource_count = typeslot::init::<ResourceGroup>();

    assert_eq!(element_count, 2); // Button, Image
    assert_eq!(resource_count, 2); // Texture, Image

    assert!(<Button as TypeSlot<ElementGroup>>::slot().is_some());
    assert!(<Texture as TypeSlot<ResourceGroup>>::slot().is_some());
    assert!(<Image as TypeSlot<ElementGroup>>::slot().is_some());
    assert!(<Image as TypeSlot<ResourceGroup>>::slot().is_some());

    assert_ne!(
        <Button as TypeSlot<ElementGroup>>::slot(),
        <Image as TypeSlot<ElementGroup>>::slot()
    );
    assert_ne!(
        <Texture as TypeSlot<ResourceGroup>>::slot(),
        <Image as TypeSlot<ResourceGroup>>::slot()
    );
}
