use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

// Define type aliases
type Grid = grid::Grid<char>;
type GridPtr = Box<Grid>;
type History = HashMap<Direction, HashSet<(usize, usize)>>;
type HistoryPtr = Box<History>;

fn parse_to_grid(input: &str) -> Grid {
    let width = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();

    Grid::from_vec(cells, width)
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    for ((_row, col), i) in grid.indexed_iter() {
        print!("{i}");
        if col == grid.size().0 - 1 {
            println!();
        }
    }
}

fn start_from(grid: &Grid) -> (usize, usize) {
    for ((row, col), i) in grid.indexed_iter() {
        if *i == '^' {
            return (row, col);
        }
    }

    panic!("No starting point found");
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::Up),
            '<' => Some(Direction::Left),
            'v' => Some(Direction::Down),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }

    fn turn_right(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn to_char(&self) -> char {
        match *self {
            Direction::Up => '^',
            Direction::Left => '<',
            Direction::Down => 'v',
            Direction::Right => '>',
        }
    }
}

struct InitState {
    now: (usize, usize),
    direction: Direction,
    grid: GridPtr,
    history: HistoryPtr,
}

impl InitState {
    fn new(grid: GridPtr, history: HistoryPtr) -> InitState {
        let now = start_from(&grid);
        let direction = Direction::from_char(grid[now]).unwrap();

        InitState {
            now,
            direction,
            grid,
            history,
        }
    }
}

struct RunnableState {
    now: (usize, usize),
    direction: Direction,
    grid: GridPtr,
    history: HistoryPtr,
}

struct LoopingState {}

struct OutOfMazeState {
    grid: GridPtr,
    count: usize,
}

impl OutOfMazeState {
    fn new(grid: GridPtr) -> OutOfMazeState {
        OutOfMazeState {
            count: grid.iter().filter(|&&c| c == 'X').count(),
            grid: grid,
        }
    }
}

enum ValidStates {
    Init(InitState),
    Runnable(RunnableState),
    Looping(LoopingState),
    OutOfMaze(OutOfMazeState),
}

impl ValidStates {
    // Consume the current state and return the next state
    fn next(self) -> ValidStates {
        match self {
            ValidStates::Init(init) => ValidStates::Runnable(RunnableState {
                now: init.now,
                direction: init.direction,
                grid: init.grid,
                history: init.history,
            }),
            ValidStates::Runnable(runnable) => runnable.next(),
            ValidStates::Looping(_) => ValidStates::Looping(LoopingState {}),
            ValidStates::OutOfMaze(out_of_maze) => ValidStates::OutOfMaze(OutOfMazeState {
                grid: out_of_maze.grid,
                count: out_of_maze.count,
            }),
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        match self {
            ValidStates::Init(init) => {
                print!("Init state: ");
                print!("Now at {:?}, ", init.now);
                print!("Direction {:?}, ", init.direction);
                print!("Out of maze: {}", false);
                println!();
                print_grid(&init.grid);
            }
            ValidStates::Runnable(runnable) => {
                print!("Runnable state: ");
                print!("Now at {:?}, ", runnable.now);
                print!("Direction {:?}, ", runnable.direction);
                print!("Out of maze: {}", false);
                println!();
                print_grid(&runnable.grid);
            }
            ValidStates::Looping(_) => {
                println!("Looping");
            }
            ValidStates::OutOfMaze(out_of_maze) => {
                println!("Out of maze, count: {}", out_of_maze.count);
            }
        }
    }
}

impl RunnableState {
    fn step(&self) -> Option<(usize, usize)> {
        let (row, col) = self.now;
        let (max_row, max_col) = (self.grid.size().0, self.grid.size().1);

        match self.direction {
            Direction::Up if row > 0 => Some((row - 1, col)),
            Direction::Down if row < max_row - 1 => Some((row + 1, col)),
            Direction::Left if col > 0 => Some((row, col - 1)),
            Direction::Right if col < max_col - 1 => Some((row, col + 1)),
            _ => None,
        }
    }

