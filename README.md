# gametheory_lab

`gametheory_lab` is a computational game theory toolkit for modeling strategic multi-agent systems, computing exact equilibrium states, and evaluating repeated dynamic interactions.

## 1. Mathematical Formalism

A finite 2-player strategic (normal-form) game is defined as a tuple:

$$\Gamma = \left( N, \lbrace S_i \rbrace_{i \in \lbrace 1, 2 \rbrace}, \lbrace u_i \rbrace_{i \in \lbrace 1, 2 \rbrace} \right)$$

where:
* $N = \lbrace 1, 2 \rbrace$ is the set of rational decision-makers.
* $S_1 = \lbrace r_1, r_2, \dots, r_m \rbrace$ is the set of pure strategies for Player 1 (Row Player).
* $S_2 = \lbrace c_1, c_2, \dots, c_n \rbrace$ is the set of pure strategies for Player 2 (Column Player).
* The joint action space is the Cartesian product $S = S_1 \times S_2$.
* The utility functions are parameterized by payoff matrices $A, B \in \mathbb{R}^{m \times n}$:

  $$u_1(r_i, c_j) = A_{ij}, \quad u_2(r_i, c_j) = B_{ij}$$

### Mixed Strategy Spaces & Expected Payoffs
Let $\Delta(S_i)$ denote the standard simplex of probability distributions over $S_i$:

$$\Delta(S_1) = \left\lbrace p \in \mathbb{R}^m \;\middle|\; \sum_{i=1}^m p_i = 1, \quad p_i \geq 0 \; \forall i \right\rbrace$$

$$\Delta(S_2) = \left\lbrace q \in \mathbb{R}^n \;\middle|\; \sum_{j=1}^n q_j = 1, \quad q_j \geq 0 \; \forall j \right\rbrace$$

For a mixed strategy profile $(p, q) \in \Delta(S_1) \times \Delta(S_2)$, the expected utility for each player is given by the bilinear forms:

$$E[u_1(p, q)] = p^T A q = \sum_{i=1}^m \sum_{j=1}^n p_i A_{ij} q_j$$

$$E[u_2(p, q)] = p^T B q = \sum_{i=1}^m \sum_{j=1}^n p_i B_{ij} q_j$$

## 2. Theoretical Approach & Algorithms

### A. Iterated Elimination of Strictly Dominated Strategies (IESDS)
A pure strategy $r_i \in S_1$ is strictly dominated by another pure strategy $r_k \in S_1$ if:

$$A_{kj} \gt A_{ij}, \quad \forall j \in \lbrace 1, \dots, n \rbrace$$

Similarly, a column strategy $c_j \in S_2$ is strictly dominated by $c_l \in S_2$ if:

$$B_{il} \gt B_{ij}, \quad \forall i \in \lbrace 1, \dots, m \rbrace$$

The algorithm proceeds iteratively:
1. Identify all row indices $i$ strictly dominated by any row $k$. Remove them to yield submatrix $A^{(t)}, B^{(t)}$.
2. Identify all column indices $j$ strictly dominated by any column $l$ in $B^{(t)}$. Remove them.
3. Repeat until no strictly dominated actions remain.

The invariant guarantees that no strictly dominated strategy can have non-zero probability in any Nash equilibrium:

$$\text{supp}(p^{\ast}) \subseteq S_1^{\infty}, \quad \text{supp}(q^{\ast}) \subseteq S_2^{\infty}$$

### B. Nash Equilibrium via Support Enumeration
A strategy profile $(p^{\ast}, q^{\ast}) \in \Delta(S_1) \times \Delta(S_2)$ constitutes a Nash Equilibrium if and only if neither player has an incentive to unilaterally deviate:

$$\forall p \in \Delta(S_1), \quad (p^{\ast})^T A q^{\ast} \geq p^T A q^{\ast}$$

$$\forall q \in \Delta(S_2), \quad (p^{\ast})^T B q^{\ast} \geq (p^{\ast})^T B q$$

#### Fundamental Indifference Principle
Let $I = \text{supp}(p^{\ast}) = \lbrace i \mid p_i^{\ast} \gt 0 \rbrace$ and $J = \text{supp}(q^{\ast}) = \lbrace j \mid q_j^{\ast} \gt 0 \rbrace$. If $(p^{\ast}, q^{\ast})$ is a Nash equilibrium, every pure strategy in a player's support must yield the exact same expected payoff against the opponent's mixed strategy:

