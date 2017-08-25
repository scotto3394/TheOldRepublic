#ifndef CLASSES
#define CLASSES

#include <iostream>
#include <math.h>
#include <memory>
#include <stdint.h>
#include <string>

//===========================================================
// Structs
//===========================================================
struct coord {
	float_t pos_x;
	float_t pos_y;
};

struct stats {
	int16_t str = 10;
	int16_t dex = 10;
	int16_t con = 10;
	int16_t wis = 10;
	int16_t inte = 10;
	int16_t cha = 10;
};

//===========================================================
// Classes
//===========================================================
class entity {
public:
	entity();
	~entity();
	void setPos(float_t x, float_t y);
	coord getPos();

	coord position;
	//coord direction (perhaps)
protected:
private:
};

//-----------------------------------------------------------
class living: public entity {
public:
	living();
	~living();
	void setStats(stats stat_mod);
	stats getStats();

	std::string name;
protected:
	stats stat_block;
private:
};

//-----------------------------------------------------------
class npc: public living {
public:
	npc();
	~npc();
protected:
private:
};

//-----------------------------------------------------------
class player: public living {
public:
	player();
	~player();
protected:
private:
};

#endif // CLASSES