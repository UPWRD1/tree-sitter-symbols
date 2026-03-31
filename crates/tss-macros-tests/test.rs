#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use serde::{Deserialize, Serialize};
use tss_macros::generate_nodes;

struct Decorator {
    expression: Expression,
}
enum ListPatternChild { CasePattern(CasePattern), Pattern(Pattern), }
struct ListPattern {
    children: std::option::Option<std::vec::Vec<ListPatternChild>>,
}
struct DictionarySplat {
    expression: Expression,
}
struct ExclamationMarkequalsSignToken;
struct NamedExpression {
    value: Expression,
    name: Identifier,
}
struct ExecToken;
struct WhileToken;
struct VerticalLineequalsSignToken;
struct SolidussolidusequalsSignToken;
struct TildeToken;
struct HyphenMinusToken;
enum ParenthesizedExpressionChild {
    Expression(Expression),
    ListSplat(ListSplat),
    ParenthesizedExpression(ParenthesizedExpression),
    Yield(Yield),
}
struct ParenthesizedExpression {
    children: ParenthesizedExpressionChild,
}
enum TypedParameterChild {
    DictionarySplatPattern(DictionarySplatPattern),
    Identifier(Identifier),
    ListSplatPattern(ListSplatPattern),
}
struct TypedParameter {}
enum _CompoundStatement {
    ClassDefinition(ClassDefinition),
    DecoratedDefinition(DecoratedDefinition),
    ForStatement(ForStatement),
    FunctionDefinition(FunctionDefinition),
    IfStatement(IfStatement),
    MatchStatement(MatchStatement),
    TryStatement(TryStatement),
    WhileStatement(WhileStatement),
    WithStatement(WithStatement),
}
struct TypeParameter {
    children: std::vec::Vec<Type>,
}
struct LineContinuation;
struct GreaterThanSigngreaterThanSignequalsSignToken;
struct CommaToken;
enum UnionPatternChild {
    ClassPattern(ClassPattern),
    ComplexPattern(ComplexPattern),
    ConcatenatedString(ConcatenatedString),
    DictPattern(DictPattern),
    DottedName(DottedName),
    False(False),
    Float(Float),
    Integer(Integer),
    ListPattern(ListPattern),
    None(None),
    SplatPattern(SplatPattern),
    String(String),
    True(True),
    TuplePattern(TuplePattern),
    UnionPattern(UnionPattern),
}
struct UnionPattern {
    children: std::option::Option<std::vec::Vec<UnionPatternChild>>,
}
struct WithStatement {
    body: Block,
    with_clause: WithClause,
}
struct ReverseSolidusToken;
struct CircumflexAccentequalsSignToken;
struct RightParenthesisToken;
enum ForStatementChild {
    Expression(Expression),
    ExpressionList(ExpressionList),
}
enum ForStatementChild { Pattern(Pattern), PatternList(PatternList), }
struct ForStatement {
    right: ForStatementChild,
    left: ForStatementChild,
    alternative: std::option::Option<ElseClause>,
    body: Block,
}
enum SetComprehensionChild { ForInClause(ForInClause), IfClause(IfClause), }
struct SetComprehension {
    body: Expression,
    children: std::vec::Vec<SetComprehensionChild>,
}
struct AsteriskasteriskequalsSignToken;
struct LessThanSignequalsSignToken;
struct AsyncToken;
enum KeywordPatternChild {
    ClassPattern(ClassPattern),
    ComplexPattern(ComplexPattern),
    ConcatenatedString(ConcatenatedString),
    DictPattern(DictPattern),
    DottedName(DottedName),
    False(False),
    Float(Float),
    Identifier(Identifier),
    Integer(Integer),
    ListPattern(ListPattern),
    None(None),
    SplatPattern(SplatPattern),
    String(String),
    True(True),
    TuplePattern(TuplePattern),
    UnionPattern(UnionPattern),
}
struct KeywordPattern {
    children: std::vec::Vec<KeywordPatternChild>,
}
struct Comment;
enum ComplexPatternChild { Float(Float), Integer(Integer), }
struct ComplexPattern {
    children: std::vec::Vec<ComplexPatternChild>,
}
struct IfClause {
    expression: Expression,
}
struct WhileStatement {
    alternative: std::option::Option<ElseClause>,
    body: Block,
    condition: Expression,
}
struct ElifToken;
struct False;
struct TypeConversion;
struct ColonToken;
enum BinaryOperatorChild {
    PercentSignToken(PercentSignToken),
    AmpersandToken(AmpersandToken),
    AsteriskToken(AsteriskToken),
    AsteriskasteriskToken(AsteriskasteriskToken),
    PlusSignToken(PlusSignToken),
    HyphenMinusToken(HyphenMinusToken),
    SolidusToken(SolidusToken),
    SolidussolidusToken(SolidussolidusToken),
    LessThanSignlessThanSignToken(LessThanSignlessThanSignToken),
    GreaterThanSigngreaterThanSignToken(GreaterThanSigngreaterThanSignToken),
    CommercialAtToken(CommercialAtToken),
    CircumflexAccentToken(CircumflexAccentToken),
    VerticalLineToken(VerticalLineToken),
}
struct BinaryOperator {
    right: PrimaryExpression,
    operator: BinaryOperatorChild,
    left: PrimaryExpression,
}
struct Lambda {
    body: Expression,
    parameters: std::option::Option<LambdaParameters>,
}
enum ListSplatChild {
    Attribute(Attribute),
    Expression(Expression),
    Identifier(Identifier),
    Subscript(Subscript),
}
struct ListSplat {
    children: ListSplatChild,
}
struct MatchStatement {
    body: Block,
    subject: std::vec::Vec<Expression>,
}
struct GreaterThanSigngreaterThanSignToken;
struct CircumflexAccentToken;
struct AssertToken;
struct Float;
enum GeneratorExpressionChild {
    ForInClause(ForInClause),
    IfClause(IfClause),
}
struct GeneratorExpression {
    body: Expression,
    children: std::vec::Vec<GeneratorExpressionChild>,
}
struct ForToken;
struct ConcatenatedString {
    children: std::vec::Vec<String>,
}
struct RaiseToken;
enum CasePatternChild {
    AsPattern(AsPattern),
    ClassPattern(ClassPattern),
    ComplexPattern(ComplexPattern),
    ConcatenatedString(ConcatenatedString),
    DictPattern(DictPattern),
    DottedName(DottedName),
    False(False),
    Float(Float),
    Integer(Integer),
    KeywordPattern(KeywordPattern),
    ListPattern(ListPattern),
    None(None),
    SplatPattern(SplatPattern),
    String(String),
    True(True),
    TuplePattern(TuplePattern),
    UnionPattern(UnionPattern),
}
struct CasePattern {
    children: std::option::Option<CasePatternChild>,
}
struct BreakToken;
struct LeftCurlyBracketToken;
struct NonlocalStatement {
    children: std::vec::Vec<Identifier>,
}
struct SolidusToken;
struct BreakStatement;
struct CaseClause {
    guard: std::option::Option<IfClause>,
    consequence: Block,
    children: std::vec::Vec<CasePattern>,
}
enum ListComprehensionChild { ForInClause(ForInClause), IfClause(IfClause), }
struct ListComprehension {
    body: Expression,
    children: std::vec::Vec<ListComprehensionChild>,
}
struct WildcardImport;
enum AugmentedAssignmentChild { Pattern(Pattern), PatternList(PatternList), }
enum AugmentedAssignmentChild {
    Assignment(Assignment),
    AugmentedAssignment(AugmentedAssignment),
    Expression(Expression),
    ExpressionList(ExpressionList),
    PatternList(PatternList),
    Yield(Yield),
}
enum AugmentedAssignmentChild {
    PercentSignequalsSignToken(PercentSignequalsSignToken),
    AmpersandequalsSignToken(AmpersandequalsSignToken),
    AsteriskasteriskequalsSignToken(AsteriskasteriskequalsSignToken),
    AsteriskequalsSignToken(AsteriskequalsSignToken),
    PlusSignequalsSignToken(PlusSignequalsSignToken),
    HyphenMinusequalsSignToken(HyphenMinusequalsSignToken),
    SolidussolidusequalsSignToken(SolidussolidusequalsSignToken),
    SolidusequalsSignToken(SolidusequalsSignToken),
    LessThanSignlessThanSignequalsSignToken(LessThanSignlessThanSignequalsSignToken),
    GreaterThanSigngreaterThanSignequalsSignToken(GreaterThanSigngreaterThanSignequalsSignToken),
    CommercialAtequalsSignToken(CommercialAtequalsSignToken),
    CircumflexAccentequalsSignToken(CircumflexAccentequalsSignToken),
    VerticalLineequalsSignToken(VerticalLineequalsSignToken),
}
struct AugmentedAssignment {
    left: AugmentedAssignmentChild,
    right: AugmentedAssignmentChild,
    operator: AugmentedAssignmentChild,
}
struct IsSpacEnotToken;
enum GenericTypeChild {
    Identifier(Identifier),
    TypeParameter(TypeParameter),
}
struct GenericType {
    children: std::vec::Vec<GenericTypeChild>,
}
enum InterpolationChild {
    Expression(Expression),
    ExpressionList(ExpressionList),
    PatternList(PatternList),
    Yield(Yield),
}
struct Interpolation {
    format_specifier: std::option::Option<FormatSpecifier>,
    expression: InterpolationChild,
    type_conversion: std::option::Option<TypeConversion>,
}
struct AsteriskToken;
struct KeywordArgument {
    name: Identifier,
    value: Expression,
}
struct PlusSignequalsSignToken;
struct StringStart;
struct PlusSignToken;
struct PercentSignToken;
struct ElseToken;
enum Expression {
    AsPattern(AsPattern),
    BooleanOperator(BooleanOperator),
    ComparisonOperator(ComparisonOperator),
    ConditionalExpression(ConditionalExpression),
    Lambda(Lambda),
    NamedExpression(NamedExpression),
    NotOperator(NotOperator),
    PrimaryExpression(PrimaryExpression),
}
struct NotOperator {
    argument: Expression,
}
enum DictPatternChild {
    HyphenMinusToken(HyphenMinusToken),
    _Token(_Token),
    ClassPattern(ClassPattern),
    ComplexPattern(ComplexPattern),
    ConcatenatedString(ConcatenatedString),
    DictPattern(DictPattern),
    DottedName(DottedName),
    False(False),
    Float(Float),
    Integer(Integer),
    ListPattern(ListPattern),
    None(None),
    SplatPattern(SplatPattern),
    String(String),
    True(True),
    TuplePattern(TuplePattern),
    UnionPattern(UnionPattern),
}
struct DictPattern {
    value: std::option::Option<std::vec::Vec<CasePattern>>,
    key: std::option::Option<std::vec::Vec<DictPatternChild>>,
    children: std::option::Option<std::vec::Vec<SplatPattern>>,
}
enum ParenthesizedListSplatChild {
    ListSplat(ListSplat),
    ParenthesizedExpression(ParenthesizedExpression),
}
struct ParenthesizedListSplat {
    children: ParenthesizedListSplatChild,
}
struct DefToken;
enum ListChild {
    Expression(Expression),
    ListSplat(ListSplat),
    ParenthesizedListSplat(ParenthesizedListSplat),
    Yield(Yield),
}
struct List {
    children: std::option::Option<std::vec::Vec<ListChild>>,
}
struct NotToken;
struct TypeAliasStatement {
    right: Type,
    left: Type,
}
struct WithToken;
enum BooleanOperatorChild { AndToken(AndToken), OrToken(OrToken), }
struct BooleanOperator {
    operator: BooleanOperatorChild,
    left: Expression,
    right: Expression,
}
struct AliasedImport {
    alias: Identifier,
    name: DottedName,
}
struct GlobalToken;
struct RightCurlyBracketToken;
struct ImportToken;
struct ContinueToken;
enum TypeChild {
    ConstrainedType(ConstrainedType),
    Expression(Expression),
    GenericType(GenericType),
    MemberType(MemberType),
    SplatType(SplatType),
    UnionType(UnionType),
}
struct Type {
    children: TypeChild,
}
struct WithClause {
    children: std::vec::Vec<WithItem>,
}
enum TryStatementChild {
    ElseClause(ElseClause),
    ExceptClause(ExceptClause),
    FinallyClause(FinallyClause),
}
struct TryStatement {
    body: Block,
    children: std::option::Option<std::vec::Vec<TryStatementChild>>,
}
struct HyphenMinusequalsSignToken;
struct EscapeSequence;
struct MatchToken;
struct YieldToken;
struct HyphenMinusgreaterThanSignToken;
enum DictionaryComprehensionChild {
    ForInClause(ForInClause),
    IfClause(IfClause),
}
struct DictionaryComprehension {
    body: Pair,
    children: std::vec::Vec<DictionaryComprehensionChild>,
}
enum ExpressionStatementChild {
    Assignment(Assignment),
    AugmentedAssignment(AugmentedAssignment),
    Expression(Expression),
    Yield(Yield),
}
struct ExpressionStatement {
    children: std::vec::Vec<ExpressionStatementChild>,
}
struct GreaterThanSignToken;
struct EscapeInterpolation;
enum _SimpleStatement {
    AssertStatement(AssertStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    DeleteStatement(DeleteStatement),
    ExecStatement(ExecStatement),
    ExpressionStatement(ExpressionStatement),
    FutureImportStatement(FutureImportStatement),
    GlobalStatement(GlobalStatement),
    ImportFromStatement(ImportFromStatement),
    ImportStatement(ImportStatement),
    NonlocalStatement(NonlocalStatement),
    PassStatement(PassStatement),
    PrintStatement(PrintStatement),
    RaiseStatement(RaiseStatement),
    ReturnStatement(ReturnStatement),
    TypeAliasStatement(TypeAliasStatement),
}
struct ClassDefinition {
    name: Identifier,
    body: Block,
    superclasses: std::option::Option<ArgumentList>,
    type_parameters: std::option::Option<TypeParameter>,
}
struct Chevron {
    expression: Expression,
}
enum ImportFromStatementChild {
    AliasedImport(AliasedImport),
    DottedName(DottedName),
}
enum ImportFromStatementChild {
    DottedName(DottedName),
    RelativeImport(RelativeImport),
}
struct ImportFromStatement {
    name: std::option::Option<std::vec::Vec<ImportFromStatementChild>>,
    module_name: ImportFromStatementChild,
    maybe_wildcard_import: std::option::Option<WildcardImport>,
}
struct WithItem {
    value: Expression,
}
struct ImportPrefix;
enum DefaultParameterChild {
    Identifier(Identifier),
    TuplePattern(TuplePattern),
}
struct DefaultParameter {
    name: DefaultParameterChild,
    value: Expression,
}
struct _FutureToken;
struct FunctionDefinition {
    name: Identifier,
    parameters: Parameters,
    body: Block,
    return_type: std::option::Option<Type>,
    type_parameters: std::option::Option<TypeParameter>,
}
enum ArgumentListChild {
    DictionarySplat(DictionarySplat),
    Expression(Expression),
    KeywordArgument(KeywordArgument),
    ListSplat(ListSplat),
    ParenthesizedExpression(ParenthesizedExpression),
}
struct ArgumentList {
    children: std::option::Option<std::vec::Vec<ArgumentListChild>>,
}
struct DelToken;
struct KeywordSeparator;
struct Slice {
    children: std::option::Option<std::vec::Vec<Expression>>,
}
enum CallChild {
    ArgumentList(ArgumentList),
    GeneratorExpression(GeneratorExpression),
}
struct Call {
    arguments: CallChild,
    function: PrimaryExpression,
}
struct GlobalStatement {
    children: std::vec::Vec<Identifier>,
}
struct LessThanSignToken;
struct InToken;
struct Await {
    primary_expression: PrimaryExpression,
}
enum StringChild {
    Interpolation(Interpolation),
    StringContent(StringContent),
    StringEnd(StringEnd),
    StringStart(StringStart),
}
struct String {
    children: std::vec::Vec<StringChild>,
}
struct LambdaToken;
struct ExpressionList {
    children: std::vec::Vec<Expression>,
}
struct SplatPattern {
    maybe_identifier: std::option::Option<Identifier>,
}
struct PercentSignequalsSignToken;
struct AssertStatement {
    children: std::vec::Vec<Expression>,
}
struct NonlocalToken;
struct OrToken;
struct FinallyClause {
    block: Block,
}
struct Attribute {
    attribute: Identifier,
    object: PrimaryExpression,
}
enum MemberTypeChild { Identifier(Identifier), Type(Type), }
struct MemberType {
    children: std::vec::Vec<MemberTypeChild>,
}
struct SolidussolidusToken;
struct EqualsSignToken;
struct ExceptClause {
    alias: std::option::Option<Expression>,
    value: std::option::Option<std::vec::Vec<Expression>>,
    block: Block,
}
struct GreaterThanSignequalsSignToken;
enum UnaryOperatorChild {
    PlusSignToken(PlusSignToken),
    HyphenMinusToken(HyphenMinusToken),
    TildeToken(TildeToken),
}
struct UnaryOperator {
    operator: UnaryOperatorChild,
    argument: PrimaryExpression,
}
enum BlockChild {
    _CompoundStatement(_CompoundStatement),
    _SimpleStatement(_SimpleStatement),
}
struct Block {
    alternative: std::option::Option<std::vec::Vec<CaseClause>>,
    children: std::option::Option<std::vec::Vec<BlockChild>>,
}
struct IfToken;
struct TypeToken;
enum DictionaryChild { DictionarySplat(DictionarySplat), Pair(Pair), }
struct Dictionary {
    children: std::option::Option<std::vec::Vec<DictionaryChild>>,
}
struct PositionalSeparator;
enum SetChild {
    Expression(Expression),
    ListSplat(ListSplat),
    ParenthesizedListSplat(ParenthesizedListSplat),
    Yield(Yield),
}
struct Set {
    children: std::vec::Vec<SetChild>,
}
enum TuplePatternChild { CasePattern(CasePattern), Pattern(Pattern), }
struct TuplePattern {
    children: std::option::Option<std::vec::Vec<TuplePatternChild>>,
}
struct ClassToken;
struct TypedDefaultParameter {}
struct LeftSquareBracketToken;
struct TryToken;
enum DeleteStatementChild {
    Expression(Expression),
    ExpressionList(ExpressionList),
}
struct DeleteStatement {
    children: DeleteStatementChild,
}
struct AsToken;
struct PassToken;
enum ListSplatPatternChild {
    Attribute(Attribute),
    Identifier(Identifier),
    Subscript(Subscript),
}
struct ListSplatPattern {
    children: ListSplatPatternChild,
}
enum StringContentChild {
    EscapeInterpolation(EscapeInterpolation),
    EscapeSequence(EscapeSequence),
}
struct StringContent {
    children: std::option::Option<std::vec::Vec<StringContentChild>>,
}
struct PassStatement;
struct CommercialAtToken;
struct SplatType {
    identifier: Identifier,
}
struct PatternList {
    children: std::vec::Vec<Pattern>,
}
enum SubscriptChild { Expression(Expression), Slice(Slice), }
struct Subscript {
    subscript: std::vec::Vec<SubscriptChild>,
    value: PrimaryExpression,
}
struct Ellipsis;
enum ImportStatementChild {
    AliasedImport(AliasedImport),
    DottedName(DottedName),
}
struct ImportStatement {
    name: std::vec::Vec<ImportStatementChild>,
}
struct True;
enum AsPatternChild {
    CasePattern(CasePattern),
    Expression(Expression),
    Identifier(Identifier),
}
struct AsPattern {
    alias: std::option::Option<AsPatternTarget>,
    children: std::vec::Vec<AsPatternChild>,
}
enum FormatExpressionChild {
    Expression(Expression),
    ExpressionList(ExpressionList),
    PatternList(PatternList),
    Yield(Yield),
}
struct FormatExpression {
    format_specifier: std::option::Option<FormatSpecifier>,
    type_conversion: std::option::Option<TypeConversion>,
    expression: FormatExpressionChild,
}
enum ReturnStatementChild {
    Expression(Expression),
    ExpressionList(ExpressionList),
}
struct ReturnStatement {
    children: std::option::Option<ReturnStatementChild>,
}
enum ComparisonOperatorChild {
    ExclamationMarkequalsSignToken(ExclamationMarkequalsSignToken),
    LessThanSignToken(LessThanSignToken),
    LessThanSignequalsSignToken(LessThanSignequalsSignToken),
    LessThanSigngreaterThanSignToken(LessThanSigngreaterThanSignToken),
    EqualsSignequalsSignToken(EqualsSignequalsSignToken),
    GreaterThanSignToken(GreaterThanSignToken),
    GreaterThanSignequalsSignToken(GreaterThanSignequalsSignToken),
    InToken(InToken),
    IsToken(IsToken),
    IsSpacEnotToken(IsSpacEnotToken),
    NotSpacEinToken(NotSpacEinToken),
}
struct ComparisonOperator {
    operators: std::vec::Vec<ComparisonOperatorChild>,
    children: std::vec::Vec<PrimaryExpression>,
}
struct AmpersandequalsSignToken;
struct SolidusequalsSignToken;
struct ElseClause {
    body: Block,
}
struct NotSpacEinToken;
enum ModuleChild {
    _CompoundStatement(_CompoundStatement),
    _SimpleStatement(_SimpleStatement),
}
struct Module {
    children: std::option::Option<std::vec::Vec<ModuleChild>>,
}
struct Pair {
    value: Expression,
    key: Expression,
}
struct SemicolonToken;
struct RightSquareBracketToken;
struct ReturnToken;
struct AwaitToken;
enum DictionarySplatPatternChild {
    Attribute(Attribute),
    Identifier(Identifier),
    Subscript(Subscript),
}
struct DictionarySplatPattern {
    children: DictionarySplatPatternChild,
}
enum IfStatementChild { ElifClause(ElifClause), ElseClause(ElseClause), }
struct IfStatement {
    condition: Expression,
    consequence: Block,
    alternative: std::option::Option<std::vec::Vec<IfStatementChild>>,
}
struct ColonequalsSignToken;
struct ConstrainedType {
    children: std::vec::Vec<Type>,
}
enum TupleChild {
    Expression(Expression),
    ListSplat(ListSplat),
    ParenthesizedListSplat(ParenthesizedListSplat),
    Yield(Yield),
}
struct Tuple {
    children: std::option::Option<std::vec::Vec<TupleChild>>,
}
struct LeftParenthesisToken;
struct FromToken;
struct Integer;
struct CommercialAtequalsSignToken;
struct LessThanSignlessThanSignToken;
struct VerticalLineToken;
struct EqualsSignequalsSignToken;
struct ContinueStatement;
struct UnionType {
    children: std::vec::Vec<Type>,
}
enum DecoratedDefinitionChild {
    ClassDefinition(ClassDefinition),
    FunctionDefinition(FunctionDefinition),
}
struct DecoratedDefinition {
    definition: DecoratedDefinitionChild,
    children: std::vec::Vec<Decorator>,
}
struct AmpersandToken;
struct AsteriskequalsSignToken;
struct FullStopToken;
struct LessThanSignlessThanSignequalsSignToken;
enum ClassPatternChild { CasePattern(CasePattern), DottedName(DottedName), }
struct ClassPattern {
    children: std::vec::Vec<ClassPatternChild>,
}
struct None;
struct FormatSpecifier {
    children: std::option::Option<std::vec::Vec<FormatExpression>>,
}
struct CaseToken;
struct AsteriskasteriskToken;
struct IsToken;
enum Parameter {
    DefaultParameter(DefaultParameter),
    DictionarySplatPattern(DictionarySplatPattern),
    Identifier(Identifier),
    KeywordSeparator(KeywordSeparator),
    ListSplatPattern(ListSplatPattern),
    PositionalSeparator(PositionalSeparator),
    TuplePattern(TuplePattern),
    TypedDefaultParameter(TypedDefaultParameter),
    TypedParameter(TypedParameter),
}
struct ExceptToken;
struct FinallyToken;
struct Identifier;
struct StringEnd;
struct PrintStatement {
    argument: std::option::Option<std::vec::Vec<Expression>>,
    maybe_chevron: std::option::Option<Chevron>,
}
enum ExecStatementChild { Identifier(Identifier), String(String), }
struct ExecStatement {
    code: ExecStatementChild,
    children: std::option::Option<std::vec::Vec<Expression>>,
}
struct LessThanSigngreaterThanSignToken;
enum ForInClauseChild { Pattern(Pattern), PatternList(PatternList), }
enum ForInClauseChild { CommaToken(CommaToken), Expression(Expression), }
struct ForInClause {
    left: ForInClauseChild,
    right: std::vec::Vec<ForInClauseChild>,
}
struct ConditionalExpression {
    children: std::vec::Vec<Expression>,
}
enum FutureImportStatementChild {
    AliasedImport(AliasedImport),
    DottedName(DottedName),
}
struct FutureImportStatement {
    name: std::vec::Vec<FutureImportStatementChild>,
}
enum PrimaryExpression {
    Attribute(Attribute),
    Await(Await),
    BinaryOperator(BinaryOperator),
    Call(Call),
    ConcatenatedString(ConcatenatedString),
    Dictionary(Dictionary),
    DictionaryComprehension(DictionaryComprehension),
    Ellipsis(Ellipsis),
    False(False),
    Float(Float),
    GeneratorExpression(GeneratorExpression),
    Identifier(Identifier),
    Integer(Integer),
    List(List),
    ListComprehension(ListComprehension),
    ListSplat(ListSplat),
    None(None),
    ParenthesizedExpression(ParenthesizedExpression),
    Set(Set),
    SetComprehension(SetComprehension),
    String(String),
    Subscript(Subscript),
    True(True),
    Tuple(Tuple),
    UnaryOperator(UnaryOperator),
}
struct PrintToken;
enum RaiseStatementChild {
    Expression(Expression),
    ExpressionList(ExpressionList),
}
struct RaiseStatement {
    cause: std::option::Option<Expression>,
    children: std::option::Option<RaiseStatementChild>,
}
struct AndToken;
struct ElifClause {
    consequence: Block,
    condition: Expression,
}
enum AssignmentChild {
    Assignment(Assignment),
    AugmentedAssignment(AugmentedAssignment),
    Expression(Expression),
    ExpressionList(ExpressionList),
    PatternList(PatternList),
    Yield(Yield),
}
enum AssignmentChild { Pattern(Pattern), PatternList(PatternList), }
struct Assignment {
    right: std::option::Option<AssignmentChild>,
    left: AssignmentChild,
}
enum RelativeImportChild {
    DottedName(DottedName),
    ImportPrefix(ImportPrefix),
}
struct RelativeImport {
    children: std::vec::Vec<RelativeImportChild>,
}
enum Pattern {
    Attribute(Attribute),
    Identifier(Identifier),
    ListPattern(ListPattern),
    ListSplatPattern(ListSplatPattern),
    Subscript(Subscript),
    TuplePattern(TuplePattern),
}
struct DottedName {
    children: std::vec::Vec<Identifier>,
}
struct LambdaParameters {
    children: std::vec::Vec<Parameter>,
}
struct Parameters {
    children: std::option::Option<std::vec::Vec<Parameter>>,
}
enum YieldChild { Expression(Expression), ExpressionList(ExpressionList), }
struct Yield {
    children: std::option::Option<YieldChild>,
}
struct _Token;


