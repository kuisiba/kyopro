#[allow(unused_macros)]
macro_rules! get {
    ([$t:ty] ; $n:expr) => {
        (0..$n)
        .map(|_| get!([$t])).collect::<Vec<_>>()
    };
    ($($t:ty),+ ; $n:expr) => {
        (0..$n).map(|_| get!($($t),+)).collect::<Vec<_>>()
    };
    ([$t:ty]) => {
        rl().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<$t>>()
    };
    ($t:ty) => {
        rl().parse::<$t>().unwrap()
    };
    ($($t:ty),*) => {
        {
            let line =rl();
            let mut iter = line.split_whitespace();
            ($(iter.next().unwrap().parse::<$t>().unwrap()),*)
        }
    };
}
fn rl() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
}
