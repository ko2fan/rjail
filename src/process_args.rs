use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(name = "rjail", about = "A FreeBSD Jails manager.")]
pub struct CommandLineOpts {
    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    pub opt: Opt
}

#[derive(Debug, PartialEq, StructOpt)]
pub enum Opt {
    #[structopt(name = "show")]
    Show {
        #[structopt(short)]
        all: bool
    },
    #[structopt(name = "create")]
    Create {
        name: String
    },
    #[structopt(name = "delete")]
    Delete {
        name: String
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_arguments_with_command() {
        // User ran "rjail show"
        // Turn this into args vector
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("rjail"));
        args.push(String::from("show"));

        // Check for success
        assert_eq!(
            CommandLineOpts::from_iter(&args),
            CommandLineOpts {
                opt: Opt::Show { all: false }
            }
        );
    }

    #[test]
    fn read_arguments_without_command() {
        // User ran "rjail"
        // Turn this into args vector
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("rjail"));

        // Check for failure
        assert_ne!(Opt::from_iter_safe(&args).is_err(), false);
    }
}
