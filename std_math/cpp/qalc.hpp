#include <string>
#include <memory>
#include "rust/cxx.h"
#include "std_math/src/lib.rs.h"
std::unique_ptr<std::string> calculate(rust::Str expression);