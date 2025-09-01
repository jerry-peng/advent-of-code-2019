use super::inputs::read_day_input;
use super::problem::Problem;
use crate::intcode::program::Program;

pub struct Day2 {}

impl Problem for Day2 {
    fn part_one(&self) -> String {
        let input = read_day_input(2);
        let mut program = Program::new(input);

        // overwrite data specified by the challenge
        program.write_data(1, 12);
        program.write_data(2, 2);
        program.run();
        program.read_data(0).to_string()
    }

    fn part_two(&self) -> String {
        let input = read_day_input(2);

        // Brute-force 0-99 for both noun/verb (index 1/2 in program data);
        // hopefully answer presents itself
        for noun in 0..100 {
            for verb in 0..100 {
                let mut program = Program::new(input.clone());
                program.write_data(1, noun);
                program.write_data(2, verb);
                program.run();
                if program.read_data(0) == 19690720 {
                    // Answer: 3146
                    return (100 * noun + verb).to_string();
                }
            }
        }
        panic!("noun/verb pair not found");
    }
}

#[cfg(test)]
mod tests {
    // tested in intcode program
}
