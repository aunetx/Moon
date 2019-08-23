flg: a
var: a, int
set: a, 0
var: b, int
set: b, 0
gto: a
flg: table1

flg: table2

mul: a, b
prt: _res

add: b, 1
ret: b

sub: 1, b
jne: _res, table2

set: b, 0

add: a, 1
ret: a

sub: 11, a
jne: _res, table1
