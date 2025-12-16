use std::{
    fs,
    io::{self},
    iter::Peekable,
    path::Path,
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

    pub fn can_accommodate(
        &self,
        line_weight: usize,
        max_line_length: usize,
        allowable_weight: usize,
    ) -> bool {
        self.weight(max_line_length) + line_weight <= allowable_weight
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

pub struct BatchesIterator<Lines, Weights>
where
    Lines: Iterator<Item = String>,
    Weights: Iterator<Item = usize>,
{
    lines: Peekable<Lines>,
    batch_weights: Weights,
    max_line_length: usize,
    allowable_weight: usize,
}

impl<Lines, Weights> BatchesIterator<Lines, Weights>
where
    Lines: Iterator<Item = String>,
    Weights: Iterator<Item = usize>,
{
    pub fn new(
        lines: Lines,
        max_line_length: usize,
        mut batch_weights: Weights,
    ) -> Result<Self, NotEnoughWeightsError> {
        let allowable_weight = batch_weights.next().ok_or(NotEnoughWeightsError)?;
        Ok(Self {
            lines: lines.peekable(),
            batch_weights,
            max_line_length,
            allowable_weight,
        })
    }
}

impl<Lines, Weights> Iterator for BatchesIterator<Lines, Weights>
where
    Lines: Iterator<Item = String>,
    Weights: Iterator<Item = usize>,
{
    type Item = Batch;

    fn next(&mut self) -> Option<Self::Item> {
        let mut batch = Batch::new();

        while let Some(line) = self.lines.peek() {
            let line_weight = line_weight(line.chars().count(), self.max_line_length);

            if !batch.can_accommodate(line_weight, self.max_line_length, self.allowable_weight) {
                if let Some(weight) = self.batch_weights.next() {
                    self.allowable_weight = weight;
                }

                return Some(batch);
            }

            batch.push(self.lines.next().unwrap());
        }

        if !batch.is_empty() {
            return Some(batch);
        }

        None
    }
}

pub fn write_batches(batches: impl Iterator<Item = Batch>, output_dir: &Path) -> io::Result<()> {
    fs::create_dir_all(output_dir)?;

    for (batch_id, batch) in batches.enumerate() {
        let batch_path = output_dir.join(format!("batch_{}.txt", batch_id));
        let content = batch.lines().join("\n");
        fs::write(batch_path, content)?;
    }

    Ok(())
}
