    Simple structure :

Instruction: opérande1, [opérande2]

Instructions :
 - var: name, type                      CREATE VARIABLE
 - set: var, (var|value)                SET VARIABLE VALUE
 - add: (var|value), (var|value)        ADD VALUE TO VARIABLE
 - sub: (var|value), (var|value)        SUBTRACT VALUE TO VARIABLE
 - mul: (var|value), (var|value)        MULTIPLY VALUE TO VARIABLE
 - div: (var|value), (var|value)        DIVIDE VALUE WITH VARIABLE
 - rst: (var|value), (var|value)        DIVIDE VALUE WITH VARIABLE (give the rest)
 - ret: var                             SET res VARIABLE INTO VAR
 - flg: name                            CREATE FLAG
 - gto: (var|value)                     GO TO INSTRUCTION
 - jmp: var, (var|value)                IF op1 IS 0 JUMP TO op2 FLAG
 - jne: var, (var|value)                IF op1 IS NOT 0 JUMP TO op2 FLAG
 - ctp: var, type                       CHANGE TYPE OF VAR
 - nll: nll                             DO NOTHING AND IS IGNORED BY INTERPRETER
 - prt: (var|value)                     PRINT VALUE TO THE SCREEN


Operations that have only one operand : ret, flg, gto, nll, prt
Types of variable possible : int, flt, str, chr, tab, dic
Reserved variable names begin by _

Operations instructions ('add', 'sub', 'div', 'mul'...) don't change operands value, but the result is set in variable 'res'
A flag is a variable that contains the id of the instruction it points to (the one that sits just after the 'flg' instruction)



Functions :
```
new Fun:name:(arg1, arg2) {
    // Commands here
    return var
}
```

Flow :
```
new Flow:name:loop {
    // That code will be looped on any device it founds
    message('msgName', 'what you want');
    return final
}
finalMsg = name.launch -> dispatcher()
```
