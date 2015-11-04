//! Routines for powering the search

use game::{Board, Move};

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// Runs A* search to find a path from the `start` to the `goal`, if it exists.
///
/// This implementation assumes a small, consistent set of possible plays from
/// every node, as might be found in a slide puzzle game.
///
/// It also makes a particular assumption that the heuristic function is such
/// that the search will evolve in a way that no board configuration will ever
/// need to be visited more than once.
pub fn a_star(start: Board, goal: &Board, moves: &[Move]) -> Option<Vec<Move>> {
    // As a special case, let's immediately check for start == goal
    if start == *goal {
        return Some(Vec::new());
    }

    let mut fringe = BinaryHeap::new();
    let mut movements = HashMap::new();

    fringe.push(AstarNode { goal: goal, node: start.clone(), path_len: 0 });
    movements.insert(start, None);

    while let Some(AstarNode { node: current, path_len, .. }) = fringe.pop() {
        let current_depth = path_len + 1;

        for &play in moves {
            if let Some(next) = current.update(play) {
                if next == *goal {
                    movements.insert(next, Some(play));
                    return Some(build_path(&movements, goal, current_depth))
                }
                else {
                    movements.entry(next.clone()).or_insert_with(|| {
                        fringe.push(AstarNode {
                            node: next,
                            goal: goal,
                            path_len: current_depth
                        });
                        Some(play)
                    });
                }
            }
        }        
    }

    None
}

fn build_path(movements: &HashMap<Board, Option<Move>>, ending: &Board,
              length: usize)
              -> Vec<Move>
{
    let mut path = Vec::with_capacity(length);
    let movement = match movements.get(ending)
        .expect("Surely the ending configuration has a path to it.") {
            &None => /* special case: the goal is the start, return the trivial
                path */ return Vec::new(),
            &Some(m) => m
        };


    path.push(movement);

    let mut cursor = ending.update(movement.reverse())
        .expect("We already found this path");

    while let Some(&Some(movement)) = movements.get(&cursor) {
        path.push(movement);
        cursor = cursor.update(movement.reverse())
            .expect("We already found this path");
    }

    path.reverse();
    path
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct AstarNode<'a, T: 'a> {
    goal: &'a T,
    node: T,
    path_len: usize
}

impl<'a> Ord for AstarNode<'a, Board> {
    fn cmp(&self, other: &Self) -> Ordering {
        let goal = self.goal;
        let other_cost = other.node.estimate_cost(goal) + other.path_len;
        let my_cost = self.node.estimate_cost(goal) + self.path_len;

        other_cost.cmp(&my_cost)
    }
}

impl<'a, T> PartialOrd for AstarNode<'a, T>
    where AstarNode<'a, T>: Ord
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    // These tests come from
    // <http://cs.cmu.edu/~adamchik/15-121/labs/HW-7%20Slide%20Puzzle/lab.html>
    use super::*;
    use game::{Board, Move};
    use game::Move::*;

    fn goal() -> Board {
        Board::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 0]).unwrap()
    }

    fn moves() -> [Move; 4] {
        [Up, Down, Left, Right]
    }

    #[test]
    fn board_1() {
        let start = Board::from_vec(vec![1, 2, 3, 4, 0, 5, 7, 8, 6]).unwrap();
        let solution = vec![Right, Down];

        assert_eq!(Some(solution), a_star(start, &goal(), &moves()));
    }

    #[test]
    fn board_2() {
        let start = Board::from_vec(vec![1, 2, 3, 7, 4, 5, 0, 8, 6]).unwrap();
        let solution = vec![Up, Right, Right, Down];

        assert_eq!(Some(solution), a_star(start, &goal(), &moves()));
    }

    #[test]
    fn board_3() {
        let start = Board::from_vec(vec![1, 2, 3, 4, 8, 0, 7, 6, 5]).unwrap();
        let solution = vec![Down, Left, Up, Right, Down];

        assert_eq!(Some(solution), a_star(start, &goal(), &moves()));
    }

    #[test]
    fn board_4() {
        let start = Board::from_vec(vec![4, 1, 3, 7, 2, 6, 5, 8, 0]).unwrap();
        let solution = vec![Left, Left, Up, Up, Right, Down, Down, Right];

        assert_eq!(Some(solution), a_star(start, &goal(), &moves()));
    }

    #[test]
    fn board_5() {
        let start = Board::from_vec(vec![1, 6, 2, 5, 3, 0, 4, 7, 8]).unwrap();
        let solution = vec![Left, Up, Right, Down, Left, Left, Down, Right,
                            Right];

        assert_eq!(Some(solution), a_star(start, &goal(), &moves()));
    }

    #[test]
    fn board_6() {
        let start = Board::from_vec(vec![5, 1, 2, 6, 3, 0, 4, 7, 8]).unwrap();
        let solution = vec![Left, Left, Up, Right, Right, Down, Left, Left,
                            Down, Right, Right];

        assert_eq!(Some(solution), a_star(start, &goal(), &moves()));
    }

    #[test]
    #[ignore]
    // This doesn't appear to be the only solution!
    fn board_7() {
        let start = Board::from_vec(vec![1, 2, 6, 3, 5, 0, 4, 7, 8]).unwrap();
        let solution = vec![Up, Left, Down, Left, Down, Right, Right, Up, Left,
                            Up, Right, Down, Down];
        let goal_board = goal();
        let mine = a_star(start.clone(), &goal_board, &moves());

        assert_eq!(Some(solution), mine);
    }

    #[test]
    fn big_board() {
        let start = Board::from_vec(vec![1, 2, 0, 3, 4, 9, 6, 7, 8, 10, 5, 11,
                                         12, 13, 14, 15]).unwrap();
        let goal = Board::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
                                        12, 13, 14, 15]).unwrap();
        let solution = vec![Down, Down, Left, Up, Right, Up, Left, Left];

        assert_eq!(Some(solution), a_star(start, &goal, &moves()));
    }
}
