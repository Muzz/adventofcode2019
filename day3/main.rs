use std::cmp;
use std::env;
use std::fs;

fn man_dist(t1: Tile, t2: Tile) -> i32 {
    (t1.x - t2.x).abs() + (t1.y - t2.y).abs()
}

#[derive(Debug)]
struct Command {
    dir: char,
    dist: u32,
}
#[derive(Debug, Clone)]
struct Tile {
    x: i32,
    y: i32,
}

fn linepath(input: Vec<&str>) -> Vec<Tile> {
    let mut commands: Vec<Command> = vec![];
    for i in input {
        let dir = i.chars().next().unwrap();
        // ah rust, you verbose beast!
        let dist = i.get(1..).unwrap().parse::<u32>().unwrap();

        commands.push(Command { dir, dist });
    }

    let mut steps: Vec<Tile> = vec![];
    let mut cursor: Tile = Tile { x: 0, y: 0 };
    for c in commands {
        match c.dir {
            'U' => {
                for dist in 0..c.dist {
                    steps.push(Tile {
                        x: cursor.x,
                        y: cursor.y + 1,
                    });
                    cursor.y += 1;
                }
            }
            'D' => {
                for dist in 0..c.dist {
                    steps.push(Tile {
                        x: cursor.x,
                        y: cursor.y - 1,
                    });
                    cursor.y -= 1;
                }
            }
            'L' => {
                for dist in 0..c.dist {
                    steps.push(Tile {
                        x: cursor.x + 1,
                        y: cursor.y,
                    });
                    cursor.x += 1;
                }
            }
            'R' => {
                for dist in 0..c.dist {
                    steps.push(Tile {
                        x: cursor.x - 1,
                        y: cursor.y,
                    });
                    cursor.x -= 1;
                }
            }
            _ => panic!("AAAAH"),
        }
    }

    return steps;
}

fn main() {
    let inputfile = "input.txt";
    let contents = fs::read_to_string(inputfile).unwrap();
    let mut lines = contents.lines();

    let input1: Vec<&str> = lines.next().unwrap().split(",").collect();
    let input2: Vec<&str> = lines.next().unwrap().split(",").collect();

    let linepath1 = linepath(input1);
    let linepath2 = linepath(input2);

    let mut crossings: Vec<Tile> = vec![];

    for l1 in &linepath1 {
        for l2 in &linepath2 {
            if (l1.x == l2.x) && l1.y == l2.y {
                crossings.push(l1.clone());
            }
        }
    }

    let mut smallest = 999999;
    for cross in crossings {
        let temp = man_dist(cross, Tile { x: 0, y: 0 });
        // println!("{:?}", temp);
        smallest = cmp::min(temp, smallest);
    }

    println!("{:?}", smallest);
}
