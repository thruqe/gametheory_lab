use crate::core::normal_form::{GameError, NormalFormGame};
use nalgebra::DMatrix;

/// Creates a discretized Cournot Duopoly game.
///
/// `quantity_levels`: number of discrete quantity choices for each firm.
/// `max_quantity`: the maximum quantity a firm can produce.
/// `a`: the intercept of the inverse demand curve (P = a - bQ).
/// `b`: the slope of the inverse demand curve.
/// `c`: the constant marginal cost.
pub fn create_cournot_duopoly(
    quantity_levels: usize,
    max_quantity: f64,
    a: f64,
    b: f64,
    c: f64,
) -> Result<NormalFormGame, GameError> {
    if quantity_levels == 0 {
        return Err(GameError::DimensionMismatch {
            rows: 0,
            cols: 0,
            actual_rows: 0,
            actual_cols: 0,
        }); // Or a more specific error, but this works for now.
    }

    let mut u1 = DMatrix::<f64>::zeros(quantity_levels, quantity_levels);
    let mut u2 = DMatrix::<f64>::zeros(quantity_levels, quantity_levels);

    let step = if quantity_levels > 1 {
        max_quantity / (quantity_levels - 1) as f64
    } else {
        0.0
    };

    let mut actions = Vec::with_capacity(quantity_levels);

    for i in 0..quantity_levels {
        let q1 = (i as f64) * step;
        actions.push(format!("Q={:.1}", q1));

        for j in 0..quantity_levels {
            let q2 = (j as f64) * step;
            let total_q = q1 + q2;

            // Inverse demand curve: P(Q) = max(0, a - b*Q)
            let price = (a - b * total_q).max(0.0);

            let profit1 = q1 * (price - c);
            let profit2 = q2 * (price - c);

            u1[(i, j)] = profit1;
            u2[(i, j)] = profit2;
        }
    }

    NormalFormGame::new(actions.clone(), actions, u1, u2)
}
