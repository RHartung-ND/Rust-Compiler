use lrlex::CTLexerBuilder;

fn main() {
    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.grammar_in_src_dir("parser.y")
                .unwrap()
        })
        .lexer_in_src_dir("scanner.l")
        .unwrap()
        .build()
        .unwrap();
}