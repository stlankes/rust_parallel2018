CC=g++
CFLAGS=-O3 -Wall

default: mem

mem.o: mem.cpp
	$(CC) $(CFLAGS) -c -o $@ $<

mem: mem.o
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -rf *.o
