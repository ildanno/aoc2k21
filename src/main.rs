mod input;
mod day_01;
mod day_02;

fn main() {
    let day = 2;

    match day {
        1 => day_01::print_solution(),
        2 => day_02::print_solution(),
        _ => println!("Day {} is not implemented", day)
    }
}
