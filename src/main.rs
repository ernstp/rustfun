use std::collections::BTreeSet;
use std::collections::PriorityQueue;
use std::num::abs;
use std::rc::Rc;
use std::rand;

#[deriving(Ord,Eq)]
struct Point {
    x: uint,
    y: uint,
    path: uint,
    cost: uint,
    parent: Option<Rc<Point>>,
}

impl Point {
    fn get_dist(&self, other: &Point) -> uint {
        (abs(self.x as int - other.x as int) + abs(self.y as int - other.y as int)) as uint
    }

    fn print_path(&self) {
        match self.parent {
            Some(ref x) => x.print_path(),
            None => () };
        println!("{},{}", self.x, self.y);
    }

    fn new(x: uint, y: uint, inparent: Option<&Rc<Point>>, target: &Point) -> Point {
        let mut p = Point {x: x,
               y: y,
               path: 0,
               cost: 0,
               parent: None};
        match inparent { Some(x) => { p.parent = Some(x.clone()); p.path = x.path + 1; },
                         None => () };
        p.cost = p.path + p.get_dist(target);
        return p;
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        // Order by reversed cost
        other.cost.partial_cmp(&self.cost)
    }
}

struct Map {
    data: Vec<Vec<uint>>,
    sizex: uint,
    sizey: uint,
}

impl Map {
    fn print(&self) {
        for row in self.data.iter() {
            for p in row.iter() {
                print!("{}", p);
            }
            println!("");
        }
    }

    fn index(&self, x: uint, y: uint) -> uint {
        y * self.sizex + x
    }

    fn avail(&self, x: uint, y: uint) -> bool {
        if x <= self.sizex && y <= self.sizey {
            self.data[y][x] == 0
        } else {
            false
        }
    }

    fn new(sizex: uint, sizey: uint) -> Map {
        let mut d = Vec::with_capacity(sizey);
        for row in range(0, sizey) {
            let mut dr = Vec::with_capacity(sizex);
            for x in range(0, sizex) {
                let r = rand::random::<uint>() % 3u;
                match r {
                    0 => dr.push(1u),
                    _ => dr.push(0u),
                }
            }
            d.push(dr);
        };
        Map {data: d, sizex: sizex, sizey: sizey }
    }
}

fn main() {
    let adjecent = vec![(1i,1i), (1i,0i), (1i,-1i), (0i,-1i), (-1i,-1i), (-1i,0i), (-1i,1i), (0i,1i)];

    let map = Map::new(10, 3);
    map.print();
    let mut used = BTreeSet::new();
    let mut openq = PriorityQueue::new();
    let start = Point {x: 0, y: 0, path: 0, cost: 0, parent: None};
    let target = Point {x: 9, y: 2, path: 0, cost: 0, parent: None};
    used.insert(map.index(start.x, start.y));

    let mut current = Rc::new(start);
    let mut best = current.clone();
    let mut best_dist = best.get_dist(&target);

    while *current != target {
        for &(x, y) in adjecent.iter() {
            let newx: int = current.x as int + x as int;
            let newy: int = current.y as int + y as int;
            if newy >= 0 && newx >= 0 && newx < map.sizex as int && newy < map.sizey as int {
                let newx: uint = newx as uint;
                let newy: uint = newy as uint;
                let index = map.index(newx, newy);
                // TODO: If the point exists in the openq and this is a shorter
                // path to this point it should be updated.
                if !used.contains(&index) && map.avail(newx, newy) {
                    let p = Point::new(newx,
                                       newy,
                                       Some(&current),
                                       &target);
                    used.insert(index);
                    openq.push(Rc::new(p));
                }
            }
        }
        match openq.pop() {
            Some(v) => { current = v; }
            None => break
        }
        let current_dist = current.get_dist(&target);
        if current_dist < best_dist || (best_dist == current_dist && current.path < best.path) {
            best = current.clone();
            best_dist = best.get_dist(&target);
        }
    }
    if *current != target {
        best.print_path();
        println!("FAIL!   {}", best.path);
    } else {
        current.print_path();
        println!("length: {}", current.path);
    }
}
