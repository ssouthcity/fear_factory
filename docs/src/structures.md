# Structure
Placeable construct that accomodates some form of labor. Structures are built using resources and require allocated devotees to operate.

## Harvester
Harvests resource nodes within a flexible range. The range of harvest is increased by allocating additional workers to the harvester, with increased allocation requirement as the range grows. The rate of harvest is dictated by the resource node, but multiple harvesters may harvest the same node for a proportionally increased yield rate. Harvesters visually reflect the harvested resource when nodes are within reach, otherwise presenting as a generic structure. If a harvester has nodes of differing types in its range, one type will be arbitratily chosen and exhausted before the other type will be harvested.