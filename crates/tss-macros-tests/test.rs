#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use tss_macros::generate_nodes;

enum RustNodes {
    _DeclarationStatement,
    _Expression,
    _Literal,
    _LiteralPattern,
    _Pattern,
    _Type,
    AbstractType,
    Arguments,
    ArrayExpression,
    ArrayType,
    AssignmentExpression,
    AssociatedType,
    AsyncBlock,
    Attribute,
    AttributeItem,
    AwaitExpression,
    BaseFieldInitializer,
    BinaryExpression,
    Block,
    BlockComment,
    BooleanLiteral,
    BoundedType,
    BracketedType,
    BreakExpression,
    CallExpression,
    CapturedPattern,
    ClosureExpression,
    ClosureParameters,
    CompoundAssignmentExpr,
    ConstBlock,
    ConstItem,
    ConstParameter,
    ContinueExpression,
    DeclarationList,
    DynamicType,
    ElseClause,
    EmptyStatement,
    EnumItem,
    EnumVariant,
    EnumVariantList,
    ExpressionStatement,
    ExternCrateDeclaration,
    ExternModifier,
    FieldDeclaration,
    FieldDeclarationList,
    FieldExpression,
    FieldInitializer,
    FieldInitializerList,
    FieldPattern,
    ForExpression,
    ForLifetimes,
    ForeignModItem,
    FragmentSpecifier,
    FunctionItem,
    FunctionModifiers,
    FunctionSignatureItem,
    FunctionType,
    GenBlock,
    GenericFunction,
    GenericPattern,
    GenericType,
    GenericTypeWithTurbofish,
    HigherRankedTraitBound,
    IfExpression,
    ImplItem,
    IndexExpression,
    InnerAttributeItem,
    InnerDocCommentMarker,
    Label,
    LetChain,
    LetCondition,
    LetDeclaration,
    Lifetime,
    LifetimeParameter,
    LineComment,
    LoopExpression,
    MacroDefinition,
    MacroInvocation,
    MacroRule,
    MatchArm,
    MatchBlock,
    MatchExpression,
    MatchPattern,
    ModItem,
    MutPattern,
    NegativeLiteral,
    NeverType,
    OrPattern,
    OrderedFieldDeclarationList,
    OuterDocCommentMarker,
    Parameter,
    Parameters,
    ParenthesizedExpression,
    PointerType,
    QualifiedType,
    RangeExpression,
    RangePattern,
    RawStringLiteral,
    RefPattern,
    ReferenceExpression,
    ReferencePattern,
    ReferenceType,
    RemainingFieldPattern,
    RemovedTraitBound,
    ReturnExpression,
    ScopedIdentifier,
    ScopedTypeIdentifier,
    ScopedUseList,
    SelfParameter,
    ShorthandFieldInitializer,
    SlicePattern,
    SourceFile,
    StaticItem,
    StringLiteral,
    StructExpression,
    StructItem,
    StructPattern,
    TokenBindingPattern,
    TokenRepetition,
    TokenRepetitionPattern,
    TokenTree,
    TokenTreePattern,
    TraitBounds,
    TraitItem,
    TryBlock,
    TryExpression,
    TupleExpression,
    TuplePattern,
    TupleStructPattern,
    TupleType,
    TypeArguments,
    TypeBinding,
    TypeCastExpression,
    TypeItem,
    TypeParameter,
    TypeParameters,
    UnaryExpression,
    UnionItem,
    UnitExpression,
    UnitType,
    UnsafeBlock,
    UseAsClause,
    UseBounds,
    UseDeclaration,
    UseList,
    UseWildcard,
    VariadicParameter,
    VisibilityModifier,
    WhereClause,
    WherePredicate,
    WhileExpression,
    YieldExpression,
    ExclamationMarkToken,
    ExclamationMarkequalsSignToken,
    QuotationMarkToken,
    NumberSignToken,
    DollarSignToken,
    PercentSignToken,
    PercentSignequalsSignToken,
    AmpersandToken,
    AmpersandampersandToken,
    AmpersandequalsSignToken,
    ApostropheToken,
    LeftParenthesisToken,
    RightParenthesisToken,
    AsteriskToken,
    AsterisksolidusToken,
    AsteriskequalsSignToken,
    PlusSignToken,
    PlusSignequalsSignToken,
    CommaToken,
    HyphenMinusToken,
    HyphenMinusequalsSignToken,
    HyphenMinusgreaterThanSignToken,
    FullStopToken,
    FullStopfullStopToken,
    FullStopfullStopfullStopToken,
    FullStopfullStopequalsSignToken,
    SolidusToken,
    SolidusasteriskToken,
    SolidussolidusToken,
    SolidusequalsSignToken,
    ColonToken,
    ColoncolonToken,
    SemicolonToken,
    LessThanSignToken,
    LessThanSignlessThanSignToken,
    LessThanSignlessThanSignequalsSignToken,
    LessThanSignequalsSignToken,
    EqualsSignToken,
    EqualsSignequalsSignToken,
    EqualsSigngreaterThanSignToken,
    GreaterThanSignToken,
    GreaterThanSignequalsSignToken,
    GreaterThanSigngreaterThanSignToken,
    GreaterThanSigngreaterThanSignequalsSignToken,
    QuestionMarkToken,
    CommercialAtToken,
    LeftSquareBracketToken,
    RightSquareBracketToken,
    CircumflexAccentToken,
    CircumflexAccentequalsSignToken,
    _Token,
    AsToken,
    AsyncToken,
    AwaitToken,
    BlockToken,
    BreakToken,
    CharLiteral,
    ConstToken,
    ContinueToken,
    Crate,
    DefaultToken,
    DocComment,
    DynToken,
    ElseToken,
    EnumToken,
    EscapeSequence,
    ExprToken,
    Expr2021Token,
    ExternToken,
    FalseToken,
    FieldIdentifier,
    FloatLiteral,
    FnToken,
    ForToken,
    GenToken,
    IdentToken,
    Identifier,
    IfToken,
    ImplToken,
    InToken,
    IntegerLiteral,
    ItemToken,
    LetToken,
    LifetimeToken,
    LiteralToken,
    LoopToken,
    MacroRulesExclamationMarkToken,
    MatchToken,
    MetaToken,
    Metavariable,
    ModToken,
    MoveToken,
    MutableSpecifier,
    PatToken,
    PatParamToken,
    PathToken,
    PrimitiveType,
    PubToken,
    RawToken,
    RefToken,
    ReturnToken,
    SelfToken,
    Shebang,
    ShorthandFieldIdentifier,
    StaticToken,
    StmtToken,
    StringContent,
    StructToken,
    Super,
    TraitToken,
    TrueToken,
    TryToken,
    TtToken,
    TyToken,
    TypeToken,
    TypeIdentifier,
    UnionToken,
    UnsafeToken,
    UseToken,
    VisToken,
    WhereToken,
    WhileToken,
    YieldToken,
    LeftCurlyBracketToken,
    VerticalLineToken,
    VerticalLineequalsSignToken,
    VerticalLineverticalLineToken,
    RightCurlyBracketToken,
}
impl std::str::FromStr for RustNodes {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "_declaration_statement" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement)
            }
            "_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression)
            }
            "_literal" => {
                return std::result::Result::Ok(RustNodes::_Literal)
            }
            "_literal_pattern" => {
                return std::result::Result::Ok(RustNodes::_LiteralPattern)
            }
            "_pattern" => {
                return std::result::Result::Ok(RustNodes::_Pattern)
            }
            "_type" => { return std::result::Result::Ok(RustNodes::_Type) }
            "abstract_type" => {
                return std::result::Result::Ok(RustNodes::AbstractType)
            }
            "arguments" => {
                return std::result::Result::Ok(RustNodes::Arguments)
            }
            "array_expression" => {
                return std::result::Result::Ok(RustNodes::ArrayExpression)
            }
            "array_type" => {
                return std::result::Result::Ok(RustNodes::ArrayType)
            }
            "assignment_expression" => {
                return std::result::Result::Ok(RustNodes::AssignmentExpression)
            }
            "associated_type" => {
                return std::result::Result::Ok(RustNodes::AssociatedType)
            }
            "async_block" => {
                return std::result::Result::Ok(RustNodes::AsyncBlock)
            }
            "attribute" => {
                return std::result::Result::Ok(RustNodes::Attribute)
            }
            "attribute_item" => {
                return std::result::Result::Ok(RustNodes::AttributeItem)
            }
            "await_expression" => {
                return std::result::Result::Ok(RustNodes::AwaitExpression)
            }
            "base_field_initializer" => {
                return std::result::Result::Ok(RustNodes::BaseFieldInitializer)
            }
            "binary_expression" => {
                return std::result::Result::Ok(RustNodes::BinaryExpression)
            }
            "block" => { return std::result::Result::Ok(RustNodes::Block) }
            "block_comment" => {
                return std::result::Result::Ok(RustNodes::BlockComment)
            }
            "boolean_literal" => {
                return std::result::Result::Ok(RustNodes::BooleanLiteral)
            }
            "bounded_type" => {
                return std::result::Result::Ok(RustNodes::BoundedType)
            }
            "bracketed_type" => {
                return std::result::Result::Ok(RustNodes::BracketedType)
            }
            "break_expression" => {
                return std::result::Result::Ok(RustNodes::BreakExpression)
            }
            "call_expression" => {
                return std::result::Result::Ok(RustNodes::CallExpression)
            }
            "captured_pattern" => {
                return std::result::Result::Ok(RustNodes::CapturedPattern)
            }
            "closure_expression" => {
                return std::result::Result::Ok(RustNodes::ClosureExpression)
            }
            "closure_parameters" => {
                return std::result::Result::Ok(RustNodes::ClosureParameters)
            }
            "compound_assignment_expr" => {
                return std::result::Result::Ok(RustNodes::CompoundAssignmentExpr)
            }
            "const_block" => {
                return std::result::Result::Ok(RustNodes::ConstBlock)
            }
            "const_item" => {
                return std::result::Result::Ok(RustNodes::ConstItem)
            }
            "const_parameter" => {
                return std::result::Result::Ok(RustNodes::ConstParameter)
            }
            "continue_expression" => {
                return std::result::Result::Ok(RustNodes::ContinueExpression)
            }
            "declaration_list" => {
                return std::result::Result::Ok(RustNodes::DeclarationList)
            }
            "dynamic_type" => {
                return std::result::Result::Ok(RustNodes::DynamicType)
            }
            "else_clause" => {
                return std::result::Result::Ok(RustNodes::ElseClause)
            }
            "empty_statement" => {
                return std::result::Result::Ok(RustNodes::EmptyStatement)
            }
            "enum_item" => {
                return std::result::Result::Ok(RustNodes::EnumItem)
            }
            "enum_variant" => {
                return std::result::Result::Ok(RustNodes::EnumVariant)
            }
            "enum_variant_list" => {
                return std::result::Result::Ok(RustNodes::EnumVariantList)
            }
            "expression_statement" => {
                return std::result::Result::Ok(RustNodes::ExpressionStatement)
            }
            "extern_crate_declaration" => {
                return std::result::Result::Ok(RustNodes::ExternCrateDeclaration)
            }
            "extern_modifier" => {
                return std::result::Result::Ok(RustNodes::ExternModifier)
            }
            "field_declaration" => {
                return std::result::Result::Ok(RustNodes::FieldDeclaration)
            }
            "field_declaration_list" => {
                return std::result::Result::Ok(RustNodes::FieldDeclarationList)
            }
            "field_expression" => {
                return std::result::Result::Ok(RustNodes::FieldExpression)
            }
            "field_initializer" => {
                return std::result::Result::Ok(RustNodes::FieldInitializer)
            }
            "field_initializer_list" => {
                return std::result::Result::Ok(RustNodes::FieldInitializerList)
            }
            "field_pattern" => {
                return std::result::Result::Ok(RustNodes::FieldPattern)
            }
            "for_expression" => {
                return std::result::Result::Ok(RustNodes::ForExpression)
            }
            "for_lifetimes" => {
                return std::result::Result::Ok(RustNodes::ForLifetimes)
            }
            "foreign_mod_item" => {
                return std::result::Result::Ok(RustNodes::ForeignModItem)
            }
            "fragment_specifier" => {
                return std::result::Result::Ok(RustNodes::FragmentSpecifier)
            }
            "function_item" => {
                return std::result::Result::Ok(RustNodes::FunctionItem)
            }
            "function_modifiers" => {
                return std::result::Result::Ok(RustNodes::FunctionModifiers)
            }
            "function_signature_item" => {
                return std::result::Result::Ok(RustNodes::FunctionSignatureItem)
            }
            "function_type" => {
                return std::result::Result::Ok(RustNodes::FunctionType)
            }
            "gen_block" => {
                return std::result::Result::Ok(RustNodes::GenBlock)
            }
            "generic_function" => {
                return std::result::Result::Ok(RustNodes::GenericFunction)
            }
            "generic_pattern" => {
                return std::result::Result::Ok(RustNodes::GenericPattern)
            }
            "generic_type" => {
                return std::result::Result::Ok(RustNodes::GenericType)
            }
            "generic_type_with_turbofish" => {
                return std::result::Result::Ok(RustNodes::GenericTypeWithTurbofish)
            }
            "higher_ranked_trait_bound" => {
                return std::result::Result::Ok(RustNodes::HigherRankedTraitBound)
            }
            "if_expression" => {
                return std::result::Result::Ok(RustNodes::IfExpression)
            }
            "impl_item" => {
                return std::result::Result::Ok(RustNodes::ImplItem)
            }
            "index_expression" => {
                return std::result::Result::Ok(RustNodes::IndexExpression)
            }
            "inner_attribute_item" => {
                return std::result::Result::Ok(RustNodes::InnerAttributeItem)
            }
            "inner_doc_comment_marker" => {
                return std::result::Result::Ok(RustNodes::InnerDocCommentMarker)
            }
            "label" => { return std::result::Result::Ok(RustNodes::Label) }
            "let_chain" => {
                return std::result::Result::Ok(RustNodes::LetChain)
            }
            "let_condition" => {
                return std::result::Result::Ok(RustNodes::LetCondition)
            }
            "let_declaration" => {
                return std::result::Result::Ok(RustNodes::LetDeclaration)
            }
            "lifetime" => {
                return std::result::Result::Ok(RustNodes::Lifetime)
            }
            "lifetime_parameter" => {
                return std::result::Result::Ok(RustNodes::LifetimeParameter)
            }
            "line_comment" => {
                return std::result::Result::Ok(RustNodes::LineComment)
            }
            "loop_expression" => {
                return std::result::Result::Ok(RustNodes::LoopExpression)
            }
            "macro_definition" => {
                return std::result::Result::Ok(RustNodes::MacroDefinition)
            }
            "macro_invocation" => {
                return std::result::Result::Ok(RustNodes::MacroInvocation)
            }
            "macro_rule" => {
                return std::result::Result::Ok(RustNodes::MacroRule)
            }
            "match_arm" => {
                return std::result::Result::Ok(RustNodes::MatchArm)
            }
            "match_block" => {
                return std::result::Result::Ok(RustNodes::MatchBlock)
            }
            "match_expression" => {
                return std::result::Result::Ok(RustNodes::MatchExpression)
            }
            "match_pattern" => {
                return std::result::Result::Ok(RustNodes::MatchPattern)
            }
            "mod_item" => {
                return std::result::Result::Ok(RustNodes::ModItem)
            }
            "mut_pattern" => {
                return std::result::Result::Ok(RustNodes::MutPattern)
            }
            "negative_literal" => {
                return std::result::Result::Ok(RustNodes::NegativeLiteral)
            }
            "never_type" => {
                return std::result::Result::Ok(RustNodes::NeverType)
            }
            "or_pattern" => {
                return std::result::Result::Ok(RustNodes::OrPattern)
            }
            "ordered_field_declaration_list" => {
                return std::result::Result::Ok(RustNodes::OrderedFieldDeclarationList)
            }
            "outer_doc_comment_marker" => {
                return std::result::Result::Ok(RustNodes::OuterDocCommentMarker)
            }
            "parameter" => {
                return std::result::Result::Ok(RustNodes::Parameter)
            }
            "parameters" => {
                return std::result::Result::Ok(RustNodes::Parameters)
            }
            "parenthesized_expression" => {
                return std::result::Result::Ok(RustNodes::ParenthesizedExpression)
            }
            "pointer_type" => {
                return std::result::Result::Ok(RustNodes::PointerType)
            }
            "qualified_type" => {
                return std::result::Result::Ok(RustNodes::QualifiedType)
            }
            "range_expression" => {
                return std::result::Result::Ok(RustNodes::RangeExpression)
            }
            "range_pattern" => {
                return std::result::Result::Ok(RustNodes::RangePattern)
            }
            "raw_string_literal" => {
                return std::result::Result::Ok(RustNodes::RawStringLiteral)
            }
            "ref_pattern" => {
                return std::result::Result::Ok(RustNodes::RefPattern)
            }
            "reference_expression" => {
                return std::result::Result::Ok(RustNodes::ReferenceExpression)
            }
            "reference_pattern" => {
                return std::result::Result::Ok(RustNodes::ReferencePattern)
            }
            "reference_type" => {
                return std::result::Result::Ok(RustNodes::ReferenceType)
            }
            "remaining_field_pattern" => {
                return std::result::Result::Ok(RustNodes::RemainingFieldPattern)
            }
            "removed_trait_bound" => {
                return std::result::Result::Ok(RustNodes::RemovedTraitBound)
            }
            "return_expression" => {
                return std::result::Result::Ok(RustNodes::ReturnExpression)
            }
            "scoped_identifier" => {
                return std::result::Result::Ok(RustNodes::ScopedIdentifier)
            }
            "scoped_type_identifier" => {
                return std::result::Result::Ok(RustNodes::ScopedTypeIdentifier)
            }
            "scoped_use_list" => {
                return std::result::Result::Ok(RustNodes::ScopedUseList)
            }
            "self_parameter" => {
                return std::result::Result::Ok(RustNodes::SelfParameter)
            }
            "shorthand_field_initializer" => {
                return std::result::Result::Ok(RustNodes::ShorthandFieldInitializer)
            }
            "slice_pattern" => {
                return std::result::Result::Ok(RustNodes::SlicePattern)
            }
            "source_file" => {
                return std::result::Result::Ok(RustNodes::SourceFile)
            }
            "static_item" => {
                return std::result::Result::Ok(RustNodes::StaticItem)
            }
            "string_literal" => {
                return std::result::Result::Ok(RustNodes::StringLiteral)
            }
            "struct_expression" => {
                return std::result::Result::Ok(RustNodes::StructExpression)
            }
            "struct_item" => {
                return std::result::Result::Ok(RustNodes::StructItem)
            }
            "struct_pattern" => {
                return std::result::Result::Ok(RustNodes::StructPattern)
            }
            "token_binding_pattern" => {
                return std::result::Result::Ok(RustNodes::TokenBindingPattern)
            }
            "token_repetition" => {
                return std::result::Result::Ok(RustNodes::TokenRepetition)
            }
            "token_repetition_pattern" => {
                return std::result::Result::Ok(RustNodes::TokenRepetitionPattern)
            }
            "token_tree" => {
                return std::result::Result::Ok(RustNodes::TokenTree)
            }
            "token_tree_pattern" => {
                return std::result::Result::Ok(RustNodes::TokenTreePattern)
            }
            "trait_bounds" => {
                return std::result::Result::Ok(RustNodes::TraitBounds)
            }
            "trait_item" => {
                return std::result::Result::Ok(RustNodes::TraitItem)
            }
            "try_block" => {
                return std::result::Result::Ok(RustNodes::TryBlock)
            }
            "try_expression" => {
                return std::result::Result::Ok(RustNodes::TryExpression)
            }
            "tuple_expression" => {
                return std::result::Result::Ok(RustNodes::TupleExpression)
            }
            "tuple_pattern" => {
                return std::result::Result::Ok(RustNodes::TuplePattern)
            }
            "tuple_struct_pattern" => {
                return std::result::Result::Ok(RustNodes::TupleStructPattern)
            }
            "tuple_type" => {
                return std::result::Result::Ok(RustNodes::TupleType)
            }
            "type_arguments" => {
                return std::result::Result::Ok(RustNodes::TypeArguments)
            }
            "type_binding" => {
                return std::result::Result::Ok(RustNodes::TypeBinding)
            }
            "type_cast_expression" => {
                return std::result::Result::Ok(RustNodes::TypeCastExpression)
            }
            "type_item" => {
                return std::result::Result::Ok(RustNodes::TypeItem)
            }
            "type_parameter" => {
                return std::result::Result::Ok(RustNodes::TypeParameter)
            }
            "type_parameters" => {
                return std::result::Result::Ok(RustNodes::TypeParameters)
            }
            "unary_expression" => {
                return std::result::Result::Ok(RustNodes::UnaryExpression)
            }
            "union_item" => {
                return std::result::Result::Ok(RustNodes::UnionItem)
            }
            "unit_expression" => {
                return std::result::Result::Ok(RustNodes::UnitExpression)
            }
            "unit_type" => {
                return std::result::Result::Ok(RustNodes::UnitType)
            }
            "unsafe_block" => {
                return std::result::Result::Ok(RustNodes::UnsafeBlock)
            }
            "use_as_clause" => {
                return std::result::Result::Ok(RustNodes::UseAsClause)
            }
            "use_bounds" => {
                return std::result::Result::Ok(RustNodes::UseBounds)
            }
            "use_declaration" => {
                return std::result::Result::Ok(RustNodes::UseDeclaration)
            }
            "use_list" => {
                return std::result::Result::Ok(RustNodes::UseList)
            }
            "use_wildcard" => {
                return std::result::Result::Ok(RustNodes::UseWildcard)
            }
            "variadic_parameter" => {
                return std::result::Result::Ok(RustNodes::VariadicParameter)
            }
            "visibility_modifier" => {
                return std::result::Result::Ok(RustNodes::VisibilityModifier)
            }
            "where_clause" => {
                return std::result::Result::Ok(RustNodes::WhereClause)
            }
            "where_predicate" => {
                return std::result::Result::Ok(RustNodes::WherePredicate)
            }
            "while_expression" => {
                return std::result::Result::Ok(RustNodes::WhileExpression)
            }
            "yield_expression" => {
                return std::result::Result::Ok(RustNodes::YieldExpression)
            }
            "!" => {
                return std::result::Result::Ok(RustNodes::ExclamationMarkToken)
            }
            "!=" => {
                return std::result::Result::Ok(RustNodes::ExclamationMarkequalsSignToken)
            }
            "\"" => {
                return std::result::Result::Ok(RustNodes::QuotationMarkToken)
            }
            "#" => {
                return std::result::Result::Ok(RustNodes::NumberSignToken)
            }
            "$" => {
                return std::result::Result::Ok(RustNodes::DollarSignToken)
            }
            "%" => {
                return std::result::Result::Ok(RustNodes::PercentSignToken)
            }
            "%=" => {
                return std::result::Result::Ok(RustNodes::PercentSignequalsSignToken)
            }
            "&" => {
                return std::result::Result::Ok(RustNodes::AmpersandToken)
            }
            "&&" => {
                return std::result::Result::Ok(RustNodes::AmpersandampersandToken)
            }
            "&=" => {
                return std::result::Result::Ok(RustNodes::AmpersandequalsSignToken)
            }
            "'" => {
                return std::result::Result::Ok(RustNodes::ApostropheToken)
            }
            "(" => {
                return std::result::Result::Ok(RustNodes::LeftParenthesisToken)
            }
            ")" => {
                return std::result::Result::Ok(RustNodes::RightParenthesisToken)
            }
            "*" => {
                return std::result::Result::Ok(RustNodes::AsteriskToken)
            }
            "*/" => {
                return std::result::Result::Ok(RustNodes::AsterisksolidusToken)
            }
            "*=" => {
                return std::result::Result::Ok(RustNodes::AsteriskequalsSignToken)
            }
            "+" => {
                return std::result::Result::Ok(RustNodes::PlusSignToken)
            }
            "+=" => {
                return std::result::Result::Ok(RustNodes::PlusSignequalsSignToken)
            }
            "," => { return std::result::Result::Ok(RustNodes::CommaToken) }
            "-" => {
                return std::result::Result::Ok(RustNodes::HyphenMinusToken)
            }
            "-=" => {
                return std::result::Result::Ok(RustNodes::HyphenMinusequalsSignToken)
            }
            "->" => {
                return std::result::Result::Ok(RustNodes::HyphenMinusgreaterThanSignToken)
            }
            "." => {
                return std::result::Result::Ok(RustNodes::FullStopToken)
            }
            ".." => {
                return std::result::Result::Ok(RustNodes::FullStopfullStopToken)
            }
            "..." => {
                return std::result::Result::Ok(RustNodes::FullStopfullStopfullStopToken)
            }
            "..=" => {
                return std::result::Result::Ok(RustNodes::FullStopfullStopequalsSignToken)
            }
            "/" => { return std::result::Result::Ok(RustNodes::SolidusToken) }
            "/*" => {
                return std::result::Result::Ok(RustNodes::SolidusasteriskToken)
            }
            "//" => {
                return std::result::Result::Ok(RustNodes::SolidussolidusToken)
            }
            "/=" => {
                return std::result::Result::Ok(RustNodes::SolidusequalsSignToken)
            }
            ":" => { return std::result::Result::Ok(RustNodes::ColonToken) }
            "::" => {
                return std::result::Result::Ok(RustNodes::ColoncolonToken)
            }
            ";" => {
                return std::result::Result::Ok(RustNodes::SemicolonToken)
            }
            "<" => {
                return std::result::Result::Ok(RustNodes::LessThanSignToken)
            }
            "<<" => {
                return std::result::Result::Ok(RustNodes::LessThanSignlessThanSignToken)
            }
            "<<=" => {
                return std::result::Result::Ok(RustNodes::LessThanSignlessThanSignequalsSignToken)
            }
            "<=" => {
                return std::result::Result::Ok(RustNodes::LessThanSignequalsSignToken)
            }
            "=" => {
                return std::result::Result::Ok(RustNodes::EqualsSignToken)
            }
            "==" => {
                return std::result::Result::Ok(RustNodes::EqualsSignequalsSignToken)
            }
            "=>" => {
                return std::result::Result::Ok(RustNodes::EqualsSigngreaterThanSignToken)
            }
            ">" => {
                return std::result::Result::Ok(RustNodes::GreaterThanSignToken)
            }
            ">=" => {
                return std::result::Result::Ok(RustNodes::GreaterThanSignequalsSignToken)
            }
            ">>" => {
                return std::result::Result::Ok(RustNodes::GreaterThanSigngreaterThanSignToken)
            }
            ">>=" => {
                return std::result::Result::Ok(RustNodes::GreaterThanSigngreaterThanSignequalsSignToken)
            }
            "?" => {
                return std::result::Result::Ok(RustNodes::QuestionMarkToken)
            }
            "@" => {
                return std::result::Result::Ok(RustNodes::CommercialAtToken)
            }
            "[" => {
                return std::result::Result::Ok(RustNodes::LeftSquareBracketToken)
            }
            "]" => {
                return std::result::Result::Ok(RustNodes::RightSquareBracketToken)
            }
            "^" => {
                return std::result::Result::Ok(RustNodes::CircumflexAccentToken)
            }
            "^=" => {
                return std::result::Result::Ok(RustNodes::CircumflexAccentequalsSignToken)
            }
            "_" => { return std::result::Result::Ok(RustNodes::_Token) }
            "as" => { return std::result::Result::Ok(RustNodes::AsToken) }
            "async" => {
                return std::result::Result::Ok(RustNodes::AsyncToken)
            }
            "await" => {
                return std::result::Result::Ok(RustNodes::AwaitToken)
            }
            "block" => {
                return std::result::Result::Ok(RustNodes::BlockToken)
            }
            "break" => {
                return std::result::Result::Ok(RustNodes::BreakToken)
            }
            "char_literal" => {
                return std::result::Result::Ok(RustNodes::CharLiteral)
            }
            "const" => {
                return std::result::Result::Ok(RustNodes::ConstToken)
            }
            "continue" => {
                return std::result::Result::Ok(RustNodes::ContinueToken)
            }
            "crate" => { return std::result::Result::Ok(RustNodes::Crate) }
            "default" => {
                return std::result::Result::Ok(RustNodes::DefaultToken)
            }
            "doc_comment" => {
                return std::result::Result::Ok(RustNodes::DocComment)
            }
            "dyn" => { return std::result::Result::Ok(RustNodes::DynToken) }
            "else" => { return std::result::Result::Ok(RustNodes::ElseToken) }
            "enum" => { return std::result::Result::Ok(RustNodes::EnumToken) }
            "escape_sequence" => {
                return std::result::Result::Ok(RustNodes::EscapeSequence)
            }
            "expr" => { return std::result::Result::Ok(RustNodes::ExprToken) }
            "expr_2021" => {
                return std::result::Result::Ok(RustNodes::Expr2021Token)
            }
            "extern" => {
                return std::result::Result::Ok(RustNodes::ExternToken)
            }
            "false" => {
                return std::result::Result::Ok(RustNodes::FalseToken)
            }
            "field_identifier" => {
                return std::result::Result::Ok(RustNodes::FieldIdentifier)
            }
            "float_literal" => {
                return std::result::Result::Ok(RustNodes::FloatLiteral)
            }
            "fn" => { return std::result::Result::Ok(RustNodes::FnToken) }
            "for" => { return std::result::Result::Ok(RustNodes::ForToken) }
            "gen" => { return std::result::Result::Ok(RustNodes::GenToken) }
            "ident" => {
                return std::result::Result::Ok(RustNodes::IdentToken)
            }
            "identifier" => {
                return std::result::Result::Ok(RustNodes::Identifier)
            }
            "if" => { return std::result::Result::Ok(RustNodes::IfToken) }
            "impl" => { return std::result::Result::Ok(RustNodes::ImplToken) }
            "in" => { return std::result::Result::Ok(RustNodes::InToken) }
            "integer_literal" => {
                return std::result::Result::Ok(RustNodes::IntegerLiteral)
            }
            "item" => { return std::result::Result::Ok(RustNodes::ItemToken) }
            "let" => { return std::result::Result::Ok(RustNodes::LetToken) }
            "lifetime" => {
                return std::result::Result::Ok(RustNodes::LifetimeToken)
            }
            "literal" => {
                return std::result::Result::Ok(RustNodes::LiteralToken)
            }
            "loop" => { return std::result::Result::Ok(RustNodes::LoopToken) }
            "macro_rules!" => {
                return std::result::Result::Ok(RustNodes::MacroRulesExclamationMarkToken)
            }
            "match" => {
                return std::result::Result::Ok(RustNodes::MatchToken)
            }
            "meta" => { return std::result::Result::Ok(RustNodes::MetaToken) }
            "metavariable" => {
                return std::result::Result::Ok(RustNodes::Metavariable)
            }
            "mod" => { return std::result::Result::Ok(RustNodes::ModToken) }
            "move" => { return std::result::Result::Ok(RustNodes::MoveToken) }
            "mutable_specifier" => {
                return std::result::Result::Ok(RustNodes::MutableSpecifier)
            }
            "pat" => { return std::result::Result::Ok(RustNodes::PatToken) }
            "pat_param" => {
                return std::result::Result::Ok(RustNodes::PatParamToken)
            }
            "path" => { return std::result::Result::Ok(RustNodes::PathToken) }
            "primitive_type" => {
                return std::result::Result::Ok(RustNodes::PrimitiveType)
            }
            "pub" => { return std::result::Result::Ok(RustNodes::PubToken) }
            "raw" => { return std::result::Result::Ok(RustNodes::RawToken) }
            "ref" => { return std::result::Result::Ok(RustNodes::RefToken) }
            "return" => {
                return std::result::Result::Ok(RustNodes::ReturnToken)
            }
            "self" => { return std::result::Result::Ok(RustNodes::SelfToken) }
            "shebang" => {
                return std::result::Result::Ok(RustNodes::Shebang)
            }
            "shorthand_field_identifier" => {
                return std::result::Result::Ok(RustNodes::ShorthandFieldIdentifier)
            }
            "static" => {
                return std::result::Result::Ok(RustNodes::StaticToken)
            }
            "stmt" => { return std::result::Result::Ok(RustNodes::StmtToken) }
            "string_content" => {
                return std::result::Result::Ok(RustNodes::StringContent)
            }
            "struct" => {
                return std::result::Result::Ok(RustNodes::StructToken)
            }
            "super" => { return std::result::Result::Ok(RustNodes::Super) }
            "trait" => {
                return std::result::Result::Ok(RustNodes::TraitToken)
            }
            "true" => { return std::result::Result::Ok(RustNodes::TrueToken) }
            "try" => { return std::result::Result::Ok(RustNodes::TryToken) }
            "tt" => { return std::result::Result::Ok(RustNodes::TtToken) }
            "ty" => { return std::result::Result::Ok(RustNodes::TyToken) }
            "type" => { return std::result::Result::Ok(RustNodes::TypeToken) }
            "type_identifier" => {
                return std::result::Result::Ok(RustNodes::TypeIdentifier)
            }
            "union" => {
                return std::result::Result::Ok(RustNodes::UnionToken)
            }
            "unsafe" => {
                return std::result::Result::Ok(RustNodes::UnsafeToken)
            }
            "use" => { return std::result::Result::Ok(RustNodes::UseToken) }
            "vis" => { return std::result::Result::Ok(RustNodes::VisToken) }
            "where" => {
                return std::result::Result::Ok(RustNodes::WhereToken)
            }
            "while" => {
                return std::result::Result::Ok(RustNodes::WhileToken)
            }
            "yield" => {
                return std::result::Result::Ok(RustNodes::YieldToken)
            }
            "{" => {
                return std::result::Result::Ok(RustNodes::LeftCurlyBracketToken)
            }
            "|" => {
                return std::result::Result::Ok(RustNodes::VerticalLineToken)
            }
            "|=" => {
                return std::result::Result::Ok(RustNodes::VerticalLineequalsSignToken)
            }
            "||" => {
                return std::result::Result::Ok(RustNodes::VerticalLineverticalLineToken)
            }
            "}" => {
                return std::result::Result::Ok(RustNodes::RightCurlyBracketToken)
            }
            err => {
                return 
                    // #[generate_nodes(tree_sitter_python)]


                    {
                        ::core::panicking::panic_fmt(format_args!("Unknown token name: \'{0}\'",
                                err));
                    }
            }
        }
    }
}
impl std::fmt::Display for RustNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::_DeclarationStatement => {
                f.write_fmt(format_args!("_declaration_statement"))
            }
            Self::_Expression => { f.write_fmt(format_args!("_expression")) }
            Self::_Literal => { f.write_fmt(format_args!("_literal")) }
            Self::_LiteralPattern => {
                f.write_fmt(format_args!("_literal_pattern"))
            }
            Self::_Pattern => { f.write_fmt(format_args!("_pattern")) }
            Self::_Type => { f.write_fmt(format_args!("_type")) }
            Self::AbstractType => {
                f.write_fmt(format_args!("abstract_type"))
            }
            Self::Arguments => { f.write_fmt(format_args!("arguments")) }
            Self::ArrayExpression => {
                f.write_fmt(format_args!("array_expression"))
            }
            Self::ArrayType => { f.write_fmt(format_args!("array_type")) }
            Self::AssignmentExpression => {
                f.write_fmt(format_args!("assignment_expression"))
            }
            Self::AssociatedType => {
                f.write_fmt(format_args!("associated_type"))
            }
            Self::AsyncBlock => { f.write_fmt(format_args!("async_block")) }
            Self::Attribute => { f.write_fmt(format_args!("attribute")) }
            Self::AttributeItem => {
                f.write_fmt(format_args!("attribute_item"))
            }
            Self::AwaitExpression => {
                f.write_fmt(format_args!("await_expression"))
            }
            Self::BaseFieldInitializer => {
                f.write_fmt(format_args!("base_field_initializer"))
            }
            Self::BinaryExpression => {
                f.write_fmt(format_args!("binary_expression"))
            }
            Self::Block => { f.write_fmt(format_args!("block")) }
            Self::BlockComment => {
                f.write_fmt(format_args!("block_comment"))
            }
            Self::BooleanLiteral => {
                f.write_fmt(format_args!("boolean_literal"))
            }
            Self::BoundedType => { f.write_fmt(format_args!("bounded_type")) }
            Self::BracketedType => {
                f.write_fmt(format_args!("bracketed_type"))
            }
            Self::BreakExpression => {
                f.write_fmt(format_args!("break_expression"))
            }
            Self::CallExpression => {
                f.write_fmt(format_args!("call_expression"))
            }
            Self::CapturedPattern => {
                f.write_fmt(format_args!("captured_pattern"))
            }
            Self::ClosureExpression => {
                f.write_fmt(format_args!("closure_expression"))
            }
            Self::ClosureParameters => {
                f.write_fmt(format_args!("closure_parameters"))
            }
            Self::CompoundAssignmentExpr => {
                f.write_fmt(format_args!("compound_assignment_expr"))
            }
            Self::ConstBlock => { f.write_fmt(format_args!("const_block")) }
            Self::ConstItem => { f.write_fmt(format_args!("const_item")) }
            Self::ConstParameter => {
                f.write_fmt(format_args!("const_parameter"))
            }
            Self::ContinueExpression => {
                f.write_fmt(format_args!("continue_expression"))
            }
            Self::DeclarationList => {
                f.write_fmt(format_args!("declaration_list"))
            }
            Self::DynamicType => { f.write_fmt(format_args!("dynamic_type")) }
            Self::ElseClause => { f.write_fmt(format_args!("else_clause")) }
            Self::EmptyStatement => {
                f.write_fmt(format_args!("empty_statement"))
            }
            Self::EnumItem => { f.write_fmt(format_args!("enum_item")) }
            Self::EnumVariant => { f.write_fmt(format_args!("enum_variant")) }
            Self::EnumVariantList => {
                f.write_fmt(format_args!("enum_variant_list"))
            }
            Self::ExpressionStatement => {
                f.write_fmt(format_args!("expression_statement"))
            }
            Self::ExternCrateDeclaration => {
                f.write_fmt(format_args!("extern_crate_declaration"))
            }
            Self::ExternModifier => {
                f.write_fmt(format_args!("extern_modifier"))
            }
            Self::FieldDeclaration => {
                f.write_fmt(format_args!("field_declaration"))
            }
            Self::FieldDeclarationList => {
                f.write_fmt(format_args!("field_declaration_list"))
            }
            Self::FieldExpression => {
                f.write_fmt(format_args!("field_expression"))
            }
            Self::FieldInitializer => {
                f.write_fmt(format_args!("field_initializer"))
            }
            Self::FieldInitializerList => {
                f.write_fmt(format_args!("field_initializer_list"))
            }
            Self::FieldPattern => {
                f.write_fmt(format_args!("field_pattern"))
            }
            Self::ForExpression => {
                f.write_fmt(format_args!("for_expression"))
            }
            Self::ForLifetimes => {
                f.write_fmt(format_args!("for_lifetimes"))
            }
            Self::ForeignModItem => {
                f.write_fmt(format_args!("foreign_mod_item"))
            }
            Self::FragmentSpecifier => {
                f.write_fmt(format_args!("fragment_specifier"))
            }
            Self::FunctionItem => {
                f.write_fmt(format_args!("function_item"))
            }
            Self::FunctionModifiers => {
                f.write_fmt(format_args!("function_modifiers"))
            }
            Self::FunctionSignatureItem => {
                f.write_fmt(format_args!("function_signature_item"))
            }
            Self::FunctionType => {
                f.write_fmt(format_args!("function_type"))
            }
            Self::GenBlock => { f.write_fmt(format_args!("gen_block")) }
            Self::GenericFunction => {
                f.write_fmt(format_args!("generic_function"))
            }
            Self::GenericPattern => {
                f.write_fmt(format_args!("generic_pattern"))
            }
            Self::GenericType => { f.write_fmt(format_args!("generic_type")) }
            Self::GenericTypeWithTurbofish => {
                f.write_fmt(format_args!("generic_type_with_turbofish"))
            }
            Self::HigherRankedTraitBound => {
                f.write_fmt(format_args!("higher_ranked_trait_bound"))
            }
            Self::IfExpression => {
                f.write_fmt(format_args!("if_expression"))
            }
            Self::ImplItem => { f.write_fmt(format_args!("impl_item")) }
            Self::IndexExpression => {
                f.write_fmt(format_args!("index_expression"))
            }
            Self::InnerAttributeItem => {
                f.write_fmt(format_args!("inner_attribute_item"))
            }
            Self::InnerDocCommentMarker => {
                f.write_fmt(format_args!("inner_doc_comment_marker"))
            }
            Self::Label => { f.write_fmt(format_args!("label")) }
            Self::LetChain => { f.write_fmt(format_args!("let_chain")) }
            Self::LetCondition => {
                f.write_fmt(format_args!("let_condition"))
            }
            Self::LetDeclaration => {
                f.write_fmt(format_args!("let_declaration"))
            }
            Self::Lifetime => { f.write_fmt(format_args!("lifetime")) }
            Self::LifetimeParameter => {
                f.write_fmt(format_args!("lifetime_parameter"))
            }
            Self::LineComment => { f.write_fmt(format_args!("line_comment")) }
            Self::LoopExpression => {
                f.write_fmt(format_args!("loop_expression"))
            }
            Self::MacroDefinition => {
                f.write_fmt(format_args!("macro_definition"))
            }
            Self::MacroInvocation => {
                f.write_fmt(format_args!("macro_invocation"))
            }
            Self::MacroRule => { f.write_fmt(format_args!("macro_rule")) }
            Self::MatchArm => { f.write_fmt(format_args!("match_arm")) }
            Self::MatchBlock => { f.write_fmt(format_args!("match_block")) }
            Self::MatchExpression => {
                f.write_fmt(format_args!("match_expression"))
            }
            Self::MatchPattern => {
                f.write_fmt(format_args!("match_pattern"))
            }
            Self::ModItem => { f.write_fmt(format_args!("mod_item")) }
            Self::MutPattern => { f.write_fmt(format_args!("mut_pattern")) }
            Self::NegativeLiteral => {
                f.write_fmt(format_args!("negative_literal"))
            }
            Self::NeverType => { f.write_fmt(format_args!("never_type")) }
            Self::OrPattern => { f.write_fmt(format_args!("or_pattern")) }
            Self::OrderedFieldDeclarationList => {
                f.write_fmt(format_args!("ordered_field_declaration_list"))
            }
            Self::OuterDocCommentMarker => {
                f.write_fmt(format_args!("outer_doc_comment_marker"))
            }
            Self::Parameter => { f.write_fmt(format_args!("parameter")) }
            Self::Parameters => { f.write_fmt(format_args!("parameters")) }
            Self::ParenthesizedExpression => {
                f.write_fmt(format_args!("parenthesized_expression"))
            }
            Self::PointerType => { f.write_fmt(format_args!("pointer_type")) }
            Self::QualifiedType => {
                f.write_fmt(format_args!("qualified_type"))
            }
            Self::RangeExpression => {
                f.write_fmt(format_args!("range_expression"))
            }
            Self::RangePattern => {
                f.write_fmt(format_args!("range_pattern"))
            }
            Self::RawStringLiteral => {
                f.write_fmt(format_args!("raw_string_literal"))
            }
            Self::RefPattern => { f.write_fmt(format_args!("ref_pattern")) }
            Self::ReferenceExpression => {
                f.write_fmt(format_args!("reference_expression"))
            }
            Self::ReferencePattern => {
                f.write_fmt(format_args!("reference_pattern"))
            }
            Self::ReferenceType => {
                f.write_fmt(format_args!("reference_type"))
            }
            Self::RemainingFieldPattern => {
                f.write_fmt(format_args!("remaining_field_pattern"))
            }
            Self::RemovedTraitBound => {
                f.write_fmt(format_args!("removed_trait_bound"))
            }
            Self::ReturnExpression => {
                f.write_fmt(format_args!("return_expression"))
            }
            Self::ScopedIdentifier => {
                f.write_fmt(format_args!("scoped_identifier"))
            }
            Self::ScopedTypeIdentifier => {
                f.write_fmt(format_args!("scoped_type_identifier"))
            }
            Self::ScopedUseList => {
                f.write_fmt(format_args!("scoped_use_list"))
            }
            Self::SelfParameter => {
                f.write_fmt(format_args!("self_parameter"))
            }
            Self::ShorthandFieldInitializer => {
                f.write_fmt(format_args!("shorthand_field_initializer"))
            }
            Self::SlicePattern => {
                f.write_fmt(format_args!("slice_pattern"))
            }
            Self::SourceFile => { f.write_fmt(format_args!("source_file")) }
            Self::StaticItem => { f.write_fmt(format_args!("static_item")) }
            Self::StringLiteral => {
                f.write_fmt(format_args!("string_literal"))
            }
            Self::StructExpression => {
                f.write_fmt(format_args!("struct_expression"))
            }
            Self::StructItem => { f.write_fmt(format_args!("struct_item")) }
            Self::StructPattern => {
                f.write_fmt(format_args!("struct_pattern"))
            }
            Self::TokenBindingPattern => {
                f.write_fmt(format_args!("token_binding_pattern"))
            }
            Self::TokenRepetition => {
                f.write_fmt(format_args!("token_repetition"))
            }
            Self::TokenRepetitionPattern => {
                f.write_fmt(format_args!("token_repetition_pattern"))
            }
            Self::TokenTree => { f.write_fmt(format_args!("token_tree")) }
            Self::TokenTreePattern => {
                f.write_fmt(format_args!("token_tree_pattern"))
            }
            Self::TraitBounds => { f.write_fmt(format_args!("trait_bounds")) }
            Self::TraitItem => { f.write_fmt(format_args!("trait_item")) }
            Self::TryBlock => { f.write_fmt(format_args!("try_block")) }
            Self::TryExpression => {
                f.write_fmt(format_args!("try_expression"))
            }
            Self::TupleExpression => {
                f.write_fmt(format_args!("tuple_expression"))
            }
            Self::TuplePattern => {
                f.write_fmt(format_args!("tuple_pattern"))
            }
            Self::TupleStructPattern => {
                f.write_fmt(format_args!("tuple_struct_pattern"))
            }
            Self::TupleType => { f.write_fmt(format_args!("tuple_type")) }
            Self::TypeArguments => {
                f.write_fmt(format_args!("type_arguments"))
            }
            Self::TypeBinding => { f.write_fmt(format_args!("type_binding")) }
            Self::TypeCastExpression => {
                f.write_fmt(format_args!("type_cast_expression"))
            }
            Self::TypeItem => { f.write_fmt(format_args!("type_item")) }
            Self::TypeParameter => {
                f.write_fmt(format_args!("type_parameter"))
            }
            Self::TypeParameters => {
                f.write_fmt(format_args!("type_parameters"))
            }
            Self::UnaryExpression => {
                f.write_fmt(format_args!("unary_expression"))
            }
            Self::UnionItem => { f.write_fmt(format_args!("union_item")) }
            Self::UnitExpression => {
                f.write_fmt(format_args!("unit_expression"))
            }
            Self::UnitType => { f.write_fmt(format_args!("unit_type")) }
            Self::UnsafeBlock => { f.write_fmt(format_args!("unsafe_block")) }
            Self::UseAsClause => {
                f.write_fmt(format_args!("use_as_clause"))
            }
            Self::UseBounds => { f.write_fmt(format_args!("use_bounds")) }
            Self::UseDeclaration => {
                f.write_fmt(format_args!("use_declaration"))
            }
            Self::UseList => { f.write_fmt(format_args!("use_list")) }
            Self::UseWildcard => { f.write_fmt(format_args!("use_wildcard")) }
            Self::VariadicParameter => {
                f.write_fmt(format_args!("variadic_parameter"))
            }
            Self::VisibilityModifier => {
                f.write_fmt(format_args!("visibility_modifier"))
            }
            Self::WhereClause => { f.write_fmt(format_args!("where_clause")) }
            Self::WherePredicate => {
                f.write_fmt(format_args!("where_predicate"))
            }
            Self::WhileExpression => {
                f.write_fmt(format_args!("while_expression"))
            }
            Self::YieldExpression => {
                f.write_fmt(format_args!("yield_expression"))
            }
            Self::ExclamationMarkToken => { f.write_fmt(format_args!("!")) }
            Self::ExclamationMarkequalsSignToken => {
                f.write_fmt(format_args!("!="))
            }
            Self::QuotationMarkToken => { f.write_fmt(format_args!("\"")) }
            Self::NumberSignToken => { f.write_fmt(format_args!("#")) }
            Self::DollarSignToken => { f.write_fmt(format_args!("$")) }
            Self::PercentSignToken => { f.write_fmt(format_args!("%")) }
            Self::PercentSignequalsSignToken => {
                f.write_fmt(format_args!("%="))
            }
            Self::AmpersandToken => { f.write_fmt(format_args!("&")) }
            Self::AmpersandampersandToken => {
                f.write_fmt(format_args!("&&"))
            }
            Self::AmpersandequalsSignToken => {
                f.write_fmt(format_args!("&="))
            }
            Self::ApostropheToken => { f.write_fmt(format_args!("\'")) }
            Self::LeftParenthesisToken => { f.write_fmt(format_args!("(")) }
            Self::RightParenthesisToken => { f.write_fmt(format_args!(")")) }
            Self::AsteriskToken => { f.write_fmt(format_args!("*")) }
            Self::AsterisksolidusToken => { f.write_fmt(format_args!("*/")) }
            Self::AsteriskequalsSignToken => {
                f.write_fmt(format_args!("*="))
            }
            Self::PlusSignToken => { f.write_fmt(format_args!("+")) }
            Self::PlusSignequalsSignToken => {
                f.write_fmt(format_args!("+="))
            }
            Self::CommaToken => { f.write_fmt(format_args!(",")) }
            Self::HyphenMinusToken => { f.write_fmt(format_args!("-")) }
            Self::HyphenMinusequalsSignToken => {
                f.write_fmt(format_args!("-="))
            }
            Self::HyphenMinusgreaterThanSignToken => {
                f.write_fmt(format_args!("->"))
            }
            Self::FullStopToken => { f.write_fmt(format_args!(".")) }
            Self::FullStopfullStopToken => { f.write_fmt(format_args!("..")) }
            Self::FullStopfullStopfullStopToken => {
                f.write_fmt(format_args!("..."))
            }
            Self::FullStopfullStopequalsSignToken => {
                f.write_fmt(format_args!("..="))
            }
            Self::SolidusToken => { f.write_fmt(format_args!("/")) }
            Self::SolidusasteriskToken => { f.write_fmt(format_args!("/*")) }
            Self::SolidussolidusToken => { f.write_fmt(format_args!("//")) }
            Self::SolidusequalsSignToken => {
                f.write_fmt(format_args!("/="))
            }
            Self::ColonToken => { f.write_fmt(format_args!(":")) }
            Self::ColoncolonToken => { f.write_fmt(format_args!("::")) }
            Self::SemicolonToken => { f.write_fmt(format_args!(";")) }
            Self::LessThanSignToken => { f.write_fmt(format_args!("<")) }
            Self::LessThanSignlessThanSignToken => {
                f.write_fmt(format_args!("<<"))
            }
            Self::LessThanSignlessThanSignequalsSignToken => {
                f.write_fmt(format_args!("<<="))
            }
            Self::LessThanSignequalsSignToken => {
                f.write_fmt(format_args!("<="))
            }
            Self::EqualsSignToken => { f.write_fmt(format_args!("=")) }
            Self::EqualsSignequalsSignToken => {
                f.write_fmt(format_args!("=="))
            }
            Self::EqualsSigngreaterThanSignToken => {
                f.write_fmt(format_args!("=>"))
            }
            Self::GreaterThanSignToken => { f.write_fmt(format_args!(">")) }
            Self::GreaterThanSignequalsSignToken => {
                f.write_fmt(format_args!(">="))
            }
            Self::GreaterThanSigngreaterThanSignToken => {
                f.write_fmt(format_args!(">>"))
            }
            Self::GreaterThanSigngreaterThanSignequalsSignToken => {
                f.write_fmt(format_args!(">>="))
            }
            Self::QuestionMarkToken => { f.write_fmt(format_args!("?")) }
            Self::CommercialAtToken => { f.write_fmt(format_args!("@")) }
            Self::LeftSquareBracketToken => { f.write_fmt(format_args!("[")) }
            Self::RightSquareBracketToken => {
                f.write_fmt(format_args!("]"))
            }
            Self::CircumflexAccentToken => { f.write_fmt(format_args!("^")) }
            Self::CircumflexAccentequalsSignToken => {
                f.write_fmt(format_args!("^="))
            }
            Self::_Token => { f.write_fmt(format_args!("_")) }
            Self::AsToken => { f.write_fmt(format_args!("as")) }
            Self::AsyncToken => { f.write_fmt(format_args!("async")) }
            Self::AwaitToken => { f.write_fmt(format_args!("await")) }
            Self::BlockToken => { f.write_fmt(format_args!("block")) }
            Self::BreakToken => { f.write_fmt(format_args!("break")) }
            Self::CharLiteral => { f.write_fmt(format_args!("char_literal")) }
            Self::ConstToken => { f.write_fmt(format_args!("const")) }
            Self::ContinueToken => { f.write_fmt(format_args!("continue")) }
            Self::Crate => { f.write_fmt(format_args!("crate")) }
            Self::DefaultToken => { f.write_fmt(format_args!("default")) }
            Self::DocComment => { f.write_fmt(format_args!("doc_comment")) }
            Self::DynToken => { f.write_fmt(format_args!("dyn")) }
            Self::ElseToken => { f.write_fmt(format_args!("else")) }
            Self::EnumToken => { f.write_fmt(format_args!("enum")) }
            Self::EscapeSequence => {
                f.write_fmt(format_args!("escape_sequence"))
            }
            Self::ExprToken => { f.write_fmt(format_args!("expr")) }
            Self::Expr2021Token => { f.write_fmt(format_args!("expr_2021")) }
            Self::ExternToken => { f.write_fmt(format_args!("extern")) }
            Self::FalseToken => { f.write_fmt(format_args!("false")) }
            Self::FieldIdentifier => {
                f.write_fmt(format_args!("field_identifier"))
            }
            Self::FloatLiteral => {
                f.write_fmt(format_args!("float_literal"))
            }
            Self::FnToken => { f.write_fmt(format_args!("fn")) }
            Self::ForToken => { f.write_fmt(format_args!("for")) }
            Self::GenToken => { f.write_fmt(format_args!("gen")) }
            Self::IdentToken => { f.write_fmt(format_args!("ident")) }
            Self::Identifier => { f.write_fmt(format_args!("identifier")) }
            Self::IfToken => { f.write_fmt(format_args!("if")) }
            Self::ImplToken => { f.write_fmt(format_args!("impl")) }
            Self::InToken => { f.write_fmt(format_args!("in")) }
            Self::IntegerLiteral => {
                f.write_fmt(format_args!("integer_literal"))
            }
            Self::ItemToken => { f.write_fmt(format_args!("item")) }
            Self::LetToken => { f.write_fmt(format_args!("let")) }
            Self::LifetimeToken => { f.write_fmt(format_args!("lifetime")) }
            Self::LiteralToken => { f.write_fmt(format_args!("literal")) }
            Self::LoopToken => { f.write_fmt(format_args!("loop")) }
            Self::MacroRulesExclamationMarkToken => {
                f.write_fmt(format_args!("macro_rules!"))
            }
            Self::MatchToken => { f.write_fmt(format_args!("match")) }
            Self::MetaToken => { f.write_fmt(format_args!("meta")) }
            Self::Metavariable => {
                f.write_fmt(format_args!("metavariable"))
            }
            Self::ModToken => { f.write_fmt(format_args!("mod")) }
            Self::MoveToken => { f.write_fmt(format_args!("move")) }
            Self::MutableSpecifier => {
                f.write_fmt(format_args!("mutable_specifier"))
            }
            Self::PatToken => { f.write_fmt(format_args!("pat")) }
            Self::PatParamToken => { f.write_fmt(format_args!("pat_param")) }
            Self::PathToken => { f.write_fmt(format_args!("path")) }
            Self::PrimitiveType => {
                f.write_fmt(format_args!("primitive_type"))
            }
            Self::PubToken => { f.write_fmt(format_args!("pub")) }
            Self::RawToken => { f.write_fmt(format_args!("raw")) }
            Self::RefToken => { f.write_fmt(format_args!("ref")) }
            Self::ReturnToken => { f.write_fmt(format_args!("return")) }
            Self::SelfToken => { f.write_fmt(format_args!("self")) }
            Self::Shebang => { f.write_fmt(format_args!("shebang")) }
            Self::ShorthandFieldIdentifier => {
                f.write_fmt(format_args!("shorthand_field_identifier"))
            }
            Self::StaticToken => { f.write_fmt(format_args!("static")) }
            Self::StmtToken => { f.write_fmt(format_args!("stmt")) }
            Self::StringContent => {
                f.write_fmt(format_args!("string_content"))
            }
            Self::StructToken => { f.write_fmt(format_args!("struct")) }
            Self::Super => { f.write_fmt(format_args!("super")) }
            Self::TraitToken => { f.write_fmt(format_args!("trait")) }
            Self::TrueToken => { f.write_fmt(format_args!("true")) }
            Self::TryToken => { f.write_fmt(format_args!("try")) }
            Self::TtToken => { f.write_fmt(format_args!("tt")) }
            Self::TyToken => { f.write_fmt(format_args!("ty")) }
            Self::TypeToken => { f.write_fmt(format_args!("type")) }
            Self::TypeIdentifier => {
                f.write_fmt(format_args!("type_identifier"))
            }
            Self::UnionToken => { f.write_fmt(format_args!("union")) }
            Self::UnsafeToken => { f.write_fmt(format_args!("unsafe")) }
            Self::UseToken => { f.write_fmt(format_args!("use")) }
            Self::VisToken => { f.write_fmt(format_args!("vis")) }
            Self::WhereToken => { f.write_fmt(format_args!("where")) }
            Self::WhileToken => { f.write_fmt(format_args!("while")) }
            Self::YieldToken => { f.write_fmt(format_args!("yield")) }
            Self::LeftCurlyBracketToken => { f.write_fmt(format_args!("{{")) }
            Self::VerticalLineToken => { f.write_fmt(format_args!("|")) }
            Self::VerticalLineequalsSignToken => {
                f.write_fmt(format_args!("|="))
            }
            Self::VerticalLineverticalLineToken => {
                f.write_fmt(format_args!("||"))
            }
            Self::RightCurlyBracketToken => {
                f.write_fmt(format_args!("}}"))
            }
            err => {
                return {
                        ::core::panicking::panic_fmt(format_args!("Unknown token name: \'{0}\'",
                                err));
                    }
            }
        }
    }
}
enum PyNodes {}
