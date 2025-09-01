use super::inputs::read_day_input;
use super::problem::Problem;

pub struct Day1 {}

impl Problem for Day1 {
    fn part_one(&self) -> String {
        let input = read_day_input(1);
        let total_module_mass: usize = input
            .lines()
            .map(|line| str::parse::<usize>(line).unwrap())
            .map(get_mass)
            .sum::<usize>();
        format!("{}", total_module_mass)
    }

    fn part_two(&self) -> String {
        let input = read_day_input(1);
        let module_masses: Vec<usize> = input
            .lines()
            .map(|line| str::parse::<usize>(line).unwrap())
            .map(get_mass)
            .collect();

        // need combined total module mass and fuel mass
        let total_module_mass: usize = module_masses.iter().sum();
        let total_fuel_mass: usize = module_masses.iter().map(get_total_fuel_mass).sum::<usize>();
        let total_mass = total_module_mass + total_fuel_mass;

        format!("{}", total_mass)
    }
}

fn get_mass(num_modules: usize) -> usize {
    // get mass from number of modules, underflow defaults to 0
    (num_modules / 3).saturating_sub(2)
}

fn get_total_fuel_mass(mass: &usize) -> usize {
    // recursively add fuel mass, as added fuel itself needs extra fuel
    let mut total_fuel_mass = 0;
    let mut extra_fuel_mass = get_fuel_mass(*mass);
    loop {
        if extra_fuel_mass != 0 {
            total_fuel_mass += extra_fuel_mass;
        } else {
            return total_fuel_mass;
        }
        extra_fuel_mass = get_fuel_mass(extra_fuel_mass);
    }
}

fn get_fuel_mass(mass: usize) -> usize {
    // get fuel mass from module mass, underflow defaults to 0
    (mass / 3).saturating_sub(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mass() {
        assert_eq!(get_mass(12), 2);
        assert_eq!(get_mass(14), 2);
        assert_eq!(get_mass(1969), 654);
        assert_eq!(get_mass(100756), 33583);
    }

    #[test]
    fn test_get_total_fuel_mass() {
        assert_eq!(get_total_fuel_mass(&14), 2);
        assert_eq!(get_total_fuel_mass(&1969), 966);
        assert_eq!(get_total_fuel_mass(&100756), 50346);
    }
}
