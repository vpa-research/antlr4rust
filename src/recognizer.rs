use crate::atn::ATN;
use crate::parser::ParserNodeType;

use crate::token_factory::TokenAware;
use crate::vocabulary::Vocabulary;

/// Major version of this runtime.
/// Used by generated parser to verify that it is compatible with current version of runtime
pub const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
/// Major version of this runtime.
/// Used by generated parser to verify that it is compatible with current version of runtime
pub const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");

const fn str_eq(lhs: &str, rhs: &str) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }

    let lhs = lhs.as_bytes();
    let rhs = rhs.as_bytes();
    let mut i = 0;

    while i < lhs.len() {
        if lhs[i] != rhs[i] {
            return false;
        }

        i += 1;
    }

    true
}

/// Used by generated parser to verify that it is compatible with current version of runtime
pub const fn check_version(major: &str, minor: &str) {
    assert!(
        str_eq(major, VERSION_MAJOR) && str_eq(minor, VERSION_MINOR),
        "parser is not compatible with current runtime version, please generate parser with the latest version of ANTLR",
    )
}
//todo just a reminder to update version to be inserted in generated parser,
//const _:[();0-!(VERSION_MAJOR == "0" && VERSION_MINOR == "2") as usize] = [];

/// **! Usually generated by ANTLR !**
pub trait Recognizer<'input>: TokenAware<'input> {
    type Node: ParserNodeType<'input, TF = Self::TF>;
    fn sempred(
        &mut self,
        _localctx: Option<&<Self::Node as ParserNodeType<'input>>::Type>,
        _rule_index: isize,
        _action_index: isize,
    ) -> bool
    where
        Self: Sized,
    {
        true
    }
    fn action(
        &mut self,
        _localctx: Option<&<Self::Node as ParserNodeType<'input>>::Type>,
        _rule_index: isize,
        _action_index: isize,
    ) where
        Self: Sized,
    {
    }

    /// Returns array of rule names.
    /// Used for debugging and error reporting
    fn get_rule_names(&self) -> &[&str] {
        &[]
    }
    fn get_vocabulary(&self) -> &dyn Vocabulary {
        unimplemented!()
    }

    /// Name of the file this recognizer was generated from
    fn get_grammar_file_name(&self) -> &str {
        ""
    }
    fn get_atn(&self) -> &ATN {
        unimplemented!()
    }
}

/// **! Usually generated by ANTLR !**
///
/// Used to make user predicates and actions callable by parser
/// Generated by ANTLR tool from actions and predicated added in grammar file
pub trait Actions<'a, P: Recognizer<'a>> {
    fn sempred(
        _localctx: Option<&<P::Node as ParserNodeType<'a>>::Type>,
        _rule_index: isize,
        _action_index: isize,
        _recog: &mut P,
    ) -> bool {
        true
    }

    fn action(
        _localctx: Option<&<P::Node as ParserNodeType<'a>>::Type>,
        _rule_index: isize,
        _action_index: isize,
        _recog: &mut P,
    ) {
    }

    /// Returns array of rule names.
    /// Used for debugging and error reporting
    fn get_rule_names(&self) -> &[&str] {
        &[]
    }
    fn get_vocabulary(&self) -> &dyn Vocabulary {
        unimplemented!()
    }

    /// Name of the file this recognizer was generated from
    fn get_grammar_file_name(&self) -> &str {
        ""
    }
    fn get_atn(&self) -> &ATN {
        unimplemented!()
    }
}

//impl Recognizer for BaseRecognizer {
//    fn get_state(&self) -> isize {
//        self.state
//    }
//
//    fn set_state(&mut self, _v: isize) {
//        self.state = _v;
//    }
//
//    fn add_error_listener(&mut self, _listener: Box<ErrorListener>) {
//        self.listeners.push(_listener)
//    }
//
//    fn remove_error_listeners(&self) {
//        unimplemented!()
//    }
//
//    fn get_error_listener_dispatch(&self) -> Box<ErrorListener> {
//        unimplemented!()
//    }
//}
//
//pub struct BaseRecognizer {
//    pub listeners: Vec<Box<ErrorListener>>,
//    pub state: isize, //    rule_names: Vec<String>,
//    //    literal_names: Vec<String>,
//    //    symbolic_names: Vec<String>,
//    //    grammar_file_name: String
//}
//
//impl BaseRecognizer {
//    pub fn new_base_recognizer() -> BaseRecognizer {
//        BaseRecognizer {
//            listeners: Vec::new(),
//            state: -1,
//        }
//    }
//
//    fn check_version(&self, _toolVersion: String) {
//        unimplemented!()
//    }
//
//    fn get_token_names(&self) -> Vec<String> {
//        unimplemented!()
//    }
//
//    fn get_rule_index_map(&self) -> Map<isize, String> {
//        unimplemented!()
//    }
//
//    fn get_token_type(&self, _tokenName: String) -> isize {
//        unimplemented!()
//    }
//
//    fn get_error_header(&self, _e: ANTLRError) -> String {
//        unimplemented!()
//    }
//
//    fn get_token_error_display(&self, _t: &Token) -> String {
//        unimplemented!()
//    }
//}
