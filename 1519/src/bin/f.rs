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
        input!(sc = sc, n: usize);
        println!("Yes");
    }
}

#[test]
fn ctf() {
    let f:Vec<_> = "709088550902439876921359662969011490817828244100611994507393920171782905026859712405088781429996152122943882490614543229".chars().collect();
    for i in 0..f.len() / 2 {
        let buff = (f[2 * i] as u8 - 48) * 10 + f[2 * i + 1] as u8 - 48;
        let buff = buff as char;
        print!("{:?}", buff);
    }
}

#[test]
fn ctf2() {
    let f: Vec<_> = "010001100100110001000001010001110111101101110100011010000110010101011111011000100110000101110011011010010110001101011111011010110110111001101111011101110110110001100101011001000110011101100101010111110110111101100110010111110110001101101111011011010110110101110101011011100110100101100011011000010111010001101001011011110110111001111101"
    .chars()
    .collect();

    let po: Vec<_> = (0..8).map(|i| 2i64.pow(i)).collect();
    let mut ans = vec![];
    for i in 0..f.len() / 8 {
        let mut buff: u8 = 0;
        for j in 0..8 {
            buff += (f[i * 8 + j] as u8 - 48) * po[7 - j] as u8;
        }
        ans.push(buff);
        let buff = buff as char;
        print!("{}", buff);
    }
}
