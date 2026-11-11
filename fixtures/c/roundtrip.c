/* SH-4 round-trip fixture: proof that the cross compiler emits
 * Dreamcast-shaped code before any recovered C exists.
 *
 * Build (Dreamcast ABI, single-precision FPU):
 *   sh-elf-gcc -m4-single-only -O2 -c roundtrip.c -o roundtrip.o
 *
 * The three shapes that matter here:
 *   sum_to     integer decrement loop, args in r4..r7, result in r0
 *   scale_f    single-precision float maths (fr registers, not dr)
 *   step_car   struct by pointer, fixed-step integrator shape
 */

typedef struct {
    float x, y, z;
    float vx, vy, vz;
    int   gear;
} car_state;

int sum_to(int n)
{
    int acc = 0;
    while (n > 0) {
        acc += n;
        n -= 1;
    }
    return acc;
}

float scale_f(float speed, float dt)
{
    return speed * dt + 1.5f;
}

void step_car(car_state *c, float dt)
{
    c->x += c->vx * dt;
    c->y += c->vy * dt;
    c->z += c->vz * dt;
    if (c->vz < 0.0f) {
        c->gear = c->gear > 1 ? c->gear - 1 : 1;
    }
}
