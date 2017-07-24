#ifndef CLASSES
#define CLASSES

#include <iostream>
#include <sstream>
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
	int16_t str;
	int16_t dex;
	int16_t con;
	int16_t wis;
	int16_t inte;
	int16_t cha;
};

//===========================================================
// Classes
//===========================================================
class entity {
public:
	entity();
	~entity();
protected:
private:
};

class living: public entity {
public:
	living();
	~living();

	std::string name;
protected:
	stats stat_block;
private:
};

class npc: public living {
public:
	npc();
	~npc();
protected:
private:
};

class player: public living {
public:
	player();
	~player();
protected:
private:
};

#endif // CLASSES