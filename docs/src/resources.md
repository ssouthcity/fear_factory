# Resources
Basic materials used for recipies. A resource is characterized by its origin and represents an abstracted ingredient that is contextualized by the recipie its used in. Resources come from 3 domains; flora, fauna and minerale. Each domain has 3 subdomain further characterizing the resource origin.

* Fauna
    * Humane - Relates to human life, originates from population centers.
    * Beastial - Relates to animals, originates from farms and waterholes.
    * Deceased - Relates to human death, originates from graveyards and morgues.

* Flora
    * Floral - Relates to small growths and blossoms, originates from meadows and fields.
    * Arboreal - Relates to trees and their byproducts, originates from forests.
    * Fungal - Relates to molds and mushrooms, originates from the undergrowth.

* Minerale
    * Alluvial[^1] - Relates to granular sediments, originates from rivers and shores. 
    * Lithic[^2] - Relates to geological rocks, originates from crags and caves.
    * Lapidary[^3] - Relates to gems and precious ores, originates from mines and quarries.

[^1]: Referred as *coastal* internally.
[^2]: Referred as *crustal* internally.
[^3]: Referred to as *crystal* internally.

## Resource Node
Construct that yields resources when harvested. Nodes contain a finite quantity of resources and disappear when completely exhausted. They spawn in continuous clusters across the map with respect to biomes.