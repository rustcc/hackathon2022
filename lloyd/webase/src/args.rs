use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "WebApp", about = "Web Server")]
pub struct CmdArgs {
    #[structopt(short = "v", parse(from_occurrences))]
    pub verbose: u32,

    #[structopt(short = "p", long = "port", default_value = "8080")]
    pub port: u32,

    #[structopt(parse(from_str))]
    pub name: String,
}

pub fn cmd_args() -> CmdArgs {
    CmdArgs::from_args()
}
