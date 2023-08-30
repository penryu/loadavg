/* -*- mode: c -*- */

/*
 * loadavg.c
 */

#include <stdio.h>              /* fprintf */
#include <stdlib.h>             /* getloadavg */

#define MAX_SAMPLES 3

static double ls[MAX_SAMPLES] = {0.0};

int main(void) {
  getloadavg(ls, MAX_SAMPLES);
  printf("%03.2f %03.2f %03.2f\n", ls[0], ls[1], ls[2]);
  return 0;
}
