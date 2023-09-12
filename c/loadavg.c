/* -*- mode: c -*- */

/* loadavg.c */

#include <stdio.h>              /* fprintf */
#include <stdlib.h>             /* getloadavg(FreeBSD, Linux, macOS) */

#if defined(__sun) && defined(__SVR4)
#include <sys/time.h>           /* hrtime_t (used in `sys/loadavg.h`) */
#include <sys/loadavg.h>        /* getloadavg(smartOS) */
#endif

static double Samples[3];

int main(void) {
  getloadavg(Samples, 3);
  printf("%03.2f %03.2f %03.2f\n", Samples[0], Samples[1], Samples[2]);
  return 0;
}
