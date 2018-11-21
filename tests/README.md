# Tests

## Valid documents
* [planes.xml](http://www.cs.utexas.edu/~mitra/csFall2018/cs329/lectures/xml/planes.xml.txt) -- references an external DTD
* [planes.xsd.xml](http://www.cs.utexas.edu/~mitra/csFall2018/cs329/lectures/xml/planes.xsd.xml.txt) -- references external schema, uses namespaces
* [planes.xsd](http://www.cs.utexas.edu/~mitra/csFall2018/cs329/lectures/xml/planes.xsd.txt) -- references external schema, uses namespaces, self-closing tags
* [note-in-dtd.xml](https://www.w3schools.com/xml/note_in_dtd.xml) -- uses an internal dtd
* [note-ex-dtd.xml](https://www.w3schools.com/xml/note_ex_dtd.xml) -- same as previous but referencing external dtd

## Invalid documents
* [xslplanes.1.xml](http://www.cs.utexas.edu/~mitra/csFall2018/cs329/lectures/xml/xslplanes.1.xml.txt) -- ProcessInstruction target starts with "xml", [which is illegal by the spec](https://www.w3.org/TR/2008/REC-xml-20081126/#sec-pi)
