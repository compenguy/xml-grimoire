use error;

use pest::Parser;

use token::Token;

#[derive(Parser)]
#[grammar = "xml1_0.pest"]
struct XmlLexer1_0;

pub use lexer::Rule as TokenType;

pub fn parse<'a>(rule: &TokenType, content: &'a str) -> Result<Token<'a>, error::Error> {
    let mut pairs = XmlLexer1_0::parse(*rule, content)
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
        assert_eq!(doc_token.children.len(), 1);

        let element_token = &doc_token.children[0];
        assert_eq!(element_token.content, "<doc></doc>");
        assert_eq!(element_token.token_type, TokenType::element);
        assert_eq!(element_token.children.len(), 2);

        let start_tag_token = &element_token.children[0];
        assert_eq!(start_tag_token.content, "<doc>");
        assert_eq!(start_tag_token.token_type, TokenType::STag);
        assert_eq!(start_tag_token.children.len(), 1);

        let start_name_token = &start_tag_token.children[0];
        assert_eq!(start_name_token.content, "doc");
        assert_eq!(start_name_token.token_type, TokenType::Name);
        assert!(start_name_token.children.is_empty());

        let end_tag_token = &element_token.children[1];
        assert_eq!(end_tag_token.content, "</doc>");
        assert_eq!(end_tag_token.token_type, TokenType::ETag);
        assert_eq!(end_tag_token.children.len(), 1);

        let end_name_token = &end_tag_token.children[0];
        assert_eq!(end_name_token.content, "doc");
        assert_eq!(end_name_token.token_type, TokenType::Name);
        assert!(end_name_token.children.is_empty());
    }
}
