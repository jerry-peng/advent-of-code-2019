use std::vec;

use super::inputs::read_day_input;
use super::problem::Problem;

pub struct Day3 {}

impl Problem for Day3 {
    fn part_one(&self) -> String {
        let input = read_day_input(3);
        let lines: Vec<&str> = input.trim().lines().collect();
        let (line1, line2) = (lines[0], lines[1]);

        let distance = get_shortest_dist_intersect(line1, line2);
        distance.to_string()
    }

    fn part_two(&self) -> String {
        let input = read_day_input(3);
        let lines: Vec<&str> = input.trim().lines().collect();
        let (line1, line2) = (lines[0], lines[1]);

        let signal_delay = get_shortest_delay_intersect(line1, line2);
        signal_delay.to_string()
    }
}

#[derive(Debug)]
/// Wire
///
/// * `fixed_axis`: wire is either horizontal (fixed y-axis), or vertical (fixed x-axis),
///                 this value represents the fixed value.
/// * `var_axis_start`: Variable axis start of trace
/// * `var_axis_end`: Variable axis end of trace
/// * `signal_delay`: Signal delay at start of trace
struct Wire {
    fixed_axis: i64,
    var_axis_start: i64,
    var_axis_end: i64,
    signal_delay: i64,
}

#[derive(Debug)]
/// Circuit consists of horizontal and vertical wires
///
/// * `horizontal_wires`: horizontal wire vec
/// * `vertical_wires`: vertical wire vec
struct Circuit {
    horizontal_wires: Vec<Wire>,
    vertical_wires: Vec<Wire>,
}

#[derive(Debug)]
enum TraceDirection {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
/// Coordinate
///
/// * `x`: x value
/// * `y`: y value
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Debug)]
/// Intersection between wires
///
/// * `coor`: coordinate
/// * `signal_delay`: signal delay
struct Intersection {
    coor: Coordinate,
    signal_delay: i64,
}

/// Get intersection with shortest manhattan distance to origin
///
/// * `line1`: input trace string of first circuit
/// * `line2`: input trace string of second circuit
fn get_shortest_dist_intersect(line1: &str, line2: &str) -> usize {
    // get traces
    let traces1: Vec<String> = line1.split(",").map(str::to_string).collect();
    let traces2: Vec<String> = line2.split(",").map(str::to_string).collect();

    // create circuits
    let circuit1 = create_circuit(traces1);
    let circuit2 = create_circuit(traces2);

    // get all intersections, then find intersection with shortest distance
    // and return distance
    let intersections = get_intersections(circuit1, circuit2);
    usize::try_from(
        intersections
            .into_iter()
            .map(|intersect| intersect.coor)
            .filter(|coor| coor.x != 0 && coor.y != 0)
            .map(|coor| coor.x.abs() + coor.y.abs())
            .min()
            .unwrap(),
    )
    .unwrap()
}

/// Get intersection with shortest combined signal delay
///
/// * `line1`: input trace string of first circuit
/// * `line2`: input trace string of second circuit
fn get_shortest_delay_intersect(line1: &str, line2: &str) -> usize {
    // get traces
    let traces1: Vec<String> = line1.split(",").map(str::to_string).collect();
    let traces2: Vec<String> = line2.split(",").map(str::to_string).collect();

    // create circuits
    let circuit1 = create_circuit(traces1);
    let circuit2 = create_circuit(traces2);

    // get all intersections, then find intersection with smallest combined signal delay
    // and return the combined delay
    let intersections = get_intersections(circuit1, circuit2);
    usize::try_from(
        intersections
            .into_iter()
            .map(|intersect| intersect.signal_delay)
            .filter(|signal_delay| *signal_delay != 0)
            .min()
            .unwrap(),
    )
    .unwrap()
}

