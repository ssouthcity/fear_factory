# World
The game space is represented as a tile-based 2D map, viewed from an isometric, aerial perspective. Tiles are cardinally aligned with north oriented towards the top-right. A tile is represented with a 256 x 128 px sprite.

Tiles beyond the initial starting region are obscured by fog of war until the player take steps towards uncovering more tiles. 

## Constructs
Entities that populate the map. A construct occupies a single tile on the map. Constructs include player-placable constructs such [structures](./structure.md) and [paths](./logistics.md#path-segment), and randomly generated, non-placable constructs such as [resouce nodes](./resource_node.md) and obstacles.

## Biomes
Tiles are categorized into types with different tilesets and properties which affect world generation and how players interact with them. Tiles of the same type grouped across large sections of the map is considered a biome. The biome type may impact what resource nodes and obstacles appear in an area during generation.

Examples/ideas:
* Rivers
* Swamps
* Dry Dry Desert
* Cool Cool Mountain
* Lethal Lava Land
* Evil Forest
* City / Walled towns


