use ltrs::{
    ensemble::adarank::AdaRank,
    eval::map::MAP,
    learner::Learner,
    loader::{svmlight::SVMLight, LtrFormat},
    ranker::Ranker,
    utils::logging::init_logging,
    DataSet,
};

fn main() {
    init_logging();

    // Get ohsumed location from the root project path /examples/ohsumed
    let corpus = std::path::Path::new("benchmarks/OHSUMED").join("Data/All/OHSUMED.txt");

    if corpus.exists() {
        log::info!("Loading corpus from {}", corpus.display());
        let ohsumed_dataset: DataSet =
            SVMLight::load(corpus.to_str().unwrap()).unwrap_or_else(|_| {
                panic!(
                    "Could not load ohsumed dataset located at {}/Data/All/OHSUMED.txt",
                    corpus.display()
                )
            });

        // Cloning a sample to test later...
        let test_sample = ohsumed_dataset[0].clone();

        let mut adarank = AdaRank::new(ohsumed_dataset, Box::new(MAP), 50, 3, 0.003, None, None);

        adarank.fit().unwrap();

        log::debug!("Finished fitting");

        let dp = test_sample.get(0).unwrap();

        let doc_label = adarank.predict(&test_sample.get(0).unwrap());
        log::info!(
            "Document {} has the score {:.2} for query {}",
            dp.get_description().unwrap(),
            doc_label,
            dp.get_query_id()
        );
    } else {
        log::error!("Corpus not found at {}", corpus.display());
        std::process::exit(1);
    }
}
