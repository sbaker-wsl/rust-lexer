// Seth Baker

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Brackets
    PARENS_L,
    PARENS_R,
    BRACKET_L,
    BRACKET_R,
    BRACE_L,
    BRACE_R,
    // Separators
    POINT,
    COMMA,
    COLON,
    SEMICOLON,
    ARROW_R,
    // Arithmetic Ops
    ADD,
    SUB,
    MUL,
    DIV,
    // Relational Ops
    EQ,
    LT,
    GT,
    NEQ,
    NLT,
    NGT,
    // Logical Ops
    NOT,
    AND,
    OR,
    // Assignment
    ASSIGN,
    // Keywords
    FUNC,
    LET,
    IF,
    ELSE,
    WHILE,
    PRINT,
    // Identifiers
    ID(String),
    // Basic Types
    TYPE_INT32,
    TYPE_FLT32,
    TYPE_CHAR,
    // Literals
    LIT_INT32(i32),
    LIT_FLT32(f32),
    LIT_CHAR(char),
    LIT_STRING(String),
    // End-of-Input
    EOI,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum LexerState {
    INITIAL,
    L_PAREN,
    R_PAREN,
    L_BRACK,
    R_BRACK,
    L_BRACE,
    R_BRACE,
    POINT,
    COMMA,
    COLON,
    SEMICOLON,
    MINUS,
    ARROW_R,
    PLUS,
    MULT,
    DIVIDE,
    ASSIGN,
    EQUALS,
    LESSTHAN,
    NOTGREATTHAN,
    GREATTHAN,
    NOTLESSTHAN,
    EXC,
    NEQUAL,
    LETTER,
    NOT,
    AND,
    OR,
    FUNC,
    LET,
    IF,
    ELSE,
    WHILE,
    PRINT,
    ID,
    T_I32,
    T_F32,
    T_CH,
    NUMBER,
    L_I32,
    L_F32,
    L_CH,
    L_STR,
    END
}
