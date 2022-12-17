use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "demo-app", about = "演示app")]
pub struct Args {
    #[structopt(short = "p", long = "port", default_value = "9001")]
    pub port: u16,
}
