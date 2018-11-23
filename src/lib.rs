extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate failure;

mod error;
pub mod lexer;
mod lexer_xmltest_not_wf;
mod lexer_xmltest_valid;
mod lexer_xmltest_wf_invalid;
mod token;