/// Create circuit from traces
///
/// * `traces`: traces
fn create_circuit(traces: Vec<String>) -> Circuit {
    let mut coor = Coordinate { x: 0, y: 0 };
    let mut signal_delay = 0;
    let mut circuit = Circuit {
        horizontal_wires: Vec::new(),
        vertical_wires: Vec::new(),
    };

    for trace in traces {
        // parse direction and distance from trace
        let (direction, distance) = parse_trace(trace);

        // based on direction, add wire to respective horizontal/vertical wire vec
        // Note: variable-axis start can be larger than end. The start/end means start/end of
        // trace.
        match direction {
            TraceDirection::Right => {
                let wire = Wire {
                    fixed_axis: coor.y,
                    var_axis_start: coor.x,
                    var_axis_end: coor.x + distance,
                    signal_delay,
                };
                circuit.horizontal_wires.push(wire);
            }
            TraceDirection::Left => {
                let wire = Wire {
                    fixed_axis: coor.y,
                    var_axis_start: coor.x,
                    var_axis_end: coor.x - distance,
                    signal_delay,
                };
                circuit.horizontal_wires.push(wire);
            }
            TraceDirection::Up => {
                let wire = Wire {
                    fixed_axis: coor.x,
                    var_axis_start: coor.y,
                    var_axis_end: coor.y + distance,
                    signal_delay,
                };
                circuit.vertical_wires.push(wire);
            }
            TraceDirection::Down => {
                let wire = Wire {
                    fixed_axis: coor.x,
                    var_axis_start: coor.y,
                    var_axis_end: coor.y - distance,
                    signal_delay,
                };
                circuit.vertical_wires.push(wire);
            }
        }

        // Update current signal delay and coordinates
        signal_delay += distance;
        match direction {
            TraceDirection::Right => coor.x += distance,
            TraceDirection::Left => coor.x -= distance,
            TraceDirection::Up => coor.y += distance,
            TraceDirection::Down => coor.y -= distance,
        }
    }

    circuit
}

/// Parse trace from trace input string
///
/// * `trace`: trace input string
fn parse_trace(trace: String) -> (TraceDirection, i64) {
    let mut chars = trace.chars();

    // first char is direction (R/L/U/D)
    let direction = chars.next().unwrap();
    // rest of the string is distance travelled
    let distance: String = chars.collect();

    // parse direction into enum
    let direction = match direction {
        'R' => TraceDirection::Right,
        'L' => TraceDirection::Left,
        'U' => TraceDirection::Up,
        'D' => TraceDirection::Down,
        _ => panic!("Unknown direction"),
    };
    // parse distance into number
    let distance = str::parse::<i64>(&distance).unwrap();

    (direction, distance)
}

/// Get intersections when given 2 circuits
///
/// * `circuit1`: circuit 1
/// * `circuit2`: circuit 2
fn get_intersections(circuit1: Circuit, circuit2: Circuit) -> Vec<Intersection> {
    let mut intersections: Vec<Intersection> = Vec::new();

    // first calculate intersections between circuit 1's horizontal and circuit 2's vertical wires,
    // then vice versa.
    for (first_circuit, second_circuit) in [(&circuit1, &circuit2), (&circuit2, &circuit1)] {
        for horizontal_wire in &first_circuit.horizontal_wires {
            for vertical_wire in &second_circuit.vertical_wires {
                // record horizontal/vertical variable-axis start/end
                let horizontal_start = horizontal_wire.var_axis_start;
                let horizontal_end = horizontal_wire.var_axis_end;
                let vertical_start = vertical_wire.var_axis_start;
                let vertical_end = vertical_wire.var_axis_end;

                // Get the variable axis range of horizontal/vertical wires
                let horizontal_range = if horizontal_start < horizontal_end {
                    horizontal_start..=horizontal_end
                } else {
                    horizontal_end..=horizontal_start
                };

                let vertical_range = if vertical_start < vertical_end {
                    vertical_start..=vertical_end
                } else {
                    vertical_end..=vertical_start
                };

                // if vertical wire's fixes axis value is within horizontal wire's
                // variable axis range, and vice versa, then we have a intersection
                if horizontal_range.contains(&vertical_wire.fixed_axis)
                    && vertical_range.contains(&horizontal_wire.fixed_axis)
                {
                    // calculate signal delays of both wires.
                    // Note: variable-axis start value can be larger than the other wire's fixed
                    // axis value, so we take absolute value of the difference.
                    let signal1 = horizontal_wire.signal_delay
                        + (vertical_wire.fixed_axis - horizontal_wire.var_axis_start).abs();
                    let signal2 = vertical_wire.signal_delay
                        + (horizontal_wire.fixed_axis - vertical_wire.var_axis_start).abs();

                    // push new intersection
                    intersections.push(Intersection {
                        coor: Coordinate {
                            x: vertical_wire.fixed_axis,
                            y: horizontal_wire.fixed_axis,
                        },
                        signal_delay: signal1 + signal2,
                    })
                }
            }
        }
    }

    intersections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_shortest_dist_intersect() {
        assert_eq!(get_shortest_dist_intersect("R8,U5,L5,D3", "U7,R6,D4,L4"), 6);
        assert_eq!(
            get_shortest_dist_intersect(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
        assert_eq!(
            get_shortest_dist_intersect(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn test_get_shortest_delay_intersect() {
        assert_eq!(
            get_shortest_delay_intersect("R8,U5,L5,D3", "U7,R6,D4,L4"),
            30
        );
        assert_eq!(
            get_shortest_delay_intersect(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            610
        );
        assert_eq!(
            get_shortest_delay_intersect(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
