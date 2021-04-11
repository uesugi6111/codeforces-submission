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
        input!(sc = sc, n: usize, k: i64);

        if k * 2 + 1 > n as i64 {
            println!("-1");
            continue;
        }

        let mut v = vec![None; n];
        let mut count = n as i64;
        for i in 0..k {
            v[(i * 2 + 1) as usize] = Some(count);
            count -= 1;
        }

        for i in 0..n {
            if n - 1 == i {
                println!(
                    "{}",
                    match v[i] {
                        Some(vv) => {
                            vv
                        }
                        None => {
                            let buff = count;
                            count -= 1;
                            buff
                        }
                    }
                );
            } else {
                print!(
                    "{} ",
                    match v[i] {
                        Some(vv) => {
                            vv
                        }
                        None => {
                            let buff = count;
                            count -= 1;
                            buff
                        }
                    }
                );
            }
        }
    }
}