$$(A q^{\ast})_i = v_1, \quad \forall i \in I \quad \text{and} \quad (A q^{\ast})_k \leq v_1, \quad \forall k \notin I$$

$$((p^{\ast})^T B)_j = v_2, \quad \forall j \in J \quad \text{and} \quad ((p^{\ast})^T B)_l \leq v_2, \quad \forall l \notin J$$

#### Enumeration Procedure
For every support size $k \in \lbrace 1, \dots, \min(m, n) \rbrace$ and all subsets $I \subseteq S_1, J \subseteq S_2$ with $|I| = |J| = k$:
1. Solve the system of linear equations for $q$:

   $$\sum_{j \in J} (A_{ij} - A_{i+1, j}) q_j = 0 \quad (\forall i \in I \setminus \lbrace\max I\rbrace), \quad \sum_{j \in J} q_j = 1$$

2. Solve the transposed system for $p$:

   $$\sum_{i \in I} (B_{ij} - B_{i, j+1}) p_i = 0 \quad (\forall j \in J \setminus \lbrace\max J\rbrace), \quad \sum_{i \in I} p_i = 1$$

3. Check probability validity: $p_i \geq 0$ and $q_j \geq 0$.

4. Check best-response inequalities for unplayed actions:

   $$\max_{k \notin I} (A q)\_k \leq v_1 \quad \text{and} \quad \max_{l \notin J} (p^T B)\_l \leq v_2$$

### C. Repeated Game Dynamics & Memory Strategies
In an infinitely repeated or $T$-round game, players execute strategies conditioned on execution history $h^t = (a^0, a^1, \dots, a^{t-1}) \in S^t$. 

Payoffs accumulate additively:

$$U_i = \sum_{t=0}^{T-1} u_i(s_1^t, s_2^t)$$

Evaluated strategies:
* **Tit-for-Tat:** $a_i^0 = \text{Cooperate}$; $a_i^t = a_{-i}^{t-1}$ for $t \geq 1$.
* **Always Defect:** $a_i^t = \text{Defect}$ for all $t$.
* **Grim Trigger (Grudger):**

$$a_i^t = \begin{cases} \text{Cooperate}, & \text{if } a_{-i}^{\tau} = \text{Cooperate} \quad \forall \tau \lt t \\ \text{Defect}, & \text{otherwise} \end{cases}$$

## 3. Computational Results

Benchmark evaluations across canonical game structures:

```text
=== 1. Prisoner's Dilemma (IESDS & Nash) ===
Surviving rows after IESDS: ["D"]
Surviving cols after IESDS: ["D"]
NE #1: P1=[0.0, 1.0], P2=[0.0, 1.0]

=== 2. Matching Pennies (Mixed Strategy NE) ===
NE #1: P1=[0.5, 0.5], P2=[0.5, 0.5] -> Expected Utility: (0.00, 0.00)

=== 3. Repeated Game Simulation (200 Rounds) ===
TitForTat vs AlwaysDefect -> Scores: (199, 204)
TitForTat vs Grudger      -> Scores: (600, 600)

```

### Analysis of Results

* **Prisoner's Dilemma:** Payoff configuration $T \gt R \gt P \gt S$ ($5 \gt 3 \gt 1 \gt 0$). Because $u_1(D, c) \gt u_1(C, c)$ for all $c \in \lbrace C, D \rbrace$, strategy $C$ is strictly dominated. IESDS eliminates $C$ in step 1, leaving the unique equilibrium $(D, D)$ with payoffs $(1, 1)$.
* **Matching Pennies:** Payoff matrix is zero-sum: $A + B = 0$, where:

$$A = \begin{pmatrix} 1 & -1 \\ -1 & 1 \end{pmatrix}$$

Pure equilibria do not exist. Support enumeration identifies the unique fully-mixed equilibrium $(p^{\ast}, q^{\ast}) = ([0.5, 0.5], [0.5, 0.5])$ with game value $v = (p^{\ast})^T A q^{\ast} = 0.00$.
* **Repeated Interaction:**
  * Against `AlwaysDefect`, `TitForTat` experiences a single exploitation event at $t=0$ yielding $(0, 5)$, followed by mutual defection $(1, 1)$ for 199 rounds:

    $$U_{\text{TFT}} = 0 + 199 \times 1 = 199, \quad U_{\text{Defect}} = 5 + 199 \times 1 = 204$$

  * Against `Grudger`, mutual cooperation holds indefinitely, avoiding trigger activation:

    $$U_{\text{TFT}} = U_{\text{Grudger}} = 200 \times 3 = 600$$
