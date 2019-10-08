#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn cross(self, vec: Point) -> f64 {
        self.x * vec.y - self.y * vec.x
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<&[f64; 2]> for Point {
    fn from(point: &[f64; 2]) -> Point {
        Point {
            x: point[0],
            y: point[1],
        }
    }
}

fn jarvis_march(points: &[Point]) -> Vec<Point> {
    let mut hull_points = Vec::new();

    if points.is_empty() {
        return hull_points;
    }

    let first_point_it = points
        .iter()
        .min_by(|a, b| (a.x - b.x).partial_cmp(&0.).unwrap())
        .unwrap();

    let mut next_point_it = first_point_it;

    loop {
        hull_points.push(*next_point_it);

        let p1 = hull_points.last().unwrap();

        next_point_it = points
            .iter()
            .max_by(|&&p2, &&p3| {
                if *p1 == p2 || (p2 - *p1).cross(p3 - *p1) > 0. {
                    return std::cmp::Ordering::Less;
                }
                std::cmp::Ordering::Greater
            })
            .unwrap();

        if next_point_it == first_point_it {
            break;
        }
    }

    hull_points
}

fn main() {
    let points: Vec<Point> = [
        [1.0, 3.0],
        [1.0, 3.0],
        [2.0, 4.0],
        [4.0, 0.0],
        [1.0, 0.0],
        [0.0, 2.0],
        [2.0, 2.0],
        [3.0, 4.0],
        [3.0, 1.0],
    ]
    .iter()
    .map(|x| x.into())
    .collect();

    let hull_points = jarvis_march(&points);

    println!("Hull points are:");
    for point in hull_points {
        println!("({}, {})", point.x, point.y);
    }
}
