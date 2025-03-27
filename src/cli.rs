use clap::{Arg, ArgAction, Command};
pub struct Args {
    pub start: Option<String>,
    pub end: Option<String>,
    pub verbose: u8,
}

pub fn parse_args() -> Args {
    let matches = Command::new("WikiGame")
        .version("1.0")
        .author("Петров Владислав vlforolymp1@gmail.com")
        .about("CLI-игра для поиска пути между статьями Википедии")
        .arg(
            Arg::new("start")
                .short('s')
                .long("start")
                .value_name("START")
                .help("Начальная статья Википедии"),
        )
        .arg(
            Arg::new("end")
                .short('e')
                .long("end")
                .value_name("END")
                .help("Конечная статья Википедии"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .action(ArgAction::Count)
                .help("Устанавливает уровень детализации логирования"),
        )
        .get_matches();

    Args {
        start: matches.get_one::<String>("start").cloned(),
        end: matches.get_one::<String>("end").cloned(),
        verbose: matches.get_count("verbose"),
    }
}
