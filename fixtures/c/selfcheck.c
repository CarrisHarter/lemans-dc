/* Driver that pins the expected behaviour of roundtrip.c on the host.
 * Exit 0 = every vector matched. */
#include <stdio.h>
#include <math.h>

int sum_to(int n);
float scale_f(float speed, float dt);

typedef struct {
    float x, y, z;
    float vx, vy, vz;
    int   gear;
} car_state;

void step_car(car_state *c, float dt);

static int failures;

static void expect_int(const char *what, int got, int want)
{
    if (got != want) {
        printf("  FAIL %s: got %d want %d\n", what, got, want);
        failures++;
    } else {
        printf("  ok   %s = %d\n", what, got);
    }
}

static void expect_float(const char *what, float got, float want)
{
    if (fabsf(got - want) > 1e-6f) {
        printf("  FAIL %s: got %.7f want %.7f\n", what, (double)got, (double)want);
        failures++;
    } else {
        printf("  ok   %s = %.7f\n", what, (double)got);
    }
}

int main(void)
{
    expect_int("sum_to(0)", sum_to(0), 0);
    expect_int("sum_to(1)", sum_to(1), 1);
    expect_int("sum_to(5)", sum_to(5), 15);
    expect_int("sum_to(10)", sum_to(10), 55);

    expect_float("scale_f(0,0)", scale_f(0.0f, 0.0f), 1.5f);
    expect_float("scale_f(2,0.5)", scale_f(2.0f, 0.5f), 2.5f);
    expect_float("scale_f(60,0.25)", scale_f(60.0f, 0.25f), 16.5f);

    car_state c = {0, 0, 0, 10.0f, 0.0f, -2.0f, 3};
    step_car(&c, 0.5f);
    expect_float("step_car x", c.x, 5.0f);
    expect_float("step_car z", c.z, -1.0f);
    expect_int("step_car gear", c.gear, 2);

    c.vz = 1.0f;
    step_car(&c, 0.5f);
    expect_int("step_car gear (climbing)", c.gear, 2);

    printf(failures ? "%d failure(s)\n" : "all vectors ok\n", failures);
    return failures ? 1 : 0;
}
