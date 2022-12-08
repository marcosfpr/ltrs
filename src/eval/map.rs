/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)
use crate::eval::Evaluator;
use crate::memory_system::elements::ranklist::RankListPermutation;

///
/// MAP (Mean Average Precision) for a set of queries is the mean of the average precision
/// scores for each query.
/// The average precision score is the sum of the precision scores for each k, divided by
/// the number of positive labels.
/// See [Medium](https://towardsdatascience.com/breaking-down-mean-average-precision-map-ae462f623a52) for more information.
///
#[derive(Debug, Clone)]
pub struct MAP;

impl Evaluator for MAP {
    ///
    /// Evaluates the MAP for a set of queries.
    ///
    fn evaluate_ranklist(&self, ranklist_permutation: &RankListPermutation) -> f32 {
        let mut average_precision = 0.0f32;
        let mut num_relevant_docs = 0;
        for idx in &ranklist_permutation.permutation {
            match ranklist_permutation.ranklist.get(*idx) {
                // TODO: permutations needs to be aware of the label?
                Ok(dp) => {
                    if dp.get_label() > 0 {
                        num_relevant_docs += 1;
                        average_precision += num_relevant_docs as f32 / (*idx as f32 + 1.0);
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
        match num_relevant_docs {
            0 => 0.0,
            _ => average_precision / num_relevant_docs as f32,
        }
    }
}

impl ToString for MAP {
    fn to_string(&self) -> String {
        "MAP".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_system::elements::datapoint::DataPoint;
    use crate::memory_system::elements::feature::Feature;
    use crate::memory_system::elements::ranklist::RankList;
    use crate::rl;
    use crate::utils::randomize;

    use approx::relative_eq;

    #[test]
    fn test_map() {
        let ranklist = rl!(
            (
                0,
                9,
                randomize::randomize_uniform::<Feature>(
                    Feature::from(0f32),
                    Feature::from(100f32),
                    20
                ),
                "doc1"
            ),
            (
                1,
                9,
                randomize::randomize_uniform(Feature::from(0f32), Feature::from(100f32), 20),
                "doc2"
            ),
            (
                1,
                9,
                randomize::randomize_uniform(Feature::from(0f32), Feature::from(100f32), 20),
                "doc3"
            ),
            (
                0,
                9,
                randomize::randomize_uniform(Feature::from(0f32), Feature::from(100f32), 20),
                "doc4"
            ),
            (
                1,
                9,
                randomize::randomize_uniform(Feature::from(0f32), Feature::from(100f32), 20),
                "doc5"
            ),
            (
                0,
                9,
                randomize::randomize_uniform(Feature::from(0f32), Feature::from(100f32), 20),
                "doc6"
            )
        );

        let map = MAP;

        let unity_permutation = RankListPermutation {
            permutation: (0..ranklist.len()).collect(),
            ranklist: &ranklist,
        };

        let map_score = map.evaluate_ranklist(&unity_permutation);

        assert!(relative_eq!(map_score, 0.588, max_relative = 0.01f32));
    }
}
