use core::panic;
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};

use clap::Clap;
use lalrpop_util::ParseError;
use pasm::{assembler::*, lexer::Lexer, mnemonic::*, parser::poco::FileParser, token::*};

#[derive(Clap, Debug)]
struct Opts {
    input: String,

    #[clap(short, long)]
    output: Option<String>,
}

fn main() -> std::io::Result<()> {
    let opts = Opts::parse();

    let input_path = &opts.input;
    let output_path = match opts.output {
        Some(output) => output,
        None => "a.dat".to_string(),
    };

    let input_file = fs::read_to_string(input_path)?;

    let items = match parse_file(input_file.as_str()) {
        Ok(result) => result,
        Err(err) => {
            println!("Parse error");
            println!("{:?}", &err);

            panic!();
        }
    };

    let (mnemonics, table) = convert(items);
    let codes = assemble(mnemonics, &table);

    let output_file = File::create(output_path)?;
    let mut output_writer = BufWriter::new(output_file);

    for line in codes {
        let line_str = line.get_line(true, true);
        output_writer.write_fmt(format_args!("{}\n", line_str))?;
    }

    output_writer.flush()?;

    Ok(())
}

fn parse_file(input: &str) -> Result<Vec<Item>, ParseError<(), Token, ()>> {
    let lexer = Lexer::new(input);
    FileParser::new().parse(lexer)
}
