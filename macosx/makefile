CC=gcc
RM=rm -f
CFLAGS=-Wall -std=c99 -O3 -I../src -I/usr/local/include #$(shell root-config --cflags)
LDFLAGS= #$(shell root-config --ldflags)
LDLIBS=-lm -L/usr/local/lib -lSDL2 -lSDL2_gfx -lassimp -Wl,-framework,Cocoa#$(shell root-config --libs)
VPATH=../src
SRCS=camera.c color.c bvh.c world.c job.c main_gui.c point_light.c ray.c render.c vector3.c material.c timer.c triangle.c list.c aabb.c debug.c primitive.c sphere.c
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
