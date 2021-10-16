use pasm::{lexer::*, mnemonic::*, parser::*, token::*};
use std::io::{stdout, Write};

use lalrpop_util::ParseError;

fn main() -> std::io::Result<()> {
    loop {
        print!(">> ");
        stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match input.as_str().trim() {
            "quit" | ":q" => break,
            input => match parse(input) {
                Ok(res) => println!("{}", &res),
                Err(err) => {
                    println!("Parse Error");
                    println!("{:#?}", &err);
                }
            },
        }
    }

    Ok(())
}

fn parse(input: &str) -> Result<Item, ParseError<(), Token, ()>> {
    let lexer = Lexer::new(input);
    poco::ItemParser::new().parse(lexer)
}