    // Consume the current state and return the next state
    // Mark as mutable to allow changing the grid and history
    fn next(mut self) -> ValidStates {
        let next = self.step();
        let is_out_of_maze = next.is_none();

        // Make a mark on the grid
        self.grid[self.now] = 'X';
        if is_out_of_maze {
            return ValidStates::OutOfMaze(OutOfMazeState::new(self.grid));
        }

        // Still in the maze
        assert!(!is_out_of_maze && next.is_some());
        let next = next.unwrap();
        let is_in_a_loop = self
            .history
            .get(&self.direction)
            .map_or(false, |steps| steps.contains(&next));
        if is_in_a_loop {
            return ValidStates::Looping(LoopingState {});
        }

        // Not in a loop
        assert!(!is_in_a_loop);
        let is_facing_obstacle = self.grid[next] == '#';
        if is_facing_obstacle {
            return ValidStates::Runnable(RunnableState {
                now: self.now,
                direction: self.direction.turn_right(),
                grid: self.grid,
                history: self.history,
            });
        }

        // No obstacle in front, so move forward
        assert!(!is_facing_obstacle);
        self.grid[next] = self.direction.to_char();
        let steps = self.history.entry(self.direction).or_insert(HashSet::new());
        steps.insert(self.now);

        ValidStates::Runnable(RunnableState {
            now: next,
            direction: self.direction,
            grid: self.grid,
            history: self.history,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_to_grid(input);
    let init_grid = Box::new(grid);
    let history = Box::new(HashMap::new());

    let init_state = InitState::new(init_grid, history);
    let mut current_state = ValidStates::Init(init_state);

    loop {
        current_state = current_state.next();
        match current_state {
            ValidStates::Looping(_) => break,
            ValidStates::OutOfMaze(_) => break,
            _ => {}
        }
    }

    let count = match current_state {
        ValidStates::OutOfMaze(out_of_maze) => out_of_maze.count as u32,
        _ => 0,
    };

    Some(count as u32)
}

fn fill_grid_with_obstacles(grid: &Grid) -> Vec<Grid> {
    let init_grid = Box::new(grid.clone());
    let history = Box::new(HashMap::new());
    let init_state = InitState::new(init_grid, history);
    let mut current_state = ValidStates::Init(init_state);

    loop {
        current_state = current_state.next();
        match current_state {
            ValidStates::Looping(_) => break,
            ValidStates::OutOfMaze(_) => break,
            _ => {}
        }
    }

    let trace = match current_state {
        ValidStates::OutOfMaze(out_of_maze) => *out_of_maze.grid,
        _ => Grid::new(0, 0),
    };

    // try to replace '.' with '#'
    let mut grids = Vec::new();
    for ((row, col), i) in trace.indexed_iter() {
        if *i == 'X' && grid.get(row, col) == Some(&'.') {
            let mut new_grid = grid.clone();
            if let Some(cell) = new_grid.get_mut(row, col) {
                *cell = '#';
            }

            grids.push(new_grid);
        }
    }

    grids
}

pub fn part_two(input: &str) -> Option<u32> {
    let init_grid = parse_to_grid(input);
    let grid_with_obstacles = fill_grid_with_obstacles(&init_grid);
    println!(
        "Number of possible obstacles: {}",
        grid_with_obstacles.len()
    );

    let count = grid_with_obstacles
        .into_par_iter()
        .map(|grid_with_obs| {
            let grid = Box::new(grid_with_obs);
            let history: HistoryPtr = Box::new(HashMap::new());
            let init_state = InitState::new(grid, history);
            let mut current_state = ValidStates::Init(init_state);

            loop {
                current_state = current_state.next();

                match current_state {
                    ValidStates::Looping(_) => return 1,
                    ValidStates::OutOfMaze(_) => return 0,
                    _ => {}
                }
            }
        })
        .sum::<u32>();

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
