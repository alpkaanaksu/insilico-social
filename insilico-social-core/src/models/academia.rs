use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use gloo_timers::future::TimeoutFuture;

const SIGNIFICANCE_LEVEL: f64 = 0.05;

use crate::utils::{self, random_int_in_range, set_panic_hook};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Academia {
    pub time: u32,
    pub last_study_id: u32,
    pub last_researcher_id: u32,

    #[wasm_bindgen(skip)]
    pub researchers: Vec<Researcher>,
    #[wasm_bindgen(skip)]
    pub studies: Vec<Study>,
}

#[derive(Serialize, Deserialize)]
pub struct Researcher {
    pub id: u32,
    pub current_study_id: Option<u32>,
    pub prob_reproduce: f64,
    pub prob_replicate: f64,
    pub prob_open_original: f64,
    pub prob_open_repeating: f64,
    pub odds_repeat_open: f64,
    pub prob_closed_has_data: f64,
}

#[derive(Serialize, Deserialize)]
pub enum StudyType {
    Original,
    Reproduction,
    Replication,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum PublicationStatus {
    Pending,
    Published,
    Unpublished,
}

#[derive(Serialize, Deserialize)]
pub enum ConfirmationStatus {
    Confirmed,
    Disconfirmed,
}

#[derive(Serialize, Deserialize)]
pub struct Study {
    pub id: u32,
    pub start_time: u32,
    pub study_type: StudyType,
    pub researcher_id: u32,
    pub true_effect_size: f64,
    pub sampled_effect_size: f64,
    pub observed_effect_size: f64,
    pub reported_effect_size: f64,
    pub duration_days: u32,
    pub publication_status: PublicationStatus,

    // If replication / reproduction study
    pub original_study_id: Option<u32>,
    pub confirmation_status: Option<ConfirmationStatus>,
}

#[derive(Serialize, Deserialize)]
pub struct Summary {
    pub time: u32,
    pub number_of_studies: u32,
    pub number_of_researchers: u32,
    pub number_of_published_studies: u32,
    pub number_of_unpublished_studies: u32,
    pub number_of_reproductions: u32,
    pub number_of_replications: u32,
    pub number_of_originals: u32,
    pub number_of_confirmed: u32,
    pub number_of_disconfirmed: u32,
    pub number_of_pending: u32,
    pub avg_effect_size: f64,
}

#[wasm_bindgen]
impl Academia {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            time: 0,
            last_study_id: 0,
            last_researcher_id: 0,
            researchers: Vec::new(),
            studies: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn init(&mut self, num_of_researches: u32) {
        for _ in 0..num_of_researches {
            self.add_researcher();
        }
    }

    #[wasm_bindgen]
    pub async fn run(
        &mut self,
        max_steps: u32,
        sleep_millis: u32,
        summary_interval: u32,
        update_function: js_sys::Function,
    ) {
        set_panic_hook();

        for _ in 0..max_steps {
            TimeoutFuture::new(sleep_millis).await;

            self.step();

            if self.time % summary_interval == 0 {
                update_function
                    .call1(&JsValue::NULL, &self.summarize())
                    .unwrap();
            }
        }
    }

    #[wasm_bindgen]
    pub fn step(&mut self) {
        for _ in 0..random_int_in_range(0, 10) {
            self.add_researcher();
        }

        for researcher in &mut self.researchers {
            researcher.step();
        }

        self.time += 1;
    }

    pub fn add_researcher(&mut self) {
        let researcher = Researcher::new(self.last_researcher_id);
        self.researchers.push(researcher);
        self.last_researcher_id += 1;
    }

    #[wasm_bindgen]
    pub fn summarize(&self) -> JsValue {
        let mut summary = Summary {
            time: self.time,
            number_of_studies: self.studies.len() as u32,
            number_of_researchers: self.researchers.len() as u32,
            number_of_published_studies: 0,
            number_of_unpublished_studies: 0,
            number_of_reproductions: 0,
            number_of_replications: 0,
            number_of_originals: 0,
            number_of_confirmed: 0,
            number_of_disconfirmed: 0,
            number_of_pending: 0,
            avg_effect_size: 0.0,
        };

        for study in &self.studies {
            match study.publication_status {
                PublicationStatus::Published => summary.number_of_published_studies += 1,
                PublicationStatus::Unpublished => summary.number_of_unpublished_studies += 1,
                _ => {}
            }

            match study.study_type {
                StudyType::Original => summary.number_of_originals += 1,
                StudyType::Reproduction => summary.number_of_reproductions += 1,
                StudyType::Replication => summary.number_of_replications += 1,
            }

            if let Some(ConfirmationStatus::Confirmed) = study.confirmation_status {
                summary.number_of_confirmed += 1;
            } else if let Some(ConfirmationStatus::Disconfirmed) = study.confirmation_status {
                summary.number_of_disconfirmed += 1;
            } else {
                summary.number_of_pending += 1;
            }
        }

        if summary.number_of_studies > 0 {
            let total_effect_size: f64 = self.studies.iter().map(|s| s.reported_effect_size).sum();
            summary.avg_effect_size = total_effect_size / (summary.number_of_studies as f64);
        }

        serde_wasm_bindgen::to_value(&summary).unwrap()
    }

    #[wasm_bindgen]
    pub fn serialize(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

impl Researcher {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            current_study_id: None,
            prob_reproduce: utils::random_prob(),
            prob_replicate: utils::random_prob(),
            prob_open_original: utils::random_prob(),
            prob_open_repeating: utils::random_prob(),
            odds_repeat_open: utils::random_prob(),
            prob_closed_has_data: utils::random_prob(),
        }
    }

    pub fn step(&mut self) {
        // Implement the logic for the researcher's step here
        // For now, we will just print the researcher's ID
        // println!("Researcher {} is stepping", self.id);
    }
}

impl Study {
    fn published_with_prob(&self, prob: f64) -> PublicationStatus {
        if utils::random_bool_with_prob(prob) {
            PublicationStatus::Published
        } else {
            PublicationStatus::Unpublished
        }
    }

    pub fn determine_publication_status(&mut self) {
        let prob_pub_null = 0.009;
        let prob_pub_conf = 0.6;
        let prob_pub_disconf = 0.7;

        match self.study_type {
            StudyType::Original => {
                if self.sampled_effect_size.abs() > SIGNIFICANCE_LEVEL {
                    self.publication_status = PublicationStatus::Published;
                } else {
                    self.publication_status = self.published_with_prob(prob_pub_null);
                }
            }
            StudyType::Replication | StudyType::Reproduction => match self.confirmation_status {
                Some(ConfirmationStatus::Confirmed) => {
                    self.publication_status = self.published_with_prob(prob_pub_conf);
                }
                Some(ConfirmationStatus::Disconfirmed) => {
                    self.publication_status = self.published_with_prob(prob_pub_disconf);
                }
                None => {}
            },
        }
    }
}
