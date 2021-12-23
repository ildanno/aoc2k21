mod input;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

fn main() {
    let day = 9;

    match day {
        1 => day_01::print_solution(),
        2 => day_02::print_solution(),
        3 => day_03::print_solution(),
        4 => day_04::print_solution(),
        5 => day_05::print_solution(),
        6 => day_06::print_solution(),
        7 => day_07::print_solution(),
        8 => day_08::print_solution(),
        9 => day_09::print_solution(),
        _ => println!("Day {} is not implemented", day)
    }
}
