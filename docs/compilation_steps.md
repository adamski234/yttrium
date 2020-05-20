# Current idea
1. Pass the ARS string into the compile function
2. Split it into strings containing top level keys and parameters
3. Split the strings into objects containing
    1. The key name, as String
    2. The parameters, as String
4. Check the parameters for containing other keys
    1. If they contain other keys, repeat steps 2. and 3. on the parameter string
5. Compile the tree into bytecode