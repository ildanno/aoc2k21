mod input;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

fn main() {
    let day = 5;

    match day {
        1 => day_01::print_solution(),
        2 => day_02::print_solution(),
        3 => day_03::print_solution(),
        4 => day_04::print_solution(),
        5 => day_05::print_solution(),
        _ => println!("Day {} is not implemented", day)
    }
}
