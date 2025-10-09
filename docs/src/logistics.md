# Logistics

## Path segment
Placable constructs that occupies one tile of the world space. Path segments automatically join adjacent segments to form longer paths. Paths may freely intersect and become intersections. A branch is considered any path in an intersection other than the arrival path. Connecting player structures with paths allows for automated item distribution.

## Porters
Workers that distribute items between player structures using paths. A number of workers may be assigned as porters at a given structure. Porters may be categorized into classes with different properties, some of which include:

* Pathfinding algorithm
* Porter speed
* Carrying capacity
* Biome affinity

When a structure has items in output, it will task available porters with deliveries at a fixed rate. Porter perform individual pathfinding, and are capable of remembering one path at a time. Porters will navigate along their remembered path, or perform discovery if no remembered path exists, in accordance with their class' pathfinding algorithm. Upon delivery, the porter will return to the originating structure and idle until tasked again. If a porter is unable to find a destination a set amount of time after deployment, the porter will give up and take the shortest path to the originating structure to begin again.

## Pathfinding algorithms
Describes a porters process for efficiently discovering and navigating paths without error.

### Common algorithm
A porter without a remembered path will pick a random branch at intersections. This process repeats until the porter arrives at a structure with an input for their tasked delivery item. The porter derives the shortest path from this structure back to their originating structure as its remembered path. During subsequent trips, the porter will pick a branch based on the following weights:


\begin{align} \text{Correct branch: }&  \frac{1}{n} + \frac{n-1}{n} \cdot \frac{m-k}{m} = \frac{-kn + k + mn}{mn}\\\ \text{Incorrect branch: }& \frac{1}{n} - \frac{1}{n} \cdot \frac{m-k}{m} = \frac{k}{mn} 
\end{align}

where *c* is the complexity, computed from the number of possible incorrect branches at every intersection along the remembered path, *m* is the number of branches the porter is capable of remembering before each branch is equally weighted and *n* is the number of branches in the intersection.