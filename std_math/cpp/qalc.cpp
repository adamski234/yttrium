#include "qalc.hpp"
#include <libqalculate/Calculator.h>
//This is a tiny wrapper around c++ classes
extern std::string calculate(std::string expression) {
	Calculator calculator = new Calculator();
	return calculator.calculateAndPrint(expression);
}