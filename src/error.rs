use lexer::TokenType;
use pest;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Document failed to parse: {}", pest_err)]
    PestParseError {
        pest_err: pest::error::Error<TokenType>,
    },
    #[fail(display = "Document parsed but produced no tokens.")]
    MatchedNone,
}
