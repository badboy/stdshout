#define _GNU_SOURCE
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include <limits.h>
#include <fcntl.h>

__attribute__((constructor)) void reopen_stdout(void)
{
	static FILE *temp_out = NULL;
	static int old_stdout = 0;
	static int fildes[2];
	static char *stdshout = NULL;

	if (temp_out == NULL) {
		stdshout = getenv("STDSHOUT_EXE");
		if (!stdshout || strlen(stdshout) == 0) {
			fprintf(stderr, "Can't find stdshout exe. Set STDSHOUT_EXE to the full path\n");
			return;
		}

		char linkname[PATH_MAX];
		ssize_t r = readlink("/proc/self/exe", linkname, PATH_MAX);
		if (r == -1) {
			fprintf(stderr, "Can't read /proc/self/exe\n");
			return;
		}
		linkname[r] = '\0';

		if (strcmp(linkname, stdshout) == 0) {
			return;
		}

		old_stdout = dup(fileno(stdout));

		int status = pipe(fildes);
		if (status == -1) {
			fprintf(stderr, "Can't open pipe\n");
			return;
		}

		dup2(fildes[1], fileno(stdout));

		switch (fork()) {
			case -1:
				fprintf(stderr, "Can't fork\n");
			case 0: /* child */
				close(fildes[1]);

				dup2(fildes[0], fileno(stdin));
				dup2(old_stdout, fileno(stdout));

				char *args[] = {
					"stdshout",
					NULL
				};

				execv(stdshout, args);
				fprintf(stderr, "Something went wrong\n");
				break;
			default:
				close(fildes[0]);

		}
	}
}
