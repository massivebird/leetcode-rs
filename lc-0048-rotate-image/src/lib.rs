struct Solution;

#[allow(unused, clippy::needless_pass_by_ref_mut, clippy::ptr_arg)]
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let size = matrix[0].len();

        macro_rules! from_flat_idx_mut {
            ($idx: expr) => {
                matrix
                    .get_mut($idx / size)
                    .unwrap()
                    .get_mut($idx - size * ($idx / size))
                    .unwrap()
            };
        }

        for layer in 0usize.. {
            // This layer's values are moved this many indices.
            let rotations = size.saturating_sub(1).saturating_sub(2 * layer);

            if rotations == 0 {
                break;
            }

            // Top and bottom edge length
            let edge_length = size.saturating_sub(2 * layer);

            // Contains indices corresponding to this layer, in rotation order.
            let layer_indices: Vec<usize> = {
                // Top edge of the box
                let mut tmp: Vec<usize> =
                    (0..).skip((size + 1) * layer).take(edge_length).collect();

                // Right-middle
                for i in 1..=edge_length.saturating_sub(2) {
                    tmp.push(tmp[edge_length - 1] + (size * i));
                }

                // Append bottom edge of the box
                for i in (0..)
                    .take(edge_length)
                    .map(|v| v + (size * (size - 1)) - ((size - 1) * layer))
                    .collect::<Vec<usize>>()
                    .into_iter()
                    .rev()
                {
                    tmp.push(i);
                }

                // Left-middle
                for i in (1..=edge_length.saturating_sub(2)).rev() {
                    tmp.push(tmp[0] + (size * i));
                }

                tmp
            };

            dbg!(&layer_indices);

            for _ in 0..rotations {
                let mut hand = *from_flat_idx_mut!(layer_indices[0]);

                for idx in layer_indices.iter().skip(1) {
                    let tmp = *from_flat_idx_mut!(*idx);
                    *from_flat_idx_mut!(*idx) = hand;
                    hand = tmp;
                }

                // Final element is in the hand. Place it in the first index.
                *from_flat_idx_mut!(layer_indices[0]) = hand;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_0() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        Solution::rotate(&mut matrix);

        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn case_1() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];

        Solution::rotate(&mut matrix);

        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }

    #[test]
    fn case_2() {
        let mut matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];

        Solution::rotate(&mut matrix);

        assert_eq!(
            matrix,
            vec![
                vec![21, 16, 11, 6, 1],
                vec![22, 17, 12, 7, 2],
                vec![23, 18, 13, 8, 3],
                vec![24, 19, 14, 9, 4],
                vec![25, 20, 15, 10, 5]
            ]
        );
    }
}
