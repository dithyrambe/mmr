use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "Say smth")]
    Say {
        #[arg()]
        string: String,
        #[arg(short, long, default_value_t = 1, help = "How many times")]
        times: u32,
    },
    #[command(about = "Write smth in a certain way")]
    Write {
        #[arg()]
        string: String,
        #[arg(short, long, default_value_t = 1, help = "How many times")]
        times: u32,
        #[arg(short, long)]
        caps: bool,
    },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Say { string, times } => {
            for _ in 0..times {
                println!("I say {}", string);
            }
        }
        Commands::Write {
            string,
            times,
            caps,
        } => {
            let output = if caps { string.to_uppercase() } else { string };
            for _ in 0..times {
                println!("I write {}", output);
            }
        }
    }
}
