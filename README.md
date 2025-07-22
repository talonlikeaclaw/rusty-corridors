# Rusty Corridors

## Short Description

A dungeon crawler with procedurally generated levels, monsters of increasing difficulty, and turn-based movement.
Game written in Rust using `bracket-lib` with help from Hands-on Rust by Herbert Wolverson.

## Story

Long ago, a high-tech underground complex was taken over by a rogue AI named ARGUS. Now it is rusted and crawling with monsters from another dimension.
**You** are a _Seeker_, one of the few brave enough to enter and try to take it back. Every floor brings you closer to the truth hidden in the metal depths.
Rumour has it there ARGUS is waiting for you.

## Basic Game Loop

1. Enter dungeon level.
2. Explore, revealing the map.
3. Encounter enemies - fight or flee.
4. Find power-ups and use them to stregthen the player.
5. Locate the exit to the level - go to 1.

## Minimum Viable Product

1. Create a basic dungeon map.
2. Place the player and let them walk around.
3. Spawn monsters, draw them, and let the player kill them by walking into them.
4. Add health and a combat system that uses it.
5. Add healing potions.
6. Display a "game over" screen when the player dies.
7. Add ARGUS to the final level and let the player win after defeating it.

## Stretch Goals

1. Add Fields-of-View.
2. Add more interesting dungeon designs.
3. Add some dungeon themes.
4. Add multiple layers to the dungeon, with ARGUS on the last one.
5. Add varied weapons to the game.
6. Move to a data-driven design for spawning enemies.
7. Consider some visual effects to make combat more visceral.
8. Consider keeping score.
