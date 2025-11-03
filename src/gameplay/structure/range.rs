use bevy::prelude::*;

/// A range shape used for area targeting or effect zones.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub enum Range {
    /// A diamond pattern with the given radius. Every point in the pattern has a manhattan distance
    /// that is equal to or less than said radius
    Diamond(i32),
}

impl Range {
    fn pattern_diamond(position: IVec2, range: i32) -> impl Iterator<Item = IVec2> {
        (-range..=range).flat_map(move |dy| {
            let max_dx = range - dy.abs();
            (-max_dx..=max_dx).filter_map(move |dx| {
                let pos = IVec2::new(position.x - dx, position.y - dy);
                (pos != position).then_some(pos)
            })
        })
    }

    /// Returns an iterator that loops over all the points in the range
    pub fn iter(&self, position: IVec2) -> impl Iterator<Item = IVec2> {
        match self {
            Self::Diamond(range) => Self::pattern_diamond(position, *range),
        }
    }

    /// Checks whether a point is contained within anothers radius
    pub fn contains(&self, position: &IVec2, other: &IVec2) -> bool {
        match self {
            Self::Diamond(range) => position.manhattan_distance(*other) <= *range as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_diamond_range_1_iter() {
        let range = Range::Diamond(1);
        let position = IVec2::new(0, 0);

        let result: HashSet<_> = range.iter(position).collect();

        // Expected positions (Manhattan distance <= 1, excluding origin)
        let expected: HashSet<_> = [
            IVec2::new(0, 1),
            IVec2::new(1, 0),
            IVec2::new(0, -1),
            IVec2::new(-1, 0),
        ]
        .into_iter()
        .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_diamond_range_2_iter() {
        let range = Range::Diamond(2);
        let position = IVec2::new(0, 0);

        let result: Vec<_> = range.iter(position).collect();

        // Diamond of radius 2 has 12 tiles excluding center
        assert_eq!(result.len(), 12);

        // Some known included points
        assert!(result.contains(&IVec2::new(0, 2)));
        assert!(result.contains(&IVec2::new(2, 0)));
        assert!(result.contains(&IVec2::new(-1, 1)));

        // Known excluded point (outside diamond)
        assert!(!result.contains(&IVec2::new(2, 2)));
    }

    #[test]
    fn test_contains_true_cases() {
        let range = Range::Diamond(2);
        let origin = IVec2::new(0, 0);

        let inside_points = [
            IVec2::new(1, 1),
            IVec2::new(2, 0),
            IVec2::new(0, -2),
            IVec2::new(-1, 1),
        ];

        for p in inside_points {
            assert!(
                range.contains(&origin, &p),
                "Expected {:?} to be inside diamond of range 2",
                p
            );
        }
    }

    #[test]
    fn test_contains_false_cases() {
        let range = Range::Diamond(2);
        let origin = IVec2::new(0, 0);

        let outside_points = [IVec2::new(2, 1), IVec2::new(3, 0), IVec2::new(-2, -2)];

        for p in outside_points {
            assert!(
                !range.contains(&origin, &p),
                "Expected {:?} to be outside diamond of range 2",
                p
            );
        }
    }

    #[test]
    fn test_iter_excludes_origin() {
        let range = Range::Diamond(3);
        let origin = IVec2::new(5, 5);

        let result: Vec<_> = range.iter(origin).collect();

        assert!(
            !result.contains(&origin),
            "Iterator should not include the origin position itself"
        );
    }

    #[test]
    fn test_iter_positions_match_contains() {
        // Sanity check: all iterator positions must satisfy `contains`
        let range = Range::Diamond(3);
        let origin = IVec2::new(0, 0);

        for pos in range.iter(origin) {
            assert!(
                range.contains(&origin, &pos),
                "Iterator yielded {:?}, but contains() returned false",
                pos
            );
        }
    }
}
