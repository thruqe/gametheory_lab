use crate::core::normal_form::NormalFormGame;
use itertools::Itertools;
use nalgebra::{DMatrix, DVector};

#[derive(Debug, Clone, PartialEq)]
pub struct NashEquilibrium {
    pub p1_strategy: Vec<f64>,
    pub p2_strategy: Vec<f64>,
}

fn solve_indifference_system(
    payoffs: &DMatrix<f64>,
    candidate_support: &[usize],
    total_actions: usize,
) -> Option<Vec<f64>> {
    let k = candidate_support.len();
    if k == 1 {
        let mut prob = vec![0.0; total_actions];
        prob[candidate_support[0]] = 1.0;
        return Some(prob);
    }

    // Build linear equality system:
    // (A_i - A_{i+1}) * q = 0 for all i in opponent support
    // sum(q) = 1
    let rows = payoffs.nrows();
    let mut a_eq = DMatrix::<f64>::zeros(rows, k);
    let mut b_eq = DVector::<f64>::zeros(rows);

    for i in 0..(rows - 1) {
        for (col_idx, &supp_j) in candidate_support.iter().enumerate() {
            a_eq[(i, col_idx)] = payoffs[(i, supp_j)] - payoffs[(i + 1, supp_j)];
        }
    }

    for col_idx in 0..k {
        a_eq[(rows - 1, col_idx)] = 1.0;
    }
    b_eq[rows - 1] = 1.0;

    let decomp = a_eq.lu();
    let sol = decomp.solve(&b_eq)?;

    // Check validity of probability distribution: q_j >= 0
    if sol.iter().all(|&v| v >= -1e-9) {
        let mut full_strategy = vec![0.0; total_actions];
        let mut sum = 0.0;
        for (col_idx, &supp_j) in candidate_support.iter().enumerate() {
            let val = sol[col_idx].max(0.0);
            full_strategy[supp_j] = val;
            sum += val;
        }
        if sum > 0.0 {
            for v in &mut full_strategy {
                *v /= sum;
            }
            return Some(full_strategy);
        }
    }

    None
}

pub fn compute_nash_equilibria(game: &NormalFormGame) -> Vec<NashEquilibrium> {
    let (m, n) = game.shape();
    let mut equilibria = Vec::new();
    let max_k = m.min(n);

    for k in 1..=max_k {
        for supp_r in (0..m).combinations(k) {
            for supp_c in (0..n).combinations(k) {
                // Player 1 indifference system across supp_r yields player 2 strategy
                let sub_u1 = DMatrix::from_rows(
                    &supp_r.iter().map(|&r| game.u1.row(r)).collect::<Vec<_>>(),
                );
                let q = match solve_indifference_system(&sub_u1, &supp_c, n) {
                    Some(strat) => strat,
                    None => continue,
                };

                // Player 2 indifference system across supp_c yields player 1 strategy
                let sub_u2_t = DMatrix::from_rows(
                    &supp_c.iter().map(|&c| game.u2.column(c).transpose()).collect::<Vec<_>>(),
                );
                let p = match solve_indifference_system(&sub_u2_t, &supp_r, m) {
                    Some(strat) => strat,
                    None => continue,
                };

                // Best response condition check
                let q_vec = DVector::from_row_slice(&q);
                let p_vec = DVector::from_row_slice(&p);

                let payoffs_p1 = &game.u1 * &q_vec;
                let max_u1 = payoffs_p1.max();
                let supp_r_valid = supp_r.iter().all(|&r| (payoffs_p1[r] - max_u1).abs() < 1e-5)
                    && payoffs_p1.iter().all(|&v| v <= max_u1 + 1e-5);

                if !supp_r_valid {
                    continue;
                }

                let payoffs_p2 = &p_vec.transpose() * &game.u2;
                let max_u2 = payoffs_p2.max();
                let supp_c_valid = supp_c.iter().all(|&c| (payoffs_p2[c] - max_u2).abs() < 1e-5)
                    && payoffs_p2.iter().all(|&v| v <= max_u2 + 1e-5);

                if !supp_c_valid {
                    continue;
                }

                let eq = NashEquilibrium {
                    p1_strategy: p,
                    p2_strategy: q,
                };

                // Deduplicate
                let exists = equilibria.iter().any(|existing: &NashEquilibrium| {
                    existing.p1_strategy.iter().zip(&eq.p1_strategy).all(|(a, b)| (a - b).abs() < 1e-5)
                        && existing.p2_strategy.iter().zip(&eq.p2_strategy).all(|(a, b)| (a - b).abs() < 1e-5)
                });

                if !exists {
                    equilibria.push(eq);
                }
            }
        }
    }

    equilibria
}
