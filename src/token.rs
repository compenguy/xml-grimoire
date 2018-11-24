use lexer::TokenType;
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub content: &'a str,
    pub children: Vec<Token<'a>>,
}

impl<'a> Token<'a> {
    pub fn new_from_pair(token_pair: Pair<'a, TokenType>) -> Self {
        Token {
            token_type: token_pair.as_rule(),
            content: token_pair.as_str(),
            children: token_pair.into_inner().map(Token::from).collect(),
        }
    }
}

impl<'a> From<Pair<'a, TokenType>> for Token<'a> {
    fn from(token_pair: Pair<'a, TokenType>) -> Self {
        Token::new_from_pair(token_pair)
    }
}

#[cfg(test)]
mod tests {
    use lexer::TokenType;
    use lexer::XmlLexer1_0;
    use pest::Parser;

    use super::*;

    #[test]
    fn test_recursive() {
        let content = "<doc></doc>";
        // document
        // └ element
        //   ├ STag
        //   │ └ Name
        //   └ ETag
        //     └ Name
        let doc_token = match XmlLexer1_0::parse(TokenType::document, content) {
            Err(e) => panic!("Lexing the XML content failed: {}", e),
            Ok(mut p) => Token::new_from_pair(p.next().unwrap()),
        };
        assert_eq!(doc_token.content, "<doc></doc>");
        assert_eq!(doc_token.token_type, TokenType::document);

        let mut doc_token_children = doc_token.children.iter();
        let element_token = doc_token_children
            .find(|t| t.token_type == TokenType::element)
            .expect("Unable to find element where it was expected");
        assert_eq!(element_token.content, "<doc></doc>");

        let mut element_token_children = element_token.children.iter();
        let start_tag_token = element_token_children
            .find(|t| t.token_type == TokenType::STag)
            .expect("Unable to find STag where it was expected");
        assert_eq!(start_tag_token.content, "<doc>");

        let mut start_tag_children = start_tag_token.children.iter();
        let start_name_token = start_tag_children
            .find(|t| t.token_type == TokenType::Name)
            .expect("Unable to find Name where it was expected");
        assert_eq!(start_name_token.content, "doc");

        // Assert that the start name token has no content-bearing children
        for token in &start_name_token.children {
            assert!(token.content.is_empty());
        }

        let end_tag_token = element_token_children
            .find(|t| t.token_type == TokenType::ETag)
            .expect("Unable to find ETag where it was expected");
        assert_eq!(end_tag_token.content, "</doc>");

        let mut end_tag_children = end_tag_token.children.iter();
        let end_name_token = end_tag_children
            .find(|t| t.token_type == TokenType::Name)
            .expect("Unable to find Name where it was expected");
        assert_eq!(end_name_token.content, "doc");

        // Assert that the end name token has no content-bearing children
        for token in &end_name_token.children {
            assert!(token.content.is_empty());
        }
    }
}
