use std::collections::HashSet;

advent_of_code::solution!(15);

type Grid = grid::Grid<char>;
type GridPtr = Box<Grid>;
type Moves = Vec<Direction>;

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    let (_max_row, max_col) = (grid.size().0, grid.size().1);
    for ((_row, col), i) in grid.indexed_iter() {
        print!("{i}");
        if col == max_col - 1 {
            println!();
        }
    }
}

fn parse_to_grid(input: &str) -> Grid {
    let width = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();

    Grid::from_vec(cells, width)
}

fn read_grid_and_moves(input: &str) -> (Grid, Moves) {
    // split by empty line
    let mut parts = input.split("\n\n");
    let grid = parse_to_grid(parts.next().unwrap());
    let moves = parts
        .next()
        .unwrap()
        .chars()
        .filter_map(|c| Direction::from_char(c))
        .collect::<Vec<Direction>>();

    (grid, moves)
}

fn start_from(grid: &Grid) -> (usize, usize) {
    for ((row, col), i) in grid.indexed_iter() {
        if *i == '@' {
            return (row, col);
        }
    }

    panic!("No starting point found");
}

fn next_cell(grid: &Grid, start: (usize, usize), dir: Direction) -> (usize, usize) {
    let (max_row, max_col) = (grid.size().0, grid.size().1);
    let (row, col) = start;
    let next = match dir {
        Direction::Up if row > 0 => (row - 1, col),
        Direction::Down if row < max_row - 1 => (row + 1, col),
        Direction::Left if col > 0 => (row, col - 1),
        Direction::Right if col < max_col - 1 => (row, col + 1),
        _ => unreachable!("Out of bounds"),
    };

    next
}

fn can_push_towards(
    grid: &Grid,
    start: (usize, usize),
    dir: Direction,
    movable_cells: &mut HashSet<(usize, usize, char)>,
) -> bool {
    println!("start: {:?}, dir: {:?}", start, dir);
    let next_cell = next_cell(grid, start, dir);
    let next_value = grid[next_cell];
    // No obstacle
    if next_value == '.' {
        return true;
    }
    // Wall
    if next_value == '#' {
        return false;
    }

    // Obstacle
    let (next_pair, next_pair_value) = if next_value == '[' {
        ((next_cell.0, next_cell.1 + 1), ']')
    } else {
        // next_value == ']'
        ((next_cell.0, next_cell.1 - 1), '[')
    };

    if dir == Direction::Left || dir == Direction::Right {
        // Push horizontally
        if can_push_towards(grid, next_cell, dir, movable_cells) {
            movable_cells.insert((next_cell.0, next_cell.1, next_value));
            return true;
        }
    } else {
        // Push vertically
        if can_push_towards(grid, next_cell, dir, movable_cells)
            && can_push_towards(grid, next_pair, dir, movable_cells)
        {
            // Push the obstacle
            movable_cells.insert((next_cell.0, next_cell.1, next_value));
            movable_cells.insert((next_pair.0, next_pair.1, next_pair_value));
            return true;
        }
    }

    false
}

fn empty_cell_along_direction(
    grid: &Grid,
    start: (usize, usize),
    dir: Direction,
) -> Option<(usize, usize)> {
    let mut now = start;

    loop {
        let next = next_cell(grid, now, dir);
        let value = grid[next];
        // Run into an obstacle
        if value == 'O' {
            now = next;
            continue;
        }

        // Run into the wall
        if value == '#' {
            return None;
        }
        // Run into an empty cell
        if value == '.' {
            return Some(next);
        }
    }
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

    #[allow(dead_code)]
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
    moves: Moves,
    grid: GridPtr,
    push: bool,
}

impl InitState {
    fn new(grid: GridPtr, moves: Moves, push: bool) -> InitState {
        let now = start_from(&grid);
        InitState {
            now,
            moves,
            grid,
            push,
        }
    }
}

struct RunnableState {
    now: (usize, usize),
    moves: Moves,
    grid: GridPtr,
}

struct PushingState {
    now: (usize, usize),
    moves: Moves,
    grid: GridPtr,
}

struct OutOfMazeState {
    grid: GridPtr,
    score: usize,
}

