use gametheory_lab::core::normal_form::NormalFormGame;
use gametheory_lab::repeated::tournament::{AlwaysDefect, Grudger, TitForTat, run_match};
use gametheory_lab::solvers::elimination::eliminate_dominated_strategies;
use gametheory_lab::solvers::support_enumeration::compute_nash_equilibria;
use nalgebra::DMatrix;

fn main() {
    println!("=== 1. Prisoner's Dilemma (IESDS & Nash) ===");
    let pd_u1 = DMatrix::from_row_slice(2, 2, &[3.0, 0.0, 5.0, 1.0]);
    let pd_u2 = DMatrix::from_row_slice(2, 2, &[3.0, 5.0, 0.0, 1.0]);

    let pd_game = NormalFormGame::new(
        vec!["C".into(), "D".into()],
        vec!["C".into(), "D".into()],
        pd_u1,
        pd_u2,
    )
    .unwrap();

    let reduced = eliminate_dominated_strategies(&pd_game);
    println!("Surviving rows after IESDS: {:?}", reduced.row_actions);
    println!("Surviving cols after IESDS: {:?}", reduced.col_actions);

    let equilibria = compute_nash_equilibria(&pd_game);
    for (i, eq) in equilibria.iter().enumerate() {
        println!(
            "NE #{}: P1={:?}, P2={:?}",
            i + 1,
            eq.p1_strategy,
            eq.p2_strategy
        );
    }

    println!("\n=== 2. Matching Pennies (Mixed Strategy NE) ===");
    let mp_u1 = DMatrix::from_row_slice(2, 2, &[1.0, -1.0, -1.0, 1.0]);
    let mp_u2 = DMatrix::from_row_slice(2, 2, &[-1.0, 1.0, 1.0, -1.0]);

    let mp_game = NormalFormGame::new(
        vec!["H".into(), "T".into()],
        vec!["H".into(), "T".into()],
        mp_u1,
        mp_u2,
    )
    .unwrap();

    let mp_eq = compute_nash_equilibria(&mp_game);
    for (i, eq) in mp_eq.iter().enumerate() {
        let (val1, val2) = mp_game
            .expected_payoffs(&eq.p1_strategy, &eq.p2_strategy)
            .unwrap();
        println!(
            "NE #{}: P1={:?}, P2={:?} -> Expected Utility: ({:.2}, {:.2})",
            i + 1,
            eq.p1_strategy,
            eq.p2_strategy,
            val1,
            val2
        );
    }

    println!("\n=== 3. Repeated Game Simulation (200 Rounds) ===");
    // Payoff structure: [Row][Col] -> (P1, P2)
    let pd_payoffs = [
        [(3.0, 3.0), (0.0, 5.0)], // Cooperate
        [(5.0, 0.0), (1.0, 1.0)], // Defect
    ];

    let mut tft = TitForTat;
    let mut def = AlwaysDefect;
    let (s_tft, s_def) = run_match(&mut tft, &mut def, pd_payoffs, 200);
    println!(
        "TitForTat vs AlwaysDefect -> Scores: ({:.0}, {:.0})",
        s_tft, s_def
    );

    let mut tft2 = TitForTat;
    let mut grudger = Grudger::new();
    let (s_tft2, s_grg) = run_match(&mut tft2, &mut grudger, pd_payoffs, 200);
    println!(
        "TitForTat vs Grudger      -> Scores: ({:.0}, {:.0})",
        s_tft2, s_grg
    );
}
