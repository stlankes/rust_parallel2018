#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <pthread.h>

#define NTHREADS 2

const unsigned int num_steps = 100000000;

int pi_serial(void)
{
	double x, sum, step;
	int i;
	struct timeval start, end;

	gettimeofday(&start, NULL);

	sum = 0.0;
	step = 1.0 / (double)num_steps;

	for (i = 0; i < num_steps; i++) {
		x = (i + 0.5) * step;
		sum += 4.0 / (1.0 + x * x);
	}

	gettimeofday(&end, NULL);

	printf("PI (serial) = %f\n", sum * step);
	printf("Time : %lf sec\n", (double) (end.tv_sec - start.tv_sec) + (double) (end.tv_usec - start.tv_usec)/1000000.0);

	return 0;
}

typedef struct {
	int start, end;
} thread_param_v1;

double step, sum;

void *thread_func_wrong(void *arg)
{
	thread_param_v1 *thr_arg = (thread_param_v1 *) arg;
	double x;
	int i;

	for (i = thr_arg->start; i < thr_arg->end; i++) {
		x = (i + 0.5) * step;
		sum += 4.0 / (1.0 + x * x);
	}

	return 0;
}

int pi_wrong(void)
{
	int i;
	struct timeval start, end;
	pthread_t threads[NTHREADS];
	thread_param_v1 thr_arg[NTHREADS];

	gettimeofday(&start, NULL);

	sum = 0.0;
	step = 1.0 / (double)num_steps;

	/* Create NTHREADS worker threads. */
	for (i = 0; i < NTHREADS; i++) {
		/* initialize arguments of the thread  */
		thr_arg[i].start = i * (num_steps / NTHREADS);
		thr_arg[i].end = (i + 1) * (num_steps / NTHREADS);

		pthread_create(&(threads[i]), NULL, thread_func_wrong, &(thr_arg[i]));
	}

	/* Wait until all threads have terminated */
	for (i = 0; i < NTHREADS; i++)
		pthread_join(threads[i], NULL);
	gettimeofday(&end, NULL);

	printf("PI (wrong) = %f\n", sum * step);
	printf("Time : %lf sec\n", (double)(end.tv_sec - start.tv_sec)+(double)(end.tv_usec - start.tv_usec)/1000000.0);

	return 0;
}

typedef struct {
	int start, end;
	double sum;
} thread_param_v2;

void *thread_func_correct(void *arg)
{
	thread_param_v2 *thr_arg = (thread_param_v2 *) arg;
	double x, sum = 0.0;
	int i;

	for (i = thr_arg->start; i < thr_arg->end; i++) {
		x = (i + 0.5) * step;
		sum += 4.0 / (1.0 + x * x);
	}

	thr_arg->sum = sum;

	return NULL;
}

int pi_correct(void)
{
	int i;
	struct timeval start, end;
	pthread_t threads[NTHREADS];
	thread_param_v2 thr_arg[NTHREADS];

	gettimeofday(&start, NULL);

	step = 1.0 / (double)num_steps;

	/* Create NTHREADS worker threads. */
	for (i = 0; i < NTHREADS; i++) {
		/* initialize arguments of the thread  */
		thr_arg[i].start = i * (num_steps / NTHREADS);
		thr_arg[i].end = (i + 1) * (num_steps / NTHREADS);
		thr_arg[i].sum = 0.0;

		pthread_create(&(threads[i]), NULL, thread_func_correct, &(thr_arg[i]));
	}

	sum = 0.0;

	/* Wait until all threads have terminated */
	for (i = 0; i < NTHREADS; i++) {
		pthread_join(threads[i], NULL);
		sum += thr_arg[i].sum;
	}

	gettimeofday(&end, NULL);

	printf("PI (correct) = %f\n", sum * step);
	printf("Time : %lf sec\n", (double)(end.tv_sec - start.tv_sec)+(double)(end.tv_usec - start.tv_usec)/1000000.0);

	return 0;
}

int main(int argc, char **argv)
{
	pi_serial();
	pi_wrong();
	pi_correct();
	return 0;
}
