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
        input!(sc = sc, a: i64, b: i64, s: Chars);
        let n = s.len();
        let mut s = s;

        let mut is_fake = false;
        let mut count = vec![0i64; 2];
        for i in 0..n / 2 {
            if s[i] == '?' && s[n - 1 - i] == '?' {
            } else if s[i] == '?' {
                count[s[n - 1 - i] as usize - 48] += 2;
                s[i] = s[n - 1 - i];
            } else if s[n - 1 - i] == '?' {
                count[s[i] as usize - 48] += 2;
                s[n - 1 - i] = s[i];
            } else if s[i] == s[n - 1 - i] {
                count[s[i] as usize - 48] += 2;
            } else {
                is_fake = true;
                break;
            }
        }

        if n % 2 == 1 {
            if s[(n - 1) / 2] == '?' {
                if a % 2 == 0 {
                    s[(n - 1) / 2] = '1';
                    count[1] += 1;
                } else {
                    s[(n - 1) / 2] = '0';
                    count[0] += 1;
                }
            } else {
                count[s[(n - 1) / 2] as usize - 48] += 1;
            }
        }

        for i in 0..n / 2 {
            if s[i] == '?' {
                if a > count[0] {
                    s[i] = '0';
                    s[n - 1 - i] = '0';
                    count[0] += 2;
                } else {
                    s[i] = '1';
                    s[n - 1 - i] = '1';
                    count[1] += 2;
                }
            }
        }

        if is_fake || !(count[0] == a && count[1] == b) {
            println!("-1");
        } else {
            for (i, v) in s.iter().enumerate() {
                if i == n - 1 {
                    println!("{}", v);
                } else {
                    print!("{}", v);
                }
            }
        }
    }
}
