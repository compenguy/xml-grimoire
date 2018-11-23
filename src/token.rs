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
        // document
        // └ element
        //   ├ STag
        //   │ └ Name
        //   └ ETag
        //     └ Name
        let doc_token = match XmlLexer1_0::parse(Rule::document, content) {
            Err(e) => panic!("Lexing the XML content failed: {}", e),
            Ok(mut p) => Token::from(p.next().unwrap()),
        };
        assert_eq!(doc_token.content, "<doc></doc>");
        assert_eq!(doc_token.token_type, Rule::document);
        assert_eq!(doc_token.children.len(), 1);

        let element_token = &doc_token.children[0];
        assert_eq!(element_token.content, "<doc></doc>");
        assert_eq!(element_token.token_type, Rule::element);
        assert_eq!(element_token.children.len(), 2);

        let start_tag_token = &element_token.children[0];
        assert_eq!(start_tag_token.content, "<doc>");
        assert_eq!(start_tag_token.token_type, Rule::STag);
        assert_eq!(start_tag_token.children.len(), 1);

        let start_name_token = &start_tag_token.children[0];
        assert_eq!(start_name_token.content, "doc");
        assert_eq!(start_name_token.token_type, Rule::Name);
        assert!(start_name_token.children.is_empty());

        let end_tag_token = &element_token.children[1];
        assert_eq!(end_tag_token.content, "</doc>");
        assert_eq!(end_tag_token.token_type, Rule::ETag);
        assert_eq!(end_tag_token.children.len(), 1);

        let end_name_token = &end_tag_token.children[0];
        assert_eq!(end_name_token.content, "doc");
        assert_eq!(end_name_token.token_type, Rule::Name);
        assert!(end_name_token.children.is_empty());
    }
}
