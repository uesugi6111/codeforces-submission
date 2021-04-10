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
        input!(sc = sc, n: usize, g: [Chars; n]);

        let mut p = vec![];
        for (i, v) in g.iter().enumerate() {
            for (j, c) in v.iter().enumerate() {
                if *c == '*' {
                    p.push((i, j));
                }
            }
        }

        if p[0].0 != p[1].0 && p[0].1 != p[1].1 {
            p.push((p[0].0, p[1].1));
            p.push((p[1].0, p[0].1));
        } else if p[0].0 == p[1].0 {
            let yy = if 1 + p[0].0 >= n {
                p[0].0 - 1
            } else {
                p[0].0 + 1
            };
            p.push((yy, p[0].1));
            p.push((yy, p[1].1));
        } else {
            let xx = if 1 + p[0].1 >= n {
                p[0].1 - 1
            } else {
                p[0].1 + 1
            };
            p.push((p[0].0, xx));
            p.push((p[1].0, xx));
        }

        for i in 0..n {
            for j in 0..n {
                if (i == p[0].0) && j == p[0].1 {
                    print!("*");
                } else if (i == p[1].0) && j == p[1].1 {
                    print!("*");
                } else if (i == p[2].0) && j == p[2].1 {
                    print!("*");
                } else if (i == p[3].0) && j == p[3].1 {
                    print!("*");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
