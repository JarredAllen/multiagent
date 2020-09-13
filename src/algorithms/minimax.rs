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
) -> (Option<Action>, Utility)
where
    State: GameState<Action = Action, Agent = Agent>,
    Action: enum_iterator::IntoEnumIterator,
    EvaluationFunction: Fn(&State) -> Utility + Copy,
    IsMaximizer: Fn(&Agent) -> bool + Copy,
    Utility: Ord + Copy,
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
    mut alpha: Option<Utility>,
    mut beta: Option<Utility>,
) -> (Option<Action>, Utility)
where
    State: GameState<Action = Action, Agent = Agent>,
    Action: enum_iterator::IntoEnumIterator,
    EvaluationFunction: Fn(&State) -> Utility + Copy,
    IsMaximizer: Fn(&Agent) -> bool + Copy,
    Utility: Ord + Copy,
{
    if state.is_finished() {
        (None, eval_fn(state))
    } else {
        let mut best_action = None;
        let mut best_utility = None;
        let maximizing = is_maximizer(&state.next_agent().unwrap());
        for action in Action::into_enum_iter().filter(|action| state.is_legal(action)) {
            let successor_state = state.successor(&action).unwrap();
            let (_, utility) = minimax_helper(
                &successor_state,
                depth - 1,
                eval_fn,
                is_maximizer,
                alpha,
                beta,
            );
            if maximizing {
                if best_utility.map_or(true, |best_utility| utility > best_utility) {
                    best_utility = Some(utility);
                    best_action = Some(action);
                    if beta.map_or(false, |beta| utility > beta) {
                        return (best_action, best_utility.unwrap());
                    }
                    if alpha.map_or(false, |alpha| alpha < utility) {
                        alpha = Some(utility);
                    }
                }
            } else {
                if best_utility.map_or(true, |best_utility| utility < best_utility) {
                    best_utility = Some(utility);
                    best_action = Some(action);
                    if alpha.map_or(false, |alpha| utility < alpha) {
                        return (best_action, best_utility.unwrap());
                    }
                    if beta.map_or(false, |beta| beta > utility) {
                        beta = Some(utility);
                    }
                }
            }
        }
        (best_action, best_utility.unwrap_or_else(|| eval_fn(state)))
    }
}
