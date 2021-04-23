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
    input!(
        n: usize,
        m: usize,
        k: i64,
        x_c: [[i64; m - 1]; n],
        y_c: [[i64; m]; n - 1]
    );

    if k % 2 == 1 {
        for _ in 0..n {
            for j in 0..m {
                if j == m - 1 {
                    println!("-1");
                } else {
                    print!("-1 ");
                }
            }
        }
        return;
    }

    let mut g = vec![vec![vec![]; m]; n];
    for i in 0..n {
        for j in 0..m {
            if j != 0 {
                g[i][j].push((x_c[i][j - 1], (i, j - 1)));
            }
            if j != m - 1 {
                g[i][j].push((x_c[i][j], (i, j + 1)));
            }
            if i != 0 {
                g[i][j].push((y_c[i - 1][j], (i - 1, j)));
            }
            if i != n - 1 {
                g[i][j].push((y_c[i][j], (i + 1, j)));
            }
            g[i][j].sort_unstable();
        }
    }
    let mut ans = vec![vec![-1; m]; n];
    let kk = k / 2 - 1;

    for i in 0..n {
        for j in 0..m {
            ans[i][j] = g[i][j]
                .iter()
                .map(|&(cost, index)| {
                    let add_cost = g[index.0][index.1][0].0;
                    (cost + add_cost * kk) * 2
                })
                .min()
                .unwrap();
        }
    }

    for v in ans {
        for (j, value) in v.iter().enumerate() {
            if j == v.len() - 1 {
                println!("{}", value);
            } else {
                print!("{} ", value);
            }
        }
    }
}
