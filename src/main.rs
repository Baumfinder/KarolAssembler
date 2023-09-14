mod welt;
mod assembler;

use assembler::assemble;

use clap::Parser;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Parser)]
struct Args {
    infile: String,
    #[arg(short, long, default_value_t=String::from("out.kdw"))]
    outfile: String,
    #[arg(short, long, default_value_t=50)]
    mem_size: usize,
}

fn main() {
    let args = Args::parse();
    let infile_path = Path::new(&args.infile);
    let outfile_path = Path::new(&args.outfile);

    let mut infile = match File::open(&infile_path) {
        Err(why) => panic!("Eingabedatei konnte nicht geöffnet werden: {}", why),
        Ok(file) => file,
    };
    let mut outfile = match File::create(&outfile_path) {
        Err(why) => panic!("Konnte Ausgabedatei nicht erstellen: {}", why),
        Ok(file) => file,
    };

    let mut infile_text = String::new();
    match infile.read_to_string(&mut infile_text) {
        Err(why) => panic!("Fehler beim Lesen der Eingabedatei: {}", why),
        Ok(_) => {},
    }

    let assembled_text = assemble(&infile_text, args.mem_size);

    match outfile.write_all(assembled_text.as_bytes()) {
        Err(why) => panic!("Fehler beim Schreiben in die Ausgabedatei: {}", why),
        Ok(_) => {},
    }
}
