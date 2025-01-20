use clap::{arg, ArgMatches, Command};

use crate::commands::{decode, encode, print, remove};

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
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("decode")
                .about("Decodes a message in a PNG file")
                .arg(arg!(<PATH> "Path to a PNG file"))
                .arg(arg!(<TYPE> "Chunk type"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("remove")
                .about("Removes a chunk type from a PNG file")
                .arg(arg!(<PATH> "Path to a PNG file"))
                .arg(arg!(<TYPE> "Chunk type"))
                .arg_required_else_help(true),
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
            let path = must_get_param(sub_matches, "PATH");
            let chunk_type = must_get_param(sub_matches, "TYPE");
            let message = must_get_param(sub_matches, "MESSAGE");
            let output = sub_matches.get_one::<String>("OUTPUT");
            encode(path, chunk_type, message, output);
        }
        Some(("decode", sub_matches)) => {
            let path = must_get_param(sub_matches, "PATH");
            let chunk_type = must_get_param(sub_matches, "TYPE");
            decode(path, chunk_type);
        }
        Some(("remove", sub_matches)) => {
            let path = must_get_param(sub_matches, "PATH");
            let chunk_type = must_get_param(sub_matches, "TYPE");
            remove(path, chunk_type);
        }
        Some(("print", sub_matches)) => {
            let path = must_get_param(sub_matches, "PATH");
            print(&path);
        }
        _ => {
            println!("Invalid command. Use -h for help.")
        }
    }
}

fn must_get_param<'a>(sub_matches: &'a ArgMatches, param: &'a str) -> &'a String {
    sub_matches.get_one::<String>(param).expect("required")
}
