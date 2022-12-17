use std::fmt::{self,Display};

// Fraction: 分数
struct Fraction(u32, u32);

impl Fraction {
    fn new(numetator: u32, denomimator: u32) -> Self {
        let gcf_value = Self::gcf(numetator,denomimator);
        Self(numetator / gcf_value, denomimator / gcf_value)
    }

    // 最大公約数(greatest common factor: gcf) 計算
    fn gcf(value1: u32, value2: u32) -> u32 {
        // ユークリッドの互除法
        // `a` > `b`
        // 0. `a` > `b`で余り`r`を求める
        // 1. `a`/`b`が０でないなら`b`/`r`で余り`r2`を求める
        // 2. `r2`が０でないなら`r`/`r2で余り`r3`を求める
        // 3. ...(繰り返すと必ず余りが0になる)
        // 4. 余りが0になったときに、**割った数が最大公約数**
        let (mut a, mut b) = if value1 > value2 {
            (value1, value2)
        } else {
            (value2, value1)
        };
        let mut r = a % b;
        while r != 0 {
            a = b;
            b = r;
            r = a % b;
        }
        b
    }
}

impl  Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", &self.0, &self.1)
    }
}

fn main() {
    let a = Fraction::new(8, 18);
    println!("{}", a);
}

