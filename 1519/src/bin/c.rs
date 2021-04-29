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
        let mut v = vec![vec![]; n];
        for i in 0..n {
            v[a[i] as usize - 1].push(b[i]);
        }

        let mut cumsums = vec![];
        for i in 0..n {
            v[i].sort_unstable_by(|a, b| b.cmp(a));
            cumsums.push(cumsum(&v[i]));
        }
        let mut index: std::collections::BTreeSet<_> = (0..n).collect();
        let ans: Vec<i64> = (1..=n)
            .map(|i| {
                let mut set = std::collections::BTreeSet::new();
                let sum = index
                    .iter()
                    .map(|&j| {
                        let len = v[j].len();
                        if len >= i {
                            cumsums[j][len - (len % i)]
                        } else {
                            set.insert(j);
                            0
                        }
                    })
                    .sum();
                index = index.difference(&set).cloned().collect();
                sum
            })
            .collect();

        for (i, v) in ans.iter().enumerate() {
            if i == ans.len() - 1 {
                println!("{}", v);
            } else {
                print!("{} ", v);
            }
        }
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
