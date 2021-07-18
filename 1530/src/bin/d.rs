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
        input!(sc = sc, n: usize, b: [i64; n]);

        let mut v = vec![vec![]; n];
        for (i, &index) in b.iter().enumerate() {
            v[index as usize - 1].push(i);
        }
        let c = v.iter().filter(|&x| !x.is_empty()).count();

        let mut index_1 = 0_i64;
        for i in 0..n {
            if v[i].len() <= 1 {
                continue;
            }
            while v[i].len() > 1 {
                let buff = v[i].pop().unwrap();

                while !(v[index_1 as usize % n].is_empty()) || index_1 as usize % n == buff {
                    index_1 += 1;
                }
                v[index_1 as usize % n].push(buff);
            }
        }
        let mut ans = vec![0; n];
        for (i, vv) in v.iter().enumerate() {
            ans[vv[0]] = i + 1;
        }
        println!("{}", c);
        for (i, &vv) in ans.iter().enumerate() {
            if i == ans.len() - 1 {
                println!("{}", vv);
            } else {
                print!("{} ", vv);
            }
        }
    }
}
