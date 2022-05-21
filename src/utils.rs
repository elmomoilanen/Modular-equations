//! Implements utilities to be used elsewhere.
//!
//! Public functions:
//! - make_index_combinations: Make all possible zero-based index combinations
//! based on an array of index upper bounds. For example, upper bounds \[2, 3]\
//! would result 2*3 index combinations from \[0, 0\] following \[0, 1\] all the
//! way to the last one \[1, 2\].
//!
//! - largest_common_dividing_power_of_two: Compute the largest dividing 2's
//! power among the arguments. Notice that if the last arg equals zero, the return
//! value will actually be the largest power between the first two args.
//!
use std::cmp;

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

/// Largest common dividing power of two for args `x`, `y` and `z`. If the
/// last arg `z` is zero, the return value will be the largest power between
/// `x` and `y`.
pub fn largest_common_dividing_power_of_two(x: u128, y: u128, z: u128) -> u8 {
    if x & 1 != 0 || y & 1 != 0 || z & 1 != 0 {
        return 0;
    }
    if x == 0 || y == 0 {
        return 0;
    }

    let x_trail_zeros = x.trailing_zeros() as u8;
    let y_trail_zeros = y.trailing_zeros() as u8;

    let min_x_t_y_t = cmp::min(x_trail_zeros, y_trail_zeros);

    if z == 0 {
        min_x_t_y_t
    } else {
        let z_trail_zeros = z.trailing_zeros() as u8;

        cmp::min(z_trail_zeros, min_x_t_y_t)
    }
}

#[cfg(test)]
mod tests {
    use super::{largest_common_dividing_power_of_two, make_index_combinations};

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

    #[test]
    fn largest_common_dividing_power_of_two_test() {
        let large_num = i128::MAX as u128 + 1;

        // [x, y, z, correct_power]
        let test_cases: [[u128; 4]; 10] = [
            [3, 8, 4, 0],
            [12, 16, 1, 0],
            [4, 0, 4, 0],
            [4, 4, 0, 2],
            [12, 16, 16, 2],
            [2, 16, 2, 1],
            [2, 32, 32, 1],
            [64, 64, 32, 5],
            [large_num, 2, large_num, 1],
            [large_num, large_num, large_num, 127],
        ];

        for test in test_cases.iter() {
            let res = largest_common_dividing_power_of_two(test[0], test[1], test[2]);

            assert_eq!(res as u128, test[3]);
        }
    }
}
