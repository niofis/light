#include <stdio.h>
#include <SDL2/SDL.h>

#include <stdio.h>
#include <stdlib.h>

#if !defined(byte)
  #define byte unsigned char
#endif

#ifdef int32_t
  #define int int32_t
#endif

#define BPP 4

#include "timer.h"
#include "ppm.h"
#include "job.h"
#include "render.h"

int gui_init()
{
  int video_flags = SDL_SWSURFACE;
  int bpp = 32;
  int running = 1;
  SDL_Window *window;
  SDL_Renderer *renderer;
  SDL_Texture *texture;
  SDL_Event event;
  struct job_desc* job;
  struct timer timer;

  job = job_new();
  
  if (SDL_Init(SDL_INIT_VIDEO) != 0)
  {
    printf("SDL_Init failed: %s\n", SDL_GetError());
	SDL_Quit();
    return 1;
  }

  window = SDL_CreateWindow("Light", 100, 100, 1280, 720, SDL_WINDOW_SHOWN);
  if (window == NULL)
  {
	  printf("SDL_CreateWindow failed: %s\n", SDL_GetError());
	  SDL_Quit();
	  return 1;
  }

  renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
  if (renderer == NULL)
  {
	  SDL_DestroyWindow(window);
	  printf("SLD_CreateRenderer failed: %s\n", SDL_GetError());
	  SDL_Quit();
	  return 1;
  }

  texture = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STREAMING, job->width, job->height);
  if (texture == NULL)
  {
	  SDL_DestroyRenderer(renderer);
	  SDL_DestroyWindow(window);
	  printf("SDL_CreateTexture failed: %s\n", SDL_GetError());
	  return 1;
  }

  timer_start(&timer);
  render(job);
  timer_stop(&timer);

  printf("Render time = %fs\n", timer.elapsed);


  SDL_UpdateTexture(texture, NULL, job->buffer, job->width * sizeof(Uint32));
  SDL_RenderClear(renderer);
  SDL_RenderCopy(renderer, texture, NULL, NULL);
  SDL_RenderPresent(renderer);

  while (running)
  {
    if (SDL_PollEvent(&event))
    {
      switch (event.type)
      {
        case SDL_KEYDOWN:

          if (event.key.keysym.sym == SDLK_ESCAPE)
          {
            running = 0;
          }
          
          break;
      }
    }
  }

  SDL_DestroyTexture(texture);
  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  
  SDL_Quit();
  
  job_del(job);

  return 0;
}

int main(int argc, char **argv)
{
  gui_init();

  return 0;
}