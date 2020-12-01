#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <math.h>
#define PI 3.14159265358979
#include <mpi.h>

int cnt_pi ( int i, int n )
{
   srand(time(NULL)+i);
   int j,cnt=0;
   for(j=0; j<n; j++)
   {
      double x = ((double) rand() / (RAND_MAX));
      double y = ((double) rand() / (RAND_MAX));
      double z = x*x + y*y;
      if(z <= 1.0) cnt++;
   }
   printf("Core %d identificou %d amostras dentro da circunferÃªncia\n",i,cnt);
   return cnt;
}

int main ( int argc, char *argv[] )
{
   int id,np;

   MPI_Init(&argc,&argv);
   MPI_Comm_size(MPI_COMM_WORLD,&np);
   MPI_Comm_rank(MPI_COMM_WORLD,&id);

   int n;

   if(id == 0)
   {
      printf("NÃºmero de amostras desejadas...\n");
      scanf("%d",&n);
   }
   int fraction = n/np;
   MPI_Bcast(&fraction,1,MPI_INT,0,MPI_COMM_WORLD);
   double est4pi = cnt_pi(id,fraction);
   double sum = 0.0;
   MPI_Reduce(&est4pi,&sum,1,MPI_DOUBLE,MPI_SUM,0,MPI_COMM_WORLD);

   if(id == 0)
   {
      est4pi = 4.0*sum/n;
      printf("\nEstimativa de Pi : %.15lf",est4pi);
      printf("  erro : %.3e\n",fabs(est4pi-PI));
   }
   MPI_Finalize();

   return 0;
}