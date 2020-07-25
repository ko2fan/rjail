mod process_args;
use process_args::CommandLineOpts;
use process_args::Opt;
use structopt::StructOpt;

extern "C" {
    fn show_jails();
}
fn main() {
    match CommandLineOpts::from_args().opt {
        Opt::Show { all } => unsafe { show_jails(); },
        Opt::Create { name } => println!("Not implemented"),
        Opt::Delete { name } => println!("Not implemented")
    }
}