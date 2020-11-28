#include <stdio.h>
#include <stdlib.h>
#include <math.h>

float f(float x){
  return sqrt(1-x*x);
}

float simps(float a, float b, float n) {
  float h,sum1=0,sum2=0,sum,y0,yn;
  int i;
  h = (b-a)/n;
  y0=f(a+0*h);
  yn=f(a+n*h);
  for(i=1; i<n; i++) {
    if(i%2 == 0)
      sum1 = sum1 + f(a+i*h);
    else
      sum2 = sum2 + f(a+i*h);
  } 
  sum = (h/3)*(y0 + yn + 2*sum1 +4*sum2);
  return sum;
}

int main(){
  float Pi;
  Pi = 4*simps(0, 1, 1000);
  printf("Valor:     %f", Pi);
  printf("\nPrecisão: %f\n",(Pi-M_PI));
	return 0;
}