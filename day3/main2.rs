use std::cmp;
use std::env;
use std::fs;

#[derive(Debug)]
struct Command {
    dir: char,
    dist: u32,
}
#[derive(Debug, Clone, Copy)]
struct Tile {
    x: i32,
    y: i32,
}

struct Crossing {
    dist1: usize,
    dist2: usize,
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

    let mut crossings: Vec<Crossing> = vec![];

    for l1 in 0..linepath1.len() {
        for l2 in 0..linepath2.len() {
            if (linepath1[l1].x == linepath2[l2].x) && linepath1[l1].y == linepath2[l2].y {
                crossings.push(Crossing {
                    dist1: l1,
                    dist2: l2,
                });
            }
        }
    }

    let mut smallest = 999999;
    for cross in crossings {
        smallest = cmp::min(smallest, cross.dist1 + cross.dist2);
    }

    println!("{:?}", smallest + 2);
}
