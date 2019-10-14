/* -*- mode: c -*- */

/*
 * loadavg.c
 */

#include <inttypes.h>           /* strtoimax */
#include <stdio.h>              /* fprintf, fputs, snprintf */
#include <stdlib.h>             /* exit, getloadavg(macOS) */
#include <unistd.h>             /* getopt */


#define BUFSIZE 128
#define MAX_SAMPLES 3
#define USAGE() { fprintf(stdout, USAGE_FORMAT, argv[0]); exit(1); }
#define WARN(msg) { fprintf(stderr, "%s: %s\n", argv[0], msg); }
#define ERROR(msg) { WARN(msg); exit(1); }


const char * const USAGE_FORMAT = "usage: %s [-s N]\n"
  "\t-s N\tCollect N number of samples\n";


int format_nums(double *nums, int c, char * const buf, int s);


int main(int argc, char **argv) {
  char buffer[BUFSIZE] = {0};
  double loads[MAX_SAMPLES] = {0.0};
  int count = MAX_SAMPLES;
  int ch;

  while ((ch = getopt(argc, argv, "s:")) != -1) {
    switch (ch) {
    case 's':
      count = (int)strtoimax(optarg, NULL, 10);
      if (count < 1 || count > MAX_SAMPLES) {
        USAGE();
        ERROR("count out of range");
      }
      break;
    case '?':
    default:
      USAGE();
      break;
    }
  }
  argc -= optind;
  argv += optind;

  if (getloadavg(loads, count) < count) {
    ERROR("failed to collect load samples");
  }

  int written = format_nums(loads, count, buffer, sizeof(buffer));
  if (written < 0) {
    ERROR("no format written");
  }

  fputs(buffer, stdout);

  return 0;
}


int
format_nums(double *nums, int c, char * const buf, int s) {
  char *cur = buf;

  while (c-- > 0) {
    int n = snprintf(cur, (size_t)(s - (cur - buf)), "%03.2f ", *nums++);
    if (n < 0) {
      return (-1);
    }
    cur += n;
  }
  *(cur - 1) = '\n';

  return (int)(cur - buf);
}
