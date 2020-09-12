use crate::GameState;

use std::cmp::Ord;

/// A function which performs a minimax for the given game state, to the given depth, and returns
/// the action which the current agent should take. This function uses alpha-beta pruning
///
/// eval_fn is the evaluation function used to evaluate different states. It should return a type
/// which has a total ordering, so that maximizing agents can pick the best and minimizing agents
/// can pick the worst.
///
/// is_maximizer should be a function which returns true if the given agent maximizes, and false if
/// it minimizes.
pub fn minimax<State, Action, Agent, Utility, EvaluationFunction, IsMaximizer>(
    state: &State,
    depth: usize,
    eval_fn: EvaluationFunction,
    is_maximizer: IsMaximizer,
) -> (Action, Utility)
where
    State: GameState<Action = Action, Agent = Agent>,
    EvaluationFunction: FnMut(&State) -> Utility,
    IsMaximizer: FnMut(&Agent) -> bool,
    Utility: Ord,
{
    minimax_helper(state, depth, eval_fn, is_maximizer, None, None)
}

/// Does minimax with alpha/beta pruning (extracted as a helper function so callers to `minimax`
/// don't need to worry about the values for alpha and beta
fn minimax_helper<State, Action, Agent, Utility, EvaluationFunction, IsMaximizer>(
    state: &State,
    depth: usize,
    eval_fn: EvaluationFunction,
    is_maximizer: IsMaximizer,
    alpha: Option<Utility>,
    beta: Option<Utility>,
) -> (Action, Utility)
where
    State: GameState<Action = Action, Agent = Agent>,
    EvaluationFunction: FnMut(&State) -> Utility,
    IsMaximizer: FnMut(&Agent) -> bool,
    Utility: Ord,
{
    unimplemented!()
}
