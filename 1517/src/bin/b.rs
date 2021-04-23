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
        input!(sc = sc, n: usize, m: usize, a: [[i64; m]; n]);

        let mut map = std::collections::BTreeMap::new();
        for i in 0..n {
            for j in 0..m {
                map.entry(a[i][j]).or_insert_with(Vec::new).push((i, j));
            }
        }
        let mut is_used = vec![vec![false; m]; n];
        let mut ans = vec![vec![-1; m]; n];
        let mut m_index = 0;
        for (j, (k, v)) in map.iter().enumerate() {
            for &(i_index, j_index) in v {
                ans[i_index][m_index] = *k;
                is_used[i_index][j_index] = true;
                m_index += 1;
                if m_index >= m {
                    break;
                }
            }
            if m_index >= m {
                break;
            }
        }
        let mut m_index = vec![0; n];
        for j in 0..m {
            for i in 0..n {
                if ans[i][j] != -1 {
                    continue;
                }
                while is_used[i][m_index[i]] {
                    m_index[i] += 1;
                }
                ans[i][j] = a[i][m_index[i]];
                is_used[i][m_index[i]] = true;
            }
        }
        for i in 0..n {
            for j in 0..m {
                if j == m - 1 {
                    println!("{}", ans[i][j]);
                } else {
                    print!("{} ", ans[i][j]);
                }
            }
        }
    }
}
