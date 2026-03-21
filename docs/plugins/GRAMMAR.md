# Grammar

- `[x]` denotes zero or one occurrences of x.
- `{x}` denotes zero or more occurrences of x.
- `(x | y)` means one of either x or y.
- different lines mean multiple possible implementations

All uppercase building blocks are _nonterminal_ and lowercase ones _terminal_ - will not be split further.

## Basis

```
Identifier:
    string variable length (letters, numbers, _ and -)

Value:
    IntValue
    FloatValue
    StringValue
    BoolValue
    ArrayValue

Type:
    int
    float
    string
    bool
    Type[]
    ObjectType

ObjectType:
    id
    game
    player
    stat
    tradable

Block:
    { {Stmt} }
```

## Program

```
Root:
    RootElement

RootElement:
    Stmt
    FnDecl
```

## Statements

```
Stmt:
    let Identifier : Type = Expression ;
    Identifier AssignmentOperator Expression ;
    Expression ;
    if ( Expression ) Block {else if ( Expression ) Block} [else Block]
    switch ( Expression ) { {(Expression| _ ) => Block} }
    while ( Expression ) Block
    do Block while ( Expression ) ;
    for ( ForControl ) Block
    break ;
    continue ;
    return [Expression] ;
    reject [Expression] ;
    throw [Expression] ;
    exec GameCommand [catch Block] ;

AssignmentOperator:
    =
    +=
    -=
    *=
    /=
    %=
    ^=
```

## Expressions

```
Expression:
    UnaryOperator Expression
    Expression BinaryOperator Expression
    ParenthesizedExpression
    FunctionCall
    Value

ParenthesizedExpression:
    Expression

UnaryOperator:
    !
    -

BinaryOperator:
    +
    -
    *
    /
    %
    ^
    ==
    !=
    <=
    >=
    &&
    ||
    as
```

## Functions

```
FnDecl:
    fn name ( [FnParamList] ) [FnReturnType] Block

FnParamList:
    FnParam {, FnParam}

FnParam:
    Identifier: Type

FnReturnType:
    -> Type FnReturnErrorIndicator
    -> Type FnReturnErrorIndicator ?

FunctionCall:
    Identifier ( Expression {, Expression} )
```
