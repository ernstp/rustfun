use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::thread::sleep;
use std::time::{Duration,Instant};
use std::cmp;

extern crate rand;
use rand::{Rng, SeedableRng, StdRng};

#[derive(Eq,Copy,Clone)]
struct Point {
    cost: u32,
    path: u32,
    index: u32,
    x: u32,
    y: u32,
    parenti: Option<u32>,
}

impl Point {
    fn get_dist(&self, other: &Point) -> u32 {
        let xd = (self.x as i32 - other.x as i32).abs();
        let yd = (self.y as i32 - other.y as i32).abs();
        let diagonal = cmp::min(xd, yd) as u32 * 10;
        let straight = (xd - yd).abs() as u32 * 7;
        diagonal + straight
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

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.cost == other.cost {
            if self.path == other.path {
                self.index.cmp(&other.index)
            } else {
                self.path.cmp(&other.path)
            }
        } else {
            self.cost.cmp(&other.cost)
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        if self.cost == other.cost {
            self.path.partial_cmp(&other.path)
        } else {
            self.cost.partial_cmp(&other.cost)
        }
    }
}

struct Map {
    data: Vec<u32>,
    sizex: u32,
    sizey: u32,
}

impl Map {
    fn print(&self, path: Option<&HashSet<u32>>) {
        println!("--------");
        for (index, d) in self.data.iter().enumerate() {
            if path.is_some() && path.unwrap().contains(&(index as u32)) {
                print!(". ");
            } else if *d == 1 {
                print!("X ");
            } else {
                print!("  ");
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
        match inparent { Some(x) => { p.parenti = Some(x.index); p.path = x.path + p.get_dist(x); },
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

        for _ in 0..size {
            let r = rng.gen::<u32>() % 3;
            match r {
                0 => d.push(1),
                _ => d.push(0),
            }
        };
        Map {data: d, sizex: sizex, sizey: sizey }
    }
}

fn find_path(map: &Map, start: Point, target: Point, visual: bool) -> HashSet<u32> {
    let adjecent = vec![(1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1), (0,1)];

    // These two should always be updated in parallel
    let mut open = HashMap::new(); // Based on position
    let mut openq = BTreeSet::new(); // Based on cost
    let mut closed = HashMap::new();

    let mut current = start.clone();
    let mut best = start.clone();
    let mut best_dist = std::u32::MAX;
    let mut iterations = 0;
    let max_iterations = (map.sizex + map.sizey) * 10;

    while current != target {
        for &(x, y) in adjecent.iter() {
            let newx: i32 = current.x as i32 + x;
            let newy: i32 = current.y as i32 + y;
            if map.in_map(&newx, &newy) {
                let newx: u32 = newx as u32;
                let newy: u32 = newy as u32;
                let index: u32 = map.index(newx, newy);
                
                if map.avail(&index) && !closed.contains_key(&index) {
                    let p = map.new_point(newx,
                                      newy,
                                      Some(&current),
                                      Some(&target));

                    let mut do_insert = false;
                    if let Some(&old_p) = open.get(&index) {
                        let old_p: Point = old_p;
                        if old_p.path > p.path {
                            openq.remove(&old_p);
                            open.remove(&index);
                            do_insert = true;
                        }
                    } else {
                        do_insert = true;
                    }
                    if do_insert {
                        openq.insert(p);
                        open.insert(index, p);
                    }
                }
            }
        }

        closed.insert(current.index, current);
        match openq.iter().next() {
            Some(v) => { current = *v; },
            None => break
        }
        open.remove(&current.index);
        openq.remove(&current);

        let curr_dist = current.get_dist(&target);
        if curr_dist < best_dist {
            best = current.clone();
            best_dist = curr_dist;
        }

        iterations += 1;
        if iterations > max_iterations {
            break;
        }
        if visual {
            map.print(Some(&current.get_path(&closed)));
            sleep(Duration::from_millis(100));
        }
    }

    if current == target {
        return current.get_path(&closed);
    } else {
        return best.get_path(&closed)
    }
}

fn main() {
    let visual = false;
    let map = Map::new(100, 100, 1867);

    let start = map.new_point(0, 0, None, None);
    let target = map.new_point(99, 99, None, None);

    let mut path = HashSet::new();

    let iterations = match visual {
        true => 1,
        false => 10000,
    };

    let time = Instant::now();
    for _ in 0..iterations {
        path = find_path(&map, start, target, visual);
    }
    let dur = Instant::now() - time;

    map.print(Some(&path));
    if path.contains(&map.index(target.x, target.y)) {
        println!("OK len: {}", path.len());
    } else {
        println!("FAIL");
    }
    println!("{}", (dur.as_secs() * 1_000_000 + dur.subsec_nanos() as u64 / 1000) / iterations);
}
