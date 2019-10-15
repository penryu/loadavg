/* -*- mode: c -*- */
/*
 * loadavg.c
 */

#include <stdio.h>              /* fprintf, */
#include <stdlib.h>             /* getloadavg(macOS) */


#define BUFSIZE 16
#define MAX_SAMPLES 3


int main() {
  double ls[MAX_SAMPLES] = {0.0};
  getloadavg(ls, MAX_SAMPLES);
  printf("%03.2f %03.2f %03.2f\n", ls[0], ls[1], ls[2]);
  return 0;
}

