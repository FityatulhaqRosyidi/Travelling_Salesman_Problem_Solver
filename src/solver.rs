

fn tsp(dist: &Vec<Vec<i32>>, pos: usize, visited: usize, memo: &mut Vec<Vec<i32>>, path: &mut Vec<Vec<usize>>, n: usize,) -> i32 
{
    if visited == (1 << n) - 1 {
        return dist[pos][0];
    }

    if memo[pos][visited] != -1 {
        return memo[pos][visited];
    }

    let mut ans = std::i32::MAX;
    let mut next_city = pos;

    for city in 0..n {
        if visited & (1 << city) == 0 {
            let new_visited = visited | (1 << city);
            let candidate = dist[pos][city] + tsp(dist, city, new_visited, memo, path, n);
            if candidate < ans {
                ans = candidate;
                next_city = city;
            }
        }
    }

    memo[pos][visited] = ans;
    path[pos][visited] = next_city;
    ans
}

fn main() {

    // modify the distance matrix as needed
    let dist = vec![
        vec![0, 10, 15, 20],
        vec![5, 0, 9, 10],
        vec![6, 13, 0, 12],
        vec![8, 8, 9, 0],
    ];

    let n = dist.len();
    let mut memo = vec![vec![-1; 1 << n]; n];
    let mut path = vec![vec![usize::MAX; 1 << n]; n];

    let start = 0;
    let visited = 1 << start;

    let answer = tsp(&dist, start, visited, &mut memo, &mut path, n);
    println!("Jarak minimum perjalanan adalah: {}", answer);

    let mut current = start;
    let mut visited = 1 << start;
    let mut route = vec![current];

    while visited != (1 << n) - 1 {
        current = path[current][visited];
        route.push(current);
        visited |= 1 << current;
    }

    route.push(start);
    println!("Lintasan yang dilalui: {:?}", route);
}
