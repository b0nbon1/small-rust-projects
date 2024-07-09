mod args;
mod compress;

use args::get_args;
use compress::compress_file;

fn main() {
    let (source, target) = match get_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    if let Err(err) = compress_file(&source, &target) {
        eprintln!("Error compressing file: {}", err);
    }
}
