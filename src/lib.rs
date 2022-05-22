pub struct SampleStatistics {
    pub sample_mean: f64,
    pub standard_error: f64,
    pub n: usize,
}

pub struct PopulationStatistics {
    pub population_mean: f64,
    pub standard_error: f64,
    pub n: usize,
}

trait GetStatistics {
    fn from_array(array: &[f64]) -> Self;
}

impl GetStatistics for SampleStatistics {
    fn from_array(array: &[f64]) -> Self {
        let n = array.len();
        let sample_mean = mean(&array);
        let standard_error = sample_standard_deviation(&array);

        SampleStatistics {
            sample_mean,
            standard_error,
            n,
        }
    }
}

impl GetStatistics for PopulationStatistics {
    fn from_array(array: &[f64]) -> Self {
        let n = array.len();
        let population_mean = mean(&array);
        let standard_error = sample_standard_deviation(&array);

        PopulationStatistics {
            population_mean,
            standard_error,
            n,
        }
    }
}

pub fn mean(list: &[f64]) -> f64 {
    let sum: f64 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}

pub fn sample_standard_deviation(array: &[f64]) -> f64 {
    let n = array.len();
    let s_mean = mean(&array);

    let mut sum = 0.0;
    for xi in array.into_iter() {
        sum += f64::powf(xi - s_mean, 2.0)
    }

    sum = sum / (n as f64 - 1.0);
    sum.sqrt()
}

pub fn population_standard_deviation(array: &[f64]) -> f64 {
    let n = array.len();
    let p_mean = mean(&array);

    let mut sum = 0.0;
    for xi in array.into_iter() {
        sum += f64::powf(xi - p_mean, 2.0)
    }

    sum = sum / (n as f64);
    sum.sqrt()
}

pub struct TTestResult {
    pub t: f64,
    pub p_value: f64,
}

pub fn two_samp_t_test(samp_1: SampleStatistics, samp_2: SampleStatistics) -> TTestResult {
    let mean_delta = samp_1.sample_mean - samp_2.sample_mean;
    let stand =
        (samp_1.standard_error / samp_1.n as f64) + (samp_2.standard_error / samp_2.n as f64);
    let t = mean_delta / stand.sqrt();

    // TODO: use t cdf for p_value
    let p_value: f64 = 0.05;

    return TTestResult { t, p_value };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_test() {
        assert!(mean(&[1.0]) == 1.0);
        assert!(mean(&[1.0, 3.0]) == 2.0);
    }

    #[test]
    fn sample_standard_deviation_test() {
        assert!(sample_standard_deviation(&[1.0]) == -1.0);
    }

    #[test]
    fn population_standard_deviation_test() {
        assert!(sample_standard_deviation(&[1.0]) == -1.0);
    }
}
