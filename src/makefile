CC=gcc
RM=rm -f
CFLAGS=-Wall -std=c99 -O3 #$(shell root-config --cflags)
LDFLAGS= #$(shell root-config --ldflags)
LDLIBS=-lm #$(shell root-config --libs)

SRCS=camera.c color.c debug.c job.c main.c sphere.c point_light.c ppm.c ray.c render.c world.c vector3.c material.c timer.c triangle.c primitive.c
OBJS=$(subst .c,.o,$(SRCS))

all: light

light: $(OBJS)
	$(CC) $(LDFLAGS) -o light $(OBJS) $(LDLIBS) 

depend: .depend

.depend: $(SRCS)
	rm -f ./.depend
	$(CC) $(CFLAGS) -MM $^>>./.depend;

clean:
	$(RM) $(OBJS)

dist-clean: clean
	$(RM) *~ .depend

include .depend
