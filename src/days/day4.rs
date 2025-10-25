use std::ops::RangeInclusive;

use super::inputs::read_day_input;
use super::problem::Problem;

pub struct Day4 {}

impl Problem for Day4 {
    fn part_one(&self) -> String {
        // parse password range
        let input = read_day_input(4);
        let range = parse_range(input);
        let mut valid_password_count = 0;

        // - since subsequent digits cannot decrease, each digit iterate from previous
        // digit value.
        // - if double is detected, propagate state to next digit
        for digit1 in 1..=9usize {
            for digit2 in digit1..=9 {
                let has_double = digit1 == digit2;

                for digit3 in digit2..=9 {
                    let has_double = has_double || digit2 == digit3;

                    for digit4 in digit3..=9 {
                        let has_double = has_double || digit3 == digit4;

                        for digit5 in digit4..=9 {
                            let has_double = has_double || digit4 == digit5;

                            // - at fifth digit, if double detected, loop all possible value for 6th
                            // digit. Otherwise, 6th digit must equal 5th digit.
                            // - if number is within range, increment valid password count.
                            if has_double {
                                for digit6 in digit5..=9 {
                                    let password = digit1 * 100000
                                        + digit2 * 10000
                                        + digit3 * 1000
                                        + digit4 * 100
                                        + digit5 * 10
                                        + digit6;
                                    if range.contains(&password) {
                                        valid_password_count += 1;
                                    }
                                }
                            } else {
                                let password = digit1 * 100000
                                    + digit2 * 10000
                                    + digit3 * 1000
                                    + digit4 * 100
                                    + digit5 * 10
                                    + digit5;
                                if range.contains(&password) {
                                    valid_password_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        valid_password_count.to_string()
    }

    fn part_two(&self) -> String {
        let input = read_day_input(4);
        let range = parse_range(input);
        let mut valid_password_count = 0;

        // - since subsequent digits cannot decrease, each digit iterate from previous
        // digit value.
        // - to detect double that is not part of larger group of repeat digits, record digit
        // repeat count, if count is 2 and next digit is not a repeat, double is detected
        for digit1 in 1..=8usize {
            for digit2 in digit1..=8 {
                let digit_repeat_count = if digit1 == digit2 { 2 } else { 1 };

                for digit3 in digit2..=9 {
                    let has_double = digit_repeat_count == 2 && digit2 != digit3;
                    let digit_repeat_count = if digit2 == digit3 {
                        digit_repeat_count + 1
                    } else {
                        1
                    };

                    for digit4 in digit3..=9 {
                        let has_double =
                            has_double || (digit_repeat_count == 2 && digit3 != digit4);
                        let digit_repeat_count = if digit3 == digit4 {
                            digit_repeat_count + 1
                        } else {
                            1
                        };

                        for digit5 in digit4..=9 {
                            let has_double =
                                has_double || (digit_repeat_count == 2 && digit4 != digit5);
                            let digit_repeat_count = if digit4 == digit5 {
                                digit_repeat_count + 1
                            } else {
                                1
                            };

                            // - at fifth digit:
                            //   - if double detected, loop all possible value for 6th digit
                            //   - if digit repeat count is 2, 6th digit cannot equal 5th digit
                            //   - if digit repeat count is 1, 6th digit must equal 5th digit
                            // - if number is within range, increment valid password count.
                            if has_double {
                                for digit6 in digit5..=9 {
                                    let password = digit1 * 100000
                                        + digit2 * 10000
                                        + digit3 * 1000
                                        + digit4 * 100
                                        + digit5 * 10
                                        + digit6;
                                    if range.contains(&password) {
                                        valid_password_count += 1;
                                    }
                                }
                            } else if digit_repeat_count == 2 {
                                for digit6 in (digit5 + 1)..=9 {
                                    let password = digit1 * 100000
                                        + digit2 * 10000
                                        + digit3 * 1000
                                        + digit4 * 100
                                        + digit5 * 10
                                        + digit6;
                                    if range.contains(&password) {
                                        valid_password_count += 1;
                                    }
                                }
                            } else if digit_repeat_count == 1 {
                                let password = digit1 * 100000
                                    + digit2 * 10000
                                    + digit3 * 1000
                                    + digit4 * 100
                                    + digit5 * 10
                                    + digit5;
                                if range.contains(&password) {
                                    valid_password_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        valid_password_count.to_string()
    }
}

/// Parse range from input
///
/// * `input`: input string
fn parse_range(input: String) -> RangeInclusive<usize> {
    let range: Vec<&str> = input.trim().split("-").collect();
    let start = str::parse::<usize>(range[0]).unwrap();
    let end = str::parse::<usize>(range[1]).unwrap();
    start..=end
}

#[cfg(test)]
mod tests {}
