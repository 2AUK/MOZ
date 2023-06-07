# include <stdlib.h>
# include <math.h>
# include <stdio.h>

# include "sphere_lebedev_rule.h"

# define SIZE 14  // size of the Lebedev grid
# define PI 3.14159265358979323846

int main () {

  double *X, *Y, *Z, *Th, *Ph, *W;
  double Y00 = 0.0, Y10 = 0.0, Y20 = 0.0;
  int i;

  // Allocate memeory for spherical grid
  X = (double*) malloc (SIZE*sizeof(double));
  Y = (double*) malloc (SIZE*sizeof(double));
  Z = (double*) malloc (SIZE*sizeof(double));
  Th = (double*) malloc (SIZE*sizeof(double));
  Ph = (double*) malloc (SIZE*sizeof(double));
  W = (double*) malloc (SIZE*sizeof(double));

  // Generate grid
  ld0014 (X, Y, Z, W);
  // Prepare grid in spherical coordinates
  for (i = 0; i<SIZE; i++) {
     xyz_to_tp (X[i], Y[i], Z[i], Ph+i, Th+i);
  }
  // free cartesian coordinates
  free (X);
  free (Y);
  free (Z);

  // evaluate integrals of spherical harmonics
  for (i = 0; i<SIZE; i++) {
     Y00 += W[i];
     Y10 += W[i]*cos(Th[i]);
     Y20 += W[i]*(3.0*cos(Th[i])*cos(Th[i]) - 1.0);
  }

  Y00 *= 0.5*1.0/sqrt(PI)*4.0*PI;
  Y10 *= 0.5*sqrt(3.0/PI)*4.0*PI;
  Y20 *= 0.25*sqrt(5.0/PI)*4.0*PI;


  printf ("Y00 = %.20f, Y10 = %.20f, Y20 = %.20f\n", Y00, Y10, Y20);


  // free memory
  free (Th);
  free (Ph);
  free (W);

  return 0;
}