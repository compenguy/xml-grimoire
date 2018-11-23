/* Conformance tests of well-formed but invalid xml documents.
 * See /tests/xmltest/xmltest.xml for short descriptions
 * of each test document.
 */

#[cfg(test)]
mod tests {
    use lexer::Rule;
    use lexer::XmlLexer1_0;
    use pest::Parser;

    fn conformance_wf_invalid(id: &str, section: &str, desc: &str, content: &str) {
        println!("[{}] {} (Section {})", id, desc, section);
        let rule = Rule::document;
        let _parsed = match XmlLexer1_0::parse(rule, content) {
            Err(e) => panic!("Lexing the XML content failed: {}", e),
            Ok(p) => p,
        };
    }

    // <!-- Start:  invalid/ -->
    #[test]
    fn conformance_wf_invalid_sa_002() {
        let id="invalid--002";
        let content = include_str!("../tests/xmltest/invalid/002.xml");
        let section = "3.2.1";
        let desc = "Tests the 'Proper Group/PE Nesting' validity constraint by fragmenting a content model between two parameter entities.";
        conformance_wf_invalid(id, section, desc, content);
    }

    #[test]
    fn conformance_wf_invalid_sa_005() {
        let id="invalid--005";
        let content = include_str!("../tests/xmltest/invalid/005.xml");
        let section = "2.8";
        let desc = "Tests the 'Proper Declaration/PE Nesting' validity constraint by fragmenting an element declaration between two parameter entities.";
        conformance_wf_invalid(id, section, desc, content);
    }

    #[test]
    fn conformance_wf_invalid_sa_006() {
        let id="invalid--006";
        let content = include_str!("../tests/xmltest/invalid/006.xml");
        let section = "2.8";
        let desc = "Tests the 'Proper Declaration/PE Nesting' validity constraint by fragmenting an element declaration between two parameter entities.";
        conformance_wf_invalid(id, section, desc, content);
    }

    #[test]
    fn conformance_wf_invalid_sa_022() {
        let id="invalid-not-sa-022";
        let content = include_str!("../tests/xmltest/invalid/not-sa/022.xml");
        let section = "3.4 [62]";
        let _output = include_str!("../tests/xmltest/invalid/not-sa/out/022.xml");
        let desc = "Test the 'Proper Conditional Section/ PE Nesting' validity constraint.";
        conformance_wf_invalid(id, section, desc, content);
    }
}

