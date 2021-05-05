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
    input!(n: usize, m: usize, w: i32, grid: [[i32; m]; n]);

    let mut e = vec![];
    let mut teleport = vec![];
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == -1 {
                e.push(vec![]);
                continue;
            }

            let mut ee = vec![];
            let number = (i * m + j) as i32;
            if grid[i][j] != 0 {
                teleport.push((number, grid[i][j]));
            }
            if j != 0 && grid[i][j - 1] != -1 {
                ee.push((number - 1, w));
            }
            if j != m - 1 && grid[i][j + 1] != -1 {
                ee.push((number + 1, w));
            }

            if i != n - 1 && grid[i + 1][j] != -1 {
                ee.push((number + m as i32, w));
            }
            if i != 0 && grid[i - 1][j] != -1 {
                ee.push((number - m as i32, w));
            }
            e.push(ee);
        }
    }

    for i in 0..teleport.len() {
        for j in i + 1..teleport.len() {
            e[teleport[i].0 as usize].push((teleport[j].0, teleport[i].1 + teleport[j].1));
            e[teleport[j].0 as usize].push((teleport[i].0, teleport[i].1 + teleport[j].1));
        }
    }
    let ans = dijk::dijkstra(&e, 0, n * m - 1).unwrap_or(-1);

    println!("{}", ans);
}

mod dijk {
    #[derive(Debug, Clone, Eq)]
    struct Node {
        pos: i32,
        cost: i32,
    }
    impl PartialEq for Node {
        fn eq(&self, other: &Node) -> bool {
            self.cost.eq(&other.cost)
        }
    }
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
            Some(other.cost.cmp(&(self.cost)))
        }
    }
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.cost.cmp(&(other.cost))
        }
    }

    pub fn dijkstra(edge: &[Vec<(i32, i32)>], start: usize, end: usize) -> Option<i32> {
        let mut dist = vec![std::i32::MAX; edge.len()];
        let mut pq = std::collections::BinaryHeap::new();

        pq.push(Node {
            pos: start as i32,
            cost: 0,
        });
        dist[start] = 0;

        let mut ret = start == end;

        while let Some(Node { pos, cost }) = pq.pop() {
            if cost > dist[pos as usize] {
                continue;
            }
            if ret {
                ret = false;
                dist[start] = std::i32::MAX;
            } else if end == pos as usize {
                return Some(cost);
            }
            for (t, c) in &edge[pos as usize] {
                let total_cost = cost + *c;
                if dist[*t as usize] <= total_cost {
                    continue;
                }
                dist[*t as usize] = total_cost;
                pq.push(Node {
                    pos: *t as i32,
                    cost: total_cost,
                });
            }
        }
        None
    }

    #[test]
    fn test_dijkstra() {
        let graph = vec![
            vec![(2, 10), (1, 1)],
            vec![(3, 2)],
            vec![(1, 1), (3, 3), (4, 1)],
            vec![(0, 7), (4, 2)],
            vec![],
        ];

        assert_eq!(dijkstra(&graph, 0, 1), Some(1));
        assert_eq!(dijkstra(&graph, 0, 3), Some(3));
        assert_eq!(dijkstra(&graph, 3, 0), Some(7));
        assert_eq!(dijkstra(&graph, 0, 4), Some(5));
        assert_eq!(dijkstra(&graph, 4, 0), None);
    }
}
