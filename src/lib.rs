use rand::Rng;

const INTERVALS: [(i32, i32); 6] = [
    (2267, 2733),
    (1079, 1421),
    (502, 748),
    (223, 402),
    (90, 223),
    (90, 223),
];

fn generate_random_sequence(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen_range(0..2)).collect()
}

fn mono_bit(sequence: &[u8]) -> bool {
    let ones_count = sequence.iter().filter(|&&bit| bit == 1).count();
    let zeros_count = sequence.len() - ones_count;
    ones_count > 9654 && ones_count < 10346 && zeros_count > 9654 && zeros_count < 10346
}

fn max_length_series(sequence: &[u8]) -> bool {
    let mut max_zeros = 0;
    let mut max_ones = 0;
    let mut zeros = 0;
    let mut ones = 0;

    for &bit in sequence {
        if bit == 1 {
            ones += 1;
            zeros = 0;
            max_ones = max_ones.max(ones);
        } else {
            zeros += 1;
            ones = 0;
            max_zeros = max_zeros.max(zeros);
        }
    }

    max_zeros <= 36 && max_ones <= 36
}

fn poker(sequence: &[u8]) -> bool {
    let m = 4;
    let k = sequence.len() / m;
    let mut frequencies = vec![0; 16];

    for i in 0..k {
        let mut value = 0;

        for j in 0..m {
            if sequence[i * m + j] == 1 {
                value |= 1 << (m - j - 1);
            }
        }

        frequencies[value] += 1;
    }

    let x3 = (16_f64 * frequencies.iter().map(|&count| count * count).sum::<usize>() as f64) / k as f64 - k as f64;
    x3 > 1.03 && x3 < 57.4
}

fn series_length(sequence: &[u8]) -> bool {
    let mut zeros_series = vec![0; 6];
    let mut ones_series = vec![0; 6];
    let mut zeros = 0;
    let mut ones = 0;

    for &bit in sequence {
        if bit == 1 {
            ones += 1;
            if zeros > 0 && zeros < 7 {
                zeros_series[zeros-1] += 1;
            }
            zeros = 0;
        } else {
            zeros += 1;
            if ones > 0 && ones < 7 {
                ones_series[ones - 1] += 1;
            }
            ones = 0;
        }
    }

    let mut zeros_passed = true;
    for (i, interval) in INTERVALS.iter().enumerate() {
        if !(zeros_series[i] > interval.0 && zeros_series[i] < interval.1) {
            zeros_passed = false;
        }
    }

    let mut ones_passed = true;
    for (i, interval) in INTERVALS.iter().enumerate() {
        if !(ones_series[i] > interval.0 && ones_series[i] < interval.1) {
            ones_passed = false;
        }
    }
    zeros_passed && ones_passed
}

#[cfg(test)]
mod tests {
    use crate::{generate_random_sequence, mono_bit, max_length_series, poker, series_length};

    #[test]
    fn mono_bit_test() {
        let sequence = generate_random_sequence(20000);
        assert!(mono_bit(&sequence));
    }

    #[test]
    fn max_length_series_test() {
        let sequence = generate_random_sequence(20000);
        assert!(max_length_series(&sequence));
    }

    #[test]
    fn poker_test() {
        let sequence = generate_random_sequence(20000);
        assert!(poker(&sequence));
    }

    #[test]
    fn series_length_test() {
        let sequence = generate_random_sequence(20000);
        assert!(series_length(&sequence));
    }
}