use docopt;

use error::Error as SkarnError;
use config::{Config, DeleteBehaviour};
use config::PatternSource::IncludeFile;
use compare::{ComparisonMethod, Content};

docopt! { Args, "
Usage: skarn --include <include-file> [options] <source> <dest>

--delete <delete-behaviour>
"
}

pub fn parse_args() -> Result<Config, SkarnError> {
    let args: Args = try!(Args::docopt().decode());

    let delete_behaviour = try!(DeleteBehaviour::from_str(args.flag_delete[]));

    debug!("delete behaviour set to: {}", delete_behaviour);

    Ok(Config {
        source_dir: Path::new(args.arg_source),
        dest_dir: Path::new(args.arg_dest),
        pattern_type: IncludeFile(Path::new(args.arg_include_file)),
        comparison_method: box Content as Box<ComparisonMethod>,
        delete_behaviour: delete_behaviour,
        include_by_default: true
    })
}
