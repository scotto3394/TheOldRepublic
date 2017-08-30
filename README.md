# TheOldRepublic
<<<<<<< HEAD
Learning project with a Star Wars flair.

Overview
=============================
Overall the intention of this project is to build a multiplayer RPG system, inspired by concepts from Star Wars: The Old Republic and Dungeons & Dragons. Mainly the goal of this project is to provide a fun side project to practice and build our coding skills, and possibly down the line as a fun game to play amonst ourselves. 

A very, very rough map involves:
=======
Learning project with a Star Wars flair

This particular branch uses the programming language [Rust](http://www.rust-lang.org/en-US/) which will be necessary to build and test these files. 

Overview
=============================
Overall the intention of this project is to build a multiplayer RPG system, inspired by concepts from Star Wars: The Old Republic and Dungeons & Dragons. Mainly the goal of this project is to provide a fun side project to practice and build our coding skills, and possibly down the line as a fun game to play amongst ourselves. 

The basic goals come down to the following:
>>>>>>> rusty

1. Building the core data structures and game systems, involving ways to build characters/NPCs/Enemies. As this solidifies, skill sets, combat mechanics, and whatever strikes our imagination should be incorporated as well.
2. Since the goal is to eventually have an experience that can be shared amonst multiple players, building network communication and means of sharing data between players is essential. Choice of architecture is undecided at the moment.
3. As we progress, there will no doubt be attempts to improve user experience from cleaner interfaces and a smoother game experience. This could include anything from windows to incorporating maps and movement. Lots of opportunities based on how we end up exploring and learning.

<<<<<<< HEAD
=======
Code layout
===============================

Core
-------------------------------
This will encompass what each player character (pc) and non-player character (npc) is capable of and the way data about them is represented/stored. 

Combat
-------------------------------
This module will incorporate many of the mathematics and dice rolls that come about in any adventure. Building upon the core module, this will take relevant character data and allow for smooth behind-the-scenes interaction with other characters. 

Network
-------------------------------
This module will attempt to build the basic behavior needed to communicate actions between players, and hence the multiplayer experience. 

Binaries
-------------------------------
There will be a separate "Client" and "Server" application to manage the discrepancy between players and GM.


Current Layout and Progress
===============================
The direction and state of this project will be in constant flux and may differ quite a bit from previous iterations of this section (or even the current iteration). However, for anyone hoping to contribute it is important that everyone is on the same page. So the following is a snapshot of ongoing plans to build this project.

Big Picture
-------------------------------
The current target is for this application to emulate an online table-top RPG experience. What this means is that the majority of the experience will lack graphical components, instead using "Theater of the mind", to experience scenarios and non-combat encounters. To aid with the combat experience, basic positional maps will likely be introduced, but sophistication is not the current target. In lieu of a pen and paper "Character Sheet", this program will endeavor to provide convenient ways to keep track of Character information, Inventory management, and other components such as skill/ability/spell usage. In addition, while the concept of using dice to determine various "checks" in the game will be retained, the goal is to automate this process in both combat/non-combat encounters as smoothly as possible. This ensures that the focus is on the story, strategy, and conversation and less on making sure you're using the appropriate die. 

At the moment, it is expected that the players will acquire the "client" portion of the application, while the GM of any campaign will be using the "server" portion of the application. Through a campaign, "master" versions of player data will be stored in JSON on the server-side along with an SQLite database for storage of party loot, as well as item tables, monster tables, spell tables, and other game relevant information. As players "login", they will retrieve their character data into memory, for local access in reference or calculating future values. As the program executes, the players should have near real-time access to statistics, character data, inventory, rolls, and other features. During combat encounters, the game will switch into "Combat Mode" and initialise the map with character positioning, as well as putting characters into a turn-based mode. Based on calculated turn orders, each player (and GM) can utilize a certain number of combat/movement actions until it transfers turn to the next player. Basic actions such as viewing map state, combat state, and viewing stats/inventory should remain in a real-time basis. In addition to regular saving of player data to the server, there will be a "logout" process which should save character states to the master copies on the server for next session.



Questions / Undetermined Components
--------------------------------
- "Admin" access to players, i.e. the limits of client programs to edit and change data.
- Character data being stored in memory on the server-side as a "master" copy. This would help prevent cheating but could potentially affect performance.
- Frequency of saving character states to file.
- Graceful handling of sudden termination or network disconnect.
- Tracking temporary effects e.g. Statuses and Buffs
- Level up system and Experience. Should this be automated like a computer game or manual like a pen and paper game.
- Tracking "time" inside combat. Movement/Standard/Full round actions like Pathfinder, Actions Point concept like Divinity, or some other framework.
>>>>>>> rusty

Help
===============================
This will be the section for build instructions and other aspects such as running the program. At the moment, there is nothing to build

Building
-------------------------------
<<<<<<< HEAD
=======
Within the Rust infastructure, it should be sufficient to run the following:
	git clone "https://github.com/scotto3394/TheOldRepublic.git"
	cd TheOldRepublic
	cargo build --release

However at the moment, testing is limited and if there are problems feel free to contact the authors (found below).
>>>>>>> rusty

Running
-------------------------------

<<<<<<< HEAD
Contact
===============================
For any current questions or comments, please contact Scott Manifold <shm.qed@gmail.com>. 
=======

Contact
===============================
For any current questions or comments, please contact Scott Manifold <shm.qed@gmail.com>. 
>>>>>>> rusty
