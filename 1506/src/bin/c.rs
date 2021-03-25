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

    for _ in 0..t {
        input!(sc = sc, s: Chars, t: Chars);
        let mut max = 0;
        let len = s.len().min(t.len());
        let (s, t) = if s.len() > t.len() { (t, s) } else { (s, t) };
        for i in (1..len + 1).rev() {
            for j in 0..len - i + 1 {
                if rolling_hash(&s[j..i + j], &t) {
                    max = max.max(i);
                }
            }
        }
        println!("{}", (s.len() + t.len() - max * 2));
    }
}

pub fn rolling_hash(s: &[char], t: &[char]) -> bool {
    let base: u128 = 2u128.pow(61) - 1;
    let l = s.len();

    let pow_b = base.wrapping_pow(l as u32);

    let mut target_hash: u128 = 0;
    let mut base_hash: u128 = 0;
    for k in 0..l {
        base_hash = base_hash.wrapping_mul(base) + s[k] as u128;
        target_hash = target_hash.wrapping_mul(base) + t[k] as u128;
    }
    if target_hash == base_hash {
        return true;
    }
    for k in 0..t.len() - l {
        target_hash = target_hash
            .wrapping_mul(base)
            .wrapping_add(t[l + k] as u128)
            .wrapping_sub((t[k] as u128).wrapping_mul(pow_b));
        if target_hash == base_hash {
            return true;
        }
    }
    false
}
