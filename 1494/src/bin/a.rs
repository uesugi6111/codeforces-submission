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
        input!(sc = sc, s: Chars);
        if s[0] == s[s.len() - 1] {
            println!("NO");
            continue;
        }

        let start = get_index(&s[0]);
        let end = get_index(&s[s.len() - 1]);
        let mid = if start.min(end) == 0 && start.max(end) == 1 {
            2
        } else if start.min(end) == 1 && start.max(end) == 2 {
            0
        } else {
            1
        };

        let mut buff = 0;
        for i in 0..s.len() {
            if s[i] == s[0] {
                buff += 1;
            } else if s[i] == s[s.len() - 1] {
                buff -= 1;
            } else {
                buff += 1;
            }
            if buff < 0 {
                break;
            }
        }
        if buff == 0 {
            println!("YES");
            continue;
        }
        buff = 0;
        for i in 0..s.len() {
            if s[i] == s[0] {
                buff += 1;
            } else if s[i] == s[s.len() - 1] {
                buff -= 1;
            } else {
                buff -= 1;
            }
            if buff < 0 {
                break;
            }
        }
        if buff == 0 {
            println!("YES");
            continue;
        }

        println!("NO");
    }
}

fn get_index(c: &char) -> usize {
    if *c == 'A' {
        0
    } else if *c == 'B' {
        1
    } else {
        2
    }
}
