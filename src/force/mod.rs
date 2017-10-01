//! The `force` module
//!
//! This module will focus on internal game logic (outside of combat)
//! and building the necessary components for a smooth game experience.


pub mod startup;
pub mod shutdown;


//=========================================================================
// Base Definitions
//=========================================================================
use std::collections::HashMap;

//-------------------------------------------------------------------------
// Enums
//-------------------------------------------------------------------------
// To Do: To be expanded, fill in Class enum
enum Class {
    Consular(u8), //cleric
    Guardian(u8), //paladin
    Juggernaut(u8), //berserker
    Smuggler(u8), //rogue
    Operative(u8), //slayer
    BountyHunter(u8), //ranger
    Commoner, //Default
}

// To Do: To be expanded, fill in Species enum
enum Species {
    Human,
    Twilek,
    Zaabrak,
    Chiss,
}

enum ItemType {
    Weapon {
        damage: Dice,
        threat_range: u8,
        threat_mult: u8,
        range_type: Rtype,
        damage_type: Dtype,
    },
    Armor { armor: Option<u8> },
    Accessory,
    Grenade,
    Holocron { charges: u8 },
    Stim,
    Adrenal,
    Medpack { charges: u8 },
    Loot,
}

// To Do: To be expanded, fill in and detail the Effect enum
// To Do: Think about how saves are incorporated into partial effects
pub enum Effect {
    Damage(Dice),
    Buff,
    Debuff,
}

// To Do: Think about Enums to specify status effects like Buffs/Debuffs
enum Saves {
    Fort(u8),
    Will(u8),
    Reflex(u8),
}
enum Slot {
    Head,
    Shoulder,
    Body,
    Hand,
    Shoes,
    Enhancement,
    MH,
    OH,
}
enum Rtype {
    Melee,
    Ranged,
}
enum Dtype {
    Energy,
    Ion,
    Concussive,
    Physical,
}
//To Do: Implement Debuf/Serialization for Interact enum.
enum Interact {
    Personal(Box<FnMut(&Entity) -> Option<Vec<Effect>>>),
    Group(Box<FnMut(&Entity, Vec<&Entity>) -> Option<Vec<Effect>>>),
    Area(Box<FnMut(&Entity, Option<Vec<&Entity>>) -> Option<Vec<Effect>>>),
    Target(Box<FnMut(&Entity, &Entity) -> Option<Vec<Effect>>>),
}
//-------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------
//type Interactive = Box<FnMut(&Entity, Option<Vec<&Entity>>) -> Option<Effect>>;

//-------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------

// To Do: Perhaps some `temporary` trait to indicate buffs and debuffs ticks.
// To Do: Fill out these traits to be more appropriate and constructive
// To Do: Implement Debug/Serialization effects for traits.
trait Object {
    fn appraise(&self) -> u16;
}

trait Equip {
    fn equip(&self, person: Entity);
}

trait Consume {
    fn activate(&self);
}

//-------------------------------------------------------------------------
// Structs
//-------------------------------------------------------------------------

//------------------------------- Items  ----------------------------------
struct Item {
    name: String,
    description: Option<String>,
    value: u32,
    slot: Option<Slot>,
    effect: Option<Interact>,
    subtype: ItemType,
}


struct Outfit {
    head: Option<Box<Item>>,
    shoulders: Option<Box<Item>>,
    body: Option<Box<Item>>,
    hand: Option<Box<Item>>,
    shoes: Option<Box<Item>>,
    enhance: Option<Vec<Box<Item>>>,
    main_hand: Option<Box<Item>>,
    off_hand: Option<Box<Item>>,
}

pub struct Dice {
    num: u8,
    size: u8,
}

struct CoreBlock {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    wisdom: u8,
    intelligence: u8,
    charisma: u8,
}

struct DefenseBlock {
    armor: u8,
    cmd: u8,
    ref_save: u8,
    fort_save: u8,
    will_save: u8,
}

#[derive(Default)]
struct AttackBlock {
    bab: u8,
    melee: u8,
    ranged: u8,
}

#[derive(Default)]
struct StatBlock {
    core: CoreBlock,
    attack: AttackBlock,
    defense: DefenseBlock,
}

struct Trait {
    name: String,
    description: String,
    effect: Interact,
}

struct Ability {
    name: String,
    description: String,
    effect: Interact,
}

//To Do: Figure out how to update HP with class hit dice
//To Do: Figure out how to dynamically keep interconnected values updated
pub struct Entity {
    hp: u32,
    level: u8,
    class: Vec<Class>,
    species: Species,
    skills: HashMap<String, u8>,
    stats: StatBlock,
    traits: HashMap<String, Trait>,
    abilities: HashMap<String, Ability>,
    statuses: Option<Vec<Effect>>,
    inventory: Vec<Box<Item>>,
    equipped: Outfit,
}



//=========================================================================
// Impl
//=========================================================================

//-------------------------------------------------------------------------
// Trait Impls
//-------------------------------------------------------------------------
// Default
impl Default for CoreBlock {
    fn default() -> CoreBlock {
        CoreBlock {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            wisdom: 10,
            intelligence: 10,
            charisma: 10,
        }
    }
}

impl Default for DefenseBlock {
    fn default() -> DefenseBlock {
        DefenseBlock {
            armor: 10,
            cmd: 10,
            fort_save: 0,
            ref_save: 0,
            will_save: 0,
        }
    }
}

impl Default for Outfit {
    fn default() -> Outfit {
        Outfit {
            head: None,
            shoulders: None,
            body: None,
            hand: None,
            shoes: None,
            enhance: None,
            main_hand: None,
            off_hand: None,
        }
    }
}
impl Default for Entity {
    fn default() -> Entity {
        let stat_block: StatBlock = Default::default();
        Entity {
            hp: 0,
            level: 0,
            class: vec![Class::Commoner],
            species: Species::Human,
            skills: HashMap::new(),
            stats: stat_block,
            traits: HashMap::new(),
            abilities: HashMap::new(),
            statuses: None,
            inventory: Vec::new(),
            equipped: Default::default(),
        }
    }
}

// To Do: Think about implementing a `textNode` trait of some sort for easy
// conversion from structs to Cursive

//-------------------------------------------------------------------------
// Method Impls
//-------------------------------------------------------------------------


//=========================================================================
// Functions
//=========================================================================
// To Do: Fill in the `drinks_served` function (aka roll macros)
pub fn drinks_served(drink: &str) {}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_smallentity() {
        let emixan: Entity = Default::default();
    }

    #[test]
    fn test_entity() {
        let emixan = Entity {
            hp: 0,
            level: 1,
            class: vec![Class::Operative(1)],
            species: Species::Chiss,
            skills: HashMap::new(),
            stats: StatBlock::default(),
            traits: HashMap::new(),
            abilities: HashMap::new(),
            statuses: None,
            inventory: Vec::<Box<Item>>::new(),
            equipped: Outfit::default(),
        };
    }

    #[test]
    fn test_spellbook() {
        use super::super::jedi_code::abilities::saber_throw;
        let saber_throw_ability = Ability {
            name: String::from("Saber Throw"),
            description: String::from("Throw a saber"),
            effect: Interact::Target(Box::new(saber_throw)),
        };
        let mut spellbook = HashMap::new();
        spellbook.insert(String::from("Saber Throw"), saber_throw_ability);
    }


}