impl OutOfMazeState {
    fn new(grid: GridPtr) -> OutOfMazeState {
        OutOfMazeState {
            score: grid
                .indexed_iter()
                .map(|((row, col), i)| {
                    if *i == 'O' || *i == '[' {
                        100 * row + col
                    } else {
                        0
                    }
                })
                .sum(),
            grid: grid,
        }
    }
}

enum ValidStates {
    Init(InitState),
    Runnable(RunnableState),
    Pushing(PushingState),
    OutOfMaze(OutOfMazeState),
}

impl ValidStates {
    // Consume the current state and return the next state
    fn next(self) -> ValidStates {
        match self {
            ValidStates::Init(init) => {
                if init.push {
                    ValidStates::Pushing(PushingState {
                        now: init.now,
                        moves: init.moves,
                        grid: init.grid,
                    })
                } else {
                    ValidStates::Runnable(RunnableState {
                        now: init.now,
                        moves: init.moves,
                        grid: init.grid,
                    })
                }
            }
            ValidStates::Runnable(runnable) => runnable.next(),
            ValidStates::Pushing(pushing) => pushing.next(),
            ValidStates::OutOfMaze(out_of_maze) => ValidStates::OutOfMaze(OutOfMazeState {
                grid: out_of_maze.grid,
                score: out_of_maze.score,
            }),
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        match self {
            ValidStates::Init(init) => {
                print!("Init state: ");
                print!("Now at {:?}, ", init.now);
                print!("Moves: {:?}, ", init.moves);
                println!();
                print_grid(&init.grid);
            }
            ValidStates::Runnable(runnable) => {
                print!("Runnable state: ");
                print!("Now at {:?}, ", runnable.now);
                print!("Moves {:?}, ", runnable.moves);
                println!();
                print_grid(&runnable.grid);
            }
            ValidStates::Pushing(pushing) => {
                print!("Pushing state: ");
                print!("Now at {:?}", pushing.now);
                if let Some(direction) = pushing.moves.first() {
                    print!(", facing {:?}", direction);
                }
                println!();
                print_grid(&pushing.grid);
            }
            ValidStates::OutOfMaze(out_of_maze) => {
                println!("Out of maze:");
                print_grid(&out_of_maze.grid);
                println!("score: {}", out_of_maze.score);
            }
        }
    }
}

impl RunnableState {
    fn step(&self) -> Option<(usize, usize)> {
        if let Some(direction) = self.moves.first() {
            let next = next_cell(&self.grid, self.now, *direction);
            return Some(next);
        }

        None
    }

    // Consume the current state and return the next state
    // Mark as mutable to allow changing the grid and history
    fn next(mut self) -> ValidStates {
        if self.moves.is_empty() {
            return ValidStates::OutOfMaze(OutOfMazeState::new(self.grid));
        }

        let next = self.step();
        let is_out_of_maze = next.is_none();
        if is_out_of_maze {
            return ValidStates::OutOfMaze(OutOfMazeState::new(self.grid));
        }

        // Still in the maze
        assert!(!is_out_of_maze && next.is_some());
        let next = next.unwrap();
        let direction = self.moves.remove(0);

        // Face a wall
        let is_facing_wall = self.grid[next] == '#';
        if is_facing_wall {
            println!(
                "Moving from {:?} to {:?} in dir {:?} but facing a wall",
                self.now, next, direction
            );
            // Don't move
            return ValidStates::Runnable(RunnableState {
                now: self.now,
                moves: self.moves,
                grid: self.grid,
            });
        }

        // Face an obstacle
        let is_facing_obstacle = self.grid[next] == 'O';
        if is_facing_obstacle {
            println!("Moving from {:?} to {:?} but facing an O", self.now, next);
            // Check if it possible to push the obstacle forward
            if let Some(empty_cell_along_direction) =
                empty_cell_along_direction(&self.grid, self.now, direction)
            {
                println!("Pushing {:?} to {:?}", next, empty_cell_along_direction);

                // We push the next cell to the empty cell
                self.grid[empty_cell_along_direction] = 'O';
                self.grid[self.now] = '.';
                self.grid[next] = '@';

                return ValidStates::Runnable(RunnableState {
                    now: next,
                    moves: self.moves,
                    grid: self.grid,
                });
            }
            println!("Can't push forward, stay at {:?}", self.now);

            // If can't push
            return ValidStates::Runnable(RunnableState {
                now: self.now,
                moves: self.moves,
                grid: self.grid,
            });
        }

        // No obstacle in front, so move forward
        assert!(!is_facing_obstacle);
        println!(
            "Moving from {:?} to {:?} in dir {:?}",
            self.now, next, direction
        );
        self.grid[next] = '@';
        self.grid[self.now] = '.';
        ValidStates::Runnable(RunnableState {
            now: next,
            moves: self.moves,
            grid: self.grid,
        })
    }
}

impl PushingState {
    fn step(&self) -> Option<(usize, usize)> {
        if let Some(direction) = self.moves.first() {
            let next = next_cell(&self.grid, self.now, *direction);
            return Some(next);
        }

        None
    }

