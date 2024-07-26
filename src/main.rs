use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(
        short = 'n',
        long = "simulate",
        action = clap::ArgAction::SetTrue,
        help = "Do not actually make any filesystem changes",
        long_help = "Do not perform any operations that modify the filesystem; merely show what would happen.",
    )]
    simulate: bool,

    #[arg(
        short = 'd',
        long = "dir",
        value_name = "DIR",
        help = "Set stow dir to DIR (default is current dir)"
    )]
    dir: Option<String>,

    #[arg(
        short = 't',
        long = "target",
        value_name = "DIR",
        help = "Set target to DIR (default is parent of stow dir)"
    )]
    target: Option<String>,

    #[arg(
        short = 'S',
        long = "stow",
        action = clap::ArgAction::SetTrue,
        help = "Stow the package names that follow this option",
    )]
    stow: bool,

    #[arg(
        short = 'D',
        long = "delete",
        action = clap::ArgAction::SetTrue,
        help = "Unstow the package names that follow this option",
    )]
    delete: bool,

    #[arg(
        short = 'R',
        long = "restow",
        action = clap::ArgAction::SetTrue,
        help = "Restow (like stow -D followed by stow -S)",
    )]
    restow: bool,

    #[arg(
        long = "ignore",
        value_name = "REGEX",
        help = "Ignore files ending in this Perl regex"
    )]
    ignore: Option<String>,

    #[arg(
        long = "defer",
        value_name = "REGEX",
        help = "Don't stow files beginning with this Perl regex\nif the file is already stowed to another package"
    )]
    defer: Option<String>,

    #[arg(
        long = "override",
        value_name = "REGEX",
        help = "Force stowing files beginning with this Perl regex\nif the file is already stowed to another package"
    )]
    override_: Option<String>, // `override` is a reserved keyword in Rust

    #[arg(
        long = "adopt",
        action = clap::ArgAction::SetTrue,
        help = "(Use with care!)  Import existing files into stow package\nfrom target.  Please read docs before using.",
    )]
    adopt: bool,

    #[arg(
        long = "dotfiles",
        action = clap::ArgAction::SetTrue,
        help = "Enables special handling for dotfiles that are\nStow packages that start with \"dot-\" and not \".\"",
    )]
    dotfiles: bool,

    #[arg(
        short = 'p',
        long = "compat",
        action = clap::ArgAction::SetTrue,
        help = "Use legacy algorithm for unstowing",
    )]
    compat: bool,

    #[arg(
        short = 'v',
        long = "verbose",
        value_name = "N",
        num_args = 0..=1,
        default_missing_value = "1",
        help = "Increase verbosity (levels are from 0 to 5;\n-v or --verbose adds 1; --verbose=N sets level)"
    )]
    verbose: Option<Option<u8>>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if cli.simulate {
        println!("No modify")
    }

    println!("Hello, world!");
}
