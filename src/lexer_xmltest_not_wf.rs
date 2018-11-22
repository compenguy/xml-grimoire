/* Conformance tests of not well-formed xml documents.
 * See /tests/xmltest/xmltest.xml for short descriptions
 * of each test document.
 */

#[cfg(test)]
mod tests {
    use lexer::Rule;
    use lexer::XmlLexer1_0;
    use pest::Parser;

    fn conformance_not_wf(id: &str, section: &str, desc: &str, content: &str) {
         println!("[{}] {} (Section {})", id, desc, section);
         let rule = Rule::document;
         let _parsed = match XmlLexer1_0::parse(rule, content) {
              Err(e) => panic!("Lexing the XML content failed: {}",  e),
              Ok(p) => p,
         };
    }

    /* <!-- Start:  not-wf/sa --> */
    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_001() {
        let id = "not-wf-sa-001";
        let section = "3.1 [41]";
        let desc = "Attribute values must start with attribute names, not '?'";
        let content = include_str!("../tests/xmltest/not-wf/sa/001.xml");
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_002() {
        let id = "not-wf-sa-002";
        let section = "2.3 [4]";
        let desc = "Names may not start with '.'; it's not a Letter";
        let content = include_str!("../tests/xmltest/not-wf/sa/002.xml");
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_003() {
        let id = "not-wf-sa-003";
        let section = "2.6 [16]";
        let desc = "Processing Instruction target name is required";
        let content = include_str!("../tests/xmltest/not-wf/sa/003.xml");
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_004() {
        let id = "not-wf-sa-004";
        let section = "2.6 [16]";
        let desc = "SGML-ism: processing instructions end in '?&gt;' not '&gt;'";
        let content = include_str!("../tests/xmltest/not-wf/sa/004.xml");
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_005() {
        let id = "not-wf-sa-005";
        let section = "2.6 [16]";
        let desc = "Processing instructions end in '?&gt;' not '?'";
        let content = include_str!("../tests/xmltest/not-wf/sa/004.xml");
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_006() {
        let id = "not-wf-sa-006";
        let content = include_str!("../tests/xmltest/not-wf/sa/006.xml");
        let section = "section 2.5 [16]";
        let desc = "XML comments may not contain '--'";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_007() {
        let id = "not-wf-sa-007";
        let content = include_str!("../tests/xmltest/not-wf/sa/007.xml");
        let section = "section 4.1 [68]";
        let desc = "General entity references have no whitespace after the entity name and before the semicolon.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_008() {
        let id = "not-wf-sa-008";
        let content = include_str!("../tests/xmltest/not-wf/sa/008.xml");
        let section = "section 2.3 [5]";
        let desc = "Entity references must include names, which don't begin with '.' (it's not a Letter or other name start character).";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_009() {
        let id = "not-wf-sa-009";
        let content = include_str!("../tests/xmltest/not-wf/sa/009.xml");
        let section = "section 4.1 [66]";
        let desc = "Character references may have only decimal or numeric strings.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_010() {
        let id = "not-wf-sa-010";
        let content = include_str!("../tests/xmltest/not-wf/sa/010.xml");
        let section = "section 4.1 [68]";
        let desc = "Ampersand may only appear as part of a general entity reference.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_011() {
        let id = "not-wf-sa-011";
        let content = include_str!("../tests/xmltest/not-wf/sa/011.xml");
        let section = "section 3.1 [41]";
        let desc = "SGML-ism:  attribute values must be explicitly assigned a value, it can't act as a boolean toggle.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_012() {
        let id = "not-wf-sa-012";
        let content = include_str!("../tests/xmltest/not-wf/sa/012.xml");
        let section = "section 2.3 [10]";
        let desc = "SGML-ism:  attribute values must be quoted in all cases.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_013() {
        let id = "not-wf-sa-013";
        let content = include_str!("../tests/xmltest/not-wf/sa/013.xml");
        let section = "section 2.3 [10]";
        let desc = "The quotes on both ends of an attribute value must match.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_014() {
        let id = "not-wf-sa-014";
        let content = include_str!("../tests/xmltest/not-wf/sa/014.xml");
        let section = "section 2.3 [10]";
        let desc = "Attribute values may not contain literal '&lt;' characters.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_015() {
        let id = "not-wf-sa-015";
        let content = include_str!("../tests/xmltest/not-wf/sa/015.xml");
        let section = "section 3.1 [41]";
        let desc = "Attribute values need a value, not just an equals sign.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_016() {
        let id = "not-wf-sa-016";
        let content = include_str!("../tests/xmltest/not-wf/sa/016.xml");
        let section = "section 3.1 [41]";
        let desc = "Attribute values need an associated name.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_017() {
        let id = "not-wf-sa-017";
        let content = include_str!("../tests/xmltest/not-wf/sa/017.xml");
        let section = "section 2.7 [18]";
        let desc = "CDATA sections need a terminating ']]&gt;'.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_018() {
        let id = "not-wf-sa-018";
        let content = include_str!("../tests/xmltest/not-wf/sa/018.xml");
        let section = "section 2.7 [19]";
        let desc = "CDATA sections begin with a literal '&lt;![CDATA[', no space.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_019() {

        let id = "not-wf-sa-019";
        let content = include_str!("../tests/xmltest/not-wf/sa/019.xml");
        let section = "section 3.1 [42]";
        let desc = "End tags may not be abbreviated as '&lt;/&gt;'.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_020() {

        let id = "not-wf-sa-020";
        let content = include_str!("../tests/xmltest/not-wf/sa/020.xml");
        let section = "section 2.3 [10]";
        let desc = "Attribute values may not contain literal '&amp;' characters except as part of an entity reference.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_021() {

        let id = "not-wf-sa-021";
        let content = include_str!("../tests/xmltest/not-wf/sa/021.xml");
        let section = "section 2.3 [10]";
        let desc = "Attribute values may not contain literal '&amp;' characters except as part of an entity reference.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_022() {

        let id = "not-wf-sa-022";
        let content = include_str!("../tests/xmltest/not-wf/sa/022.xml");
        let section = "section 4.1 [66]";
        let desc = "Character references end with semicolons, always!";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_023() {

        let id = "not-wf-sa-023";
        let content = include_str!("../tests/xmltest/not-wf/sa/023.xml");
        let section = "section 2.3 [5]";
        let desc = "Digits are not valid name start characters.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_024() {

        let id = "not-wf-sa-024";
        let content = include_str!("../tests/xmltest/not-wf/sa/024.xml");
        let section = "section 2.3 [5]";
        let desc = "Digits are not valid name start characters.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_025() {

        let id = "not-wf-sa-025";
        let content = include_str!("../tests/xmltest/not-wf/sa/025.xml");
        let section = "section 2.4 [14]";
        let desc = "Text may not contain a literal ']]&gt;' sequence.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_026() {

        let id = "not-wf-sa-026";
        let content = include_str!("../tests/xmltest/not-wf/sa/026.xml");
        let section = "section 2.4 [14]";
        let desc = "Text may not contain a literal ']]&gt;' sequence.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_027() {

        let id = "not-wf-sa-027";
        let content = include_str!("../tests/xmltest/not-wf/sa/027.xml");
        let section = "section 2.5 [15]";
        let desc = "Comments must be terminated with '--&gt;'.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_028() {

        let id = "not-wf-sa-028";
        let content = include_str!("../tests/xmltest/not-wf/sa/028.xml");
        let section = "section 2.6 [16]";
        let desc = "Processing instructions must end with '?&gt;'.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_029() {

        let id = "not-wf-sa-029";
        let content = include_str!("../tests/xmltest/not-wf/sa/029.xml");
        let section = "section 2.4 [14]";
        let desc = "Text may not contain a literal ']]&gt;' sequence.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_030() {

        let id = "not-wf-sa-030";
        let content = include_str!("../tests/xmltest/not-wf/sa/030.xml");
        let section = "section 2.2 [2]";
        let desc = "A form feed is not a legal XML character.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_031() {

        let id = "not-wf-sa-031";
        let content = include_str!("../tests/xmltest/not-wf/sa/031.xml");
        let section = "section 2.2 [2]";
        let desc = "A form feed is not a legal XML character.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_032() {

        let id = "not-wf-sa-032";
        let content = include_str!("../tests/xmltest/not-wf/sa/032.xml");
        let section = "section 2.2 [2]";
        let desc = "A form feed is not a legal XML character.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_033() {

        let id = "not-wf-sa-033";
        let content = include_str!("../tests/xmltest/not-wf/sa/033.xml");
        let section = "section 2.2 [2]";
        let desc = "An ESC (octal 033) is not a legal XML character.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_034() {

        let id = "not-wf-sa-034";
        let content = include_str!("../tests/xmltest/not-wf/sa/034.xml");
        let section = "section 2.2 [2]";
        let desc = "A form feed is not a legal XML character.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_035() {

        let id = "not-wf-sa-035";
        let content = include_str!("../tests/xmltest/not-wf/sa/035.xml");
        let section = "section 3.1 [43]";
        let desc = "The '&lt;' character is a markup delimiter and must start an element, CDATA section, PI, or comment.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_036() {

        let id = "not-wf-sa-036";
        let content = include_str!("../tests/xmltest/not-wf/sa/036.xml");
        let section = "section 2.8 [27]";
        let desc = "Text may not appear after the root element.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_037() {

        let id = "not-wf-sa-037";
        let content = include_str!("../tests/xmltest/not-wf/sa/037.xml");
        let section = "section 2.8 [27]";
        let desc = "Character references may not appear after the root element.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_038() {

        let id = "not-wf-sa-038";
        let content = include_str!("../tests/xmltest/not-wf/sa/038.xml");
        let section = "section 3.1";
        let desc = "Tests the 'Unique Att Spec' WF constraint by providing multiple values for an attribute.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_039() {

        let id = "not-wf-sa-039";
        let content = include_str!("../tests/xmltest/not-wf/sa/039.xml");
        let section = "3";
        let desc = "Tests the Element Type Match WFC - end tag name must match start tag name.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_040() {

        let id = "not-wf-sa-040";
        let content = include_str!("../tests/xmltest/not-wf/sa/040.xml");
        let section = "section 2.8 [27]";
        let desc = "Provides two document elements.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_041() {

        let id = "not-wf-sa-041";
        let content = include_str!("../tests/xmltest/not-wf/sa/041.xml");
        let section = "section 2.8 [27]";
        let desc = "Provides two document elements.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_042() {

        let id = "not-wf-sa-042";
        let content = include_str!("../tests/xmltest/not-wf/sa/042.xml");
        let section = "section 3.1 [42]";
        let desc = "Invalid End Tag";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_043() {

        let id = "not-wf-sa-043";
        let content = include_str!("../tests/xmltest/not-wf/sa/043.xml");
        let section = "section 2.8 [27]";
        let desc = "Provides #PCDATA text after the document element.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_044() {

        let id = "not-wf-sa-044";
        let content = include_str!("../tests/xmltest/not-wf/sa/044.xml");
        let section = "section 2.8 [27]";
        let desc = "Provides two document elements.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_045() {

        let id = "not-wf-sa-045";
        let content = include_str!("../tests/xmltest/not-wf/sa/045.xml");
        let section = "section 3.1 [44]";
        let desc = "Invalid Empty Element Tag";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_046() {

        let id = "not-wf-sa-046";
        let content = include_str!("../tests/xmltest/not-wf/sa/046.xml");
        let section = "section 3.1 [40]";
        let desc = "This start (or empty element) tag was not terminated correctly.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_047() {

        let id = "not-wf-sa-047";
        let content = include_str!("../tests/xmltest/not-wf/sa/047.xml");
        let section = "section 3.1 [44]";
        let desc = "Invalid empty element tag invalid whitespace";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_048() {

        let id = "not-wf-sa-048";
        let content = include_str!("../tests/xmltest/not-wf/sa/048.xml");
        let section = "section 2.8 [27]";
        let desc = "Provides a CDATA section after the roor element.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_049() {

        let id = "not-wf-sa-049";
        let content = include_str!("../tests/xmltest/not-wf/sa/049.xml");
        let section = "section 3.1 [40]";
        let desc = "Missing start tag";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_050() {

        let id = "not-wf-sa-050";
        let content = include_str!("../tests/xmltest/not-wf/sa/050.xml");
        let section = "section 2.1 [1]";
        let desc = "Empty document, with no root element.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_051() {

        let id = "not-wf-sa-051";
        let content = include_str!("../tests/xmltest/not-wf/sa/051.xml");
        let section = "section 2.7 [18]";
        let desc = "CDATA is invalid at top level of document.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_052() {

        let id = "not-wf-sa-052";
        let content = include_str!("../tests/xmltest/not-wf/sa/052.xml");
        let section = "section 4.1 [66]";
        let desc = "Invalid character reference.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_053() {

        let id = "not-wf-sa-053";
        let content = include_str!("../tests/xmltest/not-wf/sa/053.xml");
        let section = "section 3.1 [42]";
        let desc = "End tag does not match start tag.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_054() {

        let id = "not-wf-sa-054";
        let content = include_str!("../tests/xmltest/not-wf/sa/054.xml");
        let section = "section 4.2.2 [75]";
        let desc = "PUBLIC requires two literals.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_055() {

        let id = "not-wf-sa-055";
        let content = include_str!("../tests/xmltest/not-wf/sa/055.xml");
        let section = "section 2.8 [28]";
        let desc = "Invalid Document Type Definition format.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_056() {

        let id = "not-wf-sa-056";
        let content = include_str!("../tests/xmltest/not-wf/sa/056.xml");
        let section = "section 2.8 [28]";
        let desc = "Invalid Document Type Definition format - misplaced comment.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_057() {

        let id = "not-wf-sa-057";
        let content = include_str!("../tests/xmltest/not-wf/sa/057.xml");
        let section = "section 3.2 [45]";
        let desc = "This isn't SGML; comments can't exist in declarations.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_058() {

        let id = "not-wf-sa-058";
        let content = include_str!("../tests/xmltest/not-wf/sa/058.xml");
        let section = "section 3.3.1 [54]";
        let desc = "Invalid character , in ATTLIST enumeration";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_059() {

        let id = "not-wf-sa-059";
        let content = include_str!("../tests/xmltest/not-wf/sa/059.xml");
        let section = "section 3.3.1 [59]";
        let desc = "String literal must be in quotes.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_060() {

        let id = "not-wf-sa-060";
        let content = include_str!("../tests/xmltest/not-wf/sa/060.xml");
        let section = "section 3.3.1 [56]";
        let desc = "Invalid type NAME defined in ATTLIST.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_061() {

        let id = "not-wf-sa-061";
        let content = include_str!("../tests/xmltest/not-wf/sa/061.xml");
        let section = "section 4.2.2 [75]";
        let desc = "External entity declarations require whitespace between public and system IDs.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_062() {

        let id = "not-wf-sa-062";
        let content = include_str!("../tests/xmltest/not-wf/sa/062.xml");
        let section = "section 4.2 [71]";
        let desc = "Entity declarations need space after the entity name.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_063() {

        let id = "not-wf-sa-063";
        let content = include_str!("../tests/xmltest/not-wf/sa/063.xml");
        let section = "section 2.8 [29]";
        let desc = "Conditional sections may only appear in the external DTD subset.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_064() {

        let id = "not-wf-sa-064";
        let content = include_str!("../tests/xmltest/not-wf/sa/064.xml");
        let section = "section 3.3 [53]";
        let desc = "Space is required between attribute type and default values in &lt;!ATTLIST...&gt; declarations.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_065() {

        let id = "not-wf-sa-065";
        let content = include_str!("../tests/xmltest/not-wf/sa/065.xml");
        let section = "section 3.3 [53]";
        let desc = "Space is required between attribute name and type in &lt;!ATTLIST...&gt; declarations.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_066() {

        let id = "not-wf-sa-066";
        let content = include_str!("../tests/xmltest/not-wf/sa/066.xml");
        let section = "section 3.3 [52]";
        let desc = "Required whitespace is missing.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_067() {

        let id = "not-wf-sa-067";
        let content = include_str!("../tests/xmltest/not-wf/sa/067.xml");
        let section = "section 3.3 [53]";
        let desc = "Space is required between attribute type and default values in &lt;!ATTLIST...&gt; declarations.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_068() {

        let id = "not-wf-sa-068";
        let content = include_str!("../tests/xmltest/not-wf/sa/068.xml");
        let section = "section 3.3.1 [58]";
        let desc = "Space is required between NOTATION keyword and list of enumerated choices in &lt;!ATTLIST...&gt; declarations.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_069() {
        let id = "not-wf-sa-069";
        let content = include_str!("../tests/xmltest/not-wf/sa/069.xml");
        let section = "section 4.2.2 [76]";
        let desc = "Space is required before an NDATA entity annotation.";
        conformance_not_wf(id, section, desc, content);
    }

    #[test]
    #[should_panic]
    fn conformance_not_wf_sa_070() {
        let id = "not-wf-sa-070";
        let content = include_str!("../tests/xmltest/not-wf/sa/070.xml");
        let section = "section 2.5 [16]";
        let desc = "XML comments may not contain '--'";
        conformance_not_wf(id, section, desc, content);
    }
}

/*

        let id = "not-wf-sa-071";
        let content = include_str!("../tests/xmltest/not-wf/sa/071.xml");
        let section = "section 4.1 [68]";
        ENTITY can't reference itself directly or indirectly.

        let id = "not-wf-sa-072";
        let content = include_str!("../tests/xmltest/not-wf/sa/072.xml");
        let section = "section 4.1 [68]";
        Undefined ENTITY foo.

        let id = "not-wf-sa-073";
        let content = include_str!("../tests/xmltest/not-wf/sa/073.xml");
        let section = "section 4.1 [68]";
        Undefined ENTITY f.

        let id = "not-wf-sa-074";
        let content = include_str!("../tests/xmltest/not-wf/sa/074.xml");
        let section = "section 4.3.2";
        Internal general parsed entities are only well formed if
        they match the "content" production.

        let id = "not-wf-sa-075";
        let content = include_str!("../tests/xmltest/not-wf/sa/075.xml");
        let section = "section 4.1 [68]";
        ENTITY can't reference itself directly or indirectly.

        let id = "not-wf-sa-076";
        let content = include_str!("../tests/xmltest/not-wf/sa/076.xml");
        let section = "section 4.1 [68]";
        Undefined ENTITY foo.

        let id = "not-wf-sa-077";
        let content = include_str!("../tests/xmltest/not-wf/sa/077.xml");41. [68]
        Undefined ENTITY bar.

        let id = "not-wf-sa-078";
        let content = include_str!("../tests/xmltest/not-wf/sa/078.xml");
        let section = "section 4.1 [68]";
        Undefined ENTITY foo.

        let id = "not-wf-sa-079";
        let content = include_str!("../tests/xmltest/not-wf/sa/079.xml");
        let section = "section 4.1 [68]";
        ENTITY can't reference itself directly or indirectly.

        let id = "not-wf-sa-080";
        let content = include_str!("../tests/xmltest/not-wf/sa/080.xml");
        let section = "section 4.1 [68]";
        ENTITY can't reference itself directly or indirectly.

        let id = "not-wf-sa-081";
        let content = include_str!("../tests/xmltest/not-wf/sa/081.xml");
        let section = "section 3.1";
        This tests the <EM>No External Entity References</EM> WFC,
        since the entity is referred to within an attribute. 

        let id = "not-wf-sa-082";
        let content = include_str!("../tests/xmltest/not-wf/sa/082.xml");
        let section = "section 3.1";
        This tests the <EM>No External Entity References</EM> WFC,
        since the entity is referred to within an attribute. 

        let id = "not-wf-sa-083";
        let content = include_str!("../tests/xmltest/not-wf/sa/083.xml");
        let section = "section 4.2.2 [76]";
        Undefined NOTATION n.

        let id = "not-wf-sa-084";
        let content = include_str!("../tests/xmltest/not-wf/sa/084.xml");
        let section = "section 4.1";
        Tests the <EM>Parsed Entity</EM> WFC by referring to an
        unparsed entity.  (This precedes the error of not declaring
        that entity's notation, which may be detected any time before
        the DTD parsing is completed.)

        let id = "not-wf-sa-085";
        let content = include_str!("../tests/xmltest/not-wf/sa/085.xml");
        let section = "section 2.3 [13]";
        Public IDs may not contain "[".

        let id = "not-wf-sa-086";
        let content = include_str!("../tests/xmltest/not-wf/sa/086.xml");
        let section = "section 2.3 [13]";
        Public IDs may not contain "[".

        let id = "not-wf-sa-087";
        let content = include_str!("../tests/xmltest/not-wf/sa/087.xml");
        let section = "section 2.3 [13]";
        Public IDs may not contain "[".

        let id = "not-wf-sa-088";
        let content = include_str!("../tests/xmltest/not-wf/sa/088.xml");
        let section = "section 2.3 [10]";
        Attribute values are terminated by literal quote characters,
        and any entity expansion is done afterwards.

        let id = "not-wf-sa-089";
        let content = include_str!("../tests/xmltest/not-wf/sa/089.xml");
        let section = "section 4.2 [74]";
        Parameter entities "are" always parsed; NDATA annotations
        are not permitted.

        let id = "not-wf-sa-090";
        let content = include_str!("../tests/xmltest/not-wf/sa/090.xml");
        let section = "section 2.3 [10]";
        Attributes may not contain a literal "&lt;" character;
        this one has one because of reference expansion.

        let id = "not-wf-sa-091";
        let content = include_str!("../tests/xmltest/not-wf/sa/091.xml");
        let section = "section 4.2 [74]";
        Parameter entities "are" always parsed; NDATA annotations
        are not permitted.

        let id = "not-wf-sa-092";
        let content = include_str!("../tests/xmltest/not-wf/sa/092.xml");
        let section = "section 4.5";
        The replacement text of this entity has an illegal reference,
        because the character reference is expanded immediately.

        let id = "not-wf-sa-093";
        let content = include_str!("../tests/xmltest/not-wf/sa/093.xml");
        let section = "section 4.1 [66]";
        Hexadecimal character references may not use the uppercase 'X'.

        let id = "not-wf-sa-094";
        let content = include_str!("../tests/xmltest/not-wf/sa/094.xml");
        let section = "section 2.8 [24]";
        Prolog VERSION must be lowercase.

        let id = "not-wf-sa-095";
        let content = include_str!("../tests/xmltest/not-wf/sa/095.xml");
        let section = "section 2.8 [23]";
        VersionInfo must come before EncodingDecl.

        let id = "not-wf-sa-096";
        let content = include_str!("../tests/xmltest/not-wf/sa/096.xml");
        let section = "section 2.9 [32]";
        Space is required before the standalone declaration.

        let id = "not-wf-sa-097";
        let content = include_str!("../tests/xmltest/not-wf/sa/097.xml");
        let section = "section 2.8 [24]";
        Both quotes surrounding VersionNum must be the same.

        let id = "not-wf-sa-098";
        let content = include_str!("../tests/xmltest/not-wf/sa/098.xml");
        let section = "section 2.8 [23]";
        Only one "version=..." string may appear in an XML declaration.

        let id = "not-wf-sa-099";
        let content = include_str!("../tests/xmltest/not-wf/sa/099.xml");
        let section = "section 2.8 [23]";
        Only three pseudo-attributes are in the XML declaration,
        and "valid=..." is not one of them.

        let id = "not-wf-sa-100";
        let content = include_str!("../tests/xmltest/not-wf/sa/100.xml");
        let section = "section 2.9 [32]";
        Only "yes" and "no" are permitted as values of "standalone".

        let id = "not-wf-sa-101";
        let content = include_str!("../tests/xmltest/not-wf/sa/101.xml");
        let section = "section 4.3.3 [81]";
        Space is not permitted in an encoding name.

        let id = "not-wf-sa-102";
        let content = include_str!("../tests/xmltest/not-wf/sa/102.xml");
        let section = "section 2.8 [26]";
        Provides an illegal XML version number; spaces are illegal.

        let id = "not-wf-sa-103";
        let content = include_str!("../tests/xmltest/not-wf/sa/103.xml");
        let section = "section 4.3.2";
        End-tag required for element foo.

        let id = "not-wf-sa-104";
        let content = include_str!("../tests/xmltest/not-wf/sa/104.xml");
        let section = "section 4.3.2";
        Internal general parsed entities are only well formed if
        they match the "content" production.

        let id = "not-wf-sa-105";
        let content = include_str!("../tests/xmltest/not-wf/sa/105.xml");
        let section = "section 2.7";
        Invalid placement of CDATA section.

        let id = "not-wf-sa-106";
        let content = include_str!("../tests/xmltest/not-wf/sa/106.xml");
        let section = "section 4.2";
        Invalid placement of entity declaration.

        let id = "not-wf-sa-107";
        let content = include_str!("../tests/xmltest/not-wf/sa/107.xml");
        let section = "section 2.8 [28]";
        Invalid document type declaration.  CDATA alone is invalid.

        let id = "not-wf-sa-108";
        let content = include_str!("../tests/xmltest/not-wf/sa/108.xml");
        let section = "section 2.7 [19]";
        No space in '&lt;![CDATA['.

        let id = "not-wf-sa-109";
        let content = include_str!("../tests/xmltest/not-wf/sa/109.xml");
        let section = "section 4.2 [70]";
        Tags invalid within EntityDecl.

        let id = "not-wf-sa-110";
        let content = include_str!("../tests/xmltest/not-wf/sa/110.xml");
        let section = "section 4.1 [68]";
        Entity reference must be in content of element.

        let id = "not-wf-sa-111";
        let content = include_str!("../tests/xmltest/not-wf/sa/111.xml");
        let section = "section 3.1 [43]";
        Entiry reference must be in content of element not Start-tag.

        let id = "not-wf-sa-112";
        let content = include_str!("../tests/xmltest/not-wf/sa/112.xml");
        let section = "section 2.7 [19]";
        CDATA sections start '&lt;![CDATA[', not '&lt;!cdata['.

        let id = "not-wf-sa-113";
        let content = include_str!("../tests/xmltest/not-wf/sa/113.xml");
        let section = "section 2.3 [9]";
        Parameter entity values must use valid reference syntax;
        this reference is malformed.

        let id = "not-wf-sa-114";
        let content = include_str!("../tests/xmltest/not-wf/sa/114.xml");
        let section = "section 2.3 [9]";
        General entity values must use valid reference syntax;
        this reference is malformed.

        let id = "not-wf-sa-115";
        let content = include_str!("../tests/xmltest/not-wf/sa/115.xml");
        let section = "section 4.5";
        The replacement text of this entity is an illegal character
        reference, which must be rejected when it is parsed in the
        context of an attribute value.

        let id = "not-wf-sa-116";
        let content = include_str!("../tests/xmltest/not-wf/sa/116.xml");
        let section = "section 4.3.2";
        Internal general parsed entities are only well formed if
        they match the "content" production.  This is a partial
        character reference, not a full one.

        let id = "not-wf-sa-117";
        let content = include_str!("../tests/xmltest/not-wf/sa/117.xml");
        let section = "section 4.3.2";
        Internal general parsed entities are only well formed if
        they match the "content" production.  This is a partial
        character reference, not a full one.

        let id = "not-wf-sa-118";
        let content = include_str!("../tests/xmltest/not-wf/sa/118.xml");
        let section = "section 4.1 [68]";
        Entity reference expansion is not recursive.

        let id = "not-wf-sa-119";
        let content = include_str!("../tests/xmltest/not-wf/sa/119.xml");
        let section = "section 4.3.2";
        Internal general parsed entities are only well formed if
        they match the "content" production.  This is a partial
        character reference, not a full one.

        let id = "not-wf-sa-120";
        let content = include_str!("../tests/xmltest/not-wf/sa/120.xml");
        let section = "section 4.5";
        Character references are expanded in the replacement text of
        an internal entity, which is then parsed as usual.  Accordingly,
        &amp; must be doubly quoted - encoded either as <EM>&amp;amp;</EM
        or as <EM>&amp;#38;#38;</EM>.

        let id = "not-wf-sa-121";
        let content = include_str!("../tests/xmltest/not-wf/sa/121.xml");
        let section = "section 4.1 [68]";
        A name of an ENTITY was started with an invalid character.

        let id = "not-wf-sa-122";
        let content = include_str!("../tests/xmltest/not-wf/sa/122.xml");
        let section = "section 3.2.1 [47]";
        Invalid syntax mixed connectors are used.

        let id = "not-wf-sa-123";
        let content = include_str!("../tests/xmltest/not-wf/sa/123.xml");
        let section = "section 3.2.1 [48]";
        Invalid syntax mismatched parenthesis.

        let id = "not-wf-sa-124";
        let content = include_str!("../tests/xmltest/not-wf/sa/124.xml");
        let section = "section 3.2.2 [51]";
        Invalid format of Mixed-content declaration.

        let id = "not-wf-sa-125";
        let content = include_str!("../tests/xmltest/not-wf/sa/125.xml");
        let section = "section 3.2.2 [51]";
        Invalid syntax extra set of parenthesis not necessary.

        let id = "not-wf-sa-126";
        let content = include_str!("../tests/xmltest/not-wf/sa/126.xml");
        let section = "section 3.2.2 [51]";
        Invalid syntax Mixed-content must be defined as zero or more.

        let id = "not-wf-sa-127";
        let content = include_str!("../tests/xmltest/not-wf/sa/127.xml");
        let section = "section 3.2.2 [51]";
        Invalid syntax Mixed-content must be defined as zero or more.

        let id = "not-wf-sa-128";
        let content = include_str!("../tests/xmltest/not-wf/sa/128.xml");
        let section = "section 2.7 [18]";
        Invalid CDATA syntax.

        let id = "not-wf-sa-129";
        let content = include_str!("../tests/xmltest/not-wf/sa/129.xml");
        let section = "section 3.2 [45]";
        Invalid syntax for Element Type Declaration.

        let id = "not-wf-sa-130";
        let content = include_str!("../tests/xmltest/not-wf/sa/130.xml");
        let section = "section 3.2 [45]";
        Invalid syntax for Element Type Declaration.

        let id = "not-wf-sa-131";
        let content = include_str!("../tests/xmltest/not-wf/sa/131.xml");
        let section = "section 3.2 [45]";
        Invalid syntax for Element Type Declaration.

        let id = "not-wf-sa-132";
        let content = include_str!("../tests/xmltest/not-wf/sa/132.xml");
        let section = "section 3.2.1 [50]";
        Invalid syntax mixed connectors used.

        let id = "not-wf-sa-133";
        let content = include_str!("../tests/xmltest/not-wf/sa/133.xml");
        let section = "section 3.2.1";
        Illegal whitespace before optional character causes syntax error.

        let id = "not-wf-sa-134";
        let content = include_str!("../tests/xmltest/not-wf/sa/134.xml");
        let section = "section 3.2.1";
        Illegal whitespace before optional character causes syntax error.

        let id = "not-wf-sa-135";
        let content = include_str!("../tests/xmltest/not-wf/sa/135.xml");
        let section = "section 3.2.1 [47]";
        Invalid character used as connector.

        let id = "not-wf-sa-136";
        let content = include_str!("../tests/xmltest/not-wf/sa/136.xml");
        let section = "section 3.2 [45]";
        Tag omission is invalid in XML.

        let id = "not-wf-sa-137";
        let content = include_str!("../tests/xmltest/not-wf/sa/137.xml");
        let section = "section 3.2 [45]";
        Space is required before a content model.

        let id = "not-wf-sa-138";
        let content = include_str!("../tests/xmltest/not-wf/sa/138.xml");
        let section = "section 3.2.1 [48]";
        Invalid syntax for content particle. 

        let id = "not-wf-sa-139";
        let content = include_str!("../tests/xmltest/not-wf/sa/139.xml");
        let section = "section 3.2.1 [46]";
        The element-content model should not be empty.

        let id = "not-wf-sa-140";
        let content = include_str!("../tests/xmltest/not-wf/sa/140.xml");
        let section = "section 2.3 [4]";
        Character '&amp;#x309a;' is a CombiningChar, not a
        Letter, and so may not begin a name.

        let id = "not-wf-sa-141";
        let content = include_str!("../tests/xmltest/not-wf/sa/141.xml");
        let section = "section 2.3 [5]";
        Character #x0E5C is not legal in XML names.

        let id = "not-wf-sa-142";
        let content = include_str!("../tests/xmltest/not-wf/sa/142.xml");
        let section = "section 2.2 [2]";
        Character #x0000 is not legal anywhere in an XML document.

        let id = "not-wf-sa-143";
        let content = include_str!("../tests/xmltest/not-wf/sa/143.xml");
        let section = "section 2.2 [2]";
        Character #x001F is not legal anywhere in an XML document.

        let id = "not-wf-sa-144";
        let content = include_str!("../tests/xmltest/not-wf/sa/144.xml");
        let section = "section 2.2 [2]";
        Character #xFFFF is not legal anywhere in an XML document.

        let id = "not-wf-sa-145";
        let content = include_str!("../tests/xmltest/not-wf/sa/145.xml");
        let section = "section 2.2 [2]";
        Character #xD800 is not legal anywhere in an XML document.  (If it
        appeared in a UTF-16 surrogate pair, it'd represent half of a UCS-4
        character and so wouldn't really be in the document.)

        let id = "not-wf-sa-146";
        let content = include_str!("../tests/xmltest/not-wf/sa/146.xml");
        let section = "section 2.2 [2]";
        Character references must also refer to legal XML characters;
        #x00110000 is one more than the largest legal character.

        let id = "not-wf-sa-147";
        let content = include_str!("../tests/xmltest/not-wf/sa/147.xml");
        let section = "section 2.8 [22]";
        XML Declaration may not be preceded by whitespace.

        let id = "not-wf-sa-148";
        let content = include_str!("../tests/xmltest/not-wf/sa/148.xml");
        let section = "section 2.8 [22]";
        XML Declaration may not be preceded by comments or whitespace.

        let id = "not-wf-sa-149";
        let content = include_str!("../tests/xmltest/not-wf/sa/149.xml");
        let section = "section 2.8 [28]";
        XML Declaration may not be within a DTD.

        let id = "not-wf-sa-150";
        let content = include_str!("../tests/xmltest/not-wf/sa/150.xml");
        let section = "section 3.1 [43]";
        XML declarations may not be within element content.

        let id = "not-wf-sa-151";
        let content = include_str!("../tests/xmltest/not-wf/sa/151.xml");
        let section = "section 2.8 [27]";
        XML declarations may not follow document content.

        let id = "not-wf-sa-152";
        let content = include_str!("../tests/xmltest/not-wf/sa/152.xml");
        let section = "section 2.8 [22]";
        XML declarations must include the "version=..." string.

        let id = "not-wf-sa-153";
        let content = include_str!("../tests/xmltest/not-wf/sa/153.xml");
        let section = "section 4.3.2";
        Text declarations may not begin internal parsed entities;
        they may only appear at the beginning of external parsed
        (parameter or general) entities.

        let id = "not-wf-sa-154";
        let content = include_str!("../tests/xmltest/not-wf/sa/154.xml");
        let section = "section 2.8 2.6 [23, 17]";
        '&lt;?XML ...?&gt;' is neither an XML declaration
        nor a legal processing instruction target name.

        let id = "not-wf-sa-155";
        let content = include_str!("../tests/xmltest/not-wf/sa/155.xml");
        let section = "section 2.8 2.6 [23, 17]";
        '&lt;?xmL ...?&gt;' is neither an XML declaration
        nor a legal processing instruction target name.

        let id = "not-wf-sa-156";
        let content = include_str!("../tests/xmltest/not-wf/sa/156.xml");
        let section = "section 2.8 2.6 [23, 17]";
        '&lt;?xMl ...?&gt;' is neither an XML declaration
        nor a legal processing instruction target name.

        let id = "not-wf-sa-157";
        let content = include_str!("../tests/xmltest/not-wf/sa/157.xml");
        let section = "section 2.6 [17]";
        '&lt;?xmL ...?&gt;' is not a legal processing instruction
        target name.

        let id = "not-wf-sa-158";
        let content = include_str!("../tests/xmltest/not-wf/sa/158.xml");
        let section = "section 3.3 [52]";
        SGML-ism:  "#NOTATION gif" can't have attributes.

        let id = "not-wf-sa-159";
        let content = include_str!("../tests/xmltest/not-wf/sa/159.xml");
        let section = "section 2.3 [9]";
        Uses '&amp;' unquoted in an entity declaration,
        which is illegal syntax for an entity reference.

        let id = "not-wf-sa-160";
        let content = include_str!("../tests/xmltest/not-wf/sa/160.xml");
        let section = "section 2.8";
        Violates the <EM>PEs in Internal Subset</EM> WFC
        by using a PE reference within a declaration.

        let id = "not-wf-sa-161";
        let content = include_str!("../tests/xmltest/not-wf/sa/161.xml");
        let section = "section 2.8";
        Violates the <EM>PEs in Internal Subset</EM> WFC
        by using a PE reference within a declaration.

        let id = "not-wf-sa-162";
        let content = include_str!("../tests/xmltest/not-wf/sa/162.xml");
        let section = "section 2.8";
        Violates the <EM>PEs in Internal Subset</EM> WFC
        by using a PE reference within a declaration.

        let id = "not-wf-sa-163";
        let content = include_str!("../tests/xmltest/not-wf/sa/163.xml");
        let section = "section 4.1 [69]";
        Invalid placement of Parameter entity reference.

        let id = "not-wf-sa-164";
        let content = include_str!("../tests/xmltest/not-wf/sa/164.xml");
        let section = "section 4.1 [69]";
        Invalid placement of Parameter entity reference.

        let id = "not-wf-sa-165";
        let content = include_str!("../tests/xmltest/not-wf/sa/165.xml");
        let section = "section 4.2 [72]";
        Parameter entity declarations must have a space before
        the '%'.

        let id = "not-wf-sa-166";
        let content = include_str!("../tests/xmltest/not-wf/sa/166.xml");
        let section = "section 2.2 [2]";
        Character FFFF is not legal anywhere in an XML document.

        let id = "not-wf-sa-167";
        let content = include_str!("../tests/xmltest/not-wf/sa/167.xml");
        let section = "section 2.2 [2]";
        Character FFFE is not legal anywhere in an XML document.

        let id = "not-wf-sa-168";
        let content = include_str!("../tests/xmltest/not-wf/sa/168.xml");
        let section = "section 2.2 [2]";
        An unpaired surrogate (D800) is not legal anywhere
        in an XML document.

        let id = "not-wf-sa-169";
        let content = include_str!("../tests/xmltest/not-wf/sa/169.xml");
        let section = "section 2.2 [2]";
        An unpaired surrogate (DC00) is not legal anywhere
        in an XML document.

        let id = "not-wf-sa-170";
        let content = include_str!("../tests/xmltest/not-wf/sa/170.xml");
        let section = "section 2.2 [2]";
        Four byte UTF-8 encodings can encode UCS-4 characters
        which are beyond the range of legal XML characters
        (and can't be expressed in Unicode surrogate pairs).
        This document holds such a character. 

        let id = "not-wf-sa-171";
        let content = include_str!("../tests/xmltest/not-wf/sa/171.xml");
        let section = "section 2.2 [2]";
        Character FFFF is not legal anywhere in an XML document.

        let id = "not-wf-sa-172";
        let content = include_str!("../tests/xmltest/not-wf/sa/172.xml");
        let section = "section 2.2 [2]";
        Character FFFF is not legal anywhere in an XML document.

        let id = "not-wf-sa-173";
        let content = include_str!("../tests/xmltest/not-wf/sa/173.xml");
        let section = "section 2.2 [2]";
        Character FFFF is not legal anywhere in an XML document.

        let id = "not-wf-sa-174";
        let content = include_str!("../tests/xmltest/not-wf/sa/174.xml");
        let section = "section 2.2 [2]";
        Character FFFF is not legal anywhere in an XML document.

        let id = "not-wf-sa-175";
        let content = include_str!("../tests/xmltest/not-wf/sa/175.xml");
        let section = "section 2.2 [2]";
        Character FFFF is not legal anywhere in an XML document.

        let id = "not-wf-sa-176";
        let content = include_str!("../tests/xmltest/not-wf/sa/176.xml");3 [39]
        Start tags must have matching end tags.

        let id = "not-wf-sa-177";
        let content = include_str!("../tests/xmltest/not-wf/sa/177.xml");
        let section = "section 2.2 [2]";
        Character FFFF is not legal anywhere in an XML document.

        let id = "not-wf-sa-178";
        let content = include_str!("../tests/xmltest/not-wf/sa/178.xml");
        let section = "section 3.1 [41]";
        Invalid syntax matching double quote is missing.

        let id = "not-wf-sa-179";
        let content = include_str!("../tests/xmltest/not-wf/sa/179.xml");
        let section = "section 4.1 [66]";
        Invalid syntax matching double quote is missing.

        let id = "not-wf-sa-180";
        let content = include_str!("../tests/xmltest/not-wf/sa/180.xml");
        let section = "section 4.1";
        The <EM>Entity Declared</EM> WFC requires entities to be declared
        before they are used in an attribute list declaration.

        let id = "not-wf-sa-181";
        let content = include_str!("../tests/xmltest/not-wf/sa/181.xml");
        let section = "section 4.3.2";
        Internal parsed entities must match the <EM>content</EM>
        production to be well formed.

        let id = "not-wf-sa-182";
        let content = include_str!("../tests/xmltest/not-wf/sa/182.xml");
        let section = "section 4.3.2";
        Internal parsed entities must match the <EM>content</EM>
        production to be well formed.

        let id = "not-wf-sa-183";
        let content = include_str!("../tests/xmltest/not-wf/sa/183.xml");
        let section = "section 3.2.2 [51]";
        Mixed content declarations may not include content particles.

        let id = "not-wf-sa-184";
        let content = include_str!("../tests/xmltest/not-wf/sa/184.xml");
        let section = "section 3.2.2 [51]";
        In mixed content models, element names must not be
        parenthesized.

        let id = "not-wf-sa-185";
        let content = include_str!("../tests/xmltest/not-wf/sa/185.xml");
        let section = "section 4.1";
        Tests the <EM>Entity Declared</EM> WFC.
        <EM>Note:</EM>  a nonvalidating parser is permitted not to report
        this WFC violation, since it would need to read an external
        parameter entity to distinguish it from a violation of
        the <EM>Standalone Declaration</EM> VC.

        let id = "not-wf-sa-186";
        let content = include_str!("../tests/xmltest/not-wf/sa/186.xml");
        let section = "section 3.1 [44]";
        Whitespace is required between attribute/value pairs.


<!-- Start:  not-wf/not-sa -->
not-wf-not-sa-001
not-wf/not-sa/001.xml
        let section = "section 3.4 [62]";
        Conditional sections must be properly terminated ("]&gt;" used
        instead of "]]&gt;").

not-wf-not-sa-002
not-wf/not-sa/002.xml
        let section = "section 2.6 [17]";
        Processing instruction target names may not be "XML" 
        in any combination of cases.

not-wf-not-sa-003
not-wf/not-sa/003.xml
        let section = "section 3.4 [62]";
        Conditional sections must be properly terminated ("]]&gt;" omitted).

not-wf-not-sa-004
not-wf/not-sa/004.xml
        let section = "section 3.4 [62]";
    Conditional sections must be properly terminated ("]]&gt;" omitted).

not-wf-not-sa-005
not-wf/not-sa/005.xml
        let section = "section 4.1";
    Tests the <EM>Entity Declared</EM> VC by referring to an
    undefined parameter entity within an external entity.

not-wf-not-sa-006
not-wf/not-sa/006.xml
        let section = "section 3.4 [62]";
    Conditional sections need a '[' after the INCLUDE or IGNORE.

not-wf-not-sa-007
not-wf/not-sa/007.xml
        let section = "section 4.3.2 [79]";
    A &lt;!DOCTYPE ...&gt; declaration may not begin any external
    entity; it's only found once, in the document entity.

not-wf-not-sa-008
not-wf/not-sa/008.xml
        let section = "section 4.1 [69]";
    In DTDs, the '%' character must be part of a parameter
    entity reference.

not-wf-not-sa-009
not-wf/not-sa/009.xml
        let section = "section 2.8";
    This test violates WFC:PE Between Declarations in Production 28a.  
    The last character of a markup declaration is not contained in the same 
    parameter-entity text replacement.

<!-- Start:  not-wf/ext-sa -->
not-wf-ext-sa-001
not-wf/ext-sa/001.xml
        let section = "section 4.1";
    Tests the <EM>No Recursion</EM> WFC by having an external general
    entity be self-recursive.

not-wf-ext-sa-002
not-wf/ext-sa/002.xml
        let section = "section 4.3.1 4.3.2 [77, 78]";
    External entities have "text declarations", which do
    not permit the "standalone=..." attribute that's allowed
    in XML declarations.

not-wf-ext-sa-003
not-wf/ext-sa/003.xml
        let section = "section 2.6 [17]";
    Only one text declaration is permitted; a second one
    looks like an illegal processing instruction (target names
    of "xml" in any case are not allowed).

*/
