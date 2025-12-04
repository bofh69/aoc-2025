#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef unsigned long int_t;

static bool read_int(int_t *result)
{
  int_t x = 0;
  bool ok = false;
  while (1) {
    int c =  fgetc(stdin);

    if ((c < '0') || (c > '9')) {
      if (c != EOF) {
        ungetc(c, stdin);
      }
      break;
    }
    x = x * 10 + (c - '0');
    ok = true;
  }
  *result = x;
  return ok;
}

static const int_t NUMS[] = {
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
  };

static int_t pow_int(int num)
{

  if (num <= 0) {
    return 1;
  }
  if (num >= (signed) (sizeof(NUMS) / sizeof(NUMS[0]))) {
    fprintf(stderr, "Trying to pow too hard: %d\n", num);
    return 0;
  }
  return NUMS[num];
}

static int n_digits(int_t num)
{
  for(int i = 0; i < (signed) (sizeof(NUMS)/sizeof(NUMS[0])) - 1; ++i) {
    if ((num >= NUMS[i]) && (num < NUMS[i+1])) {
      return i + 1;
    }
  }
  fprintf(stderr, "Too many digits: %ld\n", num);
  return 0;
}

static int_t half_digits(int_t n)
{
  int n_n = n_digits(n);
  int_t result = n / pow_int(n_n/2);

  return result;
}

static int_t double_digits(int_t n)
{
  int n_n = n_digits(n);
  int_t result = n * pow_int(n_n) + n;

  return result;
}

static int_t count_invalids(int_t from, int_t to)
{
  int from_digits = n_digits(from);
  if (from_digits % 2) {
    from = pow_int(from_digits);
    if(from >  to) {
      return 0;
    }
  }

  int_t invalids = 0;
  int_t n = half_digits(from);
  while (1) {
    int_t d_n = double_digits(n);
     if (d_n > to) {
       break;
     }
    if (d_n >= from) {
      invalids += d_n;
    }
     ++n;
  }

  return invalids;
}

int main(int argc, char **argv)
{
  (void) argc;
  (void) argv;

  int_t from, to;

  int_t invalids = 0;

  while(1) {
    if(!read_int(&from)) {
      break;
    }
    if (fgetc(stdin) != '-') {
      fprintf(stderr, "Incorrect format\n");
      return 1;
    }
    if(!read_int(&to)) {
      break;
    }

    invalids += count_invalids(from, to);

    int c = fgetc(stdin);
    if ((c == '\n') || (c == EOF)) {
      break;
    }
    if (c != ',') {
      fprintf(stderr, "Incorrect format\n");
      return 1;
    }
  }
  printf("Part 1: %ld\n", invalids);

  return 0;
}
