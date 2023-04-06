#include <stdio.h>
#include "demo.h"

int main() {
  Bar *bar = create_bar(1);
  Foo f = {.a = 1, .b = bar};
  print_foo_bar(f);
}
