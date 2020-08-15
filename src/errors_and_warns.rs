#[derive(Debug, PartialEq)]
pub enum Error {
	NoParameterSupplied,
	EmptyCondition,
	EmptyIfConditionTrue,
	BracketsInCond,
	ParameterDelimAfterCondFalse,
	EmptyParameter,
}
#[derive(Debug, PartialEq)]
pub enum Warning {
	UnclosedKeys,
}
