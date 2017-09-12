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

}

// To Do: To be expanded, fill in and detail the Effect enum
 enum Effect {
	Damage(i16),
	Buff,
	Debuff,
}

// To Do: Think about Enums to specify status effects like Buffs/Debuffs

//-------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------
type Interactive = Box<FnMut(Entity, Option<Vec<Entity>>) -> Option<Effect>>;

//-------------------------------------------------------------------------
// Traits
//-------------------------------------------------------------------------

// To Do: Perhaps some `temporary` trait to indicate buffs and debuffs ticks.
trait Item {
    fn appraise(&self) -> u16;
}
trait Equipment {
    fn equip(&self, person: Entity);
}

//-------------------------------------------------------------------------
// Structs
//-------------------------------------------------------------------------
// To Do: Need to look into Item Types or Item Enums more closely to resolve easy equipping
// as well as quick item action resolving

struct Armor;

enum Weapon {
    Lightsaber,
    Blaster,
    Stick,
}

struct Acessory;

enum Tool {
    Grenade,
    Holocron,
    Stim,
    Adrenal,
}

struct Loot;

struct Outfit {
	head: Option<Armor>,
	shoulders: Option<Armor>,
	body: Option<Armor>,
	hand: Option<Armor>,
	shoes: Option<Armor>,
	enhance: Vec<Option<Acessory>>,
	main_hand: Option<Weapon>,
	off_hand: Option<Weapon>,
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

struct Trait
{
	name: String,
	description: String,
	effect: Interactive
}

struct Ability
{
	name: String,
	description: String,
	effect: Interactive
}

//To Do: Figure out how to update HP with class hit dice
//To Do: Figure out how to dynamically keep interconnected values updated
struct Entity {
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
		DefenseBlock { armor: 10, cmd: 10, ..Default::default()}
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
		Entity{ hp: 0, level: 0, class: vec![Class::Commoner], species: Species::Human,
			skills: HashMap::new(), stats: Default::default(), traits: HashMap::new(),
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
