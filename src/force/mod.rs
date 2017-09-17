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
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
enum Species {
	Human,
	Twilek, 
	Zaabrak,
	Chiss,
}

// To Do: To be expanded, fill in and detail the Effect enum
// To Do: Think about how saves are incorporated into partial effects
#[derive(Serialize, Deserialize, Debug)]
pub enum Effect {
	Damage(Dice),
	Buff,
	Debuff,
}

// To Do: Think about Enums to specify status effects like Buffs/Debuffs
#[derive(Serialize, Deserialize, Debug)]
enum Saves {Fort(u8), Will(u8), Reflex(u8)}
#[derive(Serialize, Deserialize, Debug)]
enum Slot{Head, Shoulder, Body, Hand, Shoes, Enhancement, MH, OH}
#[derive(Serialize, Deserialize, Debug)]
enum Rtype{Melee, Ranged}
#[derive(Serialize, Deserialize, Debug)]
enum Dtype{Energy, Ion, Concussive, Physical}
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
trait Item {
    fn appraise(&self) -> u16;
}

trait Equipment {
    fn equip(&self, person: Entity);
}

//-------------------------------------------------------------------------
// Structs
//-------------------------------------------------------------------------
// To Do: Flesh out the item types and enums with appropriate structs.
#[derive(Serialize, Deserialize, Debug)]
pub struct Dice { num_dice: u8, dice_size: u8}

#[derive(Serialize, Deserialize, Debug)]
struct Armor {
    name: String,
    value: u32,
    slot: Slot,
    armor: Option<u8>,
    effect: Option<Interact>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weapon {
    name: String,
    value: u32,
    slot: Slot,
    damage: Dice,
    threat_range: u8,
    threat_mult: u8,
    range_type: Rtype,
    damage_type: Dtype,
	effect: Option<Interact>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Accessory {
	name: String,
	value: u32,
	effect: Option<Interact>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Tool {
    Grenade,
    Holocron,
    Stim,
    Adrenal,
}

#[derive(Serialize, Deserialize, Debug)]
struct Loot;

#[derive(Serialize, Deserialize, Debug)]
struct Outfit {
	head: Option<Armor>,
	shoulders: Option<Armor>,
	body: Option<Armor>,
	hand: Option<Armor>,
	shoes: Option<Armor>,
	enhance: Vec<Option<Accessory>>,
	main_hand: Option<Weapon>,
	off_hand: Option<Weapon>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CoreBlock {
	strength: u8,
	dexterity: u8,
	constitution: u8,
	wisdom: u8,
	intelligence: u8,
	charisma: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct DefenseBlock {
	armor: u8,
	cmd: u8,
	ref_save: u8,
	fort_save: u8,
	will_save: u8,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct AttackBlock {
	bab: u8,
	melee: u8,
	ranged: u8,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct StatBlock {
	core: CoreBlock,
	attack: AttackBlock,
	defense: DefenseBlock,
}

#[derive(Serialize, Deserialize, Debug)]
struct Trait
{
	name: String,
	description: String,
	effect: Interact
}

#[derive(Serialize, Deserialize, Debug)]
struct Ability
{
	name: String,
	description: String,
	effect: Interact
}

//To Do: Figure out how to update HP with class hit dice
//To Do: Figure out how to dynamically keep interconnected values updated
#[derive(Serialize, Deserialize, Debug)]
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
	fn default() -> CoreBlock{
		CoreBlock { strength:10, dexterity:10, constitution:10, 
			wisdom:10, intelligence:10, charisma:10}
	}
}

impl Default for DefenseBlock {
	fn default() -> DefenseBlock{
		DefenseBlock { armor: 10, cmd: 10, fort_save: 0, 
            ref_save: 0, will_save: 0}
	}
}

impl Default for Outfit {
	fn default() -> Outfit {
		Outfit { head: None, shoulders: None, body: None,
			hand: None, shoes: None, enhance: vec![None],
			main_hand: None, off_hand: None,
}
	}
}
impl Default for Entity {
	fn default() -> Entity{
        let stat_block: StatBlock = Default::default();
		Entity{ hp: 0, level: 0, class: vec![Class::Commoner], species: Species::Human,
			skills: HashMap::new(), stats: stat_block, traits: HashMap::new(),
			abilities: HashMap::new(), statuses: None, inventory: Vec::new(), 
			equipped: Default::default()}
	}
}

// To Do: Think about implementing a `textNode` trait of some sort for easy conversion from structs to Cursive

//-------------------------------------------------------------------------
// Method Impls
//-------------------------------------------------------------------------


//=========================================================================
// Functions
//=========================================================================
// To Do: Fill in the `drinks_served` function (aka roll macros)
pub fn drinks_served(drink: &str) {

}



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
            hp: 0, level: 1,
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
    fn test_datasave() {
        let emixan: Entity = Default::default();
        let serial = serde_json::to_string(&emixan).unwrap();
        println!("serialized = {}", serial);

        let deserial: Entity = serde_json::from_str(&serial).unwrap();
        println!("deserialized = {:?}", deserial);
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
