#include <stdio.h>
#include <stdlib.h>

#if defined (__SVR4) && defined (__sun)
#include <sys/loadavg.h>
#endif

#define LOAD_COUNT 3

int main()
{
        double loads[LOAD_COUNT];

        if (getloadavg(loads, LOAD_COUNT) < LOAD_COUNT) {
		fputs("not enough samples", stderr);
		exit(1);
	}

	printf("%03.2f %03.2f %03.2f\n", loads[0], loads[1], loads[2]);

	return 0;
}
