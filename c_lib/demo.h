typedef struct Bar Bar;

typedef struct Foo {
  int a;
  Bar *b;
} Foo;

Bar *create_bar(int data);
void print_foo_bar(Foo foo);
