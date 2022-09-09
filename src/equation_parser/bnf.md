# BNF

```
<sentence> = <poly> ( ',' <poly> )*
<poly> = <term> ( '+' <term> | '-' <term> )*
<term> = <num>? <mono>
<mono> = <var>*
<var> = 'x_' <num> ( '^' <num> )?
```