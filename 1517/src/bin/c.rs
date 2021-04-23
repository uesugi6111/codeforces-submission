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
    input!(sc = sc, n: usize, a: [i64; n]);
    let mut ans = vec![vec![-1; n]; n];
    for (i, v) in a.iter().enumerate() {
        ans[i][i] = *v;
    }

    for i in 0..n {
        let mut pos_x = i;
        let mut pos_y = i;
        let nnn = ans[i][i] as usize;
        for _j in 0..nnn as usize - 1 {
            if pos_y > 0 && ans[pos_x][pos_y - 1] == -1 {
                ans[pos_x][pos_y - 1] = nnn as i64;
                pos_y -= 1;
            } else if pos_y < pos_x && ans[pos_x - 1][pos_y] == -1 {
                ans[pos_x - 1][pos_y] = nnn as i64;
                pos_x -= 1;
            } else if ans[pos_x + 1][pos_y] == -1 {
                ans[pos_x + 1][pos_y] = nnn as i64;
                pos_x += 1;
            } else {
                ans[pos_x][pos_y + 1] = nnn as i64;
                pos_y += 1;
            }
        }
    }
    for (i, v) in ans.iter().enumerate() {
        for (j, &value) in v.iter().take(i + 1).enumerate() {
            if j == i {
                println!("{}", value);
            } else {
                print!("{} ", value);
            }
        }
    }
}
