#[derive(Debug, Clone, Copy)]
struct Junction(i64, i64, i64);

impl Junction {
    fn make(input: &str) -> Vec<Self> {
        let junctions: Vec<Junction> = input
            .lines()
            .map(|line| {
                let mut it = line.split(',');
                Junction(
                    it.next().unwrap().parse().unwrap(),
                    it.next().unwrap().parse().unwrap(),
                    it.next().unwrap().parse().unwrap(),
                )
            })
            .collect();

        junctions
    }
}

fn dist2(a: &Junction, b: &Junction) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        } else {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }
    }
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let junctions = Junction::make(input);


    let n = junctions.len();
    let mut edges = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((dist2(&junctions[i], &junctions[j]), i, j));
        }
    }

    edges.sort_by_key(|e| e.0);

    let mut dsu = DSU::new(n);

    let limit = if junctions.len() == 20 { 10 } else { 1000 };

    for &(_, a, b) in edges.iter().take(limit) {
        dsu.union(a, b);
    }

    let mut counts = vec![0usize; n];
    for i in 0..n {
        let root = dsu.find(i);
        counts[root] += 1;
    }

    let mut comps: Vec<usize> = counts.into_iter().filter(|&x| x > 0).collect();
    comps.sort_unstable_by(|a, b| b.cmp(a));

    comps.iter().take(3).product()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> i64 {
    let junctions = Junction::make(input);


    let n = junctions.len();
    if n < 2 {
        return 0;
    }

    let mut edges = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((dist2(&junctions[i], &junctions[j]), i, j));
        }
    }

    edges.sort_unstable_by_key(|e| e.0);

    let mut dsu = DSU::new(n);
    let mut edges_count = 0;

    for (_, u, v) in edges {
        if dsu.find(u) != dsu.find(v) {
            dsu.union(u, v);
            edges_count += 1;

            if edges_count == n - 1 {
                return junctions[u].0 * junctions[v].0;
            }
        }
    }

    unreachable!()
}
