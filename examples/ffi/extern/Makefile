CC=gcc
CFLAGS=-fPIC -Wall

default: libhello.so

hello.o: hello.c
	$(CC) $(CFLAGS) -c -o $@ $<

libhello.so: hello.o
	$(CC) -shared -o $@ $<

clean:
	rm -rf *.o
