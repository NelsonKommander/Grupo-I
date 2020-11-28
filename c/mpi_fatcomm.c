#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <mpi.h>

unsigned long int fat(int n, int size){
  if (n > 0)
    return n*fat(n-size, size);
  else
    return 1;
}
int main(int argc, char *argv[])  {
   unsigned long int resul, parcial;
   int        n = 10;
   if (argc > 1) {n = atoi(argv[1]);}
   int        comm_sz;
   int        my_rank;

   const unsigned long int BUFSIZE = 1;

   MPI_Init(NULL, NULL);
   MPI_Comm_size(MPI_COMM_WORLD, &comm_sz);
   MPI_Comm_rank(MPI_COMM_WORLD, &my_rank);

   if (my_rank == 0) {
      parcial = fat(n, comm_sz);
      for (int q = 1; q < comm_sz; q += 1) {
         MPI_Recv(&resul, BUFSIZE, MPI_UNSIGNED_LONG, q, 0, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
         parcial = parcial * resul;
      }
      printf("O resultado do fatorial de %d é = %lu\n", n, parcial);
   } else {
      resul = fat(n-my_rank, comm_sz);
      MPI_Send(&resul, BUFSIZE, MPI_UNSIGNED_LONG, 0, 0, MPI_COMM_WORLD);
   }

   MPI_Finalize();

   return 0;
}
