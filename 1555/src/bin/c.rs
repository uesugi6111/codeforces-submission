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
    for _ in 0..t {
        input!(sc = sc, n: usize, a: [i64; n], b: [i64; n]);
        let a_cumsum = cumsum(&a);
        let b_cumsum = cumsum(&b.iter().rev().cloned().collect::<Vec<_>>())
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<_>>();

        let mut min = (std::i64::MAX, 0);

        for i in 0..n {
            let buff = a_cumsum[i + 1] + b_cumsum[i];
            if min.0 >= buff {
                min = (buff, i);
            }
        }

        let mut aa = a;
        let mut bb = b;
        for i in 0..n {
            if min.1 >= i {
                aa[i] = 0;
            }
            if min.1 <= i {
                bb[i] = 0;
            }
        }

        let aa_cumsum = cumsum(&aa);
        let bb_cumsum = cumsum(&bb.iter().rev().cloned().collect::<Vec<_>>())
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<_>>();

        let mut max = (0_i64, 0);

        for i in 0..n {
            let buff = aa_cumsum[i + 1] + bb_cumsum[i];
            if max.0 < buff {
                max = (buff, i);
            }
        }
        println!("{}", max.0);
    }
}

pub fn cumsum(v: &[i64]) -> Vec<i64> {
    (0..1)
        .chain(v.iter().scan(0, |c, &x| {
            *c += x;
            Some(*c)
        }))
        .collect()
}
