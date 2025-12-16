use std::{
    fs, io::{self}, path::Path
};

use thiserror::Error;

#[derive(Debug)]
pub struct Batch {
    inner: Vec<String>,
}

impl Batch {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, value: String) {
        self.inner.push(value);
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn weight(&self, max_line_length: usize) -> usize {
        self.inner
            .iter()
            .map(|l| line_weight(l.chars().count(), max_line_length))
            .sum()
    }

    pub fn lines(&self) -> &[String] {
        &self.inner
    }
}

pub fn line_weight(length: usize, max_length: usize) -> usize {
    if length == 0 {
        return 1;
    }

    (length - 1) / max_length + 1
}

#[derive(Debug, PartialEq, Eq, Clone, Error)]
#[error("Number of weights must be at least 1")]
pub struct NotEnoughWeightsError;

pub fn collect_lines_to_batches(
    lines: &[String],
    max_line_length: usize,
    batch_weights: &mut impl Iterator<Item = usize>,
) -> Result<Vec<Batch>, NotEnoughWeightsError> {
    let mut batches = Vec::new();
    let mut current_batch = Batch::new();
    let mut allowable_weight = batch_weights.next().ok_or(NotEnoughWeightsError)?;

    for line in lines {
        let line_weight = line_weight(line.chars().count(), max_line_length);
        if current_batch.weight(max_line_length) + line_weight > allowable_weight {
            batches.push(current_batch);
            current_batch = Batch::new();

            if let Some(weigth) = batch_weights.next() {
                allowable_weight = weigth
            }
        }

        current_batch.push(line.clone());
    }

    if !current_batch.is_empty() {
        batches.push(current_batch);
    }

    Ok(batches)
}

pub fn write_batches(batches: &[Batch], output_dir: &Path) -> io::Result<()> {
    fs::create_dir_all(output_dir)?;

    for (batch_id, batch) in batches.iter().enumerate() {
        let batch_path = output_dir.join(format!("batch_{}.txt", batch_id));
        let content = batch.lines().join("\n");
        fs::write(batch_path, content)?;
    }

    Ok(())
}
