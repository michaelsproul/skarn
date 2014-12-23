use docopt;

use config::{Config, SourceDir, DestDir};

docopt! { Args, "
Usage: skarn <source> <dest>
"
}

pub fn parse_args() -> Result<Config, docopt::Error> {
    let mut config = Config::new();

    let args: Args = try!(Args::docopt().decode());

    let source_dir = Path::new(args.arg_source);
    let dest_dir = Path::new(args.arg_dest);

    config.set::<SourceDir, Path>(source_dir);
    config.set::<DestDir, Path>(dest_dir);

    Ok(config)
}
