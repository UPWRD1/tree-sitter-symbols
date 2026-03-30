#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use serde::{Deserialize, Serialize};
use tss_macros::generate_nodes;

enum Slice {}
enum Tuple {}
enum WithClause {}
enum Await {}
enum Chevron {}
enum LambdaParameters {}
enum PatternList {}
enum Decorator {}
enum List {}
enum GeneratorExpression {}
enum ExceptClause {}
enum Dictionary {}
enum Module {}
enum Set {}
enum Parameters {}
enum ArgumentList {}
enum FormatSpecifier {}
enum DictionaryComprehension {}
enum RelativeImport {}
enum FinallyClause {}
enum ListComprehension {}
enum CaseClause {}
enum SetComprehension {}
// #[derive(Debug, Clone)]
enum PyNodes {
    SetComprehension(SetComprehension),
    AmpersandToken,
    SolidusequalsSignToken,
    TypeConversion,
    CaseClause(CaseClause),
    PercentSignToken,
    HyphenMinusgreaterThanSignToken,
    ListComprehension(ListComprehension),
    EqualsSignequalsSignToken,
    ExclamationMarkequalsSignToken,
    WhileToken,
    LessThanSignequalsSignToken,
    LessThanSignToken,
    PassToken,
    AmpersandequalsSignToken,
    BinaryOperator,
    AsToken,
    FinallyClause(FinallyClause),
    RelativeImport(RelativeImport),
    DictionaryComprehension(DictionaryComprehension),
    FormatSpecifier(FormatSpecifier),
    AsteriskasteriskToken,
    AsteriskToken,
    _FutureToken,
    ForToken,
    ElifToken,
    FromToken,
    TildeToken,
    DefToken,
    GlobalToken,
    Comment,
    LeftCurlyBracketToken,
    ArgumentList(ArgumentList),
    LeftParenthesisToken,
    ElseToken,
    RaiseToken,
    TryToken,
    SolidussolidusToken,
    GreaterThanSignToken,
    NonlocalToken,
    PercentSignequalsSignToken,
    LessThanSigngreaterThanSignToken,
    SemicolonToken,
    PlusSignequalsSignToken,
    DelToken,
    IsToken,
    Parameters(Parameters),
    VerticalLineToken,
    RightCurlyBracketToken,
    CircumflexAccentequalsSignToken,
    FinallyToken,
    FullStopToken,
    Set(Set),
    AliasedImport,
    CommercialAtToken,
    ColonToken,
    AsteriskasteriskequalsSignToken,
    ExceptToken,
    RightSquareBracketToken,
    SolidusToken,
    GreaterThanSignequalsSignToken,
    Call,
    HyphenMinusequalsSignToken,
    AndToken,
    AsyncToken,
    WithToken,
    GreaterThanSigngreaterThanSignToken,
    ElifClause,
    Module(Module),
    EqualsSignToken,
    ReverseSolidusToken,
    AssertToken,
    SolidussolidusequalsSignToken,
    _Token,
    Dictionary(Dictionary),
    PlusSignToken,
    IfToken,
    ExceptClause(ExceptClause),
    AugmentedAssignment,
    GeneratorExpression(GeneratorExpression),
    CaseToken,
    HyphenMinusToken,
    List(List),
    ClassToken,
    RightParenthesisToken,
    GreaterThanSigngreaterThanSignequalsSignToken,
    LineContinuation,
    UnaryOperator,
    Assignment,
    Decorator(Decorator),
    VerticalLineequalsSignToken,
    LessThanSignlessThanSignequalsSignToken,
    PatternList(PatternList),
    ColonequalsSignToken,
    CircumflexAccentToken,
    ExecToken,
    MatchToken,
    NotToken,
    NotSpacEinToken,
    ContinueToken,
    LambdaParameters(LambdaParameters),
    IsSpacEnotToken,
    ElseClause,
    LeftSquareBracketToken,
    Chevron(Chevron),
    Await(Await),
    ImportToken,
    LessThanSignlessThanSignToken,
    PrintToken,
    WithClause(WithClause),
    Ellipsis,
    InToken,
    AsteriskequalsSignToken,
    CommercialAtequalsSignToken,
    CommaToken,
    Tuple(Tuple),
    BreakToken,
    ReturnToken,
    OrToken,
    Slice(Slice),
    WildcardImport,
}
impl std::str::FromStr for PyNodes {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "&" => { return std::result::Result::Ok(PyNodes::AmpersandToken) }
            "/=" => {
                return std::result::Result::Ok(PyNodes::SolidusequalsSignToken)
            }
            "type_conversion" => {
                return std::result::Result::Ok(PyNodes::TypeConversion)
            }
            "%" => {
                return std::result::Result::Ok(PyNodes::PercentSignToken)
            }
            "->" => {
                return std::result::Result::Ok(PyNodes::HyphenMinusgreaterThanSignToken)
            }
            "==" => {
                return std::result::Result::Ok(PyNodes::EqualsSignequalsSignToken)
            }
            "!=" => {
                return std::result::Result::Ok(PyNodes::ExclamationMarkequalsSignToken)
            }
            "while" => { return std::result::Result::Ok(PyNodes::WhileToken) }
            "<=" => {
                return std::result::Result::Ok(PyNodes::LessThanSignequalsSignToken)
            }
            "<" => {
                return std::result::Result::Ok(PyNodes::LessThanSignToken)
            }
            "pass" => { return std::result::Result::Ok(PyNodes::PassToken) }
            "&=" => {
                return std::result::Result::Ok(PyNodes::AmpersandequalsSignToken)
            }
            "binary_operator" => {
                return std::result::Result::Ok(PyNodes::BinaryOperator)
            }
            "as" => { return std::result::Result::Ok(PyNodes::AsToken) }
            "**" => {
                return std::result::Result::Ok(PyNodes::AsteriskasteriskToken)
            }
            "*" => { return std::result::Result::Ok(PyNodes::AsteriskToken) }
            "__future__" => {
                return std::result::Result::Ok(PyNodes::_FutureToken)
            }
            "for" => { return std::result::Result::Ok(PyNodes::ForToken) }
            "elif" => { return std::result::Result::Ok(PyNodes::ElifToken) }
            "from" => { return std::result::Result::Ok(PyNodes::FromToken) }
            "~" => { return std::result::Result::Ok(PyNodes::TildeToken) }
            "def" => { return std::result::Result::Ok(PyNodes::DefToken) }
            "global" => {
                return std::result::Result::Ok(PyNodes::GlobalToken)
            }
            "comment" => { return std::result::Result::Ok(PyNodes::Comment) }
            "{" => {
                return std::result::Result::Ok(PyNodes::LeftCurlyBracketToken)
            }
            "(" => {
                return std::result::Result::Ok(PyNodes::LeftParenthesisToken)
            }
            "else" => { return std::result::Result::Ok(PyNodes::ElseToken) }
            "raise" => { return std::result::Result::Ok(PyNodes::RaiseToken) }
            "try" => { return std::result::Result::Ok(PyNodes::TryToken) }
            "//" => {
                return std::result::Result::Ok(PyNodes::SolidussolidusToken)
            }
            ">" => {
                return std::result::Result::Ok(PyNodes::GreaterThanSignToken)
            }
            "nonlocal" => {
                return std::result::Result::Ok(PyNodes::NonlocalToken)
            }
            "%=" => {
                return std::result::Result::Ok(PyNodes::PercentSignequalsSignToken)
            }
            "<>" => {
                return std::result::Result::Ok(PyNodes::LessThanSigngreaterThanSignToken)
            }
            ";" => { return std::result::Result::Ok(PyNodes::SemicolonToken) }
            "+=" => {
                return std::result::Result::Ok(PyNodes::PlusSignequalsSignToken)
            }
            "del" => { return std::result::Result::Ok(PyNodes::DelToken) }
            "is" => { return std::result::Result::Ok(PyNodes::IsToken) }
            "|" => {
                return std::result::Result::Ok(PyNodes::VerticalLineToken)
            }
            "}" => {
                return std::result::Result::Ok(PyNodes::RightCurlyBracketToken)
            }
            "^=" => {
                return std::result::Result::Ok(PyNodes::CircumflexAccentequalsSignToken)
            }
            "finally" => {
                return std::result::Result::Ok(PyNodes::FinallyToken)
            }
            "." => { return std::result::Result::Ok(PyNodes::FullStopToken) }
            "aliased_import" => {
                return std::result::Result::Ok(PyNodes::AliasedImport)
            }
            "@" => {
                return std::result::Result::Ok(PyNodes::CommercialAtToken)
            }
            ":" => { return std::result::Result::Ok(PyNodes::ColonToken) }
            "**=" => {
                return std::result::Result::Ok(PyNodes::AsteriskasteriskequalsSignToken)
            }
            "except" => {
                return std::result::Result::Ok(PyNodes::ExceptToken)
            }
            "]" => {
                return std::result::Result::Ok(PyNodes::RightSquareBracketToken)
            }
            "/" => { return std::result::Result::Ok(PyNodes::SolidusToken) }
            ">=" => {
                return std::result::Result::Ok(PyNodes::GreaterThanSignequalsSignToken)
            }
            "call" => { return std::result::Result::Ok(PyNodes::Call) }
            "-=" => {
                return std::result::Result::Ok(PyNodes::HyphenMinusequalsSignToken)
            }
            "and" => { return std::result::Result::Ok(PyNodes::AndToken) }
            "async" => { return std::result::Result::Ok(PyNodes::AsyncToken) }
            "with" => { return std::result::Result::Ok(PyNodes::WithToken) }
            ">>" => {
                return std::result::Result::Ok(PyNodes::GreaterThanSigngreaterThanSignToken)
            }
            "elif_clause" => {
                return std::result::Result::Ok(PyNodes::ElifClause)
            }
            "=" => {
                return std::result::Result::Ok(PyNodes::EqualsSignToken)
            }
            "\\" => {
                return std::result::Result::Ok(PyNodes::ReverseSolidusToken)
            }
            "assert" => {
                return std::result::Result::Ok(PyNodes::AssertToken)
            }
            "//=" => {
                return std::result::Result::Ok(PyNodes::SolidussolidusequalsSignToken)
            }
            "_" => { return std::result::Result::Ok(PyNodes::_Token) }
            "+" => { return std::result::Result::Ok(PyNodes::PlusSignToken) }
            "if" => { return std::result::Result::Ok(PyNodes::IfToken) }
            "augmented_assignment" => {
                return std::result::Result::Ok(PyNodes::AugmentedAssignment)
            }
            "case" => { return std::result::Result::Ok(PyNodes::CaseToken) }
            "-" => {
                return std::result::Result::Ok(PyNodes::HyphenMinusToken)
            }
            "class" => { return std::result::Result::Ok(PyNodes::ClassToken) }
            ")" => {
                return std::result::Result::Ok(PyNodes::RightParenthesisToken)
            }
            ">>=" => {
                return std::result::Result::Ok(PyNodes::GreaterThanSigngreaterThanSignequalsSignToken)
            }
            "line_continuation" => {
                return std::result::Result::Ok(PyNodes::LineContinuation)
            }
            "unary_operator" => {
                return std::result::Result::Ok(PyNodes::UnaryOperator)
            }
            "assignment" => {
                return std::result::Result::Ok(PyNodes::Assignment)
            }
            "|=" => {
                return std::result::Result::Ok(PyNodes::VerticalLineequalsSignToken)
            }
            "<<=" => {
                return std::result::Result::Ok(PyNodes::LessThanSignlessThanSignequalsSignToken)
            }
            ":=" => {
                return std::result::Result::Ok(PyNodes::ColonequalsSignToken)
            }
            "^" => {
                return std::result::Result::Ok(PyNodes::CircumflexAccentToken)
            }
            "exec" => { return std::result::Result::Ok(PyNodes::ExecToken) }
            "match" => { return std::result::Result::Ok(PyNodes::MatchToken) }
            "not" => { return std::result::Result::Ok(PyNodes::NotToken) }
            "not in" => {
                return std::result::Result::Ok(PyNodes::NotSpacEinToken)
            }
            "continue" => {
                return std::result::Result::Ok(PyNodes::ContinueToken)
            }
            "is not" => {
                return std::result::Result::Ok(PyNodes::IsSpacEnotToken)
            }
            "else_clause" => {
                return std::result::Result::Ok(PyNodes::ElseClause)
            }
            "[" => {
                return std::result::Result::Ok(PyNodes::LeftSquareBracketToken)
            }
            "import" => {
                return std::result::Result::Ok(PyNodes::ImportToken)
            }
            "<<" => {
                return std::result::Result::Ok(PyNodes::LessThanSignlessThanSignToken)
            }
            "print" => { return std::result::Result::Ok(PyNodes::PrintToken) }
            "ellipsis" => {
                return std::result::Result::Ok(PyNodes::Ellipsis)
            }
            "in" => { return std::result::Result::Ok(PyNodes::InToken) }
            "*=" => {
                return std::result::Result::Ok(PyNodes::AsteriskequalsSignToken)
            }
            "@=" => {
                return std::result::Result::Ok(PyNodes::CommercialAtequalsSignToken)
            }
            "," => { return std::result::Result::Ok(PyNodes::CommaToken) }
            "break" => { return std::result::Result::Ok(PyNodes::BreakToken) }
            "return" => {
                return std::result::Result::Ok(PyNodes::ReturnToken)
            }
            "or" => { return std::result::Result::Ok(PyNodes::OrToken) }
            "wildcard_import" => {
                return std::result::Result::Ok(PyNodes::WildcardImport)
            }
            err => {


                {
                    ::core::panicking::panic_fmt(format_args!("Unknown token name: \'{0}\'",
                            err));
                }
            }
        }
    }
}
impl std::fmt::Display for PyNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AmpersandToken => { f.write_fmt(format_args!("&")) }
            Self::SolidusequalsSignToken => {
                f.write_fmt(format_args!("/="))
            }
            Self::TypeConversion => {
                f.write_fmt(format_args!("type_conversion"))
            }
            Self::PercentSignToken => { f.write_fmt(format_args!("%")) }
            Self::HyphenMinusgreaterThanSignToken => {
                f.write_fmt(format_args!("->"))
            }
            Self::EqualsSignequalsSignToken => {
                f.write_fmt(format_args!("=="))
            }
            Self::ExclamationMarkequalsSignToken => {
                f.write_fmt(format_args!("!="))
            }
            Self::WhileToken => { f.write_fmt(format_args!("while")) }
            Self::LessThanSignequalsSignToken => {
                f.write_fmt(format_args!("<="))
            }
            Self::LessThanSignToken => { f.write_fmt(format_args!("<")) }
            Self::PassToken => { f.write_fmt(format_args!("pass")) }
            Self::AmpersandequalsSignToken => {
                f.write_fmt(format_args!("&="))
            }
            Self::BinaryOperator => {
                f.write_fmt(format_args!("binary_operator"))
            }
            Self::AsToken => { f.write_fmt(format_args!("as")) }
            Self::AsteriskasteriskToken => { f.write_fmt(format_args!("**")) }
            Self::AsteriskToken => { f.write_fmt(format_args!("*")) }
            Self::_FutureToken => { f.write_fmt(format_args!("__future__")) }
            Self::ForToken => { f.write_fmt(format_args!("for")) }
            Self::ElifToken => { f.write_fmt(format_args!("elif")) }
            Self::FromToken => { f.write_fmt(format_args!("from")) }
            Self::TildeToken => { f.write_fmt(format_args!("~")) }
            Self::DefToken => { f.write_fmt(format_args!("def")) }
            Self::GlobalToken => { f.write_fmt(format_args!("global")) }
            Self::Comment => { f.write_fmt(format_args!("comment")) }
            Self::LeftCurlyBracketToken => { f.write_fmt(format_args!("{{")) }
            Self::LeftParenthesisToken => { f.write_fmt(format_args!("(")) }
            Self::ElseToken => { f.write_fmt(format_args!("else")) }
            Self::RaiseToken => { f.write_fmt(format_args!("raise")) }
            Self::TryToken => { f.write_fmt(format_args!("try")) }
            Self::SolidussolidusToken => { f.write_fmt(format_args!("//")) }
            Self::GreaterThanSignToken => { f.write_fmt(format_args!(">")) }
            Self::NonlocalToken => { f.write_fmt(format_args!("nonlocal")) }
            Self::PercentSignequalsSignToken => {
                f.write_fmt(format_args!("%="))
            }
            Self::LessThanSigngreaterThanSignToken => {
                f.write_fmt(format_args!("<>"))
            }
            Self::SemicolonToken => { f.write_fmt(format_args!(";")) }
            Self::PlusSignequalsSignToken => {
                f.write_fmt(format_args!("+="))
            }
            Self::DelToken => { f.write_fmt(format_args!("del")) }
            Self::IsToken => { f.write_fmt(format_args!("is")) }
            Self::VerticalLineToken => { f.write_fmt(format_args!("|")) }
            Self::RightCurlyBracketToken => {
                f.write_fmt(format_args!("}}"))
            }
            Self::CircumflexAccentequalsSignToken => {
                f.write_fmt(format_args!("^="))
            }
            Self::FinallyToken => { f.write_fmt(format_args!("finally")) }
            Self::FullStopToken => { f.write_fmt(format_args!(".")) }
            Self::AliasedImport => {
                f.write_fmt(format_args!("aliased_import"))
            }
            Self::CommercialAtToken => { f.write_fmt(format_args!("@")) }
            Self::ColonToken => { f.write_fmt(format_args!(":")) }
            Self::AsteriskasteriskequalsSignToken => {
                f.write_fmt(format_args!("**="))
            }
            Self::ExceptToken => { f.write_fmt(format_args!("except")) }
            Self::RightSquareBracketToken => {
                f.write_fmt(format_args!("]"))
            }
            Self::SolidusToken => { f.write_fmt(format_args!("/")) }
            Self::GreaterThanSignequalsSignToken => {
                f.write_fmt(format_args!(">="))
            }
            Self::Call => { f.write_fmt(format_args!("call")) }
            Self::HyphenMinusequalsSignToken => {
                f.write_fmt(format_args!("-="))
            }
            Self::AndToken => { f.write_fmt(format_args!("and")) }
            Self::AsyncToken => { f.write_fmt(format_args!("async")) }
            Self::WithToken => { f.write_fmt(format_args!("with")) }
            Self::GreaterThanSigngreaterThanSignToken => {
                f.write_fmt(format_args!(">>"))
            }
            Self::ElifClause => { f.write_fmt(format_args!("elif_clause")) }
            Self::EqualsSignToken => { f.write_fmt(format_args!("=")) }
            Self::ReverseSolidusToken => { f.write_fmt(format_args!("\\")) }
            Self::AssertToken => { f.write_fmt(format_args!("assert")) }
            Self::SolidussolidusequalsSignToken => {
                f.write_fmt(format_args!("//="))
            }
            Self::_Token => { f.write_fmt(format_args!("_")) }
            Self::PlusSignToken => { f.write_fmt(format_args!("+")) }
            Self::IfToken => { f.write_fmt(format_args!("if")) }
            Self::AugmentedAssignment => {
                f.write_fmt(format_args!("augmented_assignment"))
            }
            Self::CaseToken => { f.write_fmt(format_args!("case")) }
            Self::HyphenMinusToken => { f.write_fmt(format_args!("-")) }
            Self::ClassToken => { f.write_fmt(format_args!("class")) }
            Self::RightParenthesisToken => { f.write_fmt(format_args!(")")) }
            Self::GreaterThanSigngreaterThanSignequalsSignToken => {
                f.write_fmt(format_args!(">>="))
            }
            Self::LineContinuation => {
                f.write_fmt(format_args!("line_continuation"))
            }
            Self::UnaryOperator => {
                f.write_fmt(format_args!("unary_operator"))
            }
            Self::Assignment => { f.write_fmt(format_args!("assignment")) }
            Self::VerticalLineequalsSignToken => {
                f.write_fmt(format_args!("|="))
            }
            Self::LessThanSignlessThanSignequalsSignToken => {
                f.write_fmt(format_args!("<<="))
            }
            Self::ColonequalsSignToken => { f.write_fmt(format_args!(":=")) }
            Self::CircumflexAccentToken => { f.write_fmt(format_args!("^")) }
            Self::ExecToken => { f.write_fmt(format_args!("exec")) }
            Self::MatchToken => { f.write_fmt(format_args!("match")) }
            Self::NotToken => { f.write_fmt(format_args!("not")) }
            Self::NotSpacEinToken => { f.write_fmt(format_args!("not in")) }
            Self::ContinueToken => { f.write_fmt(format_args!("continue")) }
            Self::IsSpacEnotToken => { f.write_fmt(format_args!("is not")) }
            Self::ElseClause => { f.write_fmt(format_args!("else_clause")) }
            Self::LeftSquareBracketToken => { f.write_fmt(format_args!("[")) }
            Self::ImportToken => { f.write_fmt(format_args!("import")) }
            Self::LessThanSignlessThanSignToken => {
                f.write_fmt(format_args!("<<"))
            }
            Self::PrintToken => { f.write_fmt(format_args!("print")) }
            Self::Ellipsis => { f.write_fmt(format_args!("ellipsis")) }
            Self::InToken => { f.write_fmt(format_args!("in")) }
            Self::AsteriskequalsSignToken => {
                f.write_fmt(format_args!("*="))
            }
            Self::CommercialAtequalsSignToken => {
                f.write_fmt(format_args!("@="))
            }
            Self::CommaToken => { f.write_fmt(format_args!(",")) }
            Self::BreakToken => { f.write_fmt(format_args!("break")) }
            Self::ReturnToken => { f.write_fmt(format_args!("return")) }
            Self::OrToken => { f.write_fmt(format_args!("or")) }
            Self::WildcardImport => {
                f.write_fmt(format_args!("wildcard_import"))
            }
        }
    }
}
