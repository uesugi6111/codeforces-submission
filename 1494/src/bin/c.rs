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
        input!(sc = sc, n: usize, m: usize, a: [i64; n], b: [i64; m]);
        let mut a_index = ex(&a) + 1;
        let mut b_index = ex(&b) + 1;
        let mut count = 0;
        loop {
            if a_index > a.len() as i64 - 1 || b_index > b.len() as i64 - 1 {
                break;
            }
            if a[a_index as usize] <= b[b_index as usize] {
                count += 1;
                a_index += 1;
                b_index += 1;
            } else {
                b_index += 1;
            }
        }
        a_index = ex(&a);
        b_index = ex(&b);

        loop {
            if a_index < 0 || b_index < 0 {
                break;
            }
            if a[a_index as usize] >= b[b_index as usize] {
                count += 1;
                a_index -= 1;
                b_index -= 1;
            } else {
                b_index -= 1;
            }
        }
        println!("{}", count);
    }
}

fn ex(a: &[i64]) -> i64 {
    let (mut ok, mut ng) = (-1, a.len() as i64);
    while ng - ok > 1 {
        let mid = (ng + ok) / 2;
        if a[mid as usize] < 0 {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
