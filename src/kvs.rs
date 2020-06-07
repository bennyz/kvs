
fn main() {
    let matches = App::new("kvs")
    .version("0.0.1")
    .subcommand(SubCommand::with_name("get")
                .arg(Arg::with_name("key")
                .help("key to get")))
    .subcommand(SubCommand::with_name("set")
                .arg(Arg::with_name("key")
                .help("key to set")))
                .arg(Arg::with_name("value")
                .help("value to set for key"))
    .subcommand(SubCommand::with_name("remove")
                .arg(Arg::with_name("key")
                .help("key to remove")))
    .get_matches();

    if let Some(matches) = matches.subcommand_matches("get") {
        if matches.is_present("key") {
            println!("Printing debug info...");
        }
    }    
}
