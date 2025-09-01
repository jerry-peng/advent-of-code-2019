//! IntCode program implementation

#[derive(Debug, PartialEq)]
/// IntCode program
///
/// * `data`: program data (integer vec)
/// * `counter`: program counter
pub struct Program {
    data: Vec<i64>,
    counter: usize,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    // Add instruction (opcode: 1, # parameters: 3)
    Add {
        src_index_1: usize,  // source index 1
        src_index_2: usize,  // source index 2
        result_index: usize, // sum index
    },
    // Multiply instruction (opcode: 2, # parameters: 3)
    Mult {
        src_index_1: usize,  // source index 1
        src_index_2: usize,  // source index 2
        result_index: usize, // product index
    },
    // Program halt instruction (opcode: 99, # parameters: 0)
    Halt,
}

impl Program {
    /// Ingest input and create new program
    ///
    /// * `input`: program data string (comma-delimited integers)
    pub fn new(input: String) -> Program {
        Program {
            data: input
                .trim()
                .split(",")
                .map(|item| str::parse::<i64>(item).unwrap())
                .collect(),
            counter: 0,
        }
    }

    /// Write program data
    ///
    /// * `index`: data index
    /// * `value`: value to write
    pub fn write_data(&mut self, index: usize, value: i64) {
        let mut_ref = &mut self.data[index];
        *mut_ref = value;
    }

    /// Read program data
    ///
    /// * `index`: data index
    pub fn read_data(&mut self, index: usize) -> i64 {
        self.data[index]
    }

    /// Run program
    pub fn run(&mut self) {
        loop {
            // read instruction, halt program if halt instruction encountered
            let instruction = self.read_instruction();
            if instruction == Instruction::Halt {
                break;
            }

            // execute instruction and advance program counter
            self.execute_instruction(&instruction);
            self.counter += Program::get_instruction_size(&instruction);
        }
    }

    /// Execute instruction
    ///
    /// * `instruction`: instruction
    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Add {
                src_index_1,
                src_index_2,
                result_index,
            } => {
                self.data[*result_index] = self.data[*src_index_1] + self.data[*src_index_2];
            }
            Instruction::Mult {
                src_index_1,
                src_index_2,
                result_index,
            } => {
                self.data[*result_index] = self.data[*src_index_1] * self.data[*src_index_2];
            }
            Instruction::Halt => unreachable!("Program is halted, no instruction to execute"),
        }
    }

    /// Read instruction
    fn read_instruction(&mut self) -> Instruction {
        let counter = self.counter;
        let opcode = self.data[counter];

        match opcode {
            1 => Instruction::Add {
                src_index_1: usize::try_from(self.data[counter + 1]).unwrap(),
                src_index_2: usize::try_from(self.data[counter + 2]).unwrap(),
                result_index: usize::try_from(self.data[counter + 3]).unwrap(),
            },
            2 => Instruction::Mult {
                src_index_1: usize::try_from(self.data[counter + 1]).unwrap(),
                src_index_2: usize::try_from(self.data[counter + 2]).unwrap(),
                result_index: usize::try_from(self.data[counter + 3]).unwrap(),
            },
            99 => Instruction::Halt,
            _ => panic!("Unrecognizable opcode"),
        }
    }

    /// Get instruction size from instruction
    ///
    /// * `instruction`: instruction
    fn get_instruction_size(instruction: &Instruction) -> usize {
        match instruction {
            Instruction::Add { .. } => 4,
            Instruction::Mult { .. } => 4,
            Instruction::Halt => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() {
        let mut program = Program::new(String::from("1,9,10,3,2,3,11,0,99,30,40,50"));
        program.run();
        assert_eq!(
            program.data,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        let mut program = Program::new(String::from("1,0,0,0,99"));
        program.run();
        assert_eq!(program.data, vec![2, 0, 0, 0, 99]);

        let mut program = Program::new(String::from("2,3,0,3,99"));
        program.run();
        assert_eq!(program.data, vec![2, 3, 0, 6, 99]);

        let mut program = Program::new(String::from("2,4,4,5,99,0"));
        program.run();
        assert_eq!(program.data, vec![2, 4, 4, 5, 99, 9801]);

        let mut program = Program::new(String::from("1,1,1,4,99,5,6,0,99"));
        program.run();
        assert_eq!(program.data, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
