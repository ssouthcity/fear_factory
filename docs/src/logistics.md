# Logistics

## Path segment
Placable construct that occupies one tile of the map. Path segments automatically join adjacent segments to form longer paths. Paths may freely intersect and become intersections. A branch is considered any path of an intersection other than the arrival path. Connecting player structures with paths allows for automated item distribution.

## Porter
Worker that distribute items between player structures using paths. A number of workers may be assigned as porters at a given structure. Porters may be categorized into classes with different properties, some of which include:

* Pathfinding algorithm
* Porter speed
* Carrying capacity
* Biome affinity
* Patience

When a structure has items in output, it will task available porters with deliveries at a fixed rate. Porter perform individual pathfinding, and are capable of remembering one path at a time. Porters will navigate along their remembered path in accordance with their class' pathfinding algorithm. Upon delivery, the porter will return to the originating structure and idle until tasked again. If a porter is unable to find a destination a set amount of time after deployment, the porter will give up and take the shortest path to the originating structure to begin again.

## Pathfinding algorithms
Describes a porters process for efficiently discovering and navigating paths without error.

### Common algorithm
A porter without a remembered path will pick a random branch at intersections. This process repeats until the porter arrives at a structure with an input for their tasked delivery item. If they arrive at a structure without a corresponding input, they return to the previous intersection. The porter records the shortest path from this structure back to their originating structure as its remembered path. Paths will be remembered upon successful delivery, even if the destination differs from the previously remembered path.

A porter with a remembered path will pick a branch based on the following weights:

\begin{align} \text{Correct branch: }&  1 - \frac{n-1}{n} \cdot \frac{c}{m} = \frac{-cn + c + mn}{mn}\\\ \text{Incorrect branch: }& \frac{1}{n} \cdot \frac{c}{m} = \frac{c}{mn} 
\end{align}

- ***n*** - branches leading out from an intersection from the perspective of an arriving porter. For example, a 3 way intersection has 2 branches.
- ***c*** - complexity, the total sum of branches in all intersections along the remembered path. For example, a remembered path with one 4 way intersection and two 3 way intersections has a complexity of 3 + 2 + 2 = 7. 
- ***m*** - memory, the porters ability to recall their remembered path. As ***c*** approaches ***m***, the branches become equally weighted. Determines the upper bound for ***c***.