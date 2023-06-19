mod mix;
mod mixal;

use mix::machine::Mix;
use mixal::lexer::Lexer;
use mixal::parser::Parser;

fn main() -> Result<(), &'static str>{

    let code = "LOC 0 
        X IS 10
        Y IS 20
        Z IS 30

            LDA $1, X(0)
            LDA $2, Y(0)
            ADD $3, $1, $2
            STA $3, Z(0)
            HLT
    ";
    let mut lex = Lexer::new(code);
    let tokens = lex.lex();
    
    for token in &tokens {
        println!("{}", token);
    }

    let mut parser = Parser::new(tokens);

    let ast = parser.parse()?;

    println!("{}", ast);
    println!("Hello, world!");
    Ok(())
}
