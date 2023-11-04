// impelmenting mt19937
//
// mostly stolen from wikipedia
//

pub struct Constants {
    // w: u32,
    n: u32, // w: word size (in number of bits)
    m: u32, // n: degree of recurrence
    r: u32, // m: middle word, an offset used in the recurrence relation defining the series x, 1 ≤ m < n
    a: u32, // r: separation point of one word, or the number of bits of the lower bitmask, 0 ≤ r ≤ w − 1
    b: u32, // a: coefficients of the rational normal form twist matrix
    c: u32, // b, c: TGFSR(R) tempering bitmasks
    s: u32, // s, t: TGFSR(R) tempering bit shifts
    t: u32, // u, d, l: additional Mersenne Twister tempering bit shifts/masks
    u: u32,
    // d: u32,
    l: u32,
    f: u32,
}

pub struct MT19937 {
    constants: Constants,
    mt: Vec<u32>,
    index: u32,
    lower_mask: u32,
    upper_mask: u32,
}

impl MT19937 {
    #[allow(unused)]
    pub fn new() -> MT19937 {
        let constants = Constants {
            n: 624,
            m: 397,
            r: 31,
            a: 0x9908b0df,
            b: 0x9D2C5680,
            c: 0xEFC60000,
            s: 7,
            t: 15,
            u: 11,
            l: 18,
            f: 1812433253,
        };
        let n = constants.n + 1;
        let r = constants.r;
        let lower_mask = (1 << 31) - 1;
        let upper_mask = (1 << 31);
        MT19937 {
            constants,
            mt: vec![0; n as usize],
            index: n,
            lower_mask,
            upper_mask,
        }
    }

    pub fn seed_mt(&mut self, seed: u32) {
        self.mt[0] = seed;
        for i in 1_u32..self.constants.n {
            // w is 32 since self.mt elements are u32... so we're not bothering with taking
            // lowest bits as its being casted...
            self.mt[i as usize] = self
                .constants
                .f
                .wrapping_mul((self.mt[(i - 1) as usize]) ^ (self.mt[(i - 1) as usize] >> 30))
                + i;
        }
    }

    // Generate the next n values from the series x_i
    fn twist(&mut self) {
        for i in 0..self.constants.n {
            let x: u32 = (self.mt[i as usize] & self.upper_mask)
                | (self.mt[((i + 1) % self.constants.n) as usize] & self.lower_mask);
            let mut x_a: u32 = x >> 1;
            if (x % 2) != 0 {
                // lowest bit of x is 1
                x_a = x_a ^ self.constants.a;
            }
            self.mt[i as usize] =
                self.mt[((i + self.constants.m) % self.constants.n) as usize] ^ x_a;
        }
        self.index = 0;
    }

    pub fn extract_number(&mut self) -> u32 {
        if self.index >= self.constants.n {
            if self.index > self.constants.n {
                self.seed_mt(1337);
            }
            self.twist();
        }

        let mut y: u32 = self.mt[self.index as usize];
        y = y ^ (y >> self.constants.u);
        y = y ^ ((y << self.constants.s) & self.constants.b);
        y = y ^ ((y << self.constants.t) & self.constants.c);
        y = y ^ (y >> self.constants.l);

        self.index += 1;
        y
    }
}

fn main() {
    let mut mt = MT19937::new();
    MT19937::seed_mt(&mut mt, 1131464071);
    for _ in 0..10 {
        let random = MT19937::extract_number(&mut mt);
        println!("random number = {random}");
    }
}
