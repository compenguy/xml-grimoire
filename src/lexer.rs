use error;

use pest::Parser;

use token::Token;

#[derive(Parser)]
#[grammar = "xml1_0.pest"]
pub(crate) struct XmlLexer1_0;

pub use lexer::Rule as TokenType;

pub fn parse(rule: TokenType, content: &str) -> Result<Token, error::Error> {
    let mut pairs = XmlLexer1_0::parse(rule, content)
        .map_err(|e| error::Error::PestParseError { pest_err: e })?;
    Ok(Token::from(pairs.next().ok_or(error::Error::MatchedNone)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let content = "<doc></doc>";
        // document
        // └ element
        //   ├ STag
        //   │ └ Name
        //   └ ETag
        //     └ Name
        let doc_token = match parse(TokenType::document, content) {
            Err(e) => panic!("Lexing the XML content failed: {}", e),
            Ok(token) => token,
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
