# World
The game space is represented as a tile-based 2D map, viewed from an isometric, aerial perspective. Tiles are cardinally aligned with north oriented towards the top-right. A tile is represented with a 256 x 128 px sprite.

Tiles beyond the initial starting region are obscured by fog of war until the player take steps towards uncovering more tiles. 

## Constructs
Entities that populate individual tiles on the map. Constructs include player-placable constructs such [structures](./structures.md) and [paths](./logistics.md#path-segment), and randomly generated, non-placable constructs such as [resouce nodes](./resources.md#resource-node) and obstacles.

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

## State variables
Metrics describing the impact of the player's factory activity on the world around you. Represents relationships to established society, the surrounding environment or the natural order itself. Examples:

* Notoriety - Standing with local villages
* Reverence - Reputation with followers
* Contamination - Impact on local flora and fauna
* Coherence - Environmental sanity

Construct interactions with state variables can be described as state-modifying and/or state-sensitive. A state-modifying construct is capable of contributing towards or against a state variable, while a state-sensitive construct is capable of observing and reacting to state variables. For example, a state-modifying structure may increase contamination when completing a work cycle. Conversely, a state-sensitive resource node may have its harvest rate lowered when contamination exceeds a threshold.

While some state variables permeate globally and are observable by any state-sensitive construct, others may have local scopes. Constructs that modify such states have a range of influence. Only state-sensitive constructs within range will be aware of the local state. A sensitive construct within range of multiple state-modifying constructs will experience the total sum of influences as one state variable.

### Additional state interaction mechanics
* [The Relic](./progression.md#the-relic) - Upon cultivating the relic to new tiers, flat boosts, upper and lower bounds adjustment or multipliers may be applied to the state variable to enforce a difficulty curve.
* Dialogue - Select options during NPC dialogue may impact state variables to reflect social standing.