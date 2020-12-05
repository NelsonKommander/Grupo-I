// Compilando:
// gcc sequencial.c -Wall $(apr-1-config --cflags --cppflags --includes --link-ld) -o bin

// Definição do maior tamanho que as linhas impressas vão ocupar
#define MAXIMUM_LINE_WIDTH  60

#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

typedef off_t off64_t; // A lib apr enlouquece dependendo do sistema e essa definição de tipo previne isso

// Pode ser que essa lib esteja em um lugar diferente no seu sistema, precisei colocar o apr-1.0
#include <apr-1.0/apr_pools.h>

// Buscando o inteiro nativo do sistema
typedef intptr_t intnative_t;

typedef struct tree_node{
   struct tree_node   * left_Node, * right_Node;
} tree_node;


// Criar as arvores
static inline tree_node * create_Tree(const intnative_t tree_Depth, 
apr_pool_t * const memory_Pool){
   tree_node * const root_Node = apr_palloc(memory_Pool, sizeof(tree_node));

   if (tree_Depth > 0){
      root_Node->left_Node=create_Tree(tree_Depth-1, memory_Pool);
      root_Node->right_Node=create_Tree(tree_Depth-1, memory_Pool);
   } else
      root_Node->left_Node=root_Node->right_Node=NULL;

   return root_Node;
}


// Computar checksum
static inline intnative_t compute_Tree_Checksum(
const tree_node * const root_Node){
   
   if (root_Node->left_Node)
      return compute_Tree_Checksum(root_Node->left_Node) + compute_Tree_Checksum(root_Node->right_Node) + 1;
   else
      return 1;
}


int main(int argc, char ** argv){
   // O tamanho minimo da arvore vai ser 4
   // O tamanho maximo da entrada vai ser o tamanho minimo mais dois ou a entrada
   const intnative_t minimum_Tree_Depth = 4;
   intnative_t maximum_Tree_Depth = atoi(argv[1]);
   if (maximum_Tree_Depth < minimum_Tree_Depth+2)
      maximum_Tree_Depth = minimum_Tree_Depth+2;

   apr_initialize();
   apr_pool_t * memory_Pool;

   // Criando a arvore pra brincar com a memoria
   apr_pool_create_unmanaged(&memory_Pool);
   tree_node * stretch_Tree=create_Tree(maximum_Tree_Depth + 1, memory_Pool);
   printf("stretch tree of depth %jd\t check: %jd\n", (intmax_t)maximum_Tree_Depth+1, (intmax_t)compute_Tree_Checksum(stretch_Tree));
   apr_pool_destroy(memory_Pool);

   // Criando a pool onde a arvore esquecida vai ficar
   apr_pool_create_unmanaged(&memory_Pool);
   // Criando a arvore esquecida
   tree_node * long_Lived_Tree=create_Tree(maximum_Tree_Depth, memory_Pool);

   // Buffer que vai guardar as saidas em ordem
   char output_Buffer[maximum_Tree_Depth+1][MAXIMUM_LINE_WIDTH+1];

   for (intnative_t depth = minimum_Tree_Depth; depth <= maximum_Tree_Depth; depth += 2) {
      intnative_t totalChecksum = 0;
      intnative_t iterations = 1 << (maximum_Tree_Depth - depth + minimum_Tree_Depth);

      apr_pool_t* execPool;
      apr_pool_create_unmanaged(&execPool);

      for (intnative_t i = 0; i <= iterations; i++) {
            tree_node* const arvore = create_Tree(depth, execPool);
            totalChecksum += compute_Tree_Checksum(arvore);
            apr_pool_clear(execPool);
      }

      apr_pool_destroy(execPool);

      sprintf(output_Buffer[depth], "%jd\t trees of depth %jd\t check: %jd\n", (intmax_t)iterations, (intmax_t)depth, (intmax_t)totalChecksum);
   }

   // Imprime as estatisticas da saida
   for (intnative_t depth = minimum_Tree_Depth; depth <= maximum_Tree_Depth; depth+=2)
      printf("%s", output_Buffer[depth]);

   // Imprime as estatisticas da arvore esquecida
   printf("long lived tree of depth %jd\t check: %jd\n", (intmax_t)maximum_Tree_Depth, (intmax_t)compute_Tree_Checksum(long_Lived_Tree));
   apr_pool_destroy(memory_Pool);

   apr_terminate();
   return 0;
}