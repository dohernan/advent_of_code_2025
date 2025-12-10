extern crate itertools;
extern crate itertools_num;
use core::f64;
use geo::Contains;
use geo::CoordinatePosition;
use geo::coordinate_position::CoordPos;
use geo_types::{Coord, LineString, Polygon, polygon}; // Add this import
use std::fs;

#[derive(Debug, Clone, Default, PartialEq)]
struct Point(f64, f64);
impl Point {
    fn distance(&self, x: f64, y: f64) -> f64 {
        let dx = self.0 - x;
        let dy = self.1 - y;
        (dx * dx + dy * dy).sqrt()
    }
    fn distance_to_x0_y0(&self) -> f64 {
        self.distance(0., 0.)
    }
    fn distance_to_x0_ymax(&self) -> f64 {
        self.distance(0., 99999999.)
    }
    fn distance_to_xmax_y0(&self) -> f64 {
        self.distance(99999999., 0.)
    }
    fn distance_to_xmax_ymax(&self) -> f64 {
        self.distance(99999999., 99999999.)
    }
    fn area_with(&self, other: &Point) -> f64 {
        (f64::abs(self.0 - other.0) + 1.) * (f64::abs(self.1 - other.1) + 1.)
    }
}
#[derive(Debug, Clone, Default)]
struct Grid {
    points: Vec<Point>,
    roi: Vec<Point>,
}
impl Grid {
    fn calculate_roi(&mut self) {
        self.points
            .sort_by_key(|point| (point.0 as u64, point.1 as u64));
        let mut y_start: f64 = self.points[0].1;
        let mut y_end = self.points[1].1;
        let mut x_start = self.points[1].0;

        for i in 1..self.points.len() / 2 {
            let point1: Point = self.points[2 * i].clone();
            let point2: Point = self.points[2 * i + 1].clone();
            for x in x_start as u128..point1.0 as u128 {
                self.add_to_roi(x as f64, y_start, y_end);
            }

            if point2.1 == y_end {
                y_end = point1.1
            } else if point2.1 == y_start {
                y_start = point1.1
            } else if point1.1 == y_end {
                y_end = point2.1
            } else if point1.1 == y_start {
                y_start = point2.1
            }
            x_start = point1.0;
        }

        self.add_to_roi(x_start, y_start, self.points.last().unwrap().1);
    }

    fn add_to_roi(&mut self, x: f64, y_start: f64, y_end: f64) {
        for i in y_start as u128..=y_end as u128 {
            self.roi.push(Point(x, i as f64));
        }
    }
    fn get_max_area(&self) -> f64 {
        let x0_y0_point = self.points.iter().fold(
            Point(f64::INFINITY, f64::INFINITY),
            |point1: Point, point2: &Point| {
                if point1.distance_to_x0_y0() < point2.distance_to_x0_y0() {
                    point1
                } else {
                    point2.clone()
                }
            },
        );
        let x0_ymax_point = self.points.iter().fold(
            Point(f64::INFINITY, f64::INFINITY),
            |point1: Point, point2: &Point| {
                if point1.distance_to_x0_ymax() < point2.distance_to_x0_ymax() {
                    point1
                } else {
                    point2.clone()
                }
            },
        );
        let xmax_y0_point = self.points.iter().fold(
            Point(f64::INFINITY, f64::INFINITY),
            |point1: Point, point2: &Point| {
                if point1.distance_to_xmax_y0() < point2.distance_to_xmax_y0() {
                    point1
                } else {
                    point2.clone()
                }
            },
        );
        let xmax_ymax_point = self.points.iter().fold(
            Point(f64::INFINITY, f64::INFINITY),
            |point1: Point, point2: &Point| {
                if point1.distance_to_xmax_ymax() < point2.distance_to_xmax_ymax() {
                    point1
                } else {
                    point2.clone()
                }
            },
        );

        f64::max(
            xmax_ymax_point.area_with(&x0_y0_point),
            x0_ymax_point.area_with(&xmax_y0_point),
        )
    }

    fn is_in_roi(&self, point1: &Point, point2: &Point) -> bool {
        for x in point1.0 as u128..=point2.0 as u128 {
            for y in point1.1 as u128..=point2.1 as u128 {}
        }

        self.points.iter().any(|point| {
            point.0 < point1.0.max(point2.0)
                && point.0 > point1.0.min(point2.0)
                && point.1 < point1.1.max(point2.1)
                && point.1 > point1.1.min(point2.1)
        })
    }

    fn get_max_area2(&self) -> f64 {
        let mut max_area: f64 = 0.;
        for point1 in &self.points {
            for point2 in &self.points {
                if point2.0 > point1.0 {
                    continue;
                }
                if point1 != point2 && self.is_in_roi(point1, point2) {
                    max_area = max_area.max(point1.area_with(point2));
                }
            }
        }
        max_area
    }
}

impl From<Vec<Point>> for Grid {
    fn from(points: Vec<Point>) -> Self {
        let mut grid = Grid {
            points,
            roi: vec![],
        };
        grid.calculate_roi();
        grid
    }
}

fn main() {
    let input_string = fs::read_to_string("data/input.txt").expect("Failed to read input.txt");
    // let grid = Grid::from(
    //     input_string
    //         .lines()
    //         .map(|line| {
    //             let nums: Vec<f64> = line
    //                 .split(',')
    //                 .map(|number| number.parse::<f64>().unwrap())
    //                 .collect::<Vec<f64>>();
    //             Point(nums[0], nums[1])
    //         })
    //         .collect::<Vec<Point>>(),
    // );

    let points: Vec<(f64, f64)> = input_string
        .lines()
        .map(|line| {
            let nums: Vec<f64> = line
                .split(',')
                .map(|number| number.parse::<f64>().unwrap())
                .collect::<Vec<f64>>();
            (nums[0], nums[1])
        })
        .collect::<Vec<(f64, f64)>>();

    let mut coords: Vec<Coord<f64>> = points
        .iter()
        .map(|p: &(f64, f64)| Coord { x: p.0, y: p.1 })
        .collect();
    coords.push(Coord {
        x: points.first().unwrap().0,
        y: points.first().unwrap().1,
    });
    let polygon = Polygon::new(LineString::from(coords), vec![]);
    let mut max_area = 0.;
    // for point1 in &points {
    //     for point2 in &points {
    for i in  0..&points {
        for point2 in &points {
            if point1 == point2 {
                continue;
            }
            let point1 = &points[i];
            let point2 = &points[j];
            let rect_coords = vec![
                Coord {
                    x: point1.0,
                    y: point1.1,
                },
                Coord {
                    x: point2.0,
                    y: point1.1,
                },
                Coord {
                    x: point2.0,
                    y: point2.1,
                },
                Coord {
                    x: point1.0,
                    y: point2.1,
                },
                Coord {
                    x: point1.0,
                    y: point1.1,
                },
            ];
            let rect_polygon = Polygon::new(LineString::from(rect_coords), vec![]);

            if polygon.contains(&rect_polygon) {
                let p1 = Point(point1.0, point1.1);
                let p2 = Point(point2.0, point2.1);
                let area = p1.area_with(&p2);
                if area > max_area {
                    max_area = dbg!(area);
                }
            }
        }
    }
    println!("{}", max_area);
}
