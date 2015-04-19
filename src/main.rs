use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::rc::Rc;
extern crate rand;
use rand::random;

#[derive(Ord,Eq)]
struct Point {
    x: u32,
    y: u32,
    path: u32,
    cost: u32,
    parent: Option<Rc<Point>>,
}

impl Point {
    fn get_dist(&self, other: &Point) -> u32 {
        ((self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs()) as u32
    }

    fn print_path(&self) {
        match self.parent {
            Some(ref x) => x.print_path(),
            None => () };
        println!("{},{}", self.x, self.y);
    }

    fn new(x: u32, y: u32, inparent: Option<&Rc<Point>>, target: &Point) -> Point {
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
    data: Vec<Vec<u32>>,
    sizex: u32,
    sizey: u32,
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

    fn index(&self, x: u32, y: u32) -> u32 {
        y * self.sizex + x
    }

    fn avail(&self, x: u32, y: u32) -> bool {
        if x <= self.sizex && y <= self.sizey {
            self.data[y as usize][x as usize] == 0
        } else {
            false
        }
    }

    fn new(sizex: u32, sizey: u32) -> Map {
        let mut d = Vec::with_capacity(sizey as usize);
        for row in (0..sizey) {
            let mut dr = Vec::with_capacity(sizex as usize);
            for x in (0..sizex) {
                let r = rand::random::<u32>() % 3;
                match r {
                    0 => dr.push(1),
                    _ => dr.push(0),
                }
            }
            d.push(dr);
        };
        Map {data: d, sizex: sizex, sizey: sizey }
    }
}

fn main() {
    let adjecent = vec![(1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1), (0,1)];

    let map = Map::new(10, 3);
    map.print();
    let mut used = BTreeSet::new();
    let mut openq = BinaryHeap::new();
    let start = Point {x: 0, y: 0, path: 0, cost: 0, parent: None};
    let target = Point {x: 9, y: 2, path: 0, cost: 0, parent: None};
    used.insert(map.index(start.x, start.y));

    let mut current = Rc::new(start);
    let mut best = current.clone();
    let mut best_dist = best.get_dist(&target);

    while *current != target {
        for &(x, y) in adjecent.iter() {
            let newx: i32 = current.x as i32 + x as i32;
            let newy: i32 = current.y as i32 + y as i32;
            if newy >= 0 && newx >= 0 && newx < map.sizex as i32 && newy < map.sizey as i32 {
                let newx: u32 = newx as u32;
                let newy: u32 = newy as u32;
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
