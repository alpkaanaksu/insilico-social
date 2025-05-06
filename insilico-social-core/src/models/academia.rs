use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

const SIGNIFICANCE_LEVEL: f64 = 0.05;

use crate::utils;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Academia {
    pub time: u32,
    pub last_study_id: u32,
    pub last_researcher_id: u32,

    #[wasm_bindgen(skip)]
    pub researchers: Vec<Researcher>,
    #[wasm_bindgen(skip)]
    pub studies: Vec<Study>
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
    Replication
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub enum PublicationStatus {
    Pending,
    Published,
    Unpublished
}

#[derive(Serialize, Deserialize)]
pub enum ConfirmationStatus {
    Confirmed,
    Disconfirmed
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
    pub fn serialize(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
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
            StudyType::Replication | StudyType::Reproduction => {
                match self.confirmation_status {
                    Some(ConfirmationStatus::Confirmed) => {
                        self.publication_status = self.published_with_prob(prob_pub_conf);
                    }
                    Some(ConfirmationStatus::Disconfirmed) => {
                        self.publication_status = self.published_with_prob(prob_pub_disconf);
                    }
                    None => {
                       
                    }
                }
            }
        }
    }
}