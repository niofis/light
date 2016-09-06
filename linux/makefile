CC=gcc
RM=rm -f
CFLAGS=-march=native -Wall -std=c11 -O3 -I../include #$(shell root-config --cflags)
LDFLAGS= #$(shell root-config --ldflags)
LDLIBS=-lm -lSDL2 -lassimp #-lSDL2main #-Wl,-framework,Cocoa#$(shell root-config --libs)
VPATH=../src
SRCS=camera.c color.c debug.c job.c main_gui.c point_light.c ppm.c ray.c render.c world.c vector3.c material.c timer.c triangle.c list.c aabb.c bvh.c
OPATH=../obj

#%.o : %.c
#	$(CC) $(CFLAGS) -c $< -o $(OPATH)/$@

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
