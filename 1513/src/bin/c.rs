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
        input!(sc = sc, n: i64, m: i64);
        let mm = m % 10;
        let mut s: Vec<char> = n.to_string().chars().collect();
        for i in 0..mm {
            let mut buff_v = vec![];
            for j in 0..s.len() {
                let buff = (s[j] as i64 - 48) + 1;
                if buff >= 10 {
                    s[j] = std::char::from_digit(0, 10).unwrap();
                    buff_v.push('1');
                } else {
                    s[j] = std::char::from_digit(buff as u32, 10).unwrap();
                }
            }
            s.append(&mut buff_v);
        }

        let len = s.len() as i64;
        let exp = modpow(2, m / 10, 10000009);

        println!("{}", (len * exp) % 10000009);
    }
}

pub fn modpow(base: i64, exp: i64, n: i64) -> i64 {
    let (mut base, mut exp, n) = (base, exp, n);
    assert!(
        exp >= 0,
        "negative exponent cannot be used in modular exponentiation"
    );
    if exp == 0 {
        return 1;
    }
    let mut res = 1;
    base %= n;
    loop {
        if exp % 2 == 1 {
            res *= &base;
            res %= &n;
        }
        if exp == 1 {
            return res;
        }
        exp /= 2;
        base *= base;
        base %= n;
    }
}
