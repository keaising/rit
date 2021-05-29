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
            .arg(
                Arg::with_name("url")
                .help("url need clone")
                .required(true),
                ),
            )
        .get_matches();

    let msg = matches.subcommand_matches("clone")
        .unwrap()
        .value_of("url")
        .unwrap();
    println!("{}", msg);

    Ok(())
}
