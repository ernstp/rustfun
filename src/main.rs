use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::time::Instant;

extern crate rand;
use rand::{Rng, SeedableRng, StdRng};

#[derive(Eq,Ord,Copy,Clone)]
struct Point {
    x: u32,
    y: u32,
    path: u32,
    cost: u32,
    index: u32,
    parenti: Option<u32>,
}

impl Point {
    fn get_dist(&self, other: &Point) -> u32 {
        ((self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs()) as u32
    }

    fn get_path(&self, points: &HashMap<u32, Point>) -> HashSet<u32> {
        let mut path = HashSet::new();
        let mut p = self;
        loop {
            path.insert(p.index);
            match p.parenti {
                Some(pi) => {
                    match points.get(&pi) {
                        Some(newp) => p = newp,
                        None => break
                    }
                },
                None => break
            }
        }
        return path;
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.index == other.index
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        // Order by reversed cost
        other.cost.partial_cmp(&self.cost)
    }
}

struct Map {
    data: Vec<u32>,
    sizex: u32,
    sizey: u32,
}

impl Map {
    fn print(&self, path: Option<&HashSet<u32>>) {
        for (index, d) in self.data.iter().enumerate() {
            if path.is_some() && path.unwrap().contains(&(index as u32)) {
                print!(". ");
            } else {
                print!("{} ", d);
            }
            if (index as u32 + self.sizex + 1) % self.sizex == 0 {
                println!(""); // New line
            }
        }
    }

    fn index(&self, x: u32, y: u32) -> u32 {
        y * self.sizex + x
    }

    fn new_point(&self, x: u32, y: u32, inparent: Option<&Point>, target: Option<&Point>) -> Point {
        let mut p = Point {x: x,
               y: y,
               path: 0,
               cost: std::u32::MAX,
               index: self.index(x, y),
               parenti: None};
        match inparent { Some(x) => { p.parenti = Some(x.index); p.path = x.path + 1; },
                         None => () };
        match target { Some(t) => { p.cost = p.path + p.get_dist(t); },
                         None => () };
        return p;
    }

    fn avail(&self, index: &u32) -> bool {
        self.data[(*index) as usize] == 0
    }

    fn in_map(&self, x: &i32, y: &i32) -> bool {
        *y >= 0 && *x >= 0 && *x < self.sizex as i32 && *y < self.sizey as i32
    }

    fn new(sizex: u32, sizey: u32, seed: usize) -> Map {
        let size: usize = (sizex * sizey) as usize;
        let mut d = Vec::with_capacity(size);
        let seed: &[_] = &[seed];
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        for i in 0..size {
            let r = rng.gen::<u32>() % 3;
            match r {
                0 => d.push(1),
                _ => d.push(0),
            }
        };
        Map {data: d, sizex: sizex, sizey: sizey }
    }
}

fn find_path(map: &Map, start: Point, target: Point) -> HashSet<u32> {
    let adjecent = vec![(1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1), (0,1)];

    let mut open = HashSet::new();
    let mut closed = HashMap::new();
    let mut openq = BinaryHeap::new();

    let mut current = start.clone();
    let mut best = start.clone();
    let mut best_dist = std::u32::MAX;
    let mut iterations = 0;
    let max_iterations = (map.sizex + map.sizey) * 2;

    while current != target {
        for &(x, y) in adjecent.iter() {
            let newx: i32 = current.x as i32 + x;
            let newy: i32 = current.y as i32 + y;
            if map.in_map(&newx, &newy) {
                let newx: u32 = newx as u32;
                let newy: u32 = newy as u32;
                let index: u32 = map.index(newx, newy);
                
                if map.avail(&index) && !closed.contains_key(&index) && !open.contains(&index) {
                    let p = map.new_point(newx,
                                      newy,
                                      Some(&current),
                                      Some(&target));
                    openq.push(p);
                    open.insert(index);
                }
            }
        }
        match openq.pop() {
            Some(v) => {
                // This is the "right thing" to do but it's actually faster to avoid
                // open.remove(&v.index);
                closed.insert(current.index, current);
                current = v;},
            None => break
        }

        let curr_dist = current.get_dist(&target);
        if curr_dist < best_dist {
            best = current.clone();
            best_dist = curr_dist;
        }

        iterations += 1;
        if iterations > max_iterations {
            break;
        }
    }

    if current == target {
        return current.get_path(&closed);
    } else {
        return best.get_path(&closed)
    }
}

fn main() {
    let map = Map::new(100, 100, 89);

    let start = map.new_point(0, 0, None, None);
    let target = map.new_point(99, 99, None, None);

    let mut path = HashSet::new();

    let time = Instant::now();
    for i in 0..1000 {
        path = find_path(&map, start, target);
    }
    let dur = Instant::now() - time;

    map.print(Some(&path));
    if path.contains(&map.index(target.x, target.y)) {
        println!("OK");
    } else {
        println!("FAIL");
    }
    println!("{}", (dur.as_secs() * 1_000_000 + dur.subsec_nanos() as u64 / 1000) / 1000);
}
