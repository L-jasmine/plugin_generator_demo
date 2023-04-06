#include "demo.h"
#include <stdio.h>
#include <stdlib.h>

struct Bar {
  int data;
};

Bar *create_bar(int data) {
  Bar *bar = malloc(sizeof(Bar));
  bar->data = data;
  return bar;
}

void print_foo_bar(Foo foo) { printf("Bar->data = %d\n", foo.b->data); }
