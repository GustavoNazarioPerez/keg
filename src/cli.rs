pub const USAGE: &str = "Usage: keg [OPTIONS] <COMMAND>";

pub const HELP: &str = "Usage: keg [OPTIONS] <COMMAND>

Commands:
    set <KEY> <VALUE...>  Store VALUE under KEY
    get <KEY>             Print the value for KEY
    delete <KEY>          Remove KEY
    scan [PREFIX]         List every key/value starting with PREFIX, sorted by key
    count                 Print the number of keys
    stats                 Print database statistics
    compact               Shrink the data file without losing data
    bench                 Run a stress test on the database

Options:
    -h, --help     Print help information";
