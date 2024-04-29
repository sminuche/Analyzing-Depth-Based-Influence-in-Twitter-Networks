use statrs::statistics::{Mean, Variance}};

pub fn calculate_mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

pub fn calculate_std_dev(data: &[f64]) -> f64 {
    let variance = statrs::statistics::OnlineVariance::evaluate(data);
    variance.sqrt()
}
