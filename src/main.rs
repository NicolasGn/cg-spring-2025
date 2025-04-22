use std::collections::HashMap;
use std::io;
use std::slice::Iter;

type GameHash = u64;
type GameResult = u32;

const GRID_SIZE: usize = 3;
const DICE_MAX_VALUE: u8 = 6;
const GAME_RESULT_MOD: GameResult = GameResult::pow(2, 30);

struct Config {
    depth: u8,
    grid: Grid,
}

struct Combinations {
    data: HashMap<usize, Vec<Vec<usize>>>,
}

#[derive(Clone)]
struct Position {
    line: usize,
    column: usize,
}

struct Capture {
    value: u8,
    positions: Vec<Position>,
}

#[derive(Clone)]
struct Grid {
    cells: [u8; GRID_SIZE * GRID_SIZE],
}

struct GameState {
    depth: u8,
    grid: Grid,
}

struct GameCache {
    results: HashMap<GameHash, GameResult>,
    combinations: Combinations,
}

impl Combinations {
    fn init() -> Self {
        let mut data = HashMap::new();

        data.insert(2, vec![vec![0, 1]]);

        data.insert(3, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![0, 1, 2]]);

        data.insert(
            4,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![0, 1, 2],
                vec![0, 1, 3],
                vec![0, 2, 3],
                vec![1, 2, 3],
                vec![0, 1, 2, 3],
            ],
        );

        Combinations { data }
    }

    #[inline]
    fn get(&self, n: &usize) -> Iter<Vec<usize>> {
        self.data.get(&n).unwrap().iter()
    }
}

impl Config {
    fn parse() -> Self {
        let mut config = Config {
            depth: 0,
            grid: Grid::empty(),
        };

        let mut input_line = String::new();

        io::stdin().read_line(&mut input_line).unwrap();

        config.depth = input_line.trim().parse().unwrap();

        for line in 0..3 as usize {
            let mut inputs = String::new();

            io::stdin().read_line(&mut inputs).unwrap();

            for (column, value) in inputs.split_whitespace().enumerate() {
                config
                    .grid
                    .set(&Position::new(line, column), value.parse().unwrap());
            }
        }

        config
    }
}

impl Position {
    #[inline]
    fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }

    #[inline]
    fn from_index(i: usize) -> Self {
        Position {
            line: i / GRID_SIZE,
            column: i % GRID_SIZE,
        }
    }

    #[inline]
    fn top(&self) -> Option<Self> {
        if self.line >= 1 {
            return Some(Position::new(self.line - 1, self.column));
        }

        None
    }

    #[inline]
    fn right(&self) -> Option<Self> {
        if self.column < GRID_SIZE - 1 {
            return Some(Position::new(self.line, self.column + 1));
        }

        None
    }

    #[inline]
    fn bottom(&self) -> Option<Self> {
        if self.line < GRID_SIZE - 1 {
            return Some(Position::new(self.line + 1, self.column));
        }

        None
    }

    #[inline]
    fn left(&self) -> Option<Self> {
        if self.column >= 1 {
            return Some(Position::new(self.line, self.column - 1));
        }

        None
    }
}

impl Capture {
    #[inline]
    fn new(value: u8, positions: Vec<Position>) -> Self {
        Capture { value, positions }
    }
}

