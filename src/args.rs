use clap::{arg, Command};

fn cli() -> Command {
    Command::new("pngme")
        .about("PNGs with messages")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("encode")
            .about("Encodes a message in a PNG file")
            .arg(arg!(<PATH> "Path to a PNG file"))
            .arg(arg!(<TYPE> "Chunk type"))
            .arg(arg!(<MESSAGE> "Message that will be set"))
            .arg(arg!(<OUTPUT> "Output PNG file").required(false))
            .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("decode")
            .about("Decodes a message in a PNG file")
            .arg(arg!(<PATH> "Path to a PNG file"))
            .arg(arg!(<TYPE> "Chunk type"))
            .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("remove")
            .about("Removes a chunk type from a PNG file")
            .arg(arg!(<PATH> "Path to a PNG file"))
            .arg(arg!(<TYPE> "Chunk type"))
            .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("print")
                .about("Prints message from a PNG file")
                .arg(arg!(<PATH> "Path to a PNG file"))
                .arg_required_else_help(true),
        )
}

pub fn parse() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("encode", sub_matches)) => {
            println!("encoding {}", sub_matches.get_one::<String>("PATH").expect("required"))
        }
        Some(("decode", sub_matches)) => {
            println!("decoding {}", sub_matches.get_one::<String>("PATH").expect("required"))
        }
        Some(("remove", sub_matches)) => {
            println!("removing {}", sub_matches.get_one::<String>("PATH").expect("required"))
        }
        Some(("print", sub_matches)) => {
            println!("printing {}", sub_matches.get_one::<String>("PATH").expect("required"))
        }
        _ => {
            println!("Invalid command. Use -h for help.")
        }
    }
}
