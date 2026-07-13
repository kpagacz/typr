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

    // Nodes
    SOURCE_FILE,
    LITERAL_EXPR,
    IDENT_EXPR,
    BINARY_EXPR,
    PREFIX_EXPR,
    PAREN_EXPR,
    CALL_EXPR,
    ARRAY_EXPR,
    LET_STMT,
    EXPR_STMT,
    BLOCK_EXPR,
}

impl SyntaxKind {
    pub fn from_keyword(text: &str) -> Option<Self> {
        match text {
            "let" => Some(Self::LET_KW),
            "fn" => Some(Self::FN_KW),
            "function" => Some(Self::FUNCTION_KW),
            "type_constructor" => Some(Self::TYPE_CONSTRUCTOR_KW),
            "recursive" => Some(Self::RECURSIVE_KW),
            "type" => Some(Self::TYPE_KW),
            "opaque" => Some(Self::OPAQUE_KW),
            "module" => Some(Self::MODULE_KW),
            "mod" => Some(Self::MOD_KW),
            "import" => Some(Self::IMPORT_KW),
            "as" => Some(Self::AS_KW),
            "use" => Some(Self::USE_KW),
            "test" => Some(Self::TEST_KW),
            "library" => Some(Self::LIBRARY_KW),
            "embed" => Some(Self::EMBED_KW),
            "interface" => Some(Self::INTERFACE_KW),
            "class" => Some(Self::CLASS_KW),
            "JS" => Some(Self::JS_KW),
            "return" => Some(Self::RETURN_KW),
            "break" => Some(Self::BREAK_KW),
            "next" => Some(Self::NEXT_KW),
            "record" => Some(Self::RECORD_KW),
            "object" => Some(Self::OBJECT_KW),
            "list" => Some(Self::LIST_KW),
            "any" => Some(Self::ANY_KW),
            "self" => Some(Self::SELF_KW),
            "empty" => Some(Self::EMPTY_KW),
            "if" => Some(Self::IF_KW),
            "else" => Some(Self::ELSE_KW),
            "dataframe" | "df" => Some(Self::DATAFRAME_KW),
            "array" => Some(Self::ARRAY_KW),
            "vec" => Some(Self::VEC_KW),
            "num" => Some(Self::NUM_KW),
            "int" => Some(Self::INT_KW),
            "bool" => Some(Self::BOOL_KW),
            "tuple" => Some(Self::TUPLE_KW),
            "true" => Some(Self::TRUE_KW),
            "false" => Some(Self::FALSE_KW),
            "null" => Some(Self::NULL_KW),
            "NA" => Some(Self::NA_KW),
            "for" => Some(Self::FOR_KW),
            "while" => Some(Self::WHILE_KW),
            "loop" => Some(Self::LOOP_KW),
            "and" => Some(Self::AND2),
            "or" => Some(Self::OR2),
            "in" => Some(Self::IN_OP),
            _ => None,
        }
    }
}
