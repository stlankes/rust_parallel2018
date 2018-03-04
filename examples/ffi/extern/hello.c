#include <stdio.h>

struct rust_object {
    int number;
};

int c_hello(void)
{
	printf("Hello world from C\n");

	return 0;
}

int c_print_object(struct rust_object* obj)
{
	printf("Number %d\n", obj->number);

	return 0;
}
