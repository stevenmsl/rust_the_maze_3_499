use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct Solution {}

/* key takeaways
  - check question 490 for comments explaining
    how BFS search works
  - the main difference between this one and
    question 505 is that we can't roll past
    the destination as it's a hole
    - you will need to stop while looking for
      the farthest neighbor in one direction
      if you encountered the destination
    - you need to record the paths how to get
      to the destination
  - we need to enhance the queue again to store
    the path for a given node
*/

impl Solution {
  pub fn shortest_distance(
    start: &(usize, usize),
    dest: &(usize, usize),
    maze: &Vec<Vec<usize>>,
  ) -> String {
    let rows = maze.len();
    let cols = maze[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    /*
      - we are visiting node's neighbors and will
        mark them as visited in the bfs method
      - so we need to mark the start as visited
        ourselves as bfs only visit its neighbors
        not the start node itself
    */
    visited[start.0][start.1] = true;
    Self::bfs(start, dest, maze, &mut visited)
  }

  fn bfs(
    node: &(usize, usize),
    dest: &(usize, usize),
    maze: &Vec<Vec<usize>>,
    visited: &mut Vec<Vec<bool>>,
  ) -> String {
    /*
      - the direction tuple now also includes a char
        to indicate the direction
    */
    let directions = vec![(1, 0, 'd'), (-1, 0, 'u'), (0, 1, 'r'), (0, -1, 'l')];

    /* queue needs to remember
       - the node's coordinates
       - the steps required to get to this node
         from the start
       - the path from the start to this node
    */
    let mut queue: VecDeque<((usize, usize), usize, Vec<char>)> =
      VecDeque::from([(*node, 0, vec![])]);
    let rows = maze.len();
    let cols = maze[0].len();

    /* don't forget this is a hole; you can't roll past it */
    let (dest_x, dest_y) = dest;

    let mut min_distance = usize::MAX;
    /* paths
       - a collection of paths that can lead
         to the destination from the start
       - it also has the steps required to
         get to the destination
    */
    let mut paths: Vec<(Vec<char>, usize)> = vec![];

    while queue.len() > 0 {
      /*
        - use size to make sure we have
          visited all nodes at one level
          before moving on to the next
      */
      let size = queue.len();

      for _ in 0..size {
        if let Some(node_steps) = queue.pop_front() {
          let (node_x, node_y) = node_steps.0;
          /* steps
            - total steps to reach this node from the start
          */
          let steps = node_steps.1;
          let path = node_steps.2;
          for i in 0..directions.len() {
            let (x_move, y_move, dir) = &directions[i];
            let mut test_x = node_x as isize + x_move;
            let mut test_y = node_y as isize + y_move;

            let mut additional_steps: usize = 1;
            let mut drop_to_the_hole = false;

            /* make sure the next move is within bounds */
            while test_x >= 0
              && test_x < rows as isize
              && test_y >= 0
              && test_y < cols as isize
              && maze[test_x as usize][test_y as usize] == 0
            {
              /* you can't roll pass the hole (dest)  */
              if test_x as usize == *dest_x && test_y as usize == *dest_y {
                if steps + additional_steps <= min_distance {
                  min_distance = steps + additional_steps;
                  let mut new_path = path.clone();
                  new_path.push(*dir);
                  paths.push((new_path, min_distance));
                }
                /*
                  - we use this to indicate that we
                    no longer needs to search for
                    the farthest neighbor as we
                    are already in a hole now!
                  - we can only break from the
                    while loop; we need this to
                    inform the outer for loop to
                    continue to the next direction
                */
                drop_to_the_hole = true;
                break;
              }

              /*
                - keep moving along the given direction
                - either x_move or y_move one of them will
                  be zero; so we will be just moving along
                  either x or y direction whichever the move
                  is not zero
              */
              test_x += x_move;
              test_y += y_move;
              additional_steps += 1;
            }

            if drop_to_the_hole {
              /*
                - you are in a hole now; no more neighbors
                  to visit along this direction
              */
              continue;
            }

            /*
              - to see if this node is our rightful candidate
                for the next round
              - remember we overdid by one move, so we need
                to deduct it back
            */
            let next_x = (test_x - x_move) as usize;
            let next_y = (test_y - y_move) as usize;
            additional_steps -= 1;

            if !visited[next_x][next_y] {
              visited[next_x][next_y] = true;
              /*
                - we also need to record how many steps it takes from
                  the start to reach this node
              */
              let mut new_path = path.clone();
              new_path.push(*dir);
              queue.push_back(((next_x, next_y), steps + additional_steps, new_path));
            }
          }
        }
      }
    }

    println!("paths: {:?}", paths);

    /* find the lexicographically smallest path  */
    if paths.len() > 0 {
      let mut s_path = vec!['z'];
      for (path, steps) in paths {
        /* not qualified */
        if steps > min_distance {
          continue;
        }
        let mut index = 0;
        while index < path.len() && index < s_path.len() {
          if path[index] == s_path[index] {
            index += 1;
            continue;
          }
          if path[index] < s_path[index] {
            s_path = path;
          }
          break;
        }
      }

      return String::from_iter(s_path.into_iter());
    }

    String::from("impossible")
  }

  pub fn test_fixture_1() -> Vec<Vec<usize>> {
    vec![
      vec![0, 0, 0, 0, 0],
      vec![1, 1, 0, 0, 1],
      vec![0, 0, 0, 0, 0],
      vec![0, 1, 0, 0, 1],
      vec![0, 1, 0, 0, 0],
    ]
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_1() {
    let result = Solution::shortest_distance(&(4, 3), &(0, 1), &Solution::test_fixture_1());
    assert_eq!(result, "lul");
  }

  #[test]
  fn sample_2() {
    let result = Solution::shortest_distance(&(4, 3), &(3, 0), &Solution::test_fixture_1());
    assert_eq!(result, "impossible");
  }
}
