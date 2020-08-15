#[derive(Debug)]
pub enum Error {
	NoParameterSupplied,
	EmptyCondition,
	EmptyIfConditionTrue,
	BracketsInCond,
	ParameterDelimAfterCondFalse,
	EmptyParameter,
}
#[derive(Debug)]
pub enum Warning {
	UnclosedKeys,
}
