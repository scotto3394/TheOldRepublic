#include <classes.h>

int main() {
	npc womp_rat;

	std::cout<<"You found a Womp Rat!\nWhy don't you give it a name?\n";
	std::cout<<"Womp Rate name: ";
	std::getline(std::cin, womp_rat.name);

	std::cout<< "You give " << womp_rat.name << " a nice big hug!\n";
	std::cout<< "Press Enter to exit...";
	std::cin.get();
	return 0;
}