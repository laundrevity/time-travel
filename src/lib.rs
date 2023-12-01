use std::collections::HashSet;

pub struct GameOfLife {
    pub living_cells: HashSet<(i32, i32)>,
}

impl GameOfLife {
    pub fn new() -> Self {
        Self {
            living_cells: HashSet::new()
        }
    }

    pub fn add_cell(&mut self, x: i32, y: i32) {
        self.living_cells.insert((x, y));
    }

    fn is_alive(&self, x: i32, y: i32) -> bool {
        self.living_cells.contains(&(x, y))
    }

    pub fn update(&mut self) {
        let candidates = self.candidate_cells();
        let mut next_state: HashSet<(i32, i32)> = HashSet::new();

        for (x, y) in candidates {
            let alive_neighbors = self.alive_neighbors_count(x, y);
            
            if self.is_alive(x, y) && (alive_neighbors == 2 || alive_neighbors == 3) {
                next_state.insert((x, y));
            } else if !self.is_alive(x, y) && alive_neighbors == 3 {
                next_state.insert((x, y));
            }
        }

        self.living_cells = next_state;
    }

    fn candidate_cells(&self) -> HashSet<(i32, i32)> {
        // Returns the list of cells which might change state (e.g., neighbors of all living cells, union living cells)
        let mut candidates = HashSet::new();

        for &(x, y) in self.living_cells.iter() {
            // Add the cell itself
            candidates.insert((x, y));

            // Add all neighbors
            for dx in -1..=1 {
                for dy in -1..=1 {
                    candidates.insert((x+dx, y+dy));
                }
            }
        }

        candidates
    }

    pub fn alive_neighbors_count(&self, x: i32, y: i32) -> usize {
        let mut count = 0;
        
        for dx in -1..=1 {
            for dy in -1..=1 {
                // Exclude cell itself
                if dx != 0 || dy != 0 {
                    if self.is_alive(x+dx, y+dy) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

pub struct SequenceValidator {
    pub special_neighbors: usize,
    pub special_steps_back: usize,
}

pub struct SequenceStatus {
    pub is_valid: bool,
    pub exceptions: usize,
}

impl SequenceValidator {
    pub fn new(sn: usize, ssb: usize) -> Self {
        let special_neighbors = sn;
        let special_steps_back = ssb;
        Self {
            special_neighbors,
            special_steps_back
        }
    }

    pub fn validate(&self, sequence: Vec<HashSet<(i32, i32)>>) -> SequenceStatus {
        // NOTE: valid indicates whether or not the sequence is compatible with the AUGMENTED rules
        let mut valid = true;
        let mut total_exceptions = 0;
        let mut active_exceptions: HashSet<((i32, i32), usize)> = HashSet::new();
        let mut step = 0;

        // Persistent empty-set to return for expected_next_state in case of end
        let empty_set: HashSet<(i32, i32)> = HashSet::new();

        while valid && step < sequence.len() {
            
            let state = &sequence[step];
            let mut game = GameOfLife { living_cells: state.clone() };

            // First, lets check to see if there are any active exceptions we can validate in this state
            active_exceptions = active_exceptions.into_iter().filter(|&((x,y), exception_step)| {
                // Check if it is time to justify this exception, in which case we for sure remove it (return false) from active exceptions
                if step - exception_step == self.special_steps_back {
                    // make a temp game for checking active neighbors of (x,y) in current state
                    if game.alive_neighbors_count(x, y) != self.special_neighbors {
                        valid = false; // Invalidate the entire sequence if this exception is not justified
                    }
                    false
                } else {
                    true
                }
            }).collect();

            // Advance the game state, so that we can calculate potential exceptions in next step
            game.update();
            let expected_next_state = if step + 1 < sequence.len() {
                &sequence[step + 1]
            } else {
                &empty_set
            };

            // Identify new exceptions which occur in next step
            for &cell in expected_next_state.difference(&game.living_cells) {
                println!("found an exception");
                active_exceptions.insert((cell, step + 1));
                total_exceptions += 1;
            }
            
            step += 1;
        }

        if active_exceptions.len() > 0 {
            println!("SETTING valid=false BECAUSE THERE ARE ACTIVE EXCEPTIONS");
            valid = false;
        }

        SequenceStatus { is_valid: valid, exceptions: total_exceptions }
    }

}