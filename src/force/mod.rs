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
enum Class {
	Consular(u8), //cleric
	Guardian(u8), //paladin
	Juggernaut(u8), //berserker
	Smuggler(u8), //rogue
	Operative(u8), //slayer
	BountyHunter(u8), //ranger
}

enum Effect {
	Damage(i16),
	Buff,
	Debuff,
}

//-------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------
type Interactive = Box<FnMut(Entity, Vec<Entity>) -> Effect>;


//-------------------------------------------------------------------------
// Structs
//-------------------------------------------------------------------------

struct StatBlock {
	strength: u8,
	dexterity: u8,
	wisdom: u8,
	intelligence: u8,
	charisma: u8,

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

struct Entity {
	hp: u32,
	level: u8,
	class: Vec<Class>,
	skills: HashMap<String, u8>,
	stats: StatBlock,
	traits: HashMap<String, Trait>,
	abilities: HashMap<String, Ability>,
}


//=========================================================================
// Impl
//=========================================================================
