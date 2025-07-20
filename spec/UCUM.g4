grammar UCUM;

// Parser Rules
mainTerm
    : '/' term
    | term
    ;

term
    : term '.' component
    | term '/' component
    | component
    ;

component
    : annotatable annotation
    | annotatable
    | annotation
    | factor
    | '(' term ')'
    ;

annotatable
    : simpleUnit exponent
    | simpleUnit
    ;

simpleUnit
    : ATOM_SYMBOL
    | PREFIX_SYMBOL ATOM_SYMBOL_METRIC
    ;

exponent
    : sign digits
    | digits
    ;

factor
    : digits
    ;

digits
    : digit+
    ;

digit
    : '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    ;

sign
    : '+' | '-'
    ;

annotation
    : '{' ANNOTATION_STRING '}'
    ;

// Lexer Rules
ATOM_SYMBOL
    : [A-Za-z]+  // Define based on your specific requirements
    ;

PREFIX_SYMBOL
    : [A-Za-z]+  // Define based on your specific requirements
    ;

ATOM_SYMBOL_METRIC
    : [A-Za-z]+  // Define based on your specific requirements
    ;

ANNOTATION_STRING
    : ~[{}]+  // Any character except { and }
    ;

// Whitespace handling
WS
    : [ \t\r\n]+ -> skip
    ;