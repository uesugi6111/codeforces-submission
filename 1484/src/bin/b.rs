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
        input!(sc = sc, n: usize, a: [i64; n]);
        if (0..n - 1).all(|x| a[x + 1] == a[x]) {
            println!("0");
            continue;
        }
        let mut c = (0..n - 1).map(|x| a[x + 1] - a[x]).max().unwrap_or(-1);
        let max = *a.iter().max().unwrap();
        if c < 0 {
            if (0..n - 1).all(|x| a[x + 1] == a[x] + c) {
                println!("0");
                continue;
            }
            println!("-1");
            continue;
        }
        let mut dame = false;
        let mut buff = -1;
        for i in 0..n - 1 {
            // if a[i] < a[i + 1] {
            //     c = a[i + 1] - a[i];
            // }
            // if a[i] == a[i + 1] {
            //     dame = true;
            //     break;
            // }

            if a[i] + c != a[i + 1] {
                if buff == -1 {
                    buff = a[i] + c - a[i + 1];
                    if max >= buff {
                        dame = true;
                        break;
                    }
                } else if (a[i] + c) % buff != a[i + 1] {
                    dame = true;
                    break;
                }
            }
        }
        if dame {
            println!("-1");
        } else if buff == -1 {
            println!("0");
        } else {
            println!("{} {}", buff, c);
        }
    }
}
