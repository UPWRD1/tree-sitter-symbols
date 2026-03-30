#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use tss_macros::generate_nodes;

pub enum _DeclarationStatement {
    AssociatedType,
    AttributeItem,
    ConstItem,
    EmptyStatement,
    EnumItem,
    ExternCrateDeclaration,
    ForeignModItem,
    FunctionItem,
    FunctionSignatureItem,
    ImplItem,
    InnerAttributeItem,
    LetDeclaration,
    MacroDefinition,
    MacroInvocation,
    ModItem,
    StaticItem,
    StructItem,
    TraitItem,
    TypeItem,
    UnionItem,
    UseDeclaration,
}
#[automatically_derived]
impl ::core::fmt::Debug for _DeclarationStatement {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f,
            match self {
                _DeclarationStatement::AssociatedType => "AssociatedType",
                _DeclarationStatement::AttributeItem => "AttributeItem",
                _DeclarationStatement::ConstItem => "ConstItem",
                _DeclarationStatement::EmptyStatement => "EmptyStatement",
                _DeclarationStatement::EnumItem => "EnumItem",
                _DeclarationStatement::ExternCrateDeclaration =>
                    "ExternCrateDeclaration",
                _DeclarationStatement::ForeignModItem => "ForeignModItem",
                _DeclarationStatement::FunctionItem => "FunctionItem",
                _DeclarationStatement::FunctionSignatureItem =>
                    "FunctionSignatureItem",
                _DeclarationStatement::ImplItem => "ImplItem",
                _DeclarationStatement::InnerAttributeItem =>
                    "InnerAttributeItem",
                _DeclarationStatement::LetDeclaration => "LetDeclaration",
                _DeclarationStatement::MacroDefinition => "MacroDefinition",
                _DeclarationStatement::MacroInvocation => "MacroInvocation",
                _DeclarationStatement::ModItem => "ModItem",
                _DeclarationStatement::StaticItem => "StaticItem",
                _DeclarationStatement::StructItem => "StructItem",
                _DeclarationStatement::TraitItem => "TraitItem",
                _DeclarationStatement::TypeItem => "TypeItem",
                _DeclarationStatement::UnionItem => "UnionItem",
                _DeclarationStatement::UseDeclaration => "UseDeclaration",
            })
    }
}
#[automatically_derived]
impl ::core::clone::Clone for _DeclarationStatement {
    #[inline]
    fn clone(&self) -> _DeclarationStatement {
        match self {
            _DeclarationStatement::AssociatedType =>
                _DeclarationStatement::AssociatedType,
            _DeclarationStatement::AttributeItem =>
                _DeclarationStatement::AttributeItem,
            _DeclarationStatement::ConstItem =>
                _DeclarationStatement::ConstItem,
            _DeclarationStatement::EmptyStatement =>
                _DeclarationStatement::EmptyStatement,
            _DeclarationStatement::EnumItem =>
                _DeclarationStatement::EnumItem,
            _DeclarationStatement::ExternCrateDeclaration =>
                _DeclarationStatement::ExternCrateDeclaration,
            _DeclarationStatement::ForeignModItem =>
                _DeclarationStatement::ForeignModItem,
            _DeclarationStatement::FunctionItem =>
                _DeclarationStatement::FunctionItem,
            _DeclarationStatement::FunctionSignatureItem =>
                _DeclarationStatement::FunctionSignatureItem,
            _DeclarationStatement::ImplItem =>
                _DeclarationStatement::ImplItem,
            _DeclarationStatement::InnerAttributeItem =>
                _DeclarationStatement::InnerAttributeItem,
            _DeclarationStatement::LetDeclaration =>
                _DeclarationStatement::LetDeclaration,
            _DeclarationStatement::MacroDefinition =>
                _DeclarationStatement::MacroDefinition,
            _DeclarationStatement::MacroInvocation =>
                _DeclarationStatement::MacroInvocation,
            _DeclarationStatement::ModItem => _DeclarationStatement::ModItem,
            _DeclarationStatement::StaticItem =>
                _DeclarationStatement::StaticItem,
            _DeclarationStatement::StructItem =>
                _DeclarationStatement::StructItem,
            _DeclarationStatement::TraitItem =>
                _DeclarationStatement::TraitItem,
            _DeclarationStatement::TypeItem =>
                _DeclarationStatement::TypeItem,
            _DeclarationStatement::UnionItem =>
                _DeclarationStatement::UnionItem,
            _DeclarationStatement::UseDeclaration =>
                _DeclarationStatement::UseDeclaration,
        }
    }
}
pub enum _Expression {
    _Literal,
    ArrayExpression,
    AssignmentExpression,
    AsyncBlock,
    AwaitExpression,
    BinaryExpression,
    Block,
    BreakExpression,
    CallExpression,
    ClosureExpression,
    CompoundAssignmentExpr,
    ConstBlock,
    ContinueExpression,
    FieldExpression,
    ForExpression,
    GenBlock,
    GenericFunction,
    Identifier,
    IfExpression,
    IndexExpression,
    LoopExpression,
    MatchExpression,
    Metavariable,
    ParenthesizedExpression,
    RangeExpression,
    ReferenceExpression,
    ReturnExpression,
    ScopedIdentifier,
    SelfToken,
    StructExpression,
    TryBlock,
    TryExpression,
    TupleExpression,
    TypeCastExpression,
    UnaryExpression,
    UnitExpression,
    UnsafeBlock,
    WhileExpression,
    YieldExpression,
}
#[automatically_derived]
impl ::core::fmt::Debug for _Expression {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f,
            match self {
                _Expression::_Literal => "_Literal",
                _Expression::ArrayExpression => "ArrayExpression",
                _Expression::AssignmentExpression => "AssignmentExpression",
                _Expression::AsyncBlock => "AsyncBlock",
                _Expression::AwaitExpression => "AwaitExpression",
                _Expression::BinaryExpression => "BinaryExpression",
                _Expression::Block => "Block",
                _Expression::BreakExpression => "BreakExpression",
                _Expression::CallExpression => "CallExpression",
                _Expression::ClosureExpression => "ClosureExpression",
                _Expression::CompoundAssignmentExpr =>
                    "CompoundAssignmentExpr",
                _Expression::ConstBlock => "ConstBlock",
                _Expression::ContinueExpression => "ContinueExpression",
                _Expression::FieldExpression => "FieldExpression",
                _Expression::ForExpression => "ForExpression",
                _Expression::GenBlock => "GenBlock",
                _Expression::GenericFunction => "GenericFunction",
                _Expression::Identifier => "Identifier",
                _Expression::IfExpression => "IfExpression",
                _Expression::IndexExpression => "IndexExpression",
                _Expression::LoopExpression => "LoopExpression",
                _Expression::MatchExpression => "MatchExpression",
                _Expression::Metavariable => "Metavariable",
                _Expression::ParenthesizedExpression =>
                    "ParenthesizedExpression",
                _Expression::RangeExpression => "RangeExpression",
                _Expression::ReferenceExpression => "ReferenceExpression",
                _Expression::ReturnExpression => "ReturnExpression",
                _Expression::ScopedIdentifier => "ScopedIdentifier",
                _Expression::SelfToken => "SelfToken",
                _Expression::StructExpression => "StructExpression",
                _Expression::TryBlock => "TryBlock",
                _Expression::TryExpression => "TryExpression",
                _Expression::TupleExpression => "TupleExpression",
                _Expression::TypeCastExpression => "TypeCastExpression",
                _Expression::UnaryExpression => "UnaryExpression",
                _Expression::UnitExpression => "UnitExpression",
                _Expression::UnsafeBlock => "UnsafeBlock",
                _Expression::WhileExpression => "WhileExpression",
                _Expression::YieldExpression => "YieldExpression",
            })
    }
}
#[automatically_derived]
impl ::core::clone::Clone for _Expression {
    #[inline]
    fn clone(&self) -> _Expression {
        match self {
            _Expression::_Literal => _Expression::_Literal,
            _Expression::ArrayExpression => _Expression::ArrayExpression,
            _Expression::AssignmentExpression =>
                _Expression::AssignmentExpression,
            _Expression::AsyncBlock => _Expression::AsyncBlock,
            _Expression::AwaitExpression => _Expression::AwaitExpression,
            _Expression::BinaryExpression => _Expression::BinaryExpression,
            _Expression::Block => _Expression::Block,
            _Expression::BreakExpression => _Expression::BreakExpression,
            _Expression::CallExpression => _Expression::CallExpression,
            _Expression::ClosureExpression => _Expression::ClosureExpression,
            _Expression::CompoundAssignmentExpr =>
                _Expression::CompoundAssignmentExpr,
            _Expression::ConstBlock => _Expression::ConstBlock,
            _Expression::ContinueExpression =>
                _Expression::ContinueExpression,
            _Expression::FieldExpression => _Expression::FieldExpression,
            _Expression::ForExpression => _Expression::ForExpression,
            _Expression::GenBlock => _Expression::GenBlock,
            _Expression::GenericFunction => _Expression::GenericFunction,
            _Expression::Identifier => _Expression::Identifier,
            _Expression::IfExpression => _Expression::IfExpression,
            _Expression::IndexExpression => _Expression::IndexExpression,
            _Expression::LoopExpression => _Expression::LoopExpression,
            _Expression::MatchExpression => _Expression::MatchExpression,
            _Expression::Metavariable => _Expression::Metavariable,
            _Expression::ParenthesizedExpression =>
                _Expression::ParenthesizedExpression,
            _Expression::RangeExpression => _Expression::RangeExpression,
            _Expression::ReferenceExpression =>
                _Expression::ReferenceExpression,
            _Expression::ReturnExpression => _Expression::ReturnExpression,
            _Expression::ScopedIdentifier => _Expression::ScopedIdentifier,
            _Expression::SelfToken => _Expression::SelfToken,
            _Expression::StructExpression => _Expression::StructExpression,
            _Expression::TryBlock => _Expression::TryBlock,
            _Expression::TryExpression => _Expression::TryExpression,
            _Expression::TupleExpression => _Expression::TupleExpression,
            _Expression::TypeCastExpression =>
                _Expression::TypeCastExpression,
            _Expression::UnaryExpression => _Expression::UnaryExpression,
            _Expression::UnitExpression => _Expression::UnitExpression,
            _Expression::UnsafeBlock => _Expression::UnsafeBlock,
            _Expression::WhileExpression => _Expression::WhileExpression,
            _Expression::YieldExpression => _Expression::YieldExpression,
        }
    }
}
pub enum _Type {
    AbstractType,
    ArrayType,
    BoundedType,
    DynamicType,
    FunctionType,
    GenericType,
    NeverType,
    PointerType,
    PrimitiveType,
    ReferenceType,
    RemovedTraitBound,
    ScopedTypeIdentifier,
    TupleType,
    TypeIdentifier,
    UnitType,
}
#[automatically_derived]
impl ::core::fmt::Debug for _Type {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f,
            match self {
                _Type::AbstractType => "AbstractType",
                _Type::ArrayType => "ArrayType",
                _Type::BoundedType => "BoundedType",
                _Type::DynamicType => "DynamicType",
                _Type::FunctionType => "FunctionType",
                _Type::GenericType => "GenericType",
                _Type::NeverType => "NeverType",
                _Type::PointerType => "PointerType",
                _Type::PrimitiveType => "PrimitiveType",
                _Type::ReferenceType => "ReferenceType",
                _Type::RemovedTraitBound => "RemovedTraitBound",
                _Type::ScopedTypeIdentifier => "ScopedTypeIdentifier",
                _Type::TupleType => "TupleType",
                _Type::TypeIdentifier => "TypeIdentifier",
                _Type::UnitType => "UnitType",
            })
    }
}
#[automatically_derived]
impl ::core::clone::Clone for _Type {
    #[inline]
    fn clone(&self) -> _Type {
        match self {
            _Type::AbstractType => _Type::AbstractType,
            _Type::ArrayType => _Type::ArrayType,
            _Type::BoundedType => _Type::BoundedType,
            _Type::DynamicType => _Type::DynamicType,
            _Type::FunctionType => _Type::FunctionType,
            _Type::GenericType => _Type::GenericType,
            _Type::NeverType => _Type::NeverType,
            _Type::PointerType => _Type::PointerType,
            _Type::PrimitiveType => _Type::PrimitiveType,
            _Type::ReferenceType => _Type::ReferenceType,
            _Type::RemovedTraitBound => _Type::RemovedTraitBound,
            _Type::ScopedTypeIdentifier => _Type::ScopedTypeIdentifier,
            _Type::TupleType => _Type::TupleType,
            _Type::TypeIdentifier => _Type::TypeIdentifier,
            _Type::UnitType => _Type::UnitType,
        }
    }
}
pub enum _Pattern {
    _Token,
    CapturedPattern,
    GenericPattern,
    MutPattern,
    OrPattern,
    RangePattern,
    RefPattern,
    ReferencePattern,
    RemainingFieldPattern,
    SlicePattern,
    StructPattern,
    TuplePattern,
    TupleStructPattern,
}
#[automatically_derived]
impl ::core::fmt::Debug for _Pattern {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f,
            match self {
                _Pattern::_Token => "_Token",
                _Pattern::CapturedPattern => "CapturedPattern",
                _Pattern::GenericPattern => "GenericPattern",
                _Pattern::MutPattern => "MutPattern",
                _Pattern::OrPattern => "OrPattern",
                _Pattern::RangePattern => "RangePattern",
                _Pattern::RefPattern => "RefPattern",
                _Pattern::ReferencePattern => "ReferencePattern",
                _Pattern::RemainingFieldPattern => "RemainingFieldPattern",
                _Pattern::SlicePattern => "SlicePattern",
                _Pattern::StructPattern => "StructPattern",
                _Pattern::TuplePattern => "TuplePattern",
                _Pattern::TupleStructPattern => "TupleStructPattern",
            })
    }
}
#[automatically_derived]
impl ::core::clone::Clone for _Pattern {
    #[inline]
    fn clone(&self) -> _Pattern {
        match self {
            _Pattern::_Token => _Pattern::_Token,
            _Pattern::CapturedPattern => _Pattern::CapturedPattern,
            _Pattern::GenericPattern => _Pattern::GenericPattern,
            _Pattern::MutPattern => _Pattern::MutPattern,
            _Pattern::OrPattern => _Pattern::OrPattern,
            _Pattern::RangePattern => _Pattern::RangePattern,
            _Pattern::RefPattern => _Pattern::RefPattern,
            _Pattern::ReferencePattern => _Pattern::ReferencePattern,
            _Pattern::RemainingFieldPattern =>
                _Pattern::RemainingFieldPattern,
            _Pattern::SlicePattern => _Pattern::SlicePattern,
            _Pattern::StructPattern => _Pattern::StructPattern,
            _Pattern::TuplePattern => _Pattern::TuplePattern,
            _Pattern::TupleStructPattern => _Pattern::TupleStructPattern,
        }
    }
}
enum RustNodes {
    Parameter,
    EnumVariant,
    TokenRepetitionPattern,
    LoopToken,
    StringContent,
    AsteriskequalsSignToken,
    BreakToken,
    _Pattern(_Pattern),
    Label,
    MatchPattern,
    OrderedFieldDeclarationList,
    RightParenthesisToken,
    LeftSquareBracketToken,
    ReturnToken,
    Shebang,
    HigherRankedTraitBound,
    RightSquareBracketToken,
    _Type(_Type),
    ExclamationMarkequalsSignToken,
    TryToken,
    AsToken,
    UnionToken,
    UseList,
    ClosureParameters,
    TypeParameters,
    LessThanSignlessThanSignToken,
    ApostropheToken,
    ExternToken,
    VisibilityModifier,
    TyToken,
    UseToken,
    PubToken,
    FalseToken,
    GreaterThanSignToken,
    LiteralToken,
    PathToken,
    InnerDocCommentMarker,
    DefaultToken,
    RefToken,
    ExpressionStatement,
    VerticalLineToken,
    UnsafeToken,
    ElseClause,
    SolidusequalsSignToken,
    RightCurlyBracketToken,
    ExclamationMarkToken,
    AsterisksolidusToken,
    TokenTreePattern,
    EnumVariantList,
    ForLifetimes,
    DocComment,
    TokenTree,
    NumberSignToken,
    ConstToken,
    AmpersandToken,
    SemicolonToken,
    AwaitToken,
    ForToken,
    CommercialAtToken,
    MacroRule,
    AsyncToken,
    VisToken,
    ShorthandFieldIdentifier,
    PlusSignequalsSignToken,
    AmpersandequalsSignToken,
    HyphenMinusgreaterThanSignToken,
    SolidusToken,
    GreaterThanSigngreaterThanSignequalsSignToken,
    BaseFieldInitializer,
    SelfParameter,
    LessThanSignToken,
    EscapeSequence,
    RawToken,
    ItemToken,
    ImplToken,
    SolidussolidusToken,
    MetaToken,
    HyphenMinusToken,
    Arguments,
    LeftCurlyBracketToken,
    LeftParenthesisToken,
    FieldDeclarationList,
    VerticalLineequalsSignToken,
    ConstParameter,
    StructToken,
    ColoncolonToken,
    TraitBounds,
    BlockComment,
    PercentSignequalsSignToken,
    UseBounds,
    EnumToken,
    LessThanSignequalsSignToken,
    WhereClause,
    ExprToken,
    UseAsClause,
    PercentSignToken,
    FnToken,
    FieldInitializer,
    VariadicParameter,
    FieldIdentifier,
    LetChain,
    OuterDocCommentMarker,
    FullStopToken,
    QuotationMarkToken,
    FieldPattern,
    FullStopfullStopfullStopToken,
    TtToken,
    WhileToken,
    IfToken,
    MoveToken,
    FunctionModifiers,
    CircumflexAccentToken,
    MatchToken,
    FullStopfullStopequalsSignToken,
    DollarSignToken,
    StmtToken,
    TraitToken,
    AsteriskToken,
    FullStopfullStopToken,
    FieldInitializerList,
    SolidusasteriskToken,
    GreaterThanSigngreaterThanSignToken,
    FragmentSpecifier,
    InToken,
    MutableSpecifier,
    AmpersandampersandToken,
    QualifiedType,
    ColonToken,
    YieldToken,
    MatchArm,
    StaticToken,
    GenericTypeWithTurbofish,
    TrueToken,
    CircumflexAccentequalsSignToken,
    Expr2021Token,
    Attribute,
    TypeArguments,
    QuestionMarkToken,
    LetToken,
    TokenBindingPattern,
    WhereToken,
    ElseToken,
    IdentToken,
    EqualsSigngreaterThanSignToken,
    VerticalLineverticalLineToken,
    FieldDeclaration,
    Parameters,
    WherePredicate,
    GreaterThanSignequalsSignToken,
    TypeParameter,
    TypeBinding,
    PatToken,
    ScopedUseList,
    MatchBlock,
    ContinueToken,
    Super,
    ShorthandFieldInitializer,
    GenToken,
    PlusSignToken,
    HyphenMinusequalsSignToken,
    LifetimeParameter,
    DynToken,
    Lifetime,
    LetCondition,
    LessThanSignlessThanSignequalsSignToken,
    TokenRepetition,
    LineComment,
    ExternModifier,
    TypeToken,
    UseWildcard,
    _Expression(_Expression),
    SourceFile,
    Crate,
    ModToken,
    _DeclarationStatement(_DeclarationStatement),
    EqualsSignequalsSignToken,
    DeclarationList,
    CommaToken,
    MacroRulesExclamationMarkToken,
    EqualsSignToken,
    BracketedType,
    PatParamToken,
}
impl std::str::FromStr for RustNodes {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "parameter" => {
                return std::result::Result::Ok(RustNodes::Parameter)
            }
            "enum_variant" => {
                return std::result::Result::Ok(RustNodes::EnumVariant)
            }
            "token_repetition_pattern" => {
                return std::result::Result::Ok(RustNodes::TokenRepetitionPattern)
            }
            "loop" => { return std::result::Result::Ok(RustNodes::LoopToken) }
            "string_content" => {
                return std::result::Result::Ok(RustNodes::StringContent)
            }
            "*=" => {
                return std::result::Result::Ok(RustNodes::AsteriskequalsSignToken)
            }
            "break" => {
                return std::result::Result::Ok(RustNodes::BreakToken)
            }
            "_" => {
                return std::result::Result::Ok(RustNodes::_Pattern(_Pattern::_Token))
            }
            "captured_pattern" => {
                return std::result::Result::Ok(RustNodes::_Pattern(_Pattern::CapturedPattern))
            }
            "generic_pattern" => {
                return std::result::Result::Ok(RustNodes::_Pattern(_Pattern::GenericPattern))
            }
            "mut_pattern" => {
                return std::result::Result::Ok(RustNodes::_Pattern(_Pattern::MutPattern))
            }
            "or_pattern" => {
                return std::result::Result::Ok(RustNodes::_Pattern(_Pattern::OrPattern))
            }
            "range_pattern" => {
                return std::result::Result::Ok(RustNodes::_Pattern(_Pattern::RangePattern))
            }
            "ref_pattern" => {
                return std::result::Result::Ok(RustNodes::_Pattern(_Pattern::RefPattern))
            }
            "reference_pattern" => {
                return std::result::Result::Ok(RustNodes::_Pattern(_Pattern::ReferencePattern))
            }
            "remaining_field_pattern" => {
                return std::result::Result::Ok(RustNodes::_Pattern(_Pattern::RemainingFieldPattern))
            }
            "slice_pattern" => {
                return std::result::Result::Ok(RustNodes::_Pattern(_Pattern::SlicePattern))
            }
            "struct_pattern" => {
                return std::result::Result::Ok(RustNodes::_Pattern(_Pattern::StructPattern))
            }
            "tuple_pattern" => {
                return std::result::Result::Ok(RustNodes::_Pattern(_Pattern::TuplePattern))
            }
            "tuple_struct_pattern" => {
                return std::result::Result::Ok(RustNodes::_Pattern(_Pattern::TupleStructPattern))
            }
            "label" => { return std::result::Result::Ok(RustNodes::Label) }
            "match_pattern" => {
                return std::result::Result::Ok(RustNodes::MatchPattern)
            }
            "ordered_field_declaration_list" => {
                return std::result::Result::Ok(RustNodes::OrderedFieldDeclarationList)
            }
            ")" => {
                return std::result::Result::Ok(RustNodes::RightParenthesisToken)
            }
            "[" => {
                return std::result::Result::Ok(RustNodes::LeftSquareBracketToken)
            }
            "return" => {
                return std::result::Result::Ok(RustNodes::ReturnToken)
            }
            "shebang" => {
                return std::result::Result::Ok(RustNodes::Shebang)
            }
            "higher_ranked_trait_bound" => {
                return std::result::Result::Ok(RustNodes::HigherRankedTraitBound)
            }
            "]" => {
                return std::result::Result::Ok(RustNodes::RightSquareBracketToken)
            }
            "abstract_type" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::AbstractType))
            }
            "array_type" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::ArrayType))
            }
            "bounded_type" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::BoundedType))
            }
            "dynamic_type" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::DynamicType))
            }
            "function_type" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::FunctionType))
            }
            "generic_type" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::GenericType))
            }
            "never_type" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::NeverType))
            }
            "pointer_type" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::PointerType))
            }
            "primitive_type" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::PrimitiveType))
            }
            "reference_type" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::ReferenceType))
            }
            "removed_trait_bound" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::RemovedTraitBound))
            }
            "scoped_type_identifier" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::ScopedTypeIdentifier))
            }
            "tuple_type" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::TupleType))
            }
            "type_identifier" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::TypeIdentifier))
            }
            "unit_type" => {
                return std::result::Result::Ok(RustNodes::_Type(_Type::UnitType))
            }
            "!=" => {
                return std::result::Result::Ok(RustNodes::ExclamationMarkequalsSignToken)
            }
            "try" => { return std::result::Result::Ok(RustNodes::TryToken) }
            "as" => { return std::result::Result::Ok(RustNodes::AsToken) }
            "union" => {
                return std::result::Result::Ok(RustNodes::UnionToken)
            }
            "use_list" => {
                return std::result::Result::Ok(RustNodes::UseList)
            }
            "closure_parameters" => {
                return std::result::Result::Ok(RustNodes::ClosureParameters)
            }
            "type_parameters" => {
                return std::result::Result::Ok(RustNodes::TypeParameters)
            }
            "<<" => {
                return std::result::Result::Ok(RustNodes::LessThanSignlessThanSignToken)
            }
            "'" => {
                return std::result::Result::Ok(RustNodes::ApostropheToken)
            }
            "extern" => {
                return std::result::Result::Ok(RustNodes::ExternToken)
            }
            "visibility_modifier" => {
                return std::result::Result::Ok(RustNodes::VisibilityModifier)
            }
            "ty" => { return std::result::Result::Ok(RustNodes::TyToken) }
            "use" => { return std::result::Result::Ok(RustNodes::UseToken) }
            "pub" => { return std::result::Result::Ok(RustNodes::PubToken) }
            "false" => {
                return std::result::Result::Ok(RustNodes::FalseToken)
            }
            ">" => {
                return std::result::Result::Ok(RustNodes::GreaterThanSignToken)
            }
            "literal" => {
                return std::result::Result::Ok(RustNodes::LiteralToken)
            }
            "path" => { return std::result::Result::Ok(RustNodes::PathToken) }
            "inner_doc_comment_marker" => {
                return std::result::Result::Ok(RustNodes::InnerDocCommentMarker)
            }
            "default" => {
                return std::result::Result::Ok(RustNodes::DefaultToken)
            }
            "ref" => { return std::result::Result::Ok(RustNodes::RefToken) }
            "expression_statement" => {
                return std::result::Result::Ok(RustNodes::ExpressionStatement)
            }
            "|" => {
                return std::result::Result::Ok(RustNodes::VerticalLineToken)
            }
            "unsafe" => {
                return std::result::Result::Ok(RustNodes::UnsafeToken)
            }
            "else_clause" => {
                return std::result::Result::Ok(RustNodes::ElseClause)
            }
            "/=" => {
                return std::result::Result::Ok(RustNodes::SolidusequalsSignToken)
            }
            "}" => {
                return std::result::Result::Ok(RustNodes::RightCurlyBracketToken)
            }
            "!" => {
                return std::result::Result::Ok(RustNodes::ExclamationMarkToken)
            }
            "*/" => {
                return std::result::Result::Ok(RustNodes::AsterisksolidusToken)
            }
            "token_tree_pattern" => {
                return std::result::Result::Ok(RustNodes::TokenTreePattern)
            }
            "enum_variant_list" => {
                return std::result::Result::Ok(RustNodes::EnumVariantList)
            }
            "for_lifetimes" => {
                return std::result::Result::Ok(RustNodes::ForLifetimes)
            }
            "doc_comment" => {
                return std::result::Result::Ok(RustNodes::DocComment)
            }
            "token_tree" => {
                return std::result::Result::Ok(RustNodes::TokenTree)
            }
            "#" => {
                return std::result::Result::Ok(RustNodes::NumberSignToken)
            }
            "const" => {
                return std::result::Result::Ok(RustNodes::ConstToken)
            }
            "&" => {
                return std::result::Result::Ok(RustNodes::AmpersandToken)
            }
            ";" => {
                return std::result::Result::Ok(RustNodes::SemicolonToken)
            }
            "await" => {
                return std::result::Result::Ok(RustNodes::AwaitToken)
            }
            "for" => { return std::result::Result::Ok(RustNodes::ForToken) }
            "@" => {
                return std::result::Result::Ok(RustNodes::CommercialAtToken)
            }
            "macro_rule" => {
                return std::result::Result::Ok(RustNodes::MacroRule)
            }
            "async" => {
                return std::result::Result::Ok(RustNodes::AsyncToken)
            }
            "vis" => { return std::result::Result::Ok(RustNodes::VisToken) }
            "shorthand_field_identifier" => {
                return std::result::Result::Ok(RustNodes::ShorthandFieldIdentifier)
            }
            "+=" => {
                return std::result::Result::Ok(RustNodes::PlusSignequalsSignToken)
            }
            "&=" => {
                return std::result::Result::Ok(RustNodes::AmpersandequalsSignToken)
            }
            "->" => {
                return std::result::Result::Ok(RustNodes::HyphenMinusgreaterThanSignToken)
            }
            "/" => { return std::result::Result::Ok(RustNodes::SolidusToken) }
            ">>=" => {
                return std::result::Result::Ok(RustNodes::GreaterThanSigngreaterThanSignequalsSignToken)
            }
            "base_field_initializer" => {
                return std::result::Result::Ok(RustNodes::BaseFieldInitializer)
            }
            "self_parameter" => {
                return std::result::Result::Ok(RustNodes::SelfParameter)
            }
            "<" => {
                return std::result::Result::Ok(RustNodes::LessThanSignToken)
            }
            "escape_sequence" => {
                return std::result::Result::Ok(RustNodes::EscapeSequence)
            }
            "raw" => { return std::result::Result::Ok(RustNodes::RawToken) }
            "item" => { return std::result::Result::Ok(RustNodes::ItemToken) }
            "impl" => { return std::result::Result::Ok(RustNodes::ImplToken) }
            "//" => {
                return std::result::Result::Ok(RustNodes::SolidussolidusToken)
            }
            "meta" => { return std::result::Result::Ok(RustNodes::MetaToken) }
            "-" => {
                return std::result::Result::Ok(RustNodes::HyphenMinusToken)
            }
            "arguments" => {
                return std::result::Result::Ok(RustNodes::Arguments)
            }
            "{" => {
                return std::result::Result::Ok(RustNodes::LeftCurlyBracketToken)
            }
            "(" => {
                return std::result::Result::Ok(RustNodes::LeftParenthesisToken)
            }
            "field_declaration_list" => {
                return std::result::Result::Ok(RustNodes::FieldDeclarationList)
            }
            "|=" => {
                return std::result::Result::Ok(RustNodes::VerticalLineequalsSignToken)
            }
            "const_parameter" => {
                return std::result::Result::Ok(RustNodes::ConstParameter)
            }
            "struct" => {
                return std::result::Result::Ok(RustNodes::StructToken)
            }
            "::" => {
                return std::result::Result::Ok(RustNodes::ColoncolonToken)
            }
            "trait_bounds" => {
                return std::result::Result::Ok(RustNodes::TraitBounds)
            }
            "block_comment" => {
                return std::result::Result::Ok(RustNodes::BlockComment)
            }
            "%=" => {
                return std::result::Result::Ok(RustNodes::PercentSignequalsSignToken)
            }
            "use_bounds" => {
                return std::result::Result::Ok(RustNodes::UseBounds)
            }
            "enum" => { return std::result::Result::Ok(RustNodes::EnumToken) }
            "<=" => {
                return std::result::Result::Ok(RustNodes::LessThanSignequalsSignToken)
            }
            "where_clause" => {
                return std::result::Result::Ok(RustNodes::WhereClause)
            }
            "expr" => { return std::result::Result::Ok(RustNodes::ExprToken) }
            "use_as_clause" => {
                return std::result::Result::Ok(RustNodes::UseAsClause)
            }
            "%" => {
                return std::result::Result::Ok(RustNodes::PercentSignToken)
            }
            "fn" => { return std::result::Result::Ok(RustNodes::FnToken) }
            "field_initializer" => {
                return std::result::Result::Ok(RustNodes::FieldInitializer)
            }
            "variadic_parameter" => {
                return std::result::Result::Ok(RustNodes::VariadicParameter)
            }
            "field_identifier" => {
                return std::result::Result::Ok(RustNodes::FieldIdentifier)
            }
            "let_chain" => {
                return std::result::Result::Ok(RustNodes::LetChain)
            }
            "outer_doc_comment_marker" => {
                return std::result::Result::Ok(RustNodes::OuterDocCommentMarker)
            }
            "." => {
                return std::result::Result::Ok(RustNodes::FullStopToken)
            }
            "\"" => {
                return std::result::Result::Ok(RustNodes::QuotationMarkToken)
            }
            "field_pattern" => {
                return std::result::Result::Ok(RustNodes::FieldPattern)
            }
            "..." => {
                return std::result::Result::Ok(RustNodes::FullStopfullStopfullStopToken)
            }
            "tt" => { return std::result::Result::Ok(RustNodes::TtToken) }
            "while" => {
                return std::result::Result::Ok(RustNodes::WhileToken)
            }
            "if" => { return std::result::Result::Ok(RustNodes::IfToken) }
            "move" => { return std::result::Result::Ok(RustNodes::MoveToken) }
            "function_modifiers" => {
                return std::result::Result::Ok(RustNodes::FunctionModifiers)
            }
            "^" => {
                return std::result::Result::Ok(RustNodes::CircumflexAccentToken)
            }
            "match" => {
                return std::result::Result::Ok(RustNodes::MatchToken)
            }
            "..=" => {
                return std::result::Result::Ok(RustNodes::FullStopfullStopequalsSignToken)
            }
            "$" => {
                return std::result::Result::Ok(RustNodes::DollarSignToken)
            }
            "stmt" => { return std::result::Result::Ok(RustNodes::StmtToken) }
            "trait" => {
                return std::result::Result::Ok(RustNodes::TraitToken)
            }
            "*" => {
                return std::result::Result::Ok(RustNodes::AsteriskToken)
            }
            ".." => {
                return std::result::Result::Ok(RustNodes::FullStopfullStopToken)
            }
            "field_initializer_list" => {
                return std::result::Result::Ok(RustNodes::FieldInitializerList)
            }
            "/*" => {
                return std::result::Result::Ok(RustNodes::SolidusasteriskToken)
            }
            ">>" => {
                return std::result::Result::Ok(RustNodes::GreaterThanSigngreaterThanSignToken)
            }
            "fragment_specifier" => {
                return std::result::Result::Ok(RustNodes::FragmentSpecifier)
            }
            "in" => { return std::result::Result::Ok(RustNodes::InToken) }
            "mutable_specifier" => {
                return std::result::Result::Ok(RustNodes::MutableSpecifier)
            }
            "&&" => {
                return std::result::Result::Ok(RustNodes::AmpersandampersandToken)
            }
            "qualified_type" => {
                return std::result::Result::Ok(RustNodes::QualifiedType)
            }
            ":" => { return std::result::Result::Ok(RustNodes::ColonToken) }
            "yield" => {
                return std::result::Result::Ok(RustNodes::YieldToken)
            }
            "match_arm" => {
                return std::result::Result::Ok(RustNodes::MatchArm)
            }
            "static" => {
                return std::result::Result::Ok(RustNodes::StaticToken)
            }
            "generic_type_with_turbofish" => {
                return std::result::Result::Ok(RustNodes::GenericTypeWithTurbofish)
            }
            "true" => { return std::result::Result::Ok(RustNodes::TrueToken) }
            "^=" => {
                return std::result::Result::Ok(RustNodes::CircumflexAccentequalsSignToken)
            }
            "expr_2021" => {
                return std::result::Result::Ok(RustNodes::Expr2021Token)
            }
            "attribute" => {
                return std::result::Result::Ok(RustNodes::Attribute)
            }
            "type_arguments" => {
                return std::result::Result::Ok(RustNodes::TypeArguments)
            }
            "?" => {
                return std::result::Result::Ok(RustNodes::QuestionMarkToken)
            }
            "let" => { return std::result::Result::Ok(RustNodes::LetToken) }
            "token_binding_pattern" => {
                return std::result::Result::Ok(RustNodes::TokenBindingPattern)
            }
            "where" => {
                return std::result::Result::Ok(RustNodes::WhereToken)
            }
            "else" => { return std::result::Result::Ok(RustNodes::ElseToken) }
            "ident" => {
                return std::result::Result::Ok(RustNodes::IdentToken)
            }
            "=>" => {
                return std::result::Result::Ok(RustNodes::EqualsSigngreaterThanSignToken)
            }
            "||" => {
                return std::result::Result::Ok(RustNodes::VerticalLineverticalLineToken)
            }
            "field_declaration" => {
                return std::result::Result::Ok(RustNodes::FieldDeclaration)
            }
            "parameters" => {
                return std::result::Result::Ok(RustNodes::Parameters)
            }
            "where_predicate" => {
                return std::result::Result::Ok(RustNodes::WherePredicate)
            }
            ">=" => {
                return std::result::Result::Ok(RustNodes::GreaterThanSignequalsSignToken)
            }
            "type_parameter" => {
                return std::result::Result::Ok(RustNodes::TypeParameter)
            }
            "type_binding" => {
                return std::result::Result::Ok(RustNodes::TypeBinding)
            }
            "pat" => { return std::result::Result::Ok(RustNodes::PatToken) }
            "scoped_use_list" => {
                return std::result::Result::Ok(RustNodes::ScopedUseList)
            }
            "match_block" => {
                return std::result::Result::Ok(RustNodes::MatchBlock)
            }
            "continue" => {
                return std::result::Result::Ok(RustNodes::ContinueToken)
            }
            "super" => { return std::result::Result::Ok(RustNodes::Super) }
            "shorthand_field_initializer" => {
                return std::result::Result::Ok(RustNodes::ShorthandFieldInitializer)
            }
            "gen" => { return std::result::Result::Ok(RustNodes::GenToken) }
            "+" => {
                return std::result::Result::Ok(RustNodes::PlusSignToken)
            }
            "-=" => {
                return std::result::Result::Ok(RustNodes::HyphenMinusequalsSignToken)
            }
            "lifetime_parameter" => {
                return std::result::Result::Ok(RustNodes::LifetimeParameter)
            }
            "dyn" => { return std::result::Result::Ok(RustNodes::DynToken) }
            "lifetime" => {
                return std::result::Result::Ok(RustNodes::Lifetime)
            }
            "let_condition" => {
                return std::result::Result::Ok(RustNodes::LetCondition)
            }
            "<<=" => {
                return std::result::Result::Ok(RustNodes::LessThanSignlessThanSignequalsSignToken)
            }
            "token_repetition" => {
                return std::result::Result::Ok(RustNodes::TokenRepetition)
            }
            "line_comment" => {
                return std::result::Result::Ok(RustNodes::LineComment)
            }
            "extern_modifier" => {
                return std::result::Result::Ok(RustNodes::ExternModifier)
            }
            "type" => { return std::result::Result::Ok(RustNodes::TypeToken) }
            "use_wildcard" => {
                return std::result::Result::Ok(RustNodes::UseWildcard)
            }
            "_literal" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::_Literal))
            }
            "array_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::ArrayExpression))
            }
            "assignment_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::AssignmentExpression))
            }
            "async_block" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::AsyncBlock))
            }
            "await_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::AwaitExpression))
            }
            "binary_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::BinaryExpression))
            }
            "block" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::Block))
            }
            "break_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::BreakExpression))
            }
            "call_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::CallExpression))
            }
            "closure_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::ClosureExpression))
            }
            "compound_assignment_expr" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::CompoundAssignmentExpr))
            }
            "const_block" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::ConstBlock))
            }
            "continue_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::ContinueExpression))
            }
            "field_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::FieldExpression))
            }
            "for_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::ForExpression))
            }
            "gen_block" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::GenBlock))
            }
            "generic_function" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::GenericFunction))
            }
            "identifier" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::Identifier))
            }
            "if_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::IfExpression))
            }
            "index_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::IndexExpression))
            }
            "loop_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::LoopExpression))
            }
            "match_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::MatchExpression))
            }
            "metavariable" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::Metavariable))
            }
            "parenthesized_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::ParenthesizedExpression))
            }
            "range_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::RangeExpression))
            }
            "reference_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::ReferenceExpression))
            }
            "return_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::ReturnExpression))
            }
            "scoped_identifier" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::ScopedIdentifier))
            }
            "self" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::SelfToken))
            }
            "struct_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::StructExpression))
            }
            "try_block" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::TryBlock))
            }
            "try_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::TryExpression))
            }
            "tuple_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::TupleExpression))
            }
            "type_cast_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::TypeCastExpression))
            }
            "unary_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::UnaryExpression))
            }
            "unit_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::UnitExpression))
            }
            "unsafe_block" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::UnsafeBlock))
            }
            "while_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::WhileExpression))
            }
            "yield_expression" => {
                return std::result::Result::Ok(RustNodes::_Expression(_Expression::YieldExpression))
            }
            "source_file" => {
                return std::result::Result::Ok(RustNodes::SourceFile)
            }
            "crate" => { return std::result::Result::Ok(RustNodes::Crate) }
            "mod" => { return std::result::Result::Ok(RustNodes::ModToken) }
            "associated_type" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::AssociatedType))
            }
            "attribute_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::AttributeItem))
            }
            "const_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::ConstItem))
            }
            "empty_statement" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::EmptyStatement))
            }
            "enum_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::EnumItem))
            }
            "extern_crate_declaration" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::ExternCrateDeclaration))
            }
            "foreign_mod_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::ForeignModItem))
            }
            "function_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::FunctionItem))
            }
            "function_signature_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::FunctionSignatureItem))
            }
            "impl_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::ImplItem))
            }
            "inner_attribute_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::InnerAttributeItem))
            }
            "let_declaration" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::LetDeclaration))
            }
            "macro_definition" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::MacroDefinition))
            }
            "macro_invocation" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::MacroInvocation))
            }
            "mod_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::ModItem))
            }
            "static_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::StaticItem))
            }
            "struct_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::StructItem))
            }
            "trait_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::TraitItem))
            }
            "type_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::TypeItem))
            }
            "union_item" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::UnionItem))
            }
            "use_declaration" => {
                return std::result::Result::Ok(RustNodes::_DeclarationStatement(_DeclarationStatement::UseDeclaration))
            }
            "==" => {
                return std::result::Result::Ok(RustNodes::EqualsSignequalsSignToken)
            }
            "declaration_list" => {
                return std::result::Result::Ok(RustNodes::DeclarationList)
            }
            "," => { return std::result::Result::Ok(RustNodes::CommaToken) }
            "macro_rules!" => {
                return std::result::Result::Ok(RustNodes::MacroRulesExclamationMarkToken)
            }
            "=" => {
                return std::result::Result::Ok(RustNodes::EqualsSignToken)
            }
            "bracketed_type" => {
                return std::result::Result::Ok(RustNodes::BracketedType)
            }
            "pat_param" => {
                return std::result::Result::Ok(RustNodes::PatParamToken)
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
            Self::Parameter => { f.write_fmt(format_args!("parameter")) }
            Self::EnumVariant => { f.write_fmt(format_args!("enum_variant")) }
            Self::TokenRepetitionPattern => {
                f.write_fmt(format_args!("token_repetition_pattern"))
            }
            Self::LoopToken => { f.write_fmt(format_args!("loop")) }
            Self::StringContent => {
                f.write_fmt(format_args!("string_content"))
            }
            Self::AsteriskequalsSignToken => {
                f.write_fmt(format_args!("*="))
            }
            Self::BreakToken => { f.write_fmt(format_args!("break")) }
            Self::_Pattern(_Pattern::_Token) => {
                f.write_fmt(format_args!("_"))
            }
            Self::_Pattern(_Pattern::CapturedPattern) => {
                f.write_fmt(format_args!("captured_pattern"))
            }
            Self::_Pattern(_Pattern::GenericPattern) => {
                f.write_fmt(format_args!("generic_pattern"))
            }
            Self::_Pattern(_Pattern::MutPattern) => {
                f.write_fmt(format_args!("mut_pattern"))
            }
            Self::_Pattern(_Pattern::OrPattern) => {
                f.write_fmt(format_args!("or_pattern"))
            }
            Self::_Pattern(_Pattern::RangePattern) => {
                f.write_fmt(format_args!("range_pattern"))
            }
            Self::_Pattern(_Pattern::RefPattern) => {
                f.write_fmt(format_args!("ref_pattern"))
            }
            Self::_Pattern(_Pattern::ReferencePattern) => {
                f.write_fmt(format_args!("reference_pattern"))
            }
            Self::_Pattern(_Pattern::RemainingFieldPattern) => {
                f.write_fmt(format_args!("remaining_field_pattern"))
            }
            Self::_Pattern(_Pattern::SlicePattern) => {
                f.write_fmt(format_args!("slice_pattern"))
            }
            Self::_Pattern(_Pattern::StructPattern) => {
                f.write_fmt(format_args!("struct_pattern"))
            }
            Self::_Pattern(_Pattern::TuplePattern) => {
                f.write_fmt(format_args!("tuple_pattern"))
            }
            Self::_Pattern(_Pattern::TupleStructPattern) => {
                f.write_fmt(format_args!("tuple_struct_pattern"))
            }
            Self::Label => { f.write_fmt(format_args!("label")) }
            Self::MatchPattern => {
                f.write_fmt(format_args!("match_pattern"))
            }
            Self::OrderedFieldDeclarationList => {
                f.write_fmt(format_args!("ordered_field_declaration_list"))
            }
            Self::RightParenthesisToken => { f.write_fmt(format_args!(")")) }
            Self::LeftSquareBracketToken => { f.write_fmt(format_args!("[")) }
            Self::ReturnToken => { f.write_fmt(format_args!("return")) }
            Self::Shebang => { f.write_fmt(format_args!("shebang")) }
            Self::HigherRankedTraitBound => {
                f.write_fmt(format_args!("higher_ranked_trait_bound"))
            }
            Self::RightSquareBracketToken => {
                f.write_fmt(format_args!("]"))
            }
            Self::_Type(_Type::AbstractType) => {
                f.write_fmt(format_args!("abstract_type"))
            }
            Self::_Type(_Type::ArrayType) => {
                f.write_fmt(format_args!("array_type"))
            }
            Self::_Type(_Type::BoundedType) => {
                f.write_fmt(format_args!("bounded_type"))
            }
            Self::_Type(_Type::DynamicType) => {
                f.write_fmt(format_args!("dynamic_type"))
            }
            Self::_Type(_Type::FunctionType) => {
                f.write_fmt(format_args!("function_type"))
            }
            Self::_Type(_Type::GenericType) => {
                f.write_fmt(format_args!("generic_type"))
            }
            Self::_Type(_Type::NeverType) => {
                f.write_fmt(format_args!("never_type"))
            }
            Self::_Type(_Type::PointerType) => {
                f.write_fmt(format_args!("pointer_type"))
            }
            Self::_Type(_Type::PrimitiveType) => {
                f.write_fmt(format_args!("primitive_type"))
            }
            Self::_Type(_Type::ReferenceType) => {
                f.write_fmt(format_args!("reference_type"))
            }
            Self::_Type(_Type::RemovedTraitBound) => {
                f.write_fmt(format_args!("removed_trait_bound"))
            }
            Self::_Type(_Type::ScopedTypeIdentifier) => {
                f.write_fmt(format_args!("scoped_type_identifier"))
            }
            Self::_Type(_Type::TupleType) => {
                f.write_fmt(format_args!("tuple_type"))
            }
            Self::_Type(_Type::TypeIdentifier) => {
                f.write_fmt(format_args!("type_identifier"))
            }
            Self::_Type(_Type::UnitType) => {
                f.write_fmt(format_args!("unit_type"))
            }
            Self::ExclamationMarkequalsSignToken => {
                f.write_fmt(format_args!("!="))
            }
            Self::TryToken => { f.write_fmt(format_args!("try")) }
            Self::AsToken => { f.write_fmt(format_args!("as")) }
            Self::UnionToken => { f.write_fmt(format_args!("union")) }
            Self::UseList => { f.write_fmt(format_args!("use_list")) }
            Self::ClosureParameters => {
                f.write_fmt(format_args!("closure_parameters"))
            }
            Self::TypeParameters => {
                f.write_fmt(format_args!("type_parameters"))
            }
            Self::LessThanSignlessThanSignToken => {
                f.write_fmt(format_args!("<<"))
            }
            Self::ApostropheToken => { f.write_fmt(format_args!("\'")) }
            Self::ExternToken => { f.write_fmt(format_args!("extern")) }
            Self::VisibilityModifier => {
                f.write_fmt(format_args!("visibility_modifier"))
            }
            Self::TyToken => { f.write_fmt(format_args!("ty")) }
            Self::UseToken => { f.write_fmt(format_args!("use")) }
            Self::PubToken => { f.write_fmt(format_args!("pub")) }
            Self::FalseToken => { f.write_fmt(format_args!("false")) }
            Self::GreaterThanSignToken => { f.write_fmt(format_args!(">")) }
            Self::LiteralToken => { f.write_fmt(format_args!("literal")) }
            Self::PathToken => { f.write_fmt(format_args!("path")) }
            Self::InnerDocCommentMarker => {
                f.write_fmt(format_args!("inner_doc_comment_marker"))
            }
            Self::DefaultToken => { f.write_fmt(format_args!("default")) }
            Self::RefToken => { f.write_fmt(format_args!("ref")) }
            Self::ExpressionStatement => {
                f.write_fmt(format_args!("expression_statement"))
            }
            Self::VerticalLineToken => { f.write_fmt(format_args!("|")) }
            Self::UnsafeToken => { f.write_fmt(format_args!("unsafe")) }
            Self::ElseClause => { f.write_fmt(format_args!("else_clause")) }
            Self::SolidusequalsSignToken => {
                f.write_fmt(format_args!("/="))
            }
            Self::RightCurlyBracketToken => {
                f.write_fmt(format_args!("}}"))
            }
            Self::ExclamationMarkToken => { f.write_fmt(format_args!("!")) }
            Self::AsterisksolidusToken => { f.write_fmt(format_args!("*/")) }
            Self::TokenTreePattern => {
                f.write_fmt(format_args!("token_tree_pattern"))
            }
            Self::EnumVariantList => {
                f.write_fmt(format_args!("enum_variant_list"))
            }
            Self::ForLifetimes => {
                f.write_fmt(format_args!("for_lifetimes"))
            }
            Self::DocComment => { f.write_fmt(format_args!("doc_comment")) }
            Self::TokenTree => { f.write_fmt(format_args!("token_tree")) }
            Self::NumberSignToken => { f.write_fmt(format_args!("#")) }
            Self::ConstToken => { f.write_fmt(format_args!("const")) }
            Self::AmpersandToken => { f.write_fmt(format_args!("&")) }
            Self::SemicolonToken => { f.write_fmt(format_args!(";")) }
            Self::AwaitToken => { f.write_fmt(format_args!("await")) }
            Self::ForToken => { f.write_fmt(format_args!("for")) }
            Self::CommercialAtToken => { f.write_fmt(format_args!("@")) }
            Self::MacroRule => { f.write_fmt(format_args!("macro_rule")) }
            Self::AsyncToken => { f.write_fmt(format_args!("async")) }
            Self::VisToken => { f.write_fmt(format_args!("vis")) }
            Self::ShorthandFieldIdentifier => {
                f.write_fmt(format_args!("shorthand_field_identifier"))
            }
            Self::PlusSignequalsSignToken => {
                f.write_fmt(format_args!("+="))
            }
            Self::AmpersandequalsSignToken => {
                f.write_fmt(format_args!("&="))
            }
            Self::HyphenMinusgreaterThanSignToken => {
                f.write_fmt(format_args!("->"))
            }
            Self::SolidusToken => { f.write_fmt(format_args!("/")) }
            Self::GreaterThanSigngreaterThanSignequalsSignToken => {
                f.write_fmt(format_args!(">>="))
            }
            Self::BaseFieldInitializer => {
                f.write_fmt(format_args!("base_field_initializer"))
            }
            Self::SelfParameter => {
                f.write_fmt(format_args!("self_parameter"))
            }
            Self::LessThanSignToken => { f.write_fmt(format_args!("<")) }
            Self::EscapeSequence => {
                f.write_fmt(format_args!("escape_sequence"))
            }
            Self::RawToken => { f.write_fmt(format_args!("raw")) }
            Self::ItemToken => { f.write_fmt(format_args!("item")) }
            Self::ImplToken => { f.write_fmt(format_args!("impl")) }
            Self::SolidussolidusToken => { f.write_fmt(format_args!("//")) }
            Self::MetaToken => { f.write_fmt(format_args!("meta")) }
            Self::HyphenMinusToken => { f.write_fmt(format_args!("-")) }
            Self::Arguments => { f.write_fmt(format_args!("arguments")) }
            Self::LeftCurlyBracketToken => { f.write_fmt(format_args!("{{")) }
            Self::LeftParenthesisToken => { f.write_fmt(format_args!("(")) }
            Self::FieldDeclarationList => {
                f.write_fmt(format_args!("field_declaration_list"))
            }
            Self::VerticalLineequalsSignToken => {
                f.write_fmt(format_args!("|="))
            }
            Self::ConstParameter => {
                f.write_fmt(format_args!("const_parameter"))
            }
            Self::StructToken => { f.write_fmt(format_args!("struct")) }
            Self::ColoncolonToken => { f.write_fmt(format_args!("::")) }
            Self::TraitBounds => { f.write_fmt(format_args!("trait_bounds")) }
            Self::BlockComment => {
                f.write_fmt(format_args!("block_comment"))
            }
            Self::PercentSignequalsSignToken => {
                f.write_fmt(format_args!("%="))
            }
            Self::UseBounds => { f.write_fmt(format_args!("use_bounds")) }
            Self::EnumToken => { f.write_fmt(format_args!("enum")) }
            Self::LessThanSignequalsSignToken => {
                f.write_fmt(format_args!("<="))
            }
            Self::WhereClause => { f.write_fmt(format_args!("where_clause")) }
            Self::ExprToken => { f.write_fmt(format_args!("expr")) }
            Self::UseAsClause => {
                f.write_fmt(format_args!("use_as_clause"))
            }
            Self::PercentSignToken => { f.write_fmt(format_args!("%")) }
            Self::FnToken => { f.write_fmt(format_args!("fn")) }
            Self::FieldInitializer => {
                f.write_fmt(format_args!("field_initializer"))
            }
            Self::VariadicParameter => {
                f.write_fmt(format_args!("variadic_parameter"))
            }
            Self::FieldIdentifier => {
                f.write_fmt(format_args!("field_identifier"))
            }
            Self::LetChain => { f.write_fmt(format_args!("let_chain")) }
            Self::OuterDocCommentMarker => {
                f.write_fmt(format_args!("outer_doc_comment_marker"))
            }
            Self::FullStopToken => { f.write_fmt(format_args!(".")) }
            Self::QuotationMarkToken => { f.write_fmt(format_args!("\"")) }
            Self::FieldPattern => {
                f.write_fmt(format_args!("field_pattern"))
            }
            Self::FullStopfullStopfullStopToken => {
                f.write_fmt(format_args!("..."))
            }
            Self::TtToken => { f.write_fmt(format_args!("tt")) }
            Self::WhileToken => { f.write_fmt(format_args!("while")) }
            Self::IfToken => { f.write_fmt(format_args!("if")) }
            Self::MoveToken => { f.write_fmt(format_args!("move")) }
            Self::FunctionModifiers => {
                f.write_fmt(format_args!("function_modifiers"))
            }
            Self::CircumflexAccentToken => { f.write_fmt(format_args!("^")) }
            Self::MatchToken => { f.write_fmt(format_args!("match")) }
            Self::FullStopfullStopequalsSignToken => {
                f.write_fmt(format_args!("..="))
            }
            Self::DollarSignToken => { f.write_fmt(format_args!("$")) }
            Self::StmtToken => { f.write_fmt(format_args!("stmt")) }
            Self::TraitToken => { f.write_fmt(format_args!("trait")) }
            Self::AsteriskToken => { f.write_fmt(format_args!("*")) }
            Self::FullStopfullStopToken => { f.write_fmt(format_args!("..")) }
            Self::FieldInitializerList => {
                f.write_fmt(format_args!("field_initializer_list"))
            }
            Self::SolidusasteriskToken => { f.write_fmt(format_args!("/*")) }
            Self::GreaterThanSigngreaterThanSignToken => {
                f.write_fmt(format_args!(">>"))
            }
            Self::FragmentSpecifier => {
                f.write_fmt(format_args!("fragment_specifier"))
            }
            Self::InToken => { f.write_fmt(format_args!("in")) }
            Self::MutableSpecifier => {
                f.write_fmt(format_args!("mutable_specifier"))
            }
            Self::AmpersandampersandToken => {
                f.write_fmt(format_args!("&&"))
            }
            Self::QualifiedType => {
                f.write_fmt(format_args!("qualified_type"))
            }
            Self::ColonToken => { f.write_fmt(format_args!(":")) }
            Self::YieldToken => { f.write_fmt(format_args!("yield")) }
            Self::MatchArm => { f.write_fmt(format_args!("match_arm")) }
            Self::StaticToken => { f.write_fmt(format_args!("static")) }
            Self::GenericTypeWithTurbofish => {
                f.write_fmt(format_args!("generic_type_with_turbofish"))
            }
            Self::TrueToken => { f.write_fmt(format_args!("true")) }
            Self::CircumflexAccentequalsSignToken => {
                f.write_fmt(format_args!("^="))
            }
            Self::Expr2021Token => { f.write_fmt(format_args!("expr_2021")) }
            Self::Attribute => { f.write_fmt(format_args!("attribute")) }
            Self::TypeArguments => {
                f.write_fmt(format_args!("type_arguments"))
            }
            Self::QuestionMarkToken => { f.write_fmt(format_args!("?")) }
            Self::LetToken => { f.write_fmt(format_args!("let")) }
            Self::TokenBindingPattern => {
                f.write_fmt(format_args!("token_binding_pattern"))
            }
            Self::WhereToken => { f.write_fmt(format_args!("where")) }
            Self::ElseToken => { f.write_fmt(format_args!("else")) }
            Self::IdentToken => { f.write_fmt(format_args!("ident")) }
            Self::EqualsSigngreaterThanSignToken => {
                f.write_fmt(format_args!("=>"))
            }
            Self::VerticalLineverticalLineToken => {
                f.write_fmt(format_args!("||"))
            }
            Self::FieldDeclaration => {
                f.write_fmt(format_args!("field_declaration"))
            }
            Self::Parameters => { f.write_fmt(format_args!("parameters")) }
            Self::WherePredicate => {
                f.write_fmt(format_args!("where_predicate"))
            }
            Self::GreaterThanSignequalsSignToken => {
                f.write_fmt(format_args!(">="))
            }
            Self::TypeParameter => {
                f.write_fmt(format_args!("type_parameter"))
            }
            Self::TypeBinding => { f.write_fmt(format_args!("type_binding")) }
            Self::PatToken => { f.write_fmt(format_args!("pat")) }
            Self::ScopedUseList => {
                f.write_fmt(format_args!("scoped_use_list"))
            }
            Self::MatchBlock => { f.write_fmt(format_args!("match_block")) }
            Self::ContinueToken => { f.write_fmt(format_args!("continue")) }
            Self::Super => { f.write_fmt(format_args!("super")) }
            Self::ShorthandFieldInitializer => {
                f.write_fmt(format_args!("shorthand_field_initializer"))
            }
            Self::GenToken => { f.write_fmt(format_args!("gen")) }
            Self::PlusSignToken => { f.write_fmt(format_args!("+")) }
            Self::HyphenMinusequalsSignToken => {
                f.write_fmt(format_args!("-="))
            }
            Self::LifetimeParameter => {
                f.write_fmt(format_args!("lifetime_parameter"))
            }
            Self::DynToken => { f.write_fmt(format_args!("dyn")) }
            Self::Lifetime => { f.write_fmt(format_args!("lifetime")) }
            Self::LetCondition => {
                f.write_fmt(format_args!("let_condition"))
            }
            Self::LessThanSignlessThanSignequalsSignToken => {
                f.write_fmt(format_args!("<<="))
            }
            Self::TokenRepetition => {
                f.write_fmt(format_args!("token_repetition"))
            }
            Self::LineComment => { f.write_fmt(format_args!("line_comment")) }
            Self::ExternModifier => {
                f.write_fmt(format_args!("extern_modifier"))
            }
            Self::TypeToken => { f.write_fmt(format_args!("type")) }
            Self::UseWildcard => { f.write_fmt(format_args!("use_wildcard")) }
            Self::_Expression(_Expression::_Literal) => {
                f.write_fmt(format_args!("_literal"))
            }
            Self::_Expression(_Expression::ArrayExpression) => {
                f.write_fmt(format_args!("array_expression"))
            }
            Self::_Expression(_Expression::AssignmentExpression) => {
                f.write_fmt(format_args!("assignment_expression"))
            }
            Self::_Expression(_Expression::AsyncBlock) => {
                f.write_fmt(format_args!("async_block"))
            }
            Self::_Expression(_Expression::AwaitExpression) => {
                f.write_fmt(format_args!("await_expression"))
            }
            Self::_Expression(_Expression::BinaryExpression) => {
                f.write_fmt(format_args!("binary_expression"))
            }
            Self::_Expression(_Expression::Block) => {
                f.write_fmt(format_args!("block"))
            }
            Self::_Expression(_Expression::BreakExpression) => {
                f.write_fmt(format_args!("break_expression"))
            }
            Self::_Expression(_Expression::CallExpression) => {
                f.write_fmt(format_args!("call_expression"))
            }
            Self::_Expression(_Expression::ClosureExpression) => {
                f.write_fmt(format_args!("closure_expression"))
            }
            Self::_Expression(_Expression::CompoundAssignmentExpr) => {
                f.write_fmt(format_args!("compound_assignment_expr"))
            }
            Self::_Expression(_Expression::ConstBlock) => {
                f.write_fmt(format_args!("const_block"))
            }
            Self::_Expression(_Expression::ContinueExpression) => {
                f.write_fmt(format_args!("continue_expression"))
            }
            Self::_Expression(_Expression::FieldExpression) => {
                f.write_fmt(format_args!("field_expression"))
            }
            Self::_Expression(_Expression::ForExpression) => {
                f.write_fmt(format_args!("for_expression"))
            }
            Self::_Expression(_Expression::GenBlock) => {
                f.write_fmt(format_args!("gen_block"))
            }
            Self::_Expression(_Expression::GenericFunction) => {
                f.write_fmt(format_args!("generic_function"))
            }
            Self::_Expression(_Expression::Identifier) => {
                f.write_fmt(format_args!("identifier"))
            }
            Self::_Expression(_Expression::IfExpression) => {
                f.write_fmt(format_args!("if_expression"))
            }
            Self::_Expression(_Expression::IndexExpression) => {
                f.write_fmt(format_args!("index_expression"))
            }
            Self::_Expression(_Expression::LoopExpression) => {
                f.write_fmt(format_args!("loop_expression"))
            }
            Self::_Expression(_Expression::MatchExpression) => {
                f.write_fmt(format_args!("match_expression"))
            }
            Self::_Expression(_Expression::Metavariable) => {
                f.write_fmt(format_args!("metavariable"))
            }
            Self::_Expression(_Expression::ParenthesizedExpression) => {
                f.write_fmt(format_args!("parenthesized_expression"))
            }
            Self::_Expression(_Expression::RangeExpression) => {
                f.write_fmt(format_args!("range_expression"))
            }
            Self::_Expression(_Expression::ReferenceExpression) => {
                f.write_fmt(format_args!("reference_expression"))
            }
            Self::_Expression(_Expression::ReturnExpression) => {
                f.write_fmt(format_args!("return_expression"))
            }
            Self::_Expression(_Expression::ScopedIdentifier) => {
                f.write_fmt(format_args!("scoped_identifier"))
            }
            Self::_Expression(_Expression::SelfToken) => {
                f.write_fmt(format_args!("self"))
            }
            Self::_Expression(_Expression::StructExpression) => {
                f.write_fmt(format_args!("struct_expression"))
            }
            Self::_Expression(_Expression::TryBlock) => {
                f.write_fmt(format_args!("try_block"))
            }
            Self::_Expression(_Expression::TryExpression) => {
                f.write_fmt(format_args!("try_expression"))
            }
            Self::_Expression(_Expression::TupleExpression) => {
                f.write_fmt(format_args!("tuple_expression"))
            }
            Self::_Expression(_Expression::TypeCastExpression) => {
                f.write_fmt(format_args!("type_cast_expression"))
            }
            Self::_Expression(_Expression::UnaryExpression) => {
                f.write_fmt(format_args!("unary_expression"))
            }
            Self::_Expression(_Expression::UnitExpression) => {
                f.write_fmt(format_args!("unit_expression"))
            }
            Self::_Expression(_Expression::UnsafeBlock) => {
                f.write_fmt(format_args!("unsafe_block"))
            }
            Self::_Expression(_Expression::WhileExpression) => {
                f.write_fmt(format_args!("while_expression"))
            }
            Self::_Expression(_Expression::YieldExpression) => {
                f.write_fmt(format_args!("yield_expression"))
            }
            Self::SourceFile => { f.write_fmt(format_args!("source_file")) }
            Self::Crate => { f.write_fmt(format_args!("crate")) }
            Self::ModToken => { f.write_fmt(format_args!("mod")) }
            Self::_DeclarationStatement(_DeclarationStatement::AssociatedType)
                => {
                f.write_fmt(format_args!("associated_type"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::AttributeItem)
                => {
                f.write_fmt(format_args!("attribute_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::ConstItem) => {
                f.write_fmt(format_args!("const_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::EmptyStatement)
                => {
                f.write_fmt(format_args!("empty_statement"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::EnumItem) => {
                f.write_fmt(format_args!("enum_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::ExternCrateDeclaration)
                => {
                f.write_fmt(format_args!("extern_crate_declaration"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::ForeignModItem)
                => {
                f.write_fmt(format_args!("foreign_mod_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::FunctionItem)
                => {
                f.write_fmt(format_args!("function_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::FunctionSignatureItem)
                => {
                f.write_fmt(format_args!("function_signature_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::ImplItem) => {
                f.write_fmt(format_args!("impl_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::InnerAttributeItem)
                => {
                f.write_fmt(format_args!("inner_attribute_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::LetDeclaration)
                => {
                f.write_fmt(format_args!("let_declaration"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::MacroDefinition)
                => {
                f.write_fmt(format_args!("macro_definition"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::MacroInvocation)
                => {
                f.write_fmt(format_args!("macro_invocation"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::ModItem) => {
                f.write_fmt(format_args!("mod_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::StaticItem) =>
                {
                f.write_fmt(format_args!("static_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::StructItem) =>
                {
                f.write_fmt(format_args!("struct_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::TraitItem) => {
                f.write_fmt(format_args!("trait_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::TypeItem) => {
                f.write_fmt(format_args!("type_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::UnionItem) => {
                f.write_fmt(format_args!("union_item"))
            }
            Self::_DeclarationStatement(_DeclarationStatement::UseDeclaration)
                => {
                f.write_fmt(format_args!("use_declaration"))
            }
            Self::EqualsSignequalsSignToken => {
                f.write_fmt(format_args!("=="))
            }
            Self::DeclarationList => {
                f.write_fmt(format_args!("declaration_list"))
            }
            Self::CommaToken => { f.write_fmt(format_args!(",")) }
            Self::MacroRulesExclamationMarkToken => {
                f.write_fmt(format_args!("macro_rules!"))
            }
            Self::EqualsSignToken => { f.write_fmt(format_args!("=")) }
            Self::BracketedType => {
                f.write_fmt(format_args!("bracketed_type"))
            }
            Self::PatParamToken => { f.write_fmt(format_args!("pat_param")) }
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
