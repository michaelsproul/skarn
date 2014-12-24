use std::collections::HashSet;

use docopt;

use config::Config;
use config::PatternSource::IncludeFile;
use compare::{ComparisonMethod, Content};

docopt! { Args, "
Usage: skarn --include <include_file> [options] <source> <dest>

--delete <delete_behaviour>
"
}

pub fn parse_args() -> Result<Config, docopt::Error> {
    let args: Args = try!(Args::docopt().decode());

    Ok(Config {
        source_dir: Path::new(args.arg_source),
        dest_dir: Path::new(args.arg_dest),
        pattern_type: IncludeFile(Path::new(args.arg_include_file)),
        comparison_method: box Content as Box<ComparisonMethod>,
        delete_behaviour: HashSet::new(),
        include_by_default: true
    })
}
