use clap::{App, Arg};

fn main() -> anyhow::Result<()> {
    // let content = std::fs::read_to_string(&args.path)
    // .with_context(|| format!("could not read file `{}`", path))?;
    //

    let matches = App::new("rit")
        .version("0.1.0")
        .author("keaising <keaising@gmail.com>")
        .subcommand(
            App::new("clone")
                .about("clone git repo")
                .arg(Arg::with_name("url").help("url need clone").required(true)),
        )
        .subcommand(
            App::new("invoke")
                .about("invoke website"),
        )
        .get_matches();

    match matches.subcommand_matches("clone") {
        Some(cmd) => {
            match cmd.value_of("url") {
                Some(url) => { println!("clone {}", url) }
                None => {}
            }
        }
        None => {}
    }


    match matches.subcommand_matches("invoke") {
        Some(_) => { }
        None => {}
    }

    Ok(())
}
