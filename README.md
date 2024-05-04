# cypher-lang

cypher-lang is dynamically typed interpreted language written in rust. cypher lang compiles down to cypher bytecode interpreted by cypher virtual machine.


# cypher grammar
```
program        → declaration* EOF ;
declaration    → varDecl
               | statement ;
while ->  "while" <expression> "{" <statement>* "}"
varDecl        → IDENTIFIER ( "=" expression )? ";" ;
statement      → <exprStmt>
               | <printStmt>
                  <ifStatement>;
ifStatement → "if" <expression> "{" <statement>  (else <statement>)? "}" ;
exprStmt       → <expression> ;
printStmt      → "print" <expression> ;
expression     → ('[' (<expression>)* ']') | <assignment> ;
assignment     → ( call "." )? IDENTIFIER "=" assignment
               | logic_or ;
logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;
equality       → <comparison> ( ( "!=" | "==" ) <comparison> )* ;
comparison     → <term> ( ( ">" | ">=" | "<" | "<=" ) <term> )* ;
term           → <factor> ( ( "-" | "+" ) <factor> )* ;
factor         → <unary> ( ( "%" | "/" | "*" ) <unary> )* ;
unary          → ( "!" | "-" ) <unary>
               | <primary> ;
increment      → <call> ( "++" | "--")
array index   -> identifier ('[' <expression> ']') | <call>
call           → <primary> ( "(" arguments? ")" | "." IDENTIFIER )* ;
primary        → NUMBER | STRING | "true" | "false" | "nil"|identifier
               | "(" <expression> ")" ;
```
# cypher bytecode

| opcode | operands | description |
| --------| -------- | ----------- |
| `LC` | 1 | Load constant |
| `ADD` | 2 | Addition Operator |
| `SUB` | 2 | Subtraction Operator |
| `MUL` | 2 | Multiplication Operator|
| `DIV` | 2 | Division Operator |
| `MOD` | 2 | Modulo Operator|
|`PRINT`| 0 | Call Print Function |
|`CALL`| 1| Call operator|
|`LT`| 2| Less than Operator|
|`GT`| 2| Greater than Operator|  
|`GOE`| 2| Greater than Equal Operator|  
|`LOE`| 2| Less than Equal Operator|
|`JMP`| 1 | Jump |
|`JNE`| 1 | Jump not equal |
|`NOP`| |No Operation|  




# Todo
- [x] lexer
- [x] parser
- [ ] bytecode generator
- [ ] symbol scopes and symbol table
- [ ] local scopes
- [ ] bytecode interpreter
- [ ] implement modules
- [ ] implement std