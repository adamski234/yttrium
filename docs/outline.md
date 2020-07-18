# General idea
You pass a string in the ARS language to the `compile_ars` function, which returns the compiled code as a byte array and throws an error if the syntax is invalid. Then, on triggering the rule, you pass the compiled code into `run_compiled`, including the message content as the second parameter. The function then returns as object containing the type of the result (text/embed), and the result itself.

# ARS format
The input format takes the form of `{key:key parameters}`, but there are exceptions. The database key take the form of `{key?param:param2}` and embeds are created with nested keys.

ARS supports key nesting (using the result of one key as a parameter to another key)

## Statements
Statements have following format:
```
{cond:{test}:{cond_true}:{cond_false}}
```
`cond` is a statement that takes a condition and a statement to return if the condition is true. It can also optionally take a statement to return if the condition is false
Separator between keys is still being chosen

# Bytecode format
Bytecode is in the form of `opcode parameters`. `opcode` is an unsigned short (2 bytes), and `parameters` is a string of arbitrary length and content. It gets passed to the function of the opcode as it is, with no modifications

# Plugin system
WIP

# Tests
WIP