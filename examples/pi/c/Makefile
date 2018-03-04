CC=gcc
CFLAGS=-O3 -Wall

default: pi

pi.o: pi.c
	$(CC) $(CFLAGS) -c -o $@ $<

pi: pi.o
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -rf *.o
