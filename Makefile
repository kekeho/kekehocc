CFLAGS=-std=c11 -g -static

kekehocc: kekehocc.c

test: kekehocc
	./test.sh

clean:
	rm -f kekehocc *.o *~ tmp*

.PHONY: test clean
