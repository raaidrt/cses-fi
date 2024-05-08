use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

enum Cell {
    Floor,
    Wall,
    Start,
    End,
}

#[derive(Clone, Debug, Copy)]
enum Direction { 
    L, R, U, D
}

struct Graph {
    grid: Vec<Vec<Cell>>,
    m: usize,
    n: usize,
}

fn construct_path(parents: &Vec<Vec<Option<(Direction, Option<(usize, usize)>)>>>, coordinates: (usize, usize), v: &mut Vec<Direction>) {
    match parents[coordinates.0][coordinates.1] {
        None => (),
        Some((d, None)) => {
            v.push(d)
        },
        Some((d, Some (coordinates))) => {
            construct_path(parents, coordinates, v);
            v.push(d);
        }
    }
}

impl Graph {
    fn from_grid(n: usize, m: usize, grid: Vec<Vec<u8>>) -> Graph {
        assert!(n == grid.len());
        assert!(n > 0 && grid[0].len() == m);

        let grid = grid
            .into_iter()
            .map(|elem| {
                elem.into_iter()
                    .map(|elem: u8| match elem {
                        b'.' => Cell::Floor,
                        b'#' => Cell::Wall,
                        b'A' => Cell::Start,
                        b'B' => Cell::End,
                        _ => panic!("Grid Cell should be one of ., #, A, B or newline"),
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Vec<Cell>>>();

        Graph { grid, n, m }
    }

    fn has_edges(self: &Graph, i: usize, j: usize) -> Vec<(usize, usize, Direction, usize, usize)> {
        let mut product = Vec::with_capacity(4);
        if i > 0 {
            product.push((i - 1, j, Direction::U, i, j));
        }
        if i + 1 < self.n {
            product.push((i + 1, j, Direction::D, i, j));
        }
        if j > 0 {
            product.push((i, j - 1, Direction::L, i, j));
        }
        if j + 1 < self.m {
            product.push((i, j + 1, Direction::R, i, j));
        }

        let product: Vec<(usize, usize, Direction, usize, usize)> = product.into_iter()
            .filter(|(new_i, new_j, _, _, _)| { match self.grid[*new_i][*new_j] {
                Cell::Wall => false, 
                _ => true
            } })
            .collect::<Vec<(usize, usize, Direction, usize, usize)>>();
        product
    }
    
    

    fn reach(self: &Graph) -> Option<Vec<Direction>> {
        let mut visited = HashSet::new();
        let start_coordinate = self
            .grid
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                let starts = row
                    .into_iter()
                    .enumerate()
                    .filter_map(|(j, cell)| match cell {
                        Cell::Start => Some((i, j)),
                        Cell::Floor => None,
                        Cell::End => None,
                        Cell::Wall => None,
                    })
                    .collect::<Vec<(usize, usize)>>();
                if starts.len() > 0 {
                    Some(starts[0])
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, usize)>>();
        let start_coordinate = if start_coordinate.len() > 0 {
            start_coordinate[0]
        } else {
            panic!("No start coordinate exists");
        };
        let end_coordinate = self
            .grid
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                let ends = row
                    .into_iter()
                    .enumerate()
                    .filter_map(|(j, cell)| match cell {
                        Cell::Start => None,
                        Cell::Floor => None,
                        Cell::End => Some((i, j)),
                        Cell::Wall => None,
                    })
                    .collect::<Vec<(usize, usize)>>();
                if ends.len() > 0 {
                    Some(ends[0])
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, usize)>>();
        let end_coordinate = if end_coordinate.len() > 0 {
            end_coordinate[0]
        } else {
            panic!("No end coordinate exists");
        };
        visited.insert(start_coordinate);
        let mut parents = Vec::with_capacity(self.n);
        for i in 0..self.n {
            parents.push(Vec::with_capacity(self.m));
            for _ in 0..self.m {
                parents[i].push(None);
            }
        }
        let mut frontier = vec![(start_coordinate.0, start_coordinate.1, Direction::U, 0, 0)];
        while frontier.len() > 0 {
            // println!("Frontier {:?}", visited);
            frontier = frontier.into_iter().filter(|(i, j, _, _, _)| *i != end_coordinate.0 || *j != end_coordinate.1).collect::<Vec<(usize, usize, Direction, usize, usize)>>();
            let concat_frontier = frontier
                .iter()
                .map(|(i, j, _, _, _)| self.has_edges(*i, *j))
                .collect::<Vec<Vec<(usize, usize, Direction, usize, usize)>>>();
            frontier = concat_frontier.concat();
            frontier = frontier
                .iter()
                .filter_map(|(n_i, n_j, d, i, j)| {
                    if visited.contains(&(*n_i, *n_j)) {
                        None
                    } else {
                        visited.insert((*n_i, *n_j));
                        parents[*n_i][*n_j] = Some (((*d).clone(), Some ((*i, *j))));
                        Some((*n_i, *n_j, (*d).clone(), *i, *j))
                    }
                })
                .collect::<Vec<(usize, usize, Direction, usize, usize)>>();
            // println!("Frontier {:?}", visited);
        }

        match parents[end_coordinate.0][end_coordinate.1] {
            None => None,
            Some(_) => { 
                let mut path = vec![];
                construct_path(&parents, end_coordinate, &mut path);
                Some (path)
            }
        }
    }
}

pub fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;
    let result: Vec<usize> = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();
    buffer.clear();
    let (n, m) = (result[0], result[1]);
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(n);
    for _ in 0..n {
        reader.read_line(&mut buffer)?;
        buffer.pop();
        grid.push(buffer.to_ascii_uppercase().as_bytes().to_vec());
        buffer.clear();
    }
    let graph = Graph::from_grid(n, m, grid);
    let path = graph.reach();
    match path {
        None => { println!("NO"); },
        Some(path) => {
            println!("YES");
            println!("{}", path.len());
            for direction in path {
                print!("{}", match direction {
                    Direction::U => 'U',
                    Direction::D => 'D', 
                    Direction::L => 'L',
                    Direction::R => 'R'
                });
            }
            println!()
        }
    }
    Ok (())
}
