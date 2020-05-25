How an `ARSTreeItem` is structured:  
`key` and `parameter` property:
* If it is a `String`, it contains the key enclosed in brackets. If it does not contain brackets, interpret it as a string literal
* If it is a `Vec<ARSTreeItem>`, it contains a list of `ARSTreeItems`, which should be interpreted and concatenated