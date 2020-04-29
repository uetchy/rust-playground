struct Edge {
    to: i32,
    cost: i32,
}

type AdjList = Vec<Vec<Edge>>;

fn bellman_ford(n: i32, s: i32) -> bool {
    let mut dist = vec![n; std::i32::MAX];
    false
}

fn main() {
    println!("Hello, world!");
}
