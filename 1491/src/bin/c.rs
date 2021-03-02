#[rustfmt::skip]
mod fast_input {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=fast_input::Scanner::new(std::io::stdin().lock(),4096);input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner {buf:Vec<u8>,pos: usize,}
    impl Scanner {
        pub fn new<R: std::io::Read>(mut reader: R, estimated: usize) -> Self {
            let mut buf = Vec::with_capacity(estimated);let _=std::io::copy(&mut reader,&mut buf).unwrap();if buf.last()!=Some(&b'\n'){panic!("{}", 0);}
            Scanner { buf, pos: 0 }
        }
        #[inline]
        pub fn next<T: std::str::FromStr>(&mut self) -> T where T::Err: std::fmt::Debug,{
            let mut start=None;loop{match(self.buf[self.pos],start.is_some()){(b' ',true)|(b'\n', true)|(b'\r', true)=>break,(_, true)|(b' ', false)|(b'\n',false)|(b'\r', false)=>self.pos+=1,(_, false)=>start=Some(self.pos),}}let target=&self.buf[start.unwrap()..self.pos];
            unsafe { std::str::from_utf8_unchecked(target) }.parse().unwrap()
        }
    }
}

fn main() {
    let mut sc = fast_input::Scanner::new(std::io::stdin().lock(), 4096);

    input!(sc = sc, t: usize);
    let mut ans = String::new();
    for _ in 0..t {
        input!(sc = sc, n: usize, a: [i64; n]);
        let mut imos = vec![0i64; n + 1];

        for i in (0..n).rev() {
            if a[i] == 1 {
                continue;
            }
            imos[std::cmp::min(n, i + 2)] += 1;
            imos[std::cmp::min(n, i + a[i] as usize + 1)] -= 1;
        }

        let mut cumsum = vec![0; n + 2];
        for (i, v) in imos.iter().enumerate() {
            cumsum[i + 1] = cumsum[i] + v;
        }

        let mut b: Vec<_> = (0..n).map(|x| a[x] - cumsum[x + 1] - 1).collect();

        for i in 1..n {
            if b[i - 1] < 0 {
                b[i] -= b[i - 1].abs();
                b[i - 1] = 0;
            }
        }
        if b[n - 1] < 0 {
            b[n - 1] = 0;
        }
        let sum: i64 = b.iter().sum();

        ans += &format!("{}\n", sum);
    }
    print!("{}", ans);
}
