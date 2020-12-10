extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("kvs")
        .version("1.0")
        .arg(Arg::with_name("V").short("V"))
        .subcommand(SubCommand::with_name("get").arg(Arg::with_name("KEY").required(true).index(1)))
        .subcommand(
            SubCommand::with_name("set")
                .arg(Arg::with_name("KEY").required(true).index(1))
                .arg(Arg::with_name("VALUE").required(true).index(2)),
        )
        .subcommand(SubCommand::with_name("rm").arg(Arg::with_name("KEY").required(true).index(1)))
        .get_matches();

    if matches.occurrences_of("V") > 0 {
        println!("0.1.0");
        std::process::exit(0);
    } else if let Some(_) = matches.subcommand_matches("get") {
        eprintln!("unimplemented");
        std::process::exit(1);
    } else if let Some(_) = matches.subcommand_matches("rm") {
        eprintln!("unimplemented");
        std::process::exit(1);
    } else if let Some(_) = matches.subcommand_matches("set") {
        eprintln!("unimplemented");
        std::process::exit(1);
    } else {
        std::process::exit(1);
    }
}
