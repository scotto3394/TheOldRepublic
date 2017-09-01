//! The `force` module
//!
//! This module will focus on internal game logic (outside of combat) 
//! and building the necessary components for a smooth game experience.


pub mod startup;
pub mod shutdown;

enum Class{
	Consular(u8), //cleric
	Guardian(u8), //paladin
	Juggernaut(u8), //berserker
	Smuggler(u8), //rogue
	Operative(u8), //slayer
	BountyHunter(u8), //ranger
}

struct Entity {
	hp: u32,
	level: u8,
	class: Vec<Class>,
	//Skills as a Hashmap?
	//Stats as a Struct?
	//Traits as 
}