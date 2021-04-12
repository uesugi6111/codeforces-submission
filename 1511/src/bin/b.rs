#[rustfmt::skip]
mod io_pro {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=io_pro::Scanner::new(std::io::stdin().lock());input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner{s:Box<str>,input:std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,}
    impl Scanner{
        pub fn new<R:std::io::Read>(mut reader:R)->Self{let mut sc=Scanner{s:{let mut s=String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()},input:"".split_ascii_whitespace().peekable(),};let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};sc.input=s.split_ascii_whitespace().peekable();sc}
        #[inline]pub fn next<T:std::str::FromStr>(&mut self)->T where T::Err:std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
fn main() {
    let mut sc = io_pro::Scanner::new(std::io::stdin().lock());
    input!(sc = sc, t: usize);
    let mut e = era::Eratosthenes::new();
    e.generate(100_000);
    let p = e.primes();
    for _ in 0..t {
        input!(sc = sc, a: i64, b: i64, c: i64);
        let mut ccc = 0;
        let (a, b) = (a.min(b), a.max(b));
        let a_n = 10usize.pow((a - 1) as u32);
        let b_n = 10usize.pow((b - 1) as u32);
        let c_n = 10usize.pow((c - 1) as u32);
        for &cc in p.iter() {
            ccc *= cc as i64;
            if c_n as i64 > ccc {
                continue;
            }

            let aa = *p.iter().find(|&&x| a_n as i64 <= x as i64 * ccc).unwrap();

            let bb = *p
                .iter()
                .find(|&&x| b_n as i64 <= x as i64 * ccc && aa != x)
                .unwrap();
            println!("{} {}", aa as i64 * ccc, bb as i64 * ccc);
            break;
        }
    }
}

mod era {

    ///エラトステネスの篩
    pub struct Eratosthenes {
        flags_: Vec<u8>,
        n: usize,
    }
    impl Eratosthenes {
        const K_MASK: [[u8; 8]; 8] = [
            [0xfe, 0xfd, 0xfb, 0xf7, 0xef, 0xdf, 0xbf, 0x7f],
            [0xfd, 0xdf, 0xef, 0xfe, 0x7f, 0xf7, 0xfb, 0xbf],
            [0xfb, 0xef, 0xfe, 0xbf, 0xfd, 0x7f, 0xf7, 0xdf],
            [0xf7, 0xfe, 0xbf, 0xdf, 0xfb, 0xfd, 0x7f, 0xef],
            [0xef, 0x7f, 0xfd, 0xfb, 0xdf, 0xbf, 0xfe, 0xf7],
            [0xdf, 0xf7, 0x7f, 0xfd, 0xbf, 0xfe, 0xef, 0xfb],
            [0xbf, 0xfb, 0xf7, 0x7f, 0xfe, 0xef, 0xdf, 0xfd],
            [0x7f, 0xbf, 0xdf, 0xef, 0xf7, 0xfb, 0xfd, 0xfe],
        ];

        const C0: [[usize; 8]; 8] = [
            [0, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 0, 1, 1, 1, 1],
            [2, 2, 0, 2, 0, 2, 2, 1],
            [3, 1, 1, 2, 1, 1, 3, 1],
            [3, 3, 1, 2, 1, 3, 3, 1],
            [4, 2, 2, 2, 2, 2, 4, 1],
            [5, 3, 1, 4, 1, 3, 5, 1],
            [6, 4, 2, 4, 2, 4, 6, 1],
        ];
        const K_MOD_30: [usize; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
        const C1: [usize; 8] = [6, 4, 2, 4, 2, 4, 6, 2];

        ///初期化
        pub fn new() -> Self {
            Eratosthenes {
                flags_: Vec::<u8>::new(),
                n: 0,
            }
        }

        ///素数フラグを処理
        ///- param n:usize 探索上限
        pub fn generate(&mut self, n: usize) -> &Self {
            if n > 1_000_000_001 {
                panic!();
            }
            self.n = n;
            let size = n / 30 + if n % 30 != 0 { 1 } else { 0 };
            self.flags_.clear();
            self.flags_.resize(size, 0xff);

            self.flags_[0] = 0xfe;

            let remainder = n % 30;
            self.flags_[size - 1] = match remainder {
                1..=1 => 0x0,
                2..=7 => 0x1,
                8..=11 => 0x3,
                12..=13 => 0x7,
                14..=17 => 0xf,
                18..=19 => 0x1f,
                20..=23 => 0x3f,
                24..=29 => 0x7f,
                _ => panic!(),
            };

            let quart_x = ((n as f64).sqrt() + 1.0) as usize / 30 + 1;

            for i in 0..quart_x {
                let mut flags: u8 = self.flags_[i];

                while flags != 0 {
                    let lsb = flags & flags.wrapping_neg();
                    let i_bit = lsb.trailing_zeros() as usize;

                    let m = Eratosthenes::K_MOD_30[i_bit];

                    let mut k = i_bit;
                    let mut j = i * (30 * i + 2 * m) + (m * m) / 30;

                    while j < self.flags_.len() {
                        self.flags_[j] &= Eratosthenes::K_MASK[i_bit][k];

                        j += i * Eratosthenes::C1[k] + Eratosthenes::C0[i_bit][k];
                        k = (k + 1) & 7;
                    }
                    flags &= flags - 1;
                }
            }
            self
        }

        ///素数の個数をカウント
        pub fn count(&mut self) -> usize {
            self.check_generated();

            let mut ret = [2usize, 3, 5].iter().take_while(|x| self.n >= **x).count(); // count 2, 3, 5
            for f in &self.flags_ {
                ret += f.count_ones() as usize;
            }
            ret
        }

        ///フラグから素数配列を生成
        pub fn primes(&self) -> Vec<usize> {
            self.check_generated();

            let mut ret = Vec::<usize>::new();

            [2usize, 3, 5]
                .iter()
                .take_while(|x| self.n >= **x)
                .for_each(|x| ret.push(*x));

            for (i, f) in self.flags_.iter().enumerate() {
                for (ii, m) in Eratosthenes::K_MOD_30.iter().enumerate() {
                    if (*f & (1 << ii)) != 0 {
                        ret.push(30 * i + *m);
                    }
                }
            }
            ret
        }

        fn check_generated(&self) {
            if self.n == 0 {
                panic!();
            }
        }
    }
    impl Default for Eratosthenes {
        fn default() -> Self {
            Eratosthenes::new()
        }
    }
}
