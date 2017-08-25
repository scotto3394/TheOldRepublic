#include <classes.h>

//-------------------------------------------------
//"entity" functions
//-------------------------------------------------
/// Constructor
entity::entity( ){

};

///Updating Position
void entity::setPos(float_t x, float_t y) {
	position.pos_x = x; position.pos_y = y;
	return;
};

///Reading Position
coord entity::getPos() {
	coord pos_data;
	pos_data.pos_x = position.pos_x;
	pos_data.pos_y = position.pos_y;
	return pos_data;
};

/// Destructor
entity::~entity( ){

};

//-------------------------------------------------
//"living" functions
//-------------------------------------------------
/// Constructor
living::living( ){

};

///Updating Stats
void living::setStats(stats stat_mod) {
	stat_block.str += stat_mod.str;
	stat_block.dex += stat_mod.dex;
	stat_block.con += stat_mod.con;
	stat_block.wis += stat_mod.wis;
	stat_block.inte += stat_mod.inte;
	stat_block.cha += stat_mod.cha;
	return;
};

///Reading Stats
stats living::getStats() {
	stats stat_data;
	stat_data.str = stat_block.str;
	stat_data.dex = stat_block.dex;
	stat_data.con = stat_block.con;
	stat_data.wis = stat_block.wis;
	stat_data.inte = stat_block.inte;
	stat_data.cha = stat_block.cha;
	return stat_data;
};

/// Destructor
living::~living( ){

};

//-------------------------------------------------
//"npc" functions
//-------------------------------------------------
/// Constructor
npc::npc( ){

};

/// Destructor
npc::~npc( ){

};

//-------------------------------------------------
//"player" functions
//-------------------------------------------------
/// Constructor
player::player( ){

};

/// Destructor
player::~player( ){

};