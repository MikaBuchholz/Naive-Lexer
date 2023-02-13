pub mod consts;
pub mod lexer;

use crate::lexer::Lexer;

fn main() {
    let input: String = "fn main() {
        if a > b {
            println!(\"Ass\");
        }
    } "
    .into();

    //let mut lexer = Lexer::new(input);

    let mut lexer = Lexer::from_path("./src/main.rs").unwrap();

    let toks = lexer.collect_tokens().unwrap();

    let content = lexer.get_content();

    if true {
        for tok in toks {
            println!(
                "{}\t{:?}",
                content.get(tok.loc.token_pos).unwrap().trim(),
                tok.token
            )
        }
    }

    //TODO Context analisys of tokens like & and < > []
    //TODO Implement bracket balancer                []
    //TODO Rework lexer error enum                   [x]
    //TODO Maybe add documentation to functions      []
    //TODO Seperate logic into files                 [x]
}
