use clap::{Parser, Subcommand};
use cangaroo_core::list_ifaces;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// enables debug logging
    #[arg(long)]
    debug: bool
}

#[derive(Subcommand)]
enum Commands {
    /// lists interfaces and their status
    Status {
        #[arg(short)]
        interface: Option<String>
    },
    /// watches an interface
    Watch {
        #[arg(short)]
        interface: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        println!("Debug logging enabled!")
    }

    match &cli.command {
        Some(Commands::Status { interface }) => {
            match &interface {
                Some(interface) => {
                    println!("Displaying status of {interface}")
                }
                None => {
                    let ifaces_res = list_ifaces();
                    let ifaces = match ifaces_res {
                        Ok(ifaces) => {
                            ifaces
                        },
                        Err(e) => panic!("Problem accessing interfaces: {e:?}")
                    };
                    println!("Displaying status of all interfaces:");
                    if ifaces.is_empty() { println!("No CAN interfaces found"); }
                    for i in ifaces {
                        println!("{i}");
                    }
                }
            }
        }
        Some(Commands::Watch { interface }) => {
            println!("Watching interface {interface}")
        }
        None => {}
    }
}
