#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <math.h>
#define PI 3.14159265358979
#include <mpi.h>

double estimate_pi ( int i, int n )
{
   srand(time(NULL)+i);

   int j, cnt=0;
   for (j = 0; j < n; j++) {
      double x = ((double) rand() / (RAND_MAX));
      double y = ((double) rand() / (RAND_MAX));
      double z = x*x + y*y;
      if (z <= 1.0) cnt++;
   }
   double estimate = (4.0*cnt)/n;
   printf("Core %d estima Pi em : %.15f", i, estimate);
   printf("  erro : %.3e\n", fabs(estimate-PI));

   return estimate;
}

int main (int argc, char *argv[])
{
   int id,np;

   MPI_Init(&argc, &argv);
   MPI_Comm_size(MPI_COMM_WORLD, &np);
   MPI_Comm_rank(MPI_COMM_WORLD, &id);

   int n;

   if (id == 0) {
      printf("Numero de amostras desejadas...\n");
      scanf("%d", &n);
   }
   MPI_Bcast(&n, 1, MPI_INT, 0, MPI_COMM_WORLD);
   double est4pi = estimate_pi(id, n);
   double sum = 0.0;
   MPI_Reduce(&est4pi, &sum, 1, MPI_DOUBLE, MPI_SUM, 0, MPI_COMM_WORLD);

   if(id == 0)
   {
      est4pi = sum/np;
      printf("Estimativa final de Pi: %.15lf", est4pi);
      printf("  erro : %.3e\n", fabs(est4pi - PI));
   }
   MPI_Finalize();

   return 0;
}