#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use tss_macros::generate_nodes;

struct DynamicType {}
struct NeverType;
enum FunctionItemChild {
    FunctionModifiers(FunctionModifiers),
    VisibilityModifier(VisibilityModifier),
    WhereClause(WhereClause),
}
struct ImplItem {
    type_parameters: std::option::Option<TypeParameters>,
}
struct UnitType;
struct TokenTree {
    children: std::option::Option<std::vec::Vec<TokenTreeChild>>,
}
enum UnionItemChild {
    VisibilityModifier(VisibilityModifier),
    WhereClause(WhereClause),
}
enum StructItemChild {
    VisibilityModifier(VisibilityModifier),
    WhereClause(WhereClause),
}
struct MacroDefinition {
    name: Identifier,
    children: std::option::Option<std::vec::Vec<MacroRule>>,
}
enum MatchPatterncondition {
    _Expression(_Expression),
    LetChain(LetChain),
    LetCondition(LetCondition),
}
struct TupleStructPattern {}
struct RightSquareBracketToken;
struct LeftSquareBracketToken;
struct ForLifetimes {
    children: std::vec::Vec<Lifetime>,
}
enum ScopedTypeIdentifierChild {
    BracketedType(BracketedType),
    Crate(Crate),
    GenericType(GenericType),
    Identifier(Identifier),
    Metavariable(Metavariable),
    ScopedIdentifier(ScopedIdentifier),
    SelfToken(SelfToken),
    Super(Super),
}
struct AbstractType {}
struct SelfParameter {
    children: std::vec::Vec<SelfParameterChild>,
}
struct EnumVariant {
    name: Identifier,
    value: std::option::Option<_Expression>,
    body: std::option::Option<EnumVariantbody>,
    maybe_visibility_modifier: std::option::Option<VisibilityModifier>,
}
struct FnToken;
enum UseBoundsChild { Lifetime(Lifetime), TypeIdentifier(TypeIdentifier), }
struct StmtToken;
struct ReturnExpression {
    maybe__expression: std::option::Option<_Expression>,
}
struct GenBlock {
    block: Block,
}
struct BreakToken;
struct AssociatedType {
    name: TypeIdentifier,
    type_parameters: std::option::Option<TypeParameters>,
    bounds: std::option::Option<TraitBounds>,
    maybe_where_clause: std::option::Option<WhereClause>,
}
struct TryExpression {
    _expression: _Expression,
}
struct EscapeSequence;
enum CompoundAssignmentExprChild {
    PercentSignequalsSignToken(PercentSignequalsSignToken),
    AmpersandequalsSignToken(AmpersandequalsSignToken),
    AsteriskequalsSignToken(AsteriskequalsSignToken),
    PlusSignequalsSignToken(PlusSignequalsSignToken),
    HyphenMinusequalsSignToken(HyphenMinusequalsSignToken),
    SolidusequalsSignToken(SolidusequalsSignToken),
    LessThanSignlessThanSignequalsSignToken(LessThanSignlessThanSignequalsSignToken),
    GreaterThanSigngreaterThanSignequalsSignToken(GreaterThanSigngreaterThanSignequalsSignToken),
    CircumflexAccentequalsSignToken(CircumflexAccentequalsSignToken),
    VerticalLineequalsSignToken(VerticalLineequalsSignToken),
}
struct TokenBindingPattern {}
struct ColonToken;
enum FunctionSignatureItemname {
    Identifier(Identifier),
    Metavariable(Metavariable),
}
enum DynamicTypeChild {
    FunctionType(FunctionType),
    GenericType(GenericType),
    HigherRankedTraitBound(HigherRankedTraitBound),
    ScopedTypeIdentifier(ScopedTypeIdentifier),
    TupleType(TupleType),
    TypeIdentifier(TypeIdentifier),
}
struct InnerAttributeItem {
    attribute: Attribute,
}
enum SourceFileChild {
    _DeclarationStatement(_DeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Shebang(Shebang),
}
struct AsterisksolidusToken;
struct LiteralToken;
struct Block {
    children: std::option::Option<std::vec::Vec<BlockChild>>,
}
enum EnumVariantbody {
    FieldDeclarationList(FieldDeclarationList),
    OrderedFieldDeclarationList(OrderedFieldDeclarationList),
}
struct FieldInitializer {
    value: _Expression,
    field: FieldInitializerfield,
    children: std::option::Option<std::vec::Vec<AttributeItem>>,
}
struct ExternCrateDeclaration {
    name: Identifier,
    alias: std::option::Option<Identifier>,
    children: std::vec::Vec<ExternCrateDeclarationChild>,
}
struct PatToken;
enum FunctionTypeChild {
    ForLifetimes(ForLifetimes),
    FunctionModifiers(FunctionModifiers),
}
struct UseBounds {
    children: std::option::Option<std::vec::Vec<UseBoundsChild>>,
}
struct SolidusToken;
struct MatchBlock {
    children: std::option::Option<std::vec::Vec<MatchArm>>,
}
struct TypeIdentifier;
struct ApostropheToken;
struct VisToken;
struct ElseToken;
enum EnumItemChild {
    VisibilityModifier(VisibilityModifier),
    WhereClause(WhereClause),
}
struct TypeParameter {
    bounds: std::option::Option<TraitBounds>,
    default_type: std::option::Option<_Type>,
    name: TypeIdentifier,
}
enum TokenRepetitionPatternChild {
    _Literal(_Literal),
    Crate(Crate),
    Identifier(Identifier),
    Metavariable(Metavariable),
    MutableSpecifier(MutableSpecifier),
    PrimitiveType(PrimitiveType),
    SelfToken(SelfToken),
    Super(Super),
    TokenBindingPattern(TokenBindingPattern),
    TokenRepetitionPattern(TokenRepetitionPattern),
    TokenTreePattern(TokenTreePattern),
}
struct FieldInitializerList {
    children: std::option::Option<std::vec::Vec<FieldInitializerListChild>>,
}
struct EnumToken;
struct ForToken;
struct AsteriskequalsSignToken;
struct MatchExpression {
    value: _Expression,
    body: MatchBlock,
}
struct ContinueExpression {
    maybe_label: std::option::Option<Label>,
}
struct UnsafeBlock {
    block: Block,
}
struct BracketedType {
    children: BracketedTypeChild,
}
enum ReferencePatternChild {
    _Pattern(_Pattern),
    MutableSpecifier(MutableSpecifier),
}
struct ImplToken;
struct CommercialAtToken;
struct LetChain {
    children: std::vec::Vec<LetChainChild>,
}
enum StructItembody {
    FieldDeclarationList(FieldDeclarationList),
    OrderedFieldDeclarationList(OrderedFieldDeclarationList),
}
struct AsteriskToken;
enum TraitBoundsChild {
    _Type(_Type),
    HigherRankedTraitBound(HigherRankedTraitBound),
    Lifetime(Lifetime),
}
struct BooleanLiteral;
struct IntegerLiteral;
enum TokenTreeChild {
    _Literal(_Literal),
    Crate(Crate),
    Identifier(Identifier),
    Metavariable(Metavariable),
    MutableSpecifier(MutableSpecifier),
    PrimitiveType(PrimitiveType),
    SelfToken(SelfToken),
    Super(Super),
    TokenRepetition(TokenRepetition),
    TokenTree(TokenTree),
}
struct LetToken;
struct UseToken;
struct TyToken;
enum FieldPatternname {
    FieldIdentifier(FieldIdentifier),
    ShorthandFieldIdentifier(ShorthandFieldIdentifier),
}
struct OrPattern {
    children: std::vec::Vec<_Pattern>,
}
struct ParenthesizedExpression {
    _expression: _Expression,
}
struct UseAsClause {
    path: UseAsClauseChild,
    alias: Identifier,
}
struct QuotationMarkToken;
struct AmpersandequalsSignToken;
struct ReferencePattern {
    children: std::vec::Vec<ReferencePatternChild>,
}
struct FieldPattern {
    name: FieldPatternname,
    pattern: std::option::Option<_Pattern>,
    maybe_mutable_specifier: std::option::Option<MutableSpecifier>,
}
struct RefPattern {
    _pattern: _Pattern,
}
struct TypeToken;
struct SourceFile {
    children: std::option::Option<std::vec::Vec<SourceFileChild>>,
}
enum TypeParametersChild {
    AttributeItem(AttributeItem),
    ConstParameter(ConstParameter),
    LifetimeParameter(LifetimeParameter),
    Metavariable(Metavariable),
    TypeParameter(TypeParameter),
}
struct WhileToken;
struct GenericPattern {
    type_arguments: TypeArguments,
    children: GenericPatternChild,
}
struct LoopExpression {
    body: Block,
    maybe_label: std::option::Option<Label>,
}
struct TryToken;
struct RemainingFieldPattern;
enum ExternCrateDeclarationChild {
    Crate(Crate),
    VisibilityModifier(VisibilityModifier),
}
struct RightParenthesisToken;
struct IfToken;
struct LoopToken;
struct AwaitToken;
struct ScopedIdentifier {
    name: ScopedIdentifierChild,
    path: std::option::Option<ScopedIdentifierChild>,
}
struct ConstItem {
    name: Identifier,
    value: std::option::Option<_Expression>,
}
struct TuplePattern {
    children: std::option::Option<std::vec::Vec<TuplePatternChild>>,
}
struct ExpressionStatement {
    _expression: _Expression,
}
struct CircumflexAccentequalsSignToken;
struct TraitItem {
    body: DeclarationList,
    type_parameters: std::option::Option<TypeParameters>,
    bounds: std::option::Option<TraitBounds>,
    name: TypeIdentifier,
    children: std::option::Option<std::vec::Vec<TraitItemChild>>,
}
enum UseListChild {
    Crate(Crate),
    Identifier(Identifier),
    Metavariable(Metavariable),
    ScopedIdentifier(ScopedIdentifier),
    ScopedUseList(ScopedUseList),
    SelfToken(SelfToken),
    Super(Super),
    UseAsClause(UseAsClause),
    UseList(UseList),
    UseWildcard(UseWildcard),
}
struct Shebang;
struct StructExpression {
    body: FieldInitializerList,
    name: StructExpressionChild,
}
struct RawStringLiteral {
    string_content: StringContent,
}
struct SolidusequalsSignToken;
enum ScopedUseListChild {
    Crate(Crate),
    Identifier(Identifier),
    Metavariable(Metavariable),
    ScopedIdentifier(ScopedIdentifier),
    SelfToken(SelfToken),
    Super(Super),
}
struct ReferenceExpression {
    value: _Expression,
    maybe_mutable_specifier: std::option::Option<MutableSpecifier>,
}
struct TypeParameters {
    children: std::vec::Vec<TypeParametersChild>,
}
struct FieldDeclaration {}
struct GreaterThanSigngreaterThanSignequalsSignToken;
enum SelfParameterChild {
    Lifetime(Lifetime),
    MutableSpecifier(MutableSpecifier),
    SelfToken(SelfToken),
}
enum LetChainChild { _Expression(_Expression), LetCondition(LetCondition), }
struct BinaryExpression {
    left: _Expression,
    operator: BinaryExpressionChild,
    right: _Expression,
}
struct ClosureParameters {
    children: std::option::Option<std::vec::Vec<ClosureParametersChild>>,
}
struct DeclarationList {
    children: std::option::Option<std::vec::Vec<_DeclarationStatement>>,
}
struct ColoncolonToken;
struct ExclamationMarkequalsSignToken;
struct ExclamationMarkToken;
struct FullStopfullStopequalsSignToken;
enum _Type {
    AbstractType(AbstractType),
    ArrayType(ArrayType),
    BoundedType(BoundedType),
    DynamicType(DynamicType),
    FunctionType(FunctionType),
    GenericType(GenericType),
    MacroInvocation(MacroInvocation),
    Metavariable(Metavariable),
    NeverType(NeverType),
    PointerType(PointerType),
    PrimitiveType(PrimitiveType),
    ReferenceType(ReferenceType),
    RemovedTraitBound(RemovedTraitBound),
    ScopedTypeIdentifier(ScopedTypeIdentifier),
    TupleType(TupleType),
    TypeIdentifier(TypeIdentifier),
    UnitType(UnitType),
}
struct PercentSignequalsSignToken;
struct ContinueToken;
enum BlockChild {
    _DeclarationStatement(_DeclarationStatement),
    _Expression(_Expression),
    ExpressionStatement(ExpressionStatement),
    Label(Label),
}
struct UnaryExpression {
    _expression: _Expression,
}
enum StructPatternChild {
    FieldPattern(FieldPattern),
    RemainingFieldPattern(RemainingFieldPattern),
}
enum GenericTypeWithTurbofishChild {
    ScopedIdentifier(ScopedIdentifier),
    TypeIdentifier(TypeIdentifier),
}
struct PlusSignequalsSignToken;
struct PointerType {}
struct DollarSignToken;
enum GenericFunctionChild {
    FieldExpression(FieldExpression),
    Identifier(Identifier),
    ScopedIdentifier(ScopedIdentifier),
}
struct UseWildcard {
    children: std::option::Option<UseWildcardChild>,
}
struct MetaToken;
struct WhereToken;
struct UnsafeToken;
enum TokenTreePatternChild {
    _Literal(_Literal),
    Crate(Crate),
    Identifier(Identifier),
    Metavariable(Metavariable),
    MutableSpecifier(MutableSpecifier),
    PrimitiveType(PrimitiveType),
    SelfToken(SelfToken),
    Super(Super),
    TokenBindingPattern(TokenBindingPattern),
    TokenRepetitionPattern(TokenRepetitionPattern),
    TokenTreePattern(TokenTreePattern),
}
struct PathToken;
struct MutPattern {
    children: std::vec::Vec<MutPatternChild>,
}
enum StringLiteralChild {
    EscapeSequence(EscapeSequence),
    StringContent(StringContent),
}
enum WhileExpressioncondition {
    _Expression(_Expression),
    LetChain(LetChain),
    LetCondition(LetCondition),
}
struct BlockToken;
struct EmptyStatement;
struct GenericType {
    type_arguments: TypeArguments,
}
struct Lifetime {
    identifier: Identifier,
}
struct RightCurlyBracketToken;
struct StructItem {
    name: TypeIdentifier,
    type_parameters: std::option::Option<TypeParameters>,
    body: std::option::Option<StructItembody>,
    children: std::option::Option<std::vec::Vec<StructItemChild>>,
}
enum CallExpressionChild {
    _Literal(_Literal),
    ArrayExpression(ArrayExpression),
    AssignmentExpression(AssignmentExpression),
    AsyncBlock(AsyncBlock),
    AwaitExpression(AwaitExpression),
    BinaryExpression(BinaryExpression),
    Block(Block),
    BreakExpression(BreakExpression),
    CallExpression(CallExpression),
    ClosureExpression(ClosureExpression),
    CompoundAssignmentExpr(CompoundAssignmentExpr),
    ConstBlock(ConstBlock),
    ContinueExpression(ContinueExpression),
    FieldExpression(FieldExpression),
    ForExpression(ForExpression),
    GenBlock(GenBlock),
    GenericFunction(GenericFunction),
    Identifier(Identifier),
    IfExpression(IfExpression),
    IndexExpression(IndexExpression),
    LoopExpression(LoopExpression),
    MacroInvocation(MacroInvocation),
    MatchExpression(MatchExpression),
    Metavariable(Metavariable),
    ParenthesizedExpression(ParenthesizedExpression),
    ReferenceExpression(ReferenceExpression),
    ReturnExpression(ReturnExpression),
    ScopedIdentifier(ScopedIdentifier),
    SelfToken(SelfToken),
    StructExpression(StructExpression),
    TryBlock(TryBlock),
    TryExpression(TryExpression),
    TupleExpression(TupleExpression),
    TypeCastExpression(TypeCastExpression),
    UnaryExpression(UnaryExpression),
    UnitExpression(UnitExpression),
    UnsafeBlock(UnsafeBlock),
    WhileExpression(WhileExpression),
    YieldExpression(YieldExpression),
}
struct FunctionItem {
    type_parameters: std::option::Option<TypeParameters>,
    parameters: Parameters,
    body: Block,
    name: FunctionItemname,
    return_type: std::option::Option<_Type>,
    children: std::option::Option<std::vec::Vec<FunctionItemChild>>,
}
enum TypeItemChild {
    VisibilityModifier(VisibilityModifier),
    WhereClause(WhereClause),
}
enum _Expression {
    _Literal(_Literal),
    ArrayExpression(ArrayExpression),
    AssignmentExpression(AssignmentExpression),
    AsyncBlock(AsyncBlock),
    AwaitExpression(AwaitExpression),
    BinaryExpression(BinaryExpression),
    Block(Block),
    BreakExpression(BreakExpression),
    CallExpression(CallExpression),
    ClosureExpression(ClosureExpression),
    CompoundAssignmentExpr(CompoundAssignmentExpr),
    ConstBlock(ConstBlock),
    ContinueExpression(ContinueExpression),
    FieldExpression(FieldExpression),
    ForExpression(ForExpression),
    GenBlock(GenBlock),
    GenericFunction(GenericFunction),
    Identifier(Identifier),
    IfExpression(IfExpression),
    IndexExpression(IndexExpression),
    LoopExpression(LoopExpression),
    MacroInvocation(MacroInvocation),
    MatchExpression(MatchExpression),
    Metavariable(Metavariable),
    ParenthesizedExpression(ParenthesizedExpression),
    RangeExpression(RangeExpression),
    ReferenceExpression(ReferenceExpression),
    ReturnExpression(ReturnExpression),
    ScopedIdentifier(ScopedIdentifier),
    SelfToken(SelfToken),
    StructExpression(StructExpression),
    TryBlock(TryBlock),
    TryExpression(TryExpression),
    TupleExpression(TupleExpression),
    TypeCastExpression(TypeCastExpression),
    UnaryExpression(UnaryExpression),
    UnitExpression(UnitExpression),
    UnsafeBlock(UnsafeBlock),
    WhileExpression(WhileExpression),
    YieldExpression(YieldExpression),
}
struct AmpersandampersandToken;
struct FunctionModifiers {
    children: std::option::Option<std::vec::Vec<ExternModifier>>,
}
struct BoundedType {
    children: std::vec::Vec<BoundedTypeChild>,
}
struct IdentToken;
struct TupleExpression {
    children: std::vec::Vec<TupleExpressionChild>,
}
struct CompoundAssignmentExpr {
    right: _Expression,
    left: _Expression,
    operator: CompoundAssignmentExprChild,
}
struct BlockComment {
    inner: std::option::Option<InnerDocCommentMarker>,
    doc: std::option::Option<DocComment>,
    outer: std::option::Option<OuterDocCommentMarker>,
}
enum AttributeChild {
    Crate(Crate),
    Identifier(Identifier),
    Metavariable(Metavariable),
    ScopedIdentifier(ScopedIdentifier),
    SelfToken(SelfToken),
    Super(Super),
}
struct SelfToken;
struct MacroRule {
    left: TokenTreePattern,
    right: TokenTree,
}
struct SlicePattern {
    children: std::option::Option<std::vec::Vec<_Pattern>>,
}
struct FullStopToken;
struct RawToken;
enum GenericPatternChild {
    Identifier(Identifier),
    ScopedIdentifier(ScopedIdentifier),
}
struct UnionItem {
    body: FieldDeclarationList,
    name: TypeIdentifier,
    type_parameters: std::option::Option<TypeParameters>,
    children: std::option::Option<std::vec::Vec<UnionItemChild>>,
}
enum VisibilityModifierChild {
    Crate(Crate),
    Identifier(Identifier),
    Metavariable(Metavariable),
    ScopedIdentifier(ScopedIdentifier),
    SelfToken(SelfToken),
    Super(Super),
}
struct WhereClause {
    children: std::option::Option<std::vec::Vec<WherePredicate>>,
}
struct FalseToken;
struct ExternToken;
struct UnionToken;
struct AsToken;
struct SolidussolidusToken;
struct MatchToken;
struct CallExpression {
    function: CallExpressionChild,
    arguments: Arguments,
}
struct VerticalLineverticalLineToken;
struct VerticalLineToken;
struct UseList {
    children: std::option::Option<std::vec::Vec<UseListChild>>,
}
struct ForExpression {
    value: _Expression,
    body: Block,
    pattern: _Pattern,
    maybe_label: std::option::Option<Label>,
}
struct VariadicParameter {
    pattern: std::option::Option<_Pattern>,
    maybe_mutable_specifier: std::option::Option<MutableSpecifier>,
}
enum _Pattern {
    _Token(_Token),
    _LiteralPattern(_LiteralPattern),
    CapturedPattern(CapturedPattern),
    ConstBlock(ConstBlock),
    GenericPattern(GenericPattern),
    Identifier(Identifier),
    MacroInvocation(MacroInvocation),
    MutPattern(MutPattern),
    OrPattern(OrPattern),
    RangePattern(RangePattern),
    RefPattern(RefPattern),
    ReferencePattern(ReferencePattern),
    RemainingFieldPattern(RemainingFieldPattern),
    ScopedIdentifier(ScopedIdentifier),
    SlicePattern(SlicePattern),
    StructPattern(StructPattern),
    TuplePattern(TuplePattern),
    TupleStructPattern(TupleStructPattern),
}
enum StructPatterntype {
    ScopedTypeIdentifier(ScopedTypeIdentifier),
    TypeIdentifier(TypeIdentifier),
}
struct TrueToken;
enum IfExpressionChild {
    _Expression(_Expression),
    LetChain(LetChain),
    LetCondition(LetCondition),
}
struct ForeignModItem {
    body: std::option::Option<DeclarationList>,
    extern_modifier: ExternModifier,
}
struct LeftCurlyBracketToken;
struct MacroInvocation {}
struct ExternModifier {
    maybe_string_literal: std::option::Option<StringLiteral>,
}
struct ReturnToken;
struct CapturedPattern {
    children: std::vec::Vec<_Pattern>,
}
struct FieldDeclarationList {
    children: std::option::Option<std::vec::Vec<FieldDeclarationListChild>>,
}
enum ClosureExpressionChild { _Token(_Token), _Expression(_Expression), }
enum ParametersChild {
    _Type(_Type),
    AttributeItem(AttributeItem),
    Parameter(Parameter),
    SelfParameter(SelfParameter),
    VariadicParameter(VariadicParameter),
}
struct Expr2021Token;
struct CommaToken;
struct Metavariable;
struct GenToken;
struct MutableSpecifier;
struct AssignmentExpression {
    left: _Expression,
    right: _Expression,
}
struct SemicolonToken;
struct HyphenMinusequalsSignToken;
struct ScopedTypeIdentifier {
    name: TypeIdentifier,
    path: std::option::Option<ScopedTypeIdentifierChild>,
}
struct IfExpression {
    condition: IfExpressionChild,
    alternative: std::option::Option<ElseClause>,
    consequence: Block,
}
struct FunctionSignatureItem {
    name: FunctionSignatureItemname,
    return_type: std::option::Option<_Type>,
    type_parameters: std::option::Option<TypeParameters>,
    parameters: Parameters,
    children: std::option::Option<std::vec::Vec<FunctionSignatureItemChild>>,
}
struct AwaitExpression {
    _expression: _Expression,
}
enum FunctionItemname { Identifier(Identifier), Metavariable(Metavariable), }
struct UnitExpression;
struct PlusSignToken;
enum FieldExpressionChild {
    FieldIdentifier(FieldIdentifier),
    IntegerLiteral(IntegerLiteral),
}
enum BracketedTypeChild { _Type(_Type), QualifiedType(QualifiedType), }
struct TypeItem {
    type_parameters: std::option::Option<TypeParameters>,
    name: TypeIdentifier,
}
enum FieldInitializerListChild {
    BaseFieldInitializer(BaseFieldInitializer),
    FieldInitializer(FieldInitializer),
    ShorthandFieldInitializer(ShorthandFieldInitializer),
}
struct ReferenceType {}
struct MatchArm {
    pattern: MatchPattern,
    value: _Expression,
    children: std::option::Option<std::vec::Vec<MatchArmChild>>,
}
enum ArrayExpressionChild {
    _Expression(_Expression),
    AttributeItem(AttributeItem),
}
struct TokenRepetition {
    children: std::option::Option<std::vec::Vec<TokenRepetitionChild>>,
}
struct Super;
struct GreaterThanSigngreaterThanSignToken;
struct FunctionType {
    parameters: Parameters,
    return_type: std::option::Option<_Type>,
}
struct VerticalLineequalsSignToken;
enum UseWildcardChild {
    Crate(Crate),
    Identifier(Identifier),
    Metavariable(Metavariable),
    ScopedIdentifier(ScopedIdentifier),
    SelfToken(SelfToken),
    Super(Super),
}
struct HigherRankedTraitBound {}
enum ClosureParametersChild { _Pattern(_Pattern), Parameter(Parameter), }
struct EnumItem {
    name: TypeIdentifier,
    type_parameters: std::option::Option<TypeParameters>,
    body: EnumVariantList,
    children: std::option::Option<std::vec::Vec<EnumItemChild>>,
}
struct FloatLiteral;
struct Label {
    identifier: Identifier,
}
struct TokenRepetitionPattern {
    children: std::option::Option<std::vec::Vec<TokenRepetitionPatternChild>>,
}
struct OrderedFieldDeclarationList {}
struct NumberSignToken;
struct BreakExpression {
    children: std::option::Option<std::vec::Vec<BreakExpressionChild>>,
}
enum UseDeclarationargument {
    Crate(Crate),
    Identifier(Identifier),
    Metavariable(Metavariable),
    ScopedIdentifier(ScopedIdentifier),
    ScopedUseList(ScopedUseList),
    SelfToken(SelfToken),
    Super(Super),
    UseAsClause(UseAsClause),
    UseList(UseList),
    UseWildcard(UseWildcard),
}
enum TupleExpressionChild {
    _Expression(_Expression),
    AttributeItem(AttributeItem),
}
enum StructExpressionChild {
    GenericTypeWithTurbofish(GenericTypeWithTurbofish),
    ScopedTypeIdentifier(ScopedTypeIdentifier),
    TypeIdentifier(TypeIdentifier),
}
enum ScopedIdentifierChild { Identifier(Identifier), Super(Super), }
struct _Token;
enum EnumVariantListChild {
    AttributeItem(AttributeItem),
    EnumVariant(EnumVariant),
}
struct QualifiedType {}
enum MatchArmChild {
    AttributeItem(AttributeItem),
    InnerAttributeItem(InnerAttributeItem),
}
struct ClosureExpression {
    parameters: ClosureParameters,
    return_type: std::option::Option<_Type>,
    body: ClosureExpressionChild,
}
enum GenericTypeChild {
    Identifier(Identifier),
    ScopedIdentifier(ScopedIdentifier),
    ScopedTypeIdentifier(ScopedTypeIdentifier),
    TypeIdentifier(TypeIdentifier),
}
struct Parameters {
    children: std::option::Option<std::vec::Vec<ParametersChild>>,
}
struct AttributeItem {
    attribute: Attribute,
}
struct RemovedTraitBound {
    _type: _Type,
}
enum TraitItemChild {
    VisibilityModifier(VisibilityModifier),
    WhereClause(WhereClause),
}
struct LineComment {
    outer: std::option::Option<OuterDocCommentMarker>,
    doc: std::option::Option<DocComment>,
    inner: std::option::Option<InnerDocCommentMarker>,
}
enum BinaryExpressionChild {
    ExclamationMarkequalsSignToken(ExclamationMarkequalsSignToken),
    PercentSignToken(PercentSignToken),
    AmpersandToken(AmpersandToken),
    AmpersandampersandToken(AmpersandampersandToken),
    AsteriskToken(AsteriskToken),
    PlusSignToken(PlusSignToken),
    HyphenMinusToken(HyphenMinusToken),
    SolidusToken(SolidusToken),
    LessThanSignToken(LessThanSignToken),
    LessThanSignlessThanSignToken(LessThanSignlessThanSignToken),
    LessThanSignequalsSignToken(LessThanSignequalsSignToken),
    EqualsSignequalsSignToken(EqualsSignequalsSignToken),
    GreaterThanSignToken(GreaterThanSignToken),
    GreaterThanSignequalsSignToken(GreaterThanSignequalsSignToken),
    GreaterThanSigngreaterThanSignToken(GreaterThanSigngreaterThanSignToken),
    CircumflexAccentToken(CircumflexAccentToken),
    VerticalLineToken(VerticalLineToken),
    VerticalLineverticalLineToken(VerticalLineverticalLineToken),
}
struct MatchPattern {
    condition: std::option::Option<MatchPatterncondition>,
    _pattern: _Pattern,
}
enum _Literal {
    BooleanLiteral(BooleanLiteral),
    CharLiteral(CharLiteral),
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    RawStringLiteral(RawStringLiteral),
    StringLiteral(StringLiteral),
}
struct OuterDocCommentMarker;
struct TraitBounds {
    children: std::vec::Vec<TraitBoundsChild>,
}
struct AmpersandToken;
struct CircumflexAccentToken;
struct ModItem {
    name: Identifier,
    body: std::option::Option<DeclarationList>,
    maybe_visibility_modifier: std::option::Option<VisibilityModifier>,
}
enum ImplItemtrait {
    GenericType(GenericType),
    ScopedTypeIdentifier(ScopedTypeIdentifier),
    TypeIdentifier(TypeIdentifier),
}
struct Identifier;
enum _DeclarationStatement {
    AssociatedType(AssociatedType),
    AttributeItem(AttributeItem),
    ConstItem(ConstItem),
    EmptyStatement(EmptyStatement),
    EnumItem(EnumItem),
    ExternCrateDeclaration(ExternCrateDeclaration),
    ForeignModItem(ForeignModItem),
    FunctionItem(FunctionItem),
    FunctionSignatureItem(FunctionSignatureItem),
    ImplItem(ImplItem),
    InnerAttributeItem(InnerAttributeItem),
    LetDeclaration(LetDeclaration),
    MacroDefinition(MacroDefinition),
    MacroInvocation(MacroInvocation),
    ModItem(ModItem),
    StaticItem(StaticItem),
    StructItem(StructItem),
    TraitItem(TraitItem),
    TypeItem(TypeItem),
    UnionItem(UnionItem),
    UseDeclaration(UseDeclaration),
}
struct StructToken;
enum WherePredicateChild {
    ArrayType(ArrayType),
    GenericType(GenericType),
    HigherRankedTraitBound(HigherRankedTraitBound),
    Lifetime(Lifetime),
    PointerType(PointerType),
    PrimitiveType(PrimitiveType),
    ReferenceType(ReferenceType),
    ScopedTypeIdentifier(ScopedTypeIdentifier),
    TupleType(TupleType),
    TypeIdentifier(TypeIdentifier),
}
struct EqualsSignequalsSignToken;
struct NegativeLiteral {
    children: NegativeLiteralChild,
}
struct Crate;
enum BreakExpressionChild { _Expression(_Expression), Label(Label), }
struct GenericTypeWithTurbofish {}
struct ArrayExpression {
    length: std::option::Option<_Expression>,
    children: std::option::Option<std::vec::Vec<ArrayExpressionChild>>,
}
enum ElseClauseChild { Block(Block), IfExpression(IfExpression), }
struct HyphenMinusgreaterThanSignToken;
struct GenericFunction {
    function: GenericFunctionChild,
    type_arguments: TypeArguments,
}
struct AsyncToken;
struct ArrayType {
    element: _Type,
    length: std::option::Option<_Expression>,
}
struct FragmentSpecifier;
enum TupleStructPatterntype {
    GenericType(GenericType),
    Identifier(Identifier),
    ScopedIdentifier(ScopedIdentifier),
}
struct AsyncBlock {
    block: Block,
}
enum UseAsClauseChild {
    Crate(Crate),
    Identifier(Identifier),
    Metavariable(Metavariable),
    ScopedIdentifier(ScopedIdentifier),
    SelfToken(SelfToken),
    Super(Super),
}
enum RangePatternChild {
    _LiteralPattern(_LiteralPattern),
    Crate(Crate),
    Identifier(Identifier),
    Metavariable(Metavariable),
    ScopedIdentifier(ScopedIdentifier),
    SelfToken(SelfToken),
    Super(Super),
}
struct TokenTreePattern {
    children: std::option::Option<std::vec::Vec<TokenTreePatternChild>>,
}
struct TupleType {
    children: std::vec::Vec<_Type>,
}
struct SolidusasteriskToken;
struct Parameter {
    pattern: Parameterpattern,
}
enum FieldInitializerfield {
    FieldIdentifier(FieldIdentifier),
    IntegerLiteral(IntegerLiteral),
}
struct LetCondition {
    value: _Expression,
    pattern: _Pattern,
}
enum ReferenceTypeChild {
    Lifetime(Lifetime),
    MutableSpecifier(MutableSpecifier),
}
enum FieldDeclarationListChild {
    AttributeItem(AttributeItem),
    FieldDeclaration(FieldDeclaration),
}
struct PercentSignToken;
enum BoundedTypeChild {
    _Type(_Type),
    Lifetime(Lifetime),
    UseBounds(UseBounds),
}
struct LetDeclaration {
    pattern: _Pattern,
    alternative: std::option::Option<Block>,
}
struct TraitToken;
struct ConstParameter {}
enum OrderedFieldDeclarationListChild {
    AttributeItem(AttributeItem),
    VisibilityModifier(VisibilityModifier),
}
struct ExprToken;
struct TryBlock {
    block: Block,
}
struct ElseClause {
    children: ElseClauseChild,
}
struct Attribute {
    arguments: std::option::Option<TokenTree>,
    value: std::option::Option<_Expression>,
    children: AttributeChild,
}
struct MacroRulesExclamationMarkToken;
struct ShorthandFieldInitializer {
    children: std::vec::Vec<ShorthandFieldInitializerChild>,
}
struct DynToken;
enum NegativeLiteralChild {
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
}
struct RefToken;
struct LifetimeToken;
struct ModToken;
struct GreaterThanSignequalsSignToken;
struct EnumVariantList {
    children: std::option::Option<std::vec::Vec<EnumVariantListChild>>,
}
struct StructPattern {}
struct DefaultToken;
enum AbstractTypetrait {
    BoundedType(BoundedType),
    FunctionType(FunctionType),
    GenericType(GenericType),
    RemovedTraitBound(RemovedTraitBound),
    ScopedTypeIdentifier(ScopedTypeIdentifier),
    TupleType(TupleType),
    TypeIdentifier(TypeIdentifier),
}
enum StaticItemChild {
    MutableSpecifier(MutableSpecifier),
    VisibilityModifier(VisibilityModifier),
}
enum ShorthandFieldInitializerChild {
    AttributeItem(AttributeItem),
    Identifier(Identifier),
}
struct YieldExpression {
    maybe__expression: std::option::Option<_Expression>,
}
enum ScopedIdentifierChild {
    BracketedType(BracketedType),
    Crate(Crate),
    GenericType(GenericType),
    Identifier(Identifier),
    Metavariable(Metavariable),
    ScopedIdentifier(ScopedIdentifier),
    SelfToken(SelfToken),
    Super(Super),
}
struct RangeExpression {
    children: std::option::Option<std::vec::Vec<_Expression>>,
}
struct TypeBinding {
    name: TypeIdentifier,
    type_arguments: std::option::Option<TypeArguments>,
}
struct LessThanSignToken;
struct VisibilityModifier {
    children: std::option::Option<VisibilityModifierChild>,
}
struct CharLiteral;
enum TuplePatternChild {
    _Pattern(_Pattern),
    ClosureExpression(ClosureExpression),
}
struct FullStopfullStopToken;
struct TypeArguments {
    children: std::vec::Vec<TypeArgumentsChild>,
}
struct StaticToken;
struct ItemToken;
struct FullStopfullStopfullStopToken;
struct IndexExpression {
    children: std::vec::Vec<_Expression>,
}
enum ArgumentsChild {
    _Expression(_Expression),
    AttributeItem(AttributeItem),
}
struct PatParamToken;
struct TypeCastExpression {
    value: _Expression,
}
enum _LiteralPattern {
    BooleanLiteral(BooleanLiteral),
    CharLiteral(CharLiteral),
    FloatLiteral(FloatLiteral),
    IntegerLiteral(IntegerLiteral),
    NegativeLiteral(NegativeLiteral),
    RawStringLiteral(RawStringLiteral),
    StringLiteral(StringLiteral),
}
struct EqualsSignToken;
struct MoveToken;
enum TypeArgumentsChild {
    _Literal(_Literal),
    _Type(_Type),
    Block(Block),
    Lifetime(Lifetime),
    TraitBounds(TraitBounds),
    TypeBinding(TypeBinding),
}
struct ScopedUseList {
    list: UseList,
    path: std::option::Option<ScopedUseListChild>,
}
struct QuestionMarkToken;
struct ConstToken;
struct StringLiteral {
    children: std::option::Option<std::vec::Vec<StringLiteralChild>>,
}
enum FunctionSignatureItemChild {
    FunctionModifiers(FunctionModifiers),
    VisibilityModifier(VisibilityModifier),
    WhereClause(WhereClause),
}
enum Parameterpattern { _Pattern(_Pattern), SelfToken(SelfToken), }
struct LessThanSignequalsSignToken;
struct DocComment;
struct WhileExpression {
    body: Block,
    condition: WhileExpressioncondition,
    maybe_label: std::option::Option<Label>,
}
struct PrimitiveType;
struct InToken;
struct FieldIdentifier;
struct StringContent;
struct FieldExpression {
    field: FieldExpressionChild,
    value: _Expression,
}
enum FunctionTypetrait {
    ScopedTypeIdentifier(ScopedTypeIdentifier),
    TypeIdentifier(TypeIdentifier),
}
struct YieldToken;
enum MutPatternChild {
    _Pattern(_Pattern),
    MutableSpecifier(MutableSpecifier),
}
struct EqualsSigngreaterThanSignToken;
struct PubToken;
struct LifetimeParameter {
    name: Lifetime,
    bounds: std::option::Option<TraitBounds>,
}
struct RangePattern {
    right: std::option::Option<RangePatternChild>,
    left: std::option::Option<RangePatternChild>,
}
struct ShorthandFieldIdentifier;
struct Arguments {
    children: std::option::Option<std::vec::Vec<ArgumentsChild>>,
}
struct LessThanSignlessThanSignequalsSignToken;
struct GreaterThanSignToken;
struct LeftParenthesisToken;
struct BaseFieldInitializer {
    _expression: _Expression,
}
struct InnerDocCommentMarker;
enum ConstParameterChild {
    _Literal(_Literal),
    Block(Block),
    Identifier(Identifier),
    NegativeLiteral(NegativeLiteral),
}
enum TokenRepetitionChild {
    _Literal(_Literal),
    Crate(Crate),
    Identifier(Identifier),
    Metavariable(Metavariable),
    MutableSpecifier(MutableSpecifier),
    PrimitiveType(PrimitiveType),
    SelfToken(SelfToken),
    Super(Super),
    TokenRepetition(TokenRepetition),
    TokenTree(TokenTree),
}
struct ConstBlock {
    body: Block,
}
struct UseDeclaration {
    argument: UseDeclarationargument,
    maybe_visibility_modifier: std::option::Option<VisibilityModifier>,
}
struct HyphenMinusToken;
struct LessThanSignlessThanSignToken;
struct StaticItem {}
struct WherePredicate {
    bounds: TraitBounds,
    left: WherePredicateChild,
}
enum MacroInvocationmacro {
    Identifier(Identifier),
    ScopedIdentifier(ScopedIdentifier),
}
struct TtToken;


