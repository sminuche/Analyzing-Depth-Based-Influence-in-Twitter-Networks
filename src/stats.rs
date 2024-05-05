use std::fs;
use statistics::{mean, variance};

pub fn calculate_statistics_from_file(file_path: &str) -> Option<(f64, f64)> {
    if let Ok(content) = fs::read_to_string(file_path) {
        let data: Vec<f64> = content
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();

        if let Some((mean_value, std_dev_value)) = calculate_statistics(&data) {
            return Some((mean_value, std_dev_value));
        }
    }

    None
}

fn calculate_statistics(data: &[f64]) -> Option<(f64, f64)> {
    if data.is_empty() {
        return None;
    }

    let mean_value = mean(data);
    let std_dev_value = variance(data).sqrt();

    Some((mean_value, std_dev_value))
}

