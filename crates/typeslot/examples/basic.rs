use typeslot::prelude::*;
use typeslot::register_typeslot;

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

struct Fire;
struct Ice;
struct Lightning;

struct Elemental<T>(core::marker::PhantomData<T>);

register_typeslot!(Elemental<Fire>, EnemyGroup);
register_typeslot!(Elemental<Ice>, EnemyGroup);
register_typeslot!(Elemental<Lightning>, BossGroup);

fn print_slot<G, T>(name: &str)
where
    G: SlotGroup,
    T: TypeSlot<G>,
{
    println!("  {name}: {}", T::slot());
}

fn main() {
    EnemyGroup::init();
    AllyGroup::init();
    BossGroup::init();

    println!("Enemies:");
    print_slot::<EnemyGroup, Orc>("Orc");
    print_slot::<EnemyGroup, Dragon>("Dragon");

    println!("Allies:");
    print_slot::<AllyGroup, Knight>("Knight");

    println!("Bosses:");
    print_slot::<BossGroup, Dragon>("Dragon");
    print_slot::<BossGroup, Knight>("Knight");

    println!("Elementals:");
    print_slot::<EnemyGroup, Elemental<Fire>>("Fire");
    print_slot::<EnemyGroup, Elemental<Ice>>("Ice");
    print_slot::<BossGroup, Elemental<Lightning>>("Lightning");
}
