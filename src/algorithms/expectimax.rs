use crate::GameState;

use std::{cmp::Ord, ops::{Add, Mul}};

/// A function which performs expectimax for the given game state, to the given depth, and returns
/// the action which the current agent should take.
///
/// eval_fn is the evaluation function used to evaluate different states. It should return a type
/// which has a total ordering, so that maximizing agents can pick the best and minimizing agents
/// can pick the worst.
///
/// agent_type_fn should take an agent and return whether it is a Maximizer, Minimizer, or
/// Randomizer
pub fn expectimax<State, Action, Agent, Utility, EvaluationFunction, AgentKindFunction>(
    state: &State,
    depth: usize,
    eval_fn: EvaluationFunction,
    agent_type_fn: AgentKindFunction,
) -> (Option<Action>, Utility)
where
    State: GameState<Action = Action, Agent = Agent>,
    Action: enum_iterator::IntoEnumIterator,
    EvaluationFunction: Fn(&State) -> Utility + Copy,
    AgentKindFunction: Fn(&Agent) -> ExpectimaxAgentType + Copy,
    Utility: Ord + Copy + Add<Output=Utility> + Mul<f32, Output=Utility> + Default,
{
    if state.is_finished() {
        (None, eval_fn(state))
    } else {
        match agent_type_fn(&state.next_agent().unwrap()) {
            ExpectimaxAgentType::Maximizer => {
                let mut best_action = None;
                let mut best_utility = None;
                for action in Action::into_enum_iter().filter(|action| state.is_legal(action)) {
                    let successor_state = state.successor(&action).unwrap();
                    let (_, utility) = expectimax(
                        &successor_state,
                        depth - 1,
                        eval_fn,
                        agent_type_fn,
                    );
                    if best_utility.map_or(true, |best_utility| utility > best_utility) {
                        best_utility = Some(utility);
                        best_action = Some(action);
                    }
                }
                (best_action, best_utility.unwrap_or_else(|| eval_fn(state)))
            },
            ExpectimaxAgentType::Minimizer => {
                let mut best_action = None;
                let mut best_utility = None;
                for action in Action::into_enum_iter().filter(|action| state.is_legal(action)) {
                    let successor_state = state.successor(&action).unwrap();
                    let (_, utility) = expectimax(
                        &successor_state,
                        depth - 1,
                        eval_fn,
                        agent_type_fn,
                    );
                    if best_utility.map_or(true, |best_utility| utility < best_utility) {
                        best_utility = Some(utility);
                        best_action = Some(action);
                    }
                }
                (best_action, best_utility.unwrap_or_else(|| eval_fn(state)))
            },
            ExpectimaxAgentType::Random => {
                let mut utility = Utility::default();
                let actions: Vec<_> = Action::into_enum_iter().filter(|action| state.is_legal(action)).collect();
                let fractional_weight = 1.0 / actions.len() as f32;
                for action in actions {
                    let successor_state = state.successor(&action).unwrap();
                    let (_, action_utility) = expectimax(
                        &successor_state,
                        depth - 1,
                        eval_fn,
                        agent_type_fn,
                    );
                    utility = utility + action_utility * fractional_weight;
                }
                (None, utility)
            },
        }
    }
}

pub enum ExpectimaxAgentType {
    Maximizer,
    Minimizer,
    Random,
}