    // Consume the current state and return the next state
    // Mark as mutable to allow changing the grid and history
    fn next(mut self) -> ValidStates {
        if self.moves.is_empty() {
            return ValidStates::OutOfMaze(OutOfMazeState::new(self.grid));
        }

        let next = self.step();
        let is_out_of_maze = next.is_none();
        if is_out_of_maze {
            return ValidStates::OutOfMaze(OutOfMazeState::new(self.grid));
        }

        // Still in the maze
        assert!(!is_out_of_maze && next.is_some());
        let next = next.unwrap();
        let direction = self.moves.remove(0);

        // Face a wall
        let is_facing_wall = self.grid[next] == '#';
        if is_facing_wall {
            println!(
                "Moving from {:?} to {:?} in dir {:?} but facing a wall",
                self.now, next, direction
            );
            // Don't move
            return ValidStates::Pushing(PushingState {
                now: self.now,
                moves: self.moves,
                grid: self.grid,
            });
        }

        // Face an obstacle
        let is_facing_obstacle = self.grid[next] == '[' || self.grid[next] == ']';
        if is_facing_obstacle {
            println!(
                "Moving from {:?} to {:?} but facing an {:?}",
                self.now, next, self.grid[next]
            );

            let mut movable_cells = HashSet::new();
            let can_push = can_push_towards(&self.grid, self.now, direction, &mut movable_cells);
            // Check if it possible to push the obstacle forward
            if can_push {
                println!("Pushing towards {:?}", direction);
                let cells_after_move = movable_cells
                    .iter()
                    .map(|cell| {
                        let (row, col, value) = *cell;
                        let next = next_cell(&self.grid, (row, col), direction);
                        (next.0, next.1, value)
                    })
                    .collect::<Vec<_>>();

                for (row, col, _) in &movable_cells {
                    self.grid[(*row, *col)] = '.';
                }
                for (row, col, value) in cells_after_move {
                    self.grid[(row, col)] = value;
                }
                self.grid[self.now] = '.';
                self.grid[next] = '@';

                return ValidStates::Pushing(PushingState {
                    now: next,
                    moves: self.moves,
                    grid: self.grid,
                });
            }
            println!("Can't push forward, stay at {:?}", self.now);

            // If can't push
            return ValidStates::Pushing(PushingState {
                now: self.now,
                moves: self.moves,
                grid: self.grid,
            });
        }

        // No obstacle in front, so move forward
        assert!(!is_facing_obstacle);
        self.grid[next] = '@';
        self.grid[self.now] = '.';
        ValidStates::Pushing(PushingState {
            now: next,
            moves: self.moves,
            grid: self.grid,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, moves) = read_grid_and_moves(input);
    println!("Moves: {:?}", moves);
    let init_state = InitState::new(Box::new(grid), moves, false);
    let mut current_state = ValidStates::Init(init_state);

    loop {
        current_state = current_state.next();
        match current_state {
            ValidStates::OutOfMaze(_) => break,
            _ => {}
        }
    }

    let count = match current_state {
        ValidStates::OutOfMaze(out_of_maze) => out_of_maze.score as u32,
        _ => {
            println!("Unexpected state:");
            current_state.print();
            0
        }
    };

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, moves) = read_grid_and_moves(input);
    let init_state = InitState::new(Box::new(grid), moves, true);
    let mut current_state = ValidStates::Init(init_state);

    loop {
        current_state = current_state.next();
        match current_state {
            ValidStates::OutOfMaze(_) => break,
            _ => {}
        }
    }

    current_state.print();
    let count = match current_state {
        ValidStates::OutOfMaze(out_of_maze) => out_of_maze.score as u32,
        _ => {
            println!("Unexpected state:");
            current_state.print();
            0
        }
    };

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
