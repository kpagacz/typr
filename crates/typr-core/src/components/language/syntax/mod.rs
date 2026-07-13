#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SyntaxKind {
    // Keywords
    LET_KW,
    FN_KW,
    FUNCTION_KW,
    TYPE_CONSTRUCTOR_KW,
    RECURSIVE_KW,
    TYPE_KW,
    OPAQUE_KW,
    MODULE_KW,
    MOD_KW,
    IMPORT_KW,
    AS_KW,
    USE_KW,
    TEST_KW,
    LIBRARY_KW,
    EMBED_KW,
    INTERFACE_KW,
    CLASS_KW,
    JS_KW,
    RETURN_KW,
    BREAK_KW,
    NEXT_KW,

    // Record identifiers
    RECORD_KW,
    OBJECT_KW,
    LIST_KW,

    ANY_KW,
    SELF_KW,
    EMPTY_KW,

    // Conditional
    IF_KW,
    ELSE_KW,

    // Types keywords
    DATAFRAME_KW,
    ARRAY_KW,
    VEC_KW,
    NUM_KW,
    INT_KW,
    BOOL_KW,
    TUPLE_KW,

    // Dynamic tokens
    IDENT,
    NUMBER,
    STRING,

    // Value keywords
    TRUE_KW,
    FALSE_KW,
    NULL_KW,
    NA_KW,

    // Loops
    FOR_KW,
    WHILE_KW,
    LOOP_KW,

    // Annotations
    AT_EXPORT,
    AT_PUB,
    AT_TESTABLE,
    AT_IMPORT_FROM,

    // Operators
    ADD,
    ADD2,
    MINUS,
    MINUS2,
    MUL,
    MUL2,
    DIV,
    DIV2,
    AT,
    AT2,
    MODULO,
    MODULO2,
    PIPE,
    PIPE2,
    DOLLAR,
    DOLLAR2,
    EQ,
    EQ2,
    DOT,
    DOT2,
    DOT3,
    NOT_EQ,
    LESSER_OR_EQUAL,
    GREATER_OR_EQUAL,
    LESSER_THAN,
    GREATER_THAN,
    IN_OP,
    AND,
    AND2,
    OR,
    OR2,
    CUSTOM,
    AS_EXCL,
    L_ARROW, // <-
    R_ARROW, // ->

    // Punctuation
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_PAREN,
    R_PAREN,
    L_VECTORIAL,
    R_VECTORIAL,
    SEMICOLON,
    COLON,
    COLON2,
    COMMA,
    STAR, // *
    EXCLAMATION,
    CARET, // ^
    QUESTION_MARK,
    UNDERSCORE,
    BACKSLASH,

    // Trivia
    WHITESPACE,
    COMMENT,

    // Special
    ERROR,
    EOF,
}
