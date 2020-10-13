#include "./qalc.hpp"
#include <libqalculate/Calculator.h>
//This is a tiny wrapper around c++ classes
std::unique_ptr<std::string> calculate(rust::Str expression) {
	auto calculator = std::make_unique<Calculator>();
	return std::make_unique<std::string>(calculator->calculateAndPrint(std::string(expression)));
}