use std::env;
use std::error::Error;

pub fn get_args() -> Result<(String, String), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err("Usage: `source` `target`".into());
    }

    Ok((args[1].clone(), args[2].clone()))
}
