use bevy::prelude::*;

// import slice from the same directory
use crate::game_of_life::slice::Slice;

#[derive(Resource)]
pub struct GameOfLife {
    /// max size of the slice buffer 
    slice_buffer_max_size: usize,
    /// current slice index, must be in range [0, slice_buffer_max_size)
    curr_slice_idx: i64,
    /// previous slice index, must be in range [0, slice_buffer_max_size)
    prev_slice_idx: i64,
    /// next slice index - may be None if no next slice has been computed. Otherwise, must be in range [0, slice_buffer_max_size)
    next_slice_idx: Option<i64>,
    /// slice buffer - stores some history of the game
    slice_buffer: Vec<Slice>,
}

impl GameOfLife {
    /// Create a new GameOfLife with the given slice buffer size and slice size, and initialize the slice
    pub fn new(slice_buffer_max_size: usize, slice_size: usize) -> GameOfLife {
        let mut game = GameOfLife {
            slice_buffer_max_size,
            curr_slice_idx: 0,
            prev_slice_idx: -1,
            next_slice_idx: None,
            slice_buffer: Vec::with_capacity(slice_buffer_max_size),
        };
        
        game.init(slice_size);

        game
    }

    /// Initialize the slice buffer with a new, randomly generated slice of the given size.
    /// Panics if the slice buffer is not empty.
    fn init(&mut self, slice_size: usize) {
        if self.slice_buffer.len() > 0 {
            panic!("GameOfLife::init() called on a non-empty slice buffer");
        }
        let mut slice = Slice::new(slice_size);
        slice.randomize();
        self.slice_buffer.push(slice);
    }

    /// Reset the game. This is like new(), but without the re-allocation.
    /// Panics if the slice buffer is not empty.
    pub fn reset (&mut self) {
        let slice_size = self.slice_buffer[0].get_size();
        self.curr_slice_idx = 0;
        self.prev_slice_idx = -1;
        self.next_slice_idx = None;
        self.slice_buffer.clear();
        self.init(slice_size);
    }

    /// Get the current slice as read-only
    pub fn get_curr_slice(&self) -> &Slice {
        &self.slice_buffer[self.curr_slice_idx as usize]
    }

    /// Get the current slice as mutable
    pub fn get_curr_slice_mut(&mut self) -> &mut Slice {
        &mut self.slice_buffer[self.curr_slice_idx as usize]
    }

    /// Get the previous slice as read-only
    /// Panics if there is no previous slice
    pub fn get_prev_slice(&self) -> &Slice {
        if self.prev_slice_idx < 0 {
            panic!("GameOfLife::get_prev_slice() called on a slice buffer with no previous slice");
        }
        &self.slice_buffer[self.prev_slice_idx as usize]
    }

    /// Get the previous slice as mutable.
    /// Panics if there is no previous slice.
    pub fn get_prev_slice_mut(&mut self) -> &mut Slice {
        if self.prev_slice_idx < 0 {
            panic!("GameOfLife::get_prev_slice_mut() called on a slice buffer with no previous slice");
        }
        &mut self.slice_buffer[self.prev_slice_idx as usize]
    }

    /// Get the next slice as read-only.
    /// Panics if there is no next slice.
    pub fn get_next_slice(&self) -> Option<&Slice> {
        match self.next_slice_idx {
            Some(idx) => {
                if idx < 0 {
                    panic!("GameOfLife::get_next_slice() called on a slice buffer with no next slice");
                }
                Some(&self.slice_buffer[idx as usize])
            },
            None => None,
        }
    }

    /// Get the next slice as mutable.
    /// Panics if there is no next slice.
    pub fn get_next_slice_mut(&mut self) -> Option<&mut Slice> {
        match self.next_slice_idx {
            Some(idx) => {
                if idx < 0 {
                    panic!("GameOfLife::get_next_slice_mut() called on a slice buffer with no next slice");
                }
                Some(&mut self.slice_buffer[idx as usize])
            },
            None => None,
        }
    }

    /// Step forward one generation in the game by either reloading the next slice from the slice buffer, or computing it if it does not exist.
    /// If the slice buffer is full, the oldest slice is removed. 
    /// Currently, this utilizes the naive optimized algorithm.
    pub fn step_forward(&mut self) {
        // if there is no next slice, compute it
        if self.curr_slice_idx == self.slice_buffer.len() as i64 - 1 {
            let mut slice = self.get_curr_slice().clone();
            slice.next_generation_naive_optimized();
            
            // if the slice buffer is full, remove the oldest slice and update the indices
            if self.slice_buffer.len() >= self.slice_buffer_max_size {
                self.slice_buffer.remove(0);
                self.prev_slice_idx -= 1;
                self.curr_slice_idx -= 1;
            }

            self.slice_buffer.push(slice);
            self.next_slice_idx = Some(self.slice_buffer.len() as i64 - 1);
        } 

        // update the indices
        self.prev_slice_idx = self.curr_slice_idx;
        self.curr_slice_idx = self.next_slice_idx.unwrap();
        self.next_slice_idx = match self.curr_slice_idx == self.slice_buffer.len() as i64 - 1 {
            true => None,
            false => Some(self.curr_slice_idx + 1),
        };
    }

    /// Step backward one generation in the gam by reloading the previous slice from the slice buffer, then updating the indices.
    pub fn step_backward(&mut self) {
        // if there is no previous slice, do nothing
        if self.prev_slice_idx < 0 {
            return;
        }

        // update the indices
        self.next_slice_idx = Some(self.curr_slice_idx);
        self.curr_slice_idx = self.prev_slice_idx;
        self.prev_slice_idx = match self.curr_slice_idx == 0 {
            true => -1,
            false => self.curr_slice_idx - 1,
        };
    }

}
