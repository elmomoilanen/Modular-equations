//! Implements some utilities to be used elsewhere.
//!

/// Make all index combinations given usize index upper bounds `index_upper_bounds`.
/// E.g. upper bounds \[2, 2\] would result 2*2 index combinations as follows:
/// \[\[0, 0\], \[0, 1\], \[1, 0\], \[1, 1\]\].
pub fn make_index_combinations(index_upper_bounds: &[usize]) -> Option<Vec<Vec<usize>>> {
    if index_upper_bounds.is_empty() || index_upper_bounds.iter().any(|&val| val == 0) {
        // invalid case, cannot make index combinations
        return None;
    }

    let indices: Vec<Vec<usize>> = index_upper_bounds
        .iter()
        .map(|&count| (0..count).collect())
        .collect();

    let mut combs: Vec<Vec<usize>> = vec![];
    let mut stack: Vec<usize> = vec![];

    make_combs(&mut combs, &mut stack, &indices, 0);

    Some(combs)
}

fn make_combs(
    combs: &mut Vec<Vec<usize>>,
    stack: &mut Vec<usize>,
    indices: &Vec<Vec<usize>>,
    idx: usize,
) {
    for i in indices[idx].iter() {
        stack.push(*i);

        if idx == indices.len() - 1 {
            combs.push(stack.to_vec());
        } else {
            make_combs(combs, stack, indices, idx + 1);
        }

        stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::make_index_combinations;

    fn verify_combination(correct_comb: &Vec<Vec<usize>>, test_comb: &Vec<Vec<usize>>) {
        assert_eq!(
            correct_comb.len(),
            test_comb.len(),
            "corr len: {}, test len: {}",
            correct_comb.len(),
            test_comb.len()
        );

        let it = correct_comb.iter().zip(test_comb.iter());

        for (j, (corr, test)) in it.enumerate() {
            assert_eq!(
                corr.len(),
                test.len(),
                "comb: {}, corr: {:?}, test: {:?}",
                j,
                corr,
                test
            );
            assert_eq!(corr, test);
        }
    }

    #[test]
    fn index_combination_invalid_case() {
        let idx_upper_bounds: [usize; 4] = [1, 1, 2, 0];

        match make_index_combinations(&idx_upper_bounds) {
            Some(_) => panic!("got return value `Some(_)`"),
            None => assert!(true),
        }
    }

    #[test]
    fn index_combination_bound_one_two_times() {
        let idx_upper_bounds: [usize; 2] = [1; 2];

        let correct_index_combs: Vec<Vec<usize>> = vec![vec![0, 0]];

        match make_index_combinations(&idx_upper_bounds) {
            None => panic!("got return value `None`"),
            Some(combinations) => {
                verify_combination(&correct_index_combs, &combinations);
            }
        }
    }

    #[test]
    fn index_combination_bound_two_two_times() {
        let idx_upper_bounds: [usize; 2] = [2; 2];

        let correct_index_combs: Vec<Vec<usize>> =
            vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]];

        match make_index_combinations(&idx_upper_bounds) {
            None => panic!("got return value `None`"),
            Some(combinations) => {
                verify_combination(&correct_index_combs, &combinations);
            }
        }
    }

    #[test]
    fn index_combination_bound_two_three_times() {
        let idx_upper_bounds: [usize; 3] = [2; 3];

        let correct_index_combs: Vec<Vec<usize>> = vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![0, 1, 1],
            vec![1, 0, 0],
            vec![1, 0, 1],
            vec![1, 1, 0],
            vec![1, 1, 1],
        ];

        match make_index_combinations(&idx_upper_bounds) {
            None => panic!("got return value `None`"),
            Some(combinations) => {
                verify_combination(&correct_index_combs, &combinations);
            }
        }
    }

    #[test]
    fn index_combination_three_mix() {
        let idx_upper_bounds: [usize; 3] = [2, 2, 3];

        let correct_index_combs: Vec<Vec<usize>> = vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 0, 2],
            vec![0, 1, 0],
            vec![0, 1, 1],
            vec![0, 1, 2],
            vec![1, 0, 0],
            vec![1, 0, 1],
            vec![1, 0, 2],
            vec![1, 1, 0],
            vec![1, 1, 1],
            vec![1, 1, 2],
        ];

        match make_index_combinations(&idx_upper_bounds) {
            None => panic!("got return value `None`"),
            Some(combinations) => {
                verify_combination(&correct_index_combs, &combinations);
            }
        }
    }

    #[test]
    fn index_combination_two_mix() {
        let idx_upper_bounds: [usize; 2] = [4, 2];

        let correct_index_combs: Vec<Vec<usize>> = vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 1],
            vec![2, 0],
            vec![2, 1],
            vec![3, 0],
            vec![3, 1],
        ];

        match make_index_combinations(&idx_upper_bounds) {
            None => panic!("got return value `None`"),
            Some(combinations) => {
                verify_combination(&correct_index_combs, &combinations);
            }
        }
    }
}
