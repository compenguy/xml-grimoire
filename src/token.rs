use pest::iterators::Pair;

#[derive(Debug)]
pub struct Token<'a,R: pest::RuleType> {
    token_type: R,
    content: &'a str,
    children: Vec<Token<'a,R>>,
}

impl<'a, R: pest::RuleType> Token<'a,R> {
    pub fn new_from_pair(token_pair: Pair<'a,R>) -> Self {
        Token {
            token_type: token_pair.as_rule(),
            content: token_pair.as_str(),
            /* Suppress empty tokens */
            children: token_pair.into_inner().filter(|x| ! x.as_str().is_empty()).map(Token::from).collect(),
        }
    }
}

impl<'a, R: pest::RuleType> From<Pair<'a,R>> for Token<'a,R> {
    fn from(token_pair: Pair<'a,R>) -> Self {
        Token::new_from_pair(token_pair)
    }
}

#[cfg(test)]
mod tests {
    use lexer::Rule;
    use lexer::XmlLexer1_0;
    use pest::Parser;

    use super::*;

    #[test]
    fn test_recursive() {
        let content = "<doc></doc>";
        let _doc_token = match XmlLexer1_0::parse(Rule::document, content) {
            Err(e) => panic!("Lexing the XML content failed: {}", e),
            Ok(mut p) => Token::from(p.next().unwrap()),
        };
        /* TODO: actually verify the structure */
    }
}
