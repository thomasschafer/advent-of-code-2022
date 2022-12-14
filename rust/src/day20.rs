use std::fs;

fn modulus<T>(num: T, mod_by: T) -> T
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Rem<Output = T>,
{
    (num % mod_by + mod_by) % mod_by
}

fn parse_numbers(file_path: &str) -> Vec<(usize, i64)> {
    let file_contents = fs::read_to_string(file_path).unwrap();

    file_contents
        .split('\n')
        .map(|s| s.parse::<i64>().unwrap())
        .enumerate()
        .collect::<Vec<_>>()
}

fn mix(numbers_with_indices: &mut Vec<(usize, i64)>) {
    let length = numbers_with_indices.len();

    for start_idx in 0..length {
        let cur_idx = numbers_with_indices
            .iter()
            .position(|&(idx, _)| idx == start_idx)
            .unwrap();
        let (_, num) = numbers_with_indices[cur_idx];
        let mut new_idx = modulus(cur_idx as i64 + num, length as i64 - 1) as usize;
        if new_idx == 0 {
            new_idx = length - 1;
        }
        if new_idx != cur_idx {
            if new_idx < cur_idx {
                for idx in (new_idx..cur_idx).rev() {
                    numbers_with_indices[idx + 1] = numbers_with_indices[idx]
                }
            }
            if new_idx > cur_idx {
                for idx in cur_idx..new_idx {
                    numbers_with_indices[idx] = numbers_with_indices[idx + 1]
                }
            }

            numbers_with_indices[new_idx] = (start_idx, num);
        }
    }
}

fn sum_of_grove_coordinates(numbers_with_indices: Vec<(usize, i64)>) -> i64 {
    let numbers = numbers_with_indices
        .iter()
        .map(|&(_, num)| num)
        .collect::<Vec<i64>>();
    let length = numbers.len();
    let idx_of_zero = numbers.iter().position(|&a| a == 0).unwrap();

    numbers[(idx_of_zero + 1000) % length]
        + numbers[(idx_of_zero + 2000) % length]
        + numbers[(idx_of_zero + 3000) % length]
}

pub fn part1(file_path: &str) -> i64 {
    let mut numbers_with_indices = parse_numbers(file_path);

    mix(&mut numbers_with_indices);
    sum_of_grove_coordinates(numbers_with_indices)
}

pub fn part2(file_path: &str) -> i64 {
    let mut numbers_with_indices = parse_numbers(file_path)
        .iter()
        .map(|&(idx, num)| (idx, num * 811589153))
        .collect::<Vec<_>>();

    for _ in 0..10 {
        mix(&mut numbers_with_indices);
    }
    sum_of_grove_coordinates(numbers_with_indices)
}
