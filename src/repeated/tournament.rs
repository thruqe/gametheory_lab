#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Cooperate = 0,
    Defect = 1,
}

pub trait Strategy {
    fn name(&self) -> &'static str;
    fn select_action(&mut self, history: &[(Action, Action)]) -> Action;
    fn reset(&mut self) {}
}

pub struct TitForTat;
impl Strategy for TitForTat {
    fn name(&self) -> &'static str {
        "TitForTat"
    }
    fn select_action(&mut self, history: &[(Action, Action)]) -> Action {
        match history.last() {
            None => Action::Cooperate,
            Some((_, opp_action)) => *opp_action,
        }
    }
}

pub struct AlwaysDefect;
impl Strategy for AlwaysDefect {
    fn name(&self) -> &'static str {
        "AlwaysDefect"
    }
    fn select_action(&mut self, _history: &[(Action, Action)]) -> Action {
        Action::Defect
    }
}

pub struct Grudger {
    betrayed: bool,
}
impl Grudger {
    pub fn new() -> Self {
        Self { betrayed: false }
    }
}
impl Strategy for Grudger {
    fn name(&self) -> &'static str {
        "Grudger"
    }
    fn select_action(&mut self, history: &[(Action, Action)]) -> Action {
        if let Some((_, opp_action)) = history.last() {
            if *opp_action == Action::Defect {
                self.betrayed = true;
            }
        }
        if self.betrayed {
            Action::Defect
        } else {
            Action::Cooperate
        }
    }
    fn reset(&mut self) {
        self.betrayed = false;
    }
}

pub fn run_match(
    s1: &mut dyn Strategy,
    s2: &mut dyn Strategy,
    payoffs: [[(f64, f64); 2]; 2],
    rounds: usize,
) -> (f64, f64) {
    let mut h1 = Vec::with_capacity(rounds);
    let mut h2 = Vec::with_capacity(rounds);
    let mut score1 = 0.0;
    let mut score2 = 0.0;

    s1.reset();
    s2.reset();

    for _ in 0..rounds {
        let a1 = s1.select_action(&h1);
        let a2 = s2.select_action(&h2);

        let (p1, p2) = payoffs[a1 as usize][a2 as usize];
        score1 += p1;
        score2 += p2;

        h1.push((a1, a2));
        h2.push((a2, a1));
    }

    (score1, score2)
}
