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

// Dragon is both a regular enemy and a boss.
#[derive(TypeSlot)]
#[slot(EnemyGroup, BossGroup)]
struct Dragon;

// Knight is both an ally and a boss-tier ally.
#[derive(TypeSlot)]
#[slot(AllyGroup, BossGroup)]
struct Knight;

fn main() {
    EnemyGroup::init();
    AllyGroup::init();
    BossGroup::init();

    println!("Enemies:");
    println!("  Orc:    {}", EnemyGroup::slot::<Orc>());
    println!("  Dragon: {}", EnemyGroup::slot::<Dragon>());

    println!("Allies:");
    println!("  Knight: {}", AllyGroup::slot::<Knight>());

    println!("Bosses:");
    println!("  Dragon: {}", BossGroup::slot::<Dragon>());
    println!("  Knight: {}", BossGroup::slot::<Knight>());
}
