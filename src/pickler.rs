use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, Read};
use std::process;

use serde_json as json;
use serde_pickle as pickle;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: pickle (decode | transcode | to_json | from_json) [filename]");
        println!("");
        println!("Input is either given file or stdin.");
        println!("decode:    decode and display pickle");
        println!("transcode: decode and re-encode pickle");
        println!("to_json:   decode and jsonify pickle");
        println!("from_json: encode pickle from json");
        process::exit(1);
    }

    let reader: Box<dyn Read> = if args.len() == 3 {
        Box::new(File::open(&args[2])?)
    } else {
        Box::new(stdin())
    };

    match &*args[1] {
        "decode" => {
            let decoded: pickle::Value = pickle::value_from_reader(reader)?;
            println!("{:#?}", decoded);
        }
        "transcode" => {
            let decoded: pickle::Value = pickle::value_from_reader(reader)?;
            pickle::value_to_writer(&mut stdout(), &decoded, true)?;
        }
        "to_json" => {
            let decoded: json::Value = pickle::from_reader(reader)?;
            println!("{:#?}", decoded);
        }
        "from_json" => {
            let decoded: json::Value = json::from_reader(reader)?;
            pickle::to_writer(&mut stdout(), &decoded, false)?;
        }
        _ => {
            println!("No such subcommand.");
            process::exit(1);
        }
    }
    Ok(())
}
