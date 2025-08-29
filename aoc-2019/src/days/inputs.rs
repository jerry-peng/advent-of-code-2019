use std::fs;

/// Read input for specific day, if both parts use same input
///
/// * `day`: day #
pub fn read_day_input(day: usize) -> String {
    let file_path = format!("inputs/day{}.txt", day);
    let file_read_err_msg = format!("Unable to read input file: {}", &file_path);
    fs::read_to_string(&file_path).expect(&file_read_err_msg)
}

/// Read input for specific day/part
///
/// * `day`: day #
/// * `part`: part #
pub fn read_part_input(day: usize, part: usize) -> String {
    let file_path = format!("inputs/day{}_part{}.txt", day, part);
    let file_read_err_msg = format!("Unable to read input file: {}", &file_path);
    fs::read_to_string(&file_path).expect(&file_read_err_msg)
}
