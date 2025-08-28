use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "calc", version, about = "CLI калькулятор на Rust")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add { a: f64, b: f64 },
    Sub { a: f64, b: f64 },
    Mul { a: f64, b: f64 },
    Div { a: f64, b: f64 },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Add { a, b } => a + b,
        Command::Sub { a, b } => a - b,
        Command::Mul { a, b } => a * b,
        Command::Div { a, b } => {
            if b == 0.0 {
                eprintln!("Division by zero");
                std::process::exit(3);
            }
            a / b
        }
    };
    println!("{result}");
}
