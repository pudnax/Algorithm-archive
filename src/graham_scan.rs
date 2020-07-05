#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl From<&[i64; 2]> for Point {
    fn from(point: &[i64; 2]) -> Point {
        Point {
            x: point[0] as f64,
            y: point[1] as f64,
        }
    }
}

fn ccw(a: Point, b: Point, c: Point) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn graham_scan(points: &mut Vec<Point>) -> Vec<Point> {
    let mut low_index = 0;
    for i in 1..points.len() {
        if points[i].y < points[low_index].y {
            low_index = i;
        }
    }
    points.swap(0, low_index);
    let pivot = points[0];

    points.sort_by(|&a, &b| ccw(pivot, a, b).partial_cmp(&0.).unwrap());

    let mut m = 1;

    for mut i in 2..points.len() {
        while ccw(points[m - 1], points[m], points[i]) <= 0. {
            if m > 1 {
                m -= 1;
                continue;
            } else if i == points.len() {
                break;
            } else {
                i += 1;
            }
        }
        m += 1;
        points.swap(i, m);
    }

    points[0..=m].to_vec()
}

fn main() {
    let mut points: Vec<Point> = [
        [-5, 2],
        [5, 7],
        [-6, -12],
        [-14, -14],
        [9, 9],
        [-1, -1],
        [-10, 11],
        [-6, 15],
        [-6, -8],
        [15, -9],
        [7, -7],
        [-2, -9],
        [6, -5],
        [0, 14],
        [2, 8],
    ]
    .iter()
    .map(|x| x.into())
    .collect();
    println!("original points are as follows:\n");
    println!("{:?}", points);
    let hull = graham_scan(&mut points);
    println!("points in hull are as follows:\n");
    println!("{:?}", hull);
}
