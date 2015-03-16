#include <stdio.h>
#include <SDL/SDL.h>

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

#include <immintrin.h>

int gui_init()
{
  int video_flags = SDL_SWSURFACE;
  int bpp = 32;
  int running = 1;
  SDL_Surface* screen;
  SDL_Surface* surface;
  SDL_Event event;
  struct job_desc* job;
  struct timer timer;
  float constants[] = {0, 1,2, 3, 1.0f, 4.0f};
  __m256 ymm0 = _mm256_broadcast_ss(constants);

  job = job_new();
  
  if (SDL_Init(SDL_INIT_VIDEO) < 0)
  {
    printf("SDL_Init failed: %s\n", SDL_GetError());
    return 1;
  }

  SDL_WM_SetCaption("Light", NULL);

  surface = SDL_CreateRGBSurface(video_flags, job->width, job->height, bpp, 0, 0, 0, 0);

  screen = SDL_SetVideoMode(job->width, job->height, bpp, video_flags);
  if (!screen)
  {
    printf("SDL_SetVideoMode failed: %s\n", SDL_GetError());
    return 2;
  }

  timer_start(&timer);
  render(job);
  timer_stop(&timer);

  printf("Render time = %fs\n", timer.elapsed);

  SDL_LockSurface(surface);
  memcpy(surface->pixels, job->buffer, sizeof(int) * job->width * job->height);
  SDL_UnlockSurface(surface);

  SDL_BlitSurface(surface, NULL, screen, NULL);
  SDL_Flip(screen);
  

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
    
    //SDL_BlitSurface(surface, NULL, screen, NULL);
    //SDL_Flip(screen);
  }

  SDL_FreeSurface(screen);
  SDL_FreeSurface(surface);
  SDL_Quit();
  
  job_del(job);

  return 0;
}

int main(int argc, char *argv[])
{
  gui_init();

  return 0;
}
