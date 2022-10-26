use clap::{arg, command, ArgAction};

fn main() {
    let matches = command!() // requires `cargo` feature
        .next_line_help(true)
        .arg(arg!(--f --File <VALUE>).required(true).action(ArgAction::Set))
        .arg(arg!(--pt  --proxy <VALUE>).required(true).action(ArgAction::Set))
        .get_matches();

    println!(
        "--File {:?}",
        matches.get_one::<String>("f").expect("required")
    );
    println!(
        "--pt : {:?}",
        matches.get_one::<String>("pt").expect("required")
    );
}