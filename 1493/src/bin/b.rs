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
    let is_mirror = {
        let mut is_mirror = std::collections::HashSet::new();
        is_mirror.insert(1);
        is_mirror.insert(2);
        is_mirror.insert(5);
        is_mirror.insert(8);
        is_mirror.insert(0);
        is_mirror
    };
    let convert = vec![0, 1, 5, 0, 0, 2, 0, 0, 8, 0];

    let mut sc = fast_input::Scanner::new(std::io::stdin().lock(), 4096);

    input!(sc = sc, t: usize);

    for _ in 0..t {
        input!(sc = sc, h: i64, m: i64, a: Chars);
        let mut h_1 = a[0] as i64 - 48;
        let mut h_2 = a[1] as i64 - 48;
        let mut m_1 = a[3] as i64 - 48;
        let mut m_2 = a[4] as i64 - 48;

        loop {
            let mut mm = m_1 * 10 + m_2;
            let mut hh = h_1 * 10 + h_2;
            if is_mirror.contains(&h_1)
                && is_mirror.contains(&h_2)
                && is_mirror.contains(&m_1)
                && is_mirror.contains(&m_2)
                && convert[m_2 as usize] * 10 + convert[m_1 as usize] < h
                && convert[h_2 as usize] * 10 + convert[h_1 as usize] < m
            {
                println!("{}{}:{}{}", h_1, h_2, m_1, m_2);
                break;
            }

            mm += 1;
            if mm == m {
                mm = 0;
                hh += 1;
                if hh == h {
                    hh = 0;
                }
            }
            h_1 = hh / 10;
            h_2 = hh % 10;
            m_1 = mm / 10;
            m_2 = mm % 10;
        }
    }
}
