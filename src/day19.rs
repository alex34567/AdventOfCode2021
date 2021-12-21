use lazy_static::lazy_static;
use std::cell::{Cell, RefCell};
use std::cmp::max;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Dimmention {
    X,
    Y,
    Z,
}

impl Dimmention {
    fn to_index(self) -> usize {
        match self {
            Dimmention::X => 0,
            Dimmention::Y => 1,
            Dimmention::Z => 2,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct RotationAxis(Dimmention, i8);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Rotation {
    axis: [RotationAxis; 3],
}

const NULL_ROTATE: Rotation = Rotation {
    axis: [
        RotationAxis(Dimmention::X, 1),
        RotationAxis(Dimmention::Y, 1),
        RotationAxis(Dimmention::Z, 1),
    ],
};

impl Rotation {
    fn combine(self, rhs: Self) -> Self {
        let mut ret = self;
        for (i, rhs_axis) in rhs.axis.iter().enumerate() {
            ret.axis[i] = self.axis[rhs_axis.0.to_index()];
            ret.axis[i].1 *= rhs_axis.1;
        }
        ret
    }
}

#[derive(Debug)]
struct ScannerRef {
    other_scanner: usize,
    rotation: Rotation,
    translation: Point,
}

#[derive(Debug)]
struct Scanner {
    transform_to_0: Cell<Option<(Rotation, Point)>>,
    stations: HashSet<Point>,
    other_scanners: RefCell<Vec<ScannerRef>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point([i32; 3]);

const NULL_POINT: Point = Point([0; 3]);

impl Point {
    fn translate(mut self, rhs: Self) -> Self {
        for (axis, trans_axis) in self.0.iter_mut().zip(rhs.0) {
            *axis += trans_axis
        }
        self
    }

    fn rotate(self, rotation: Rotation) -> Self {
        let mut ret = [0; 3];
        for (i, axis) in rotation.axis.iter().enumerate() {
            ret[i] = self.0[axis.0.to_index()] * (axis.1 as i32)
        }
        Point(ret)
    }

    fn negate(mut self) -> Self {
        for x in self.0.iter_mut() {
            *x *= -1;
        }
        self
    }

    fn taxi_distance(self, rhs: Self) -> i32 {
        let mut distance = 0;
        for (lhs_axis, rhs_axis) in self.0.iter().zip(&rhs.0) {
            distance += (lhs_axis - rhs_axis).abs()
        }
        distance
    }
}

static INPUT: &str = include_str!("input/Day19.txt");

lazy_static! {
    static ref ROTATION_TABLE: &'static [Rotation] = {
        static SIN_COS_OF_90: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut ret = HashSet::new();
        for (yaw_sin, yaw_cos) in &SIN_COS_OF_90 {
            let yaw_x_axis = if *yaw_sin != 0 {
                RotationAxis(Dimmention::Y, *yaw_sin * -1)
            } else {
                RotationAxis(Dimmention::X, *yaw_cos)
            };

            let yaw_y_axis = if *yaw_sin != 0 {
                RotationAxis(Dimmention::X, *yaw_sin)
            } else {
                RotationAxis(Dimmention::Y, *yaw_cos)
            };

            let yaw_rotate = Rotation {
                axis: [yaw_x_axis, yaw_y_axis, RotationAxis(Dimmention::Z, 1)],
            };

            for (pitch_sin, pitch_cos) in &SIN_COS_OF_90 {
                let pitch_x_axis = if *pitch_sin != 0 {
                    RotationAxis(Dimmention::Z, *pitch_sin)
                } else {
                    RotationAxis(Dimmention::X, *pitch_cos)
                };

                let pitch_z_axis = if *pitch_sin != 0 {
                    RotationAxis(Dimmention::X, *pitch_sin * -1)
                } else {
                    RotationAxis(Dimmention::Z, *pitch_cos)
                };

                let pitch_rotate = Rotation {
                    axis: [pitch_x_axis, RotationAxis(Dimmention::Y, 1), pitch_z_axis],
                };

                for (roll_sin, roll_cos) in &SIN_COS_OF_90 {
                    let roll_y_axis = if *roll_sin != 0 {
                        RotationAxis(Dimmention::Z, *roll_sin * -1)
                    } else {
                        RotationAxis(Dimmention::Y, *roll_cos)
                    };

                    let roll_z_axis = if *roll_sin != 0 {
                        RotationAxis(Dimmention::Y, *roll_sin)
                    } else {
                        RotationAxis(Dimmention::Z, *roll_cos)
                    };

                    let roll_rotate = Rotation {
                        axis: [RotationAxis(Dimmention::X, 1), roll_y_axis, roll_z_axis],
                    };
                    ret.insert(yaw_rotate.combine(pitch_rotate).combine(roll_rotate));
                }
            }
        }
        ret.into_iter().collect::<Vec<_>>().leak()
    };
}

fn calc_distance_to_0(
    scanners: &[Scanner],
    curr_scanner: &Scanner,
    curr_rotation: Rotation,
    curr_translate: Point,
) {
    curr_scanner
        .transform_to_0
        .set(Some((curr_rotation, curr_translate)));

    for other_scanner_ref in curr_scanner.other_scanners.borrow().iter() {
        let other_scanner = &scanners[other_scanner_ref.other_scanner];
        if other_scanner.transform_to_0.get().is_some() {
            continue;
        }

        calc_distance_to_0(
            scanners,
            other_scanner,
            other_scanner_ref.rotation.combine(curr_rotation),
            curr_translate.translate(other_scanner_ref.translation.rotate(curr_rotation)),
        )
    }
}

pub fn day19() {
    let mut scanners = Vec::new();
    for scanner in INPUT.trim().split("\n\n") {
        let mut stations = Vec::new();
        for line in scanner.split('\n').skip(1) {
            let station = Point(
                line.split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap(),
            );
            stations.push(station);
        }
        scanners.push(Scanner {
            transform_to_0: Cell::new(None),
            stations: stations.iter().copied().collect::<HashSet<_>>(),
            other_scanners: RefCell::new(Vec::new()),
        })
    }

    for (i, first_scanner) in scanners.iter().enumerate() {
        for (j, second_scanner) in scanners.iter().enumerate() {
            if i == j {
                continue;
            }
            'ref_frame: for first_ref_frame in first_scanner.stations.iter() {
                let first_scanner_stations = first_scanner
                    .stations
                    .iter()
                    .map(|x| x.translate(first_ref_frame.negate()))
                    .collect::<HashSet<_>>();

                for rotation in ROTATION_TABLE.iter() {
                    let second_scanner_ref_points =
                        second_scanner.stations.iter().map(|x| x.rotate(*rotation));

                    for second_ref_frame in second_scanner_ref_points {
                        let second_scanner_stations = second_scanner
                            .stations
                            .iter()
                            .map(|x| x.rotate(*rotation).translate(second_ref_frame.negate()))
                            .collect::<HashSet<_>>();

                        let overlap = first_scanner_stations.intersection(&second_scanner_stations);
                        if overlap.count() >= 12 {
                            first_scanner.other_scanners.borrow_mut().push(ScannerRef {
                                other_scanner: j,
                                rotation: *rotation,
                                translation: first_ref_frame.translate(second_ref_frame.negate()),
                            });
                            break 'ref_frame;
                        }
                    }
                }
            }
        }
    }

    calc_distance_to_0(&scanners, &scanners[0], NULL_ROTATE, NULL_POINT);

    let mut stations = HashSet::new();

    for scanner in &scanners {
        for station in &scanner.stations {
            let trans_to_0 = scanner.transform_to_0.get().unwrap();
            let abs_station = station.rotate(trans_to_0.0).translate(trans_to_0.1);
            stations.insert(abs_station);
        }
    }

    println!("Part1: {}", stations.len());

    let mut part2 = 0;

    for (i, lhs_scanner) in scanners.iter().enumerate() {
        for rhs_scanner in &scanners[i + 1..] {
            let lhs_loc = lhs_scanner.transform_to_0.get().unwrap().1;
            let rhs_loc = rhs_scanner.transform_to_0.get().unwrap().1;
            part2 = max(part2, lhs_loc.taxi_distance(rhs_loc));
        }
    }

    println!("Part2: {}", part2);
}