impl Grid {
    fn empty() -> Self {
        Grid {
            cells: [0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    #[inline]
    fn get(&self, position: &Position) -> u8 {
        self.cells[GRID_SIZE * position.line + position.column]
    }

    #[inline]
    fn set(&mut self, position: &Position, value: u8) -> &Self {
        self.cells[GRID_SIZE * position.line + position.column] = value;

        self
    }

    #[inline]
    fn free_cells(&self) -> Vec<Position> {
        let mut cells = Vec::with_capacity(self.cells.len());

        for i in 0..self.cells.len() {
            if self.cells[i] == 0 {
                cells.push(Position::from_index(i));
            }
        }

        cells
    }

    #[inline]
    fn neighbour_dices(&self, position: &Position) -> Vec<Position> {
        let mut neighbours = Vec::with_capacity(4);

        if let Some(top) = position.top() {
            let value = self.get(&top);

            if value > 0 {
                neighbours.push(top);
            }
        }

        if let Some(right) = position.right() {
            let value: u8 = self.get(&right);

            if value > 0 {
                neighbours.push(right);
            }
        }

        if let Some(bottom) = position.bottom() {
            let value = self.get(&bottom);

            if value > 0 {
                neighbours.push(bottom);
            }
        }

        if let Some(left) = position.left() {
            let value = self.get(&left);

            if value > 0 {
                neighbours.push(left);
            }
        }

        neighbours
    }

    #[inline]
    fn captures(&self, position: &Position, combinations: &Combinations) -> Vec<Capture> {
        let mut captures = Vec::with_capacity(4);
        let dices = self.neighbour_dices(position);
        let count = dices.len();

        if count < 2 {
            return captures;
        }

        'combination: for combination in combinations.get(&count) {
            let mut positions = Vec::with_capacity(4);
            let mut value = 0;

            for index in combination.iter() {
                let position = &dices[*index];
                positions.push(position.clone());
                value += self.get(position);

                if value > DICE_MAX_VALUE {
                    continue 'combination;
                }
            }

            captures.push(Capture::new(value, positions));
        }

        captures
    }
}

impl GameState {
    fn init(grid: Grid) -> Self {
        GameState { grid, depth: 0 }
    }

    #[inline]
    fn next(&self, position: &Position, capture: Option<&Capture>) -> Self {
        let mut grid = self.grid.clone();

        match capture {
            None => {
                grid.set(position, 1);
            }
            Some(capture) => {
                grid.set(position, capture.value);

                for captured in capture.positions.iter() {
                    grid.set(captured, 0);
                }
            }
        };

        GameState {
            grid,
            depth: self.depth + 1,
        }
    }

    fn play(&self, position: &Position, combinations: &Combinations) -> Vec<Self> {
        let captures = self.grid.captures(position, combinations);

        if captures.len() == 0 {
            return vec![self.next(position, None)];
        }

        let next_states = captures
            .iter()
            .map(|capture| self.next(position, Some(&capture)))
            .collect();

        next_states
    }

    #[inline]
    fn is_final(&self) -> bool {
        for i in 0..self.grid.cells.len() {
            if self.grid.cells[i] == 0 {
                return false;
            }
        }

        true
    }

    #[inline]
    fn result(&self) -> GameResult {
        100000000 * self.grid.cells[0] as GameResult
            + 10000000 * self.grid.cells[1] as GameResult
            + 1000000 * self.grid.cells[2] as GameResult
            + 100000 * self.grid.cells[3] as GameResult
            + 10000 * self.grid.cells[4] as GameResult
            + 1000 * self.grid.cells[5] as GameResult
            + 100 * self.grid.cells[6] as GameResult
            + 10 * self.grid.cells[7] as GameResult
            + self.grid.cells[8] as GameResult
    }

    #[inline]
    fn hash(&self) -> GameHash {
        let mut hash = self.depth as GameHash;

        for i in 0..self.grid.cells.len() {
            hash <<= 4;
            hash |= self.grid.cells[i] as GameHash;
        }

        hash
    }

    fn compute(&self, max_depth: u8, cache: &mut GameCache) -> GameResult {
        let hash = self.hash();

        if let Some(cached_result) = cache.results.get(&hash) {
            return *cached_result;
        }

        if self.depth == max_depth || self.is_final() {
            let result = self.result();

            cache.results.insert(hash, result);

            return result;
        }

        let mut result = 0;

        for position in self.grid.free_cells().iter() {
            for next_state in self.play(position, &cache.combinations).iter() {
                let next_state_result = next_state.compute(max_depth, cache);

                result = (result + next_state_result) % GAME_RESULT_MOD;
            }
        }

        cache.results.insert(hash, result);

        result
    }
}

impl GameCache {
    fn new() -> Self {
        GameCache {
            results: HashMap::new(),
            combinations: Combinations::init(),
        }
    }
}

fn main() {
    let config = Config::parse();

    let game = GameState::init(config.grid);
    let mut cache = GameCache::new();

    let result = game.compute(config.depth, &mut cache);

    println!("{}", result);
}
