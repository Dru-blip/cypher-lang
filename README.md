# cypher-lang

cypher-lang is dynamically typed interpreted language written in rust. cypher lang compiles down to cypher bytecode interpreted by cypher virtual machine.

# cypher syntax

### Data Types
```
let boolean = false or true
let strings = "hello world!"
let numbers = 555
let arrays = [1,2,3,4]
```
### control flow

#### if Statements
```
let i=5
if  i%2==0{
   print("even")
}else{
   print("odd")
}
```

#### while loop

```
let n=0
while n<=10{
   print(n)
   n++
}
```

### Functions

```
def fact(num){
   let res=1
   let i=1
   while i<num{
      res=res*i
      i++
   }
   return res
}

let hello=def (){
   print("hello world!")
}

hello()
let a=fact(5) 

```
### Builtins

#### len() function
```
let a=len("hello") //returns length of string
let b=len([1,2,3]) //returns length of array
```

#### push function

```
let a=[1,2,3,4]
push(a,5)
```

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

| opcode       | operands | description                 |
| ------------ | -------- | --------------------------- |
| `LC`         | 1        | Load constant               |
| `ADD`        | 2        | Addition Operator           |
| `SUB`        | 2        | Subtraction Operator        |
| `MUL`        | 2        | Multiplication Operator     |
| `DIV`        | 2        | Division Operator           |
| `MOD`        | 2        | Modulo Operator             |
| `PRINT`      | 0        | Call Print Function         |
| `CALL`       | 1        | Call operator               |
| `LT`         | 2        | Less than Operator          |
| `GT`         | 2        | Greater than Operator       |
| `GOE`        | 2        | Greater than Equal Operator |
| `LOE`        | 2        | Less than Equal Operator    |
| `JMP`        | 1        | Jump                        |
| `JNE`        | 1        | Jump not equal              |
| `Get Global` | 1        | get global variable         |
| `NOP`        |          | No Operation                |

### source
```
let a=3

if a<5{
    print("less")
}else{
    print("greater")
}
```
generated  bytecode
```
examples/hello.cy:
      lc     3
      get    3
      lc     5
      le
      jne    14
      lc     less
      print
      jmp    17
      lc     greater
      print
```

# Todo
- [x] lexer
- [x] parser
- [x] symbol table
- [x] global variables
- [x] bytecode generator
- [ ] local scopes
- [ ] functions and function calls
- [ ] implement vm
- [ ] implement modules
- [ ] implement std
