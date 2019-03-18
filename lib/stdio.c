#include <stdio.h>
#include <stdint.h>
#include <stdarg.h>

const char* floatFormat = "%f\n";
const char* IntFormat = "%d\n";
const char* strFormat = "%s\n";

void print(const char* format, ... ) {
    va_list ap;
    va_start(ap, format);
    vprintf(format, ap);
    va_end(ap);
}
