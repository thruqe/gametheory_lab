use crate::core::normal_form::NormalFormGame;
use nalgebra::DMatrix;

pub fn eliminate_dominated_strategies(game: &NormalFormGame) -> NormalFormGame {
    let mut u1 = game.u1.clone();
    let mut u2 = game.u2.clone();
    let mut rows = game.row_actions.clone();
    let mut cols = game.col_actions.clone();

    let mut changed = true;

    while changed {
        changed = false;

        // 1. Check Player 1 strictly dominated rows
        let current_rows = u1.nrows();
        let mut surviving_rows = Vec::new();

        for i in 0..current_rows {
            let mut is_dominated = false;
            for k in 0..current_rows {
                if k == i {
                    continue;
                }
                // Check if row k strictly beats row i across all active columns
                let strictly_dominates = (0..u1.ncols()).all(|c| u1[(k, c)] > u1[(i, c)]);
                if strictly_dominates {
                    is_dominated = true;
                    break;
                }
            }
            if !is_dominated {
                surviving_rows.push(i);
            }
        }

        if surviving_rows.len() < current_rows {
            let next_u1 = DMatrix::from_rows(
                &surviving_rows
                    .iter()
                    .map(|&r| u1.row(r))
                    .collect::<Vec<_>>(),
            );
            let next_u2 = DMatrix::from_rows(
                &surviving_rows
                    .iter()
                    .map(|&r| u2.row(r))
                    .collect::<Vec<_>>(),
            );
            rows = surviving_rows.iter().map(|&r| rows[r].clone()).collect();
            u1 = next_u1;
            u2 = next_u2;
            changed = true;
        }

        // 2. Check Player 2 strictly dominated columns
        let current_cols = u2.ncols();
        let mut surviving_cols = Vec::new();

        for j in 0..current_cols {
            let mut is_dominated = false;
            for k in 0..current_cols {
                if k == j {
                    continue;
                }
                // Check if column k strictly beats column j across all active rows
                let strictly_dominates = (0..u2.nrows()).all(|r| u2[(r, k)] > u2[(r, j)]);
                if strictly_dominates {
                    is_dominated = true;
                    break;
                }
            }
            if !is_dominated {
                surviving_cols.push(j);
            }
        }

        if surviving_cols.len() < current_cols {
            let next_u1 = DMatrix::from_columns(
                &surviving_cols
                    .iter()
                    .map(|&c| u1.column(c))
                    .collect::<Vec<_>>(),
            );
            let next_u2 = DMatrix::from_columns(
                &surviving_cols
                    .iter()
                    .map(|&c| u2.column(c))
                    .collect::<Vec<_>>(),
            );
            cols = surviving_cols.iter().map(|&c| cols[c].clone()).collect();
            u1 = next_u1;
            u2 = next_u2;
            changed = true;
        }
    }

    NormalFormGame::new(rows, cols, u1, u2).expect("Valid subgame dimensions")
}
