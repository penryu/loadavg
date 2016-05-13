#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#if defined (__SVR4) && defined (__sun)
#include <sys/loadavg.h>
#endif

#define MAX_SAMPLES 3

void usage() {
  printf("Usage: loadavg [-c N]\n");
}

int main(int argc, char **argv) {
  double loads[MAX_SAMPLES];
  int loadCount = 3;
  int ch;

  while ((ch = getopt(argc, argv, "c:")) != -1) {
    switch (ch) {
    case 'c':
      loadCount = atoi(optarg);
      if (loadCount < 1 || loadCount > MAX_SAMPLES) {
        usage();
        fprintf(stderr, "%s: count must be in range 1-%d\n", argv[0], MAX_SAMPLES);
        exit(1);
      }
      break;
    case '?':
    default:
      usage();
      break;
    }
  }
  argc -= optind;
  argv += optind;

  if (getloadavg(loads, loadCount) < loadCount) {
    fprintf(stderr, "not enough samples");
    exit(1);
  }

  for (int i = 0; i < loadCount; ++i) {
    printf("%03.2f ", loads[i]);
  }
  printf("\n");

  return 0;
}
