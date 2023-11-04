#include <stdint.h>
#include <stdio.h>

// n = 624
// r = 31

uint32_t state[624] = {0};
uint32_t ind = 624;
const uint32_t lower_mask = (uint32_t)(1 << 31) - 1;
const uint32_t upper_mask = 1 << 31;
uint32_t f = 1812433253;
uint32_t a = 0x9908b0df;
uint32_t m = 397;
uint32_t u = 11;
uint32_t s = 7;
uint32_t b = 0x9D2C5680;
uint32_t t = 15;
uint32_t c = 0xEFC60000;
uint32_t l = 18;

void seed_mt(uint32_t seed) {
  for (int i = 0; i < 624; i++) {
    state[i] = 0;
  }
  ind = 624;
  state[0] = seed;
  for (int i = 1; i < 624; i++) {
    state[i] = (f * (state[i - 1] ^ (state[i - 1] >> 30)) + i);
  }
}

void twist() {
  for (int i = 0; i < 624; i++) {
    uint32_t x = (state[i] & upper_mask) | (state[(i + 1) % 624] & lower_mask);
    uint32_t xA = x >> 1;
    if ((x % 2) != 0) {
      xA = xA ^ a;
    }
    state[i] = state[(i + m) % 624] ^ xA;
  }
  ind = 0;
}

uint32_t extract_number() {
  if (ind >= 624) {
    twist();
  }
  uint32_t y = state[ind];
  y = y ^ (y >> u);
  y = y ^ ((y << s) & b);
  y = y ^ ((y << t) & c);
  y = y ^ (y >> l);

  ind = ind + 1;
  return y;
}

int main() {
  uint32_t seed = 1131464071;
  seed_mt(seed);
  uint32_t rnd;
  for (int i = 0; i < 10; i++) {
    rnd = extract_number();
    printf("random number = %u\n", rnd);
  }
  return 0;
}
