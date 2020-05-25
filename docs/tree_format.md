How an `ARSTreeItem` is structured:  
`key` property:
* If it is a `String`, it contains the key at least starting with a bracket, but does not have to end with it. If it does not contain brackets, interpret it as a string literal
* If it is a `Vec<ARSTreeItem>`, it contains a list of `ARSTreeItems`, which should be interpreted and concatenated

`parameter` property:
* If it is a `String`, it contains the key at least ending with a bracket, but does not have to start with it. If it does not contain brackets, interpret it as a string literal
* If it is a `Vec<ARSTreeItem>`, it contains a list of `ARSTreeItems`, which should be interpreted and concatenated