use nalgebra::DMatrix;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error(
        "Payoff matrix dimension mismatch: expected ({rows}, {cols}), got ({actual_rows}, {actual_cols})"
    )]
    DimensionMismatch {
        rows: usize,
        cols: usize,
        actual_rows: usize,
        actual_cols: usize,
    },
    #[error("Strategy vector length mismatch: expected {expected}, got {actual}")]
    StrategyLengthMismatch { expected: usize, actual: usize },
    #[error("Invalid probability distribution: sum must be 1.0, got {0}")]
    InvalidDistribution(f64),
}

#[derive(Clone, Debug)]
pub struct NormalFormGame {
    pub row_actions: Vec<String>,
    pub col_actions: Vec<String>,
    pub u1: DMatrix<f64>,
    pub u2: DMatrix<f64>,
}

impl NormalFormGame {
    pub fn new(
        row_actions: Vec<String>,
        col_actions: Vec<String>,
        u1: DMatrix<f64>,
        u2: DMatrix<f64>,
    ) -> Result<Self, GameError> {
        let rows = row_actions.len();
        let cols = col_actions.len();

        if (u1.nrows(), u1.ncols()) != (rows, cols) {
            return Err(GameError::DimensionMismatch {
                rows,
                cols,
                actual_rows: u1.nrows(),
                actual_cols: u1.ncols(),
            });
        }
        if (u2.nrows(), u2.ncols()) != (rows, cols) {
            return Err(GameError::DimensionMismatch {
                rows,
                cols,
                actual_rows: u2.nrows(),
                actual_cols: u2.ncols(),
            });
        }

        Ok(Self {
            row_actions,
            col_actions,
            u1,
            u2,
        })
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.u1.nrows(), self.u1.ncols())
    }

    pub fn is_zero_sum(&self, epsilon: f64) -> bool {
        let sum = &self.u1 + &self.u2;
        sum.iter().all(|&v| v.abs() < epsilon)
    }

    /// Computes E[u1] = p^T * u1 * q and E[u2] = p^T * u2 * q
    pub fn expected_payoffs(&self, p: &[f64], q: &[f64]) -> Result<(f64, f64), GameError> {
        let (m, n) = self.shape();
        if p.len() != m {
            return Err(GameError::StrategyLengthMismatch {
                expected: m,
                actual: p.len(),
            });
        }
        if q.len() != n {
            return Err(GameError::StrategyLengthMismatch {
                expected: n,
                actual: q.len(),
            });
        }

        let p_vec = nalgebra::DVector::from_row_slice(p);
        let q_vec = nalgebra::DVector::from_row_slice(q);

        let e1 = (&p_vec.transpose() * &self.u1 * &q_vec)[(0, 0)];
        let e2 = (&p_vec.transpose() * &self.u2 * &q_vec)[(0, 0)];

        Ok((e1, e2))
    }
}

