# include <stdlib.h>
# include <math.h>
# include <stdio.h>

# include "sphere_lebedev_rule.h"

# define SIZE 5810  // size of the Lebedev grid
# define PI 3.14159265358979323846

int main () {

  double *X, *Y, *Z, *Th, *Ph, *W;
  double Y00 = 0.0, Y10 = 0.0, Y20 = 0.0, cos2theta = 0.0, zero_int = 0.0;
  int i;

  // Allocate memeory for spherical grid
  X = (double*) malloc (SIZE*sizeof(double));
  Y = (double*) malloc (SIZE*sizeof(double));
  Z = (double*) malloc (SIZE*sizeof(double));
  Th = (double*) malloc (SIZE*sizeof(double));
  Ph = (double*) malloc (SIZE*sizeof(double));
  W = (double*) malloc (SIZE*sizeof(double));

  double Thi = 0.0, Phi = 0.0;

  // Generate grid
  ld5810 (X, Y, Z, W);
  // Prepare grid in spherical coordinates
  for (i = 0; i<SIZE; i++) {
     xyz_to_tp (X[i], Y[i], Z[i], Th+i, Ph+i);
  }
  // free cartesian coordinates
  free (X);
  free (Y);
  free (Z);

  // evaluate integrals of spherical harmonics
  for (i = 0; i<SIZE; i++) {
     Thi = Th[i] * (PI / 180.0);
     Phi = Ph[i] * (PI / 180.0);
     printf("(%.2f, %.2f, %.9f), %.9f, %.9f\n", Thi * (180.0 / PI), Phi * (180.0 / PI), W[i], cos(2.0 * Thi), cos(2.0 * Phi));
     zero_int += W[i]*(1 + 3.0 * cos(2.0 * Phi));
     cos2theta += W[i]*cos(Phi * 2.0);
     Y00 += W[i];
     Y10 += W[i]*cos(Phi);
     Y20 += W[i]*(3.0*cos(Phi)*cos(Phi) - 1.0);
  }
  
  zero_int *= 4.0*PI;
  cos2theta *= 4.0*PI;
  Y00 *= 0.5*1.0/sqrt(PI)*4.0*PI;
  Y10 *= 0.5*sqrt(3.0/PI)*4.0*PI;
  Y20 *= 0.25*sqrt(5.0/PI)*4.0*PI;


  printf ("grid = %d, zero_int = %.20f, cos(2\\theta) = %.20f, Y00 = %.20f, Y10 = %.20f, Y20 = %.20f\n", SIZE, zero_int, cos2theta, Y00, Y10, Y20);


  // free memory
  free (Th);
  free (Ph);
  free (W);

  return 0;
}