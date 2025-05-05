const SIGNIFICANCE_LEVEL: f64 = 0.05;

pub mod utils;

#[wasm_bindgen]
pub struct Academia {
    pub researchers: Vec<Researcher>,
    pub studies: Vec<Study>,
}

pub struct Researcher {
    pub academia: Academia,
    pub id: String,
    pub current_study: Option<Study>,
    pub prob_reproduce: f64,
    pub prob_replicate: f64,
    pub prob_open_original: f64,
    pub prob_open_replicate: f64,
    pub odds_repeat_open: f64,
    pub prob_closed_has_data: f64,
}

pub enum StudyType {
    Original,
    Reproduction,
    Replication
}

pub enum PublicationStatus {
    Pending,
    Published,
    Unpublished
}

pub enum ConfirmationStatus {
    Confirmed,
    Disconfirmed
}

pub struct Study {
    pub study_type: StudyType,
    pub researcher_id: String,
    pub true_effect_size: f64,
    pub sampled_effect_size: f64,
    pub observed_effect_size: f64,
    pub reported_effect_size: f64,
    pub duration_days: u32,
    pub publication_status: PublicationStatus,

    // If replication study
    pub replication_of: Option<Study>,
    pub confirmation_status: Option<ConfirmationStatus>,

    // If reproduction study
    pub reproduction_of: Option<Study>
}

#[wasm_bindgen]
impl Academia {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            researchers: Vec::new(),
            ongoing_studies: Vec::new(),
        }
    }

    pub fn add_researcher(&mut self, researcher: Researcher) {
        self.researchers.push(researcher);
    }

    pub fn add_study(&mut self, study: Study) {
        self.ongoing_studies.push(study);
    }

    pub fn get_random_free_study(&self) -> Option<&Study> {
        self.ongoing_studies.iter().find(|study| {
            
            // Check if the study is free (not assigned to any researcher)
            !self.researchers.iter().any(|r| r.id == study.researcher_id)
        })
    }

    pub fn step(&mut self) {
        for researcher in &mut self.researchers {
            researcher.step();
        }
    }

    pub fn serialize(&self) -> JsValue {
        JsValue::from_serde(self).unwrap()
    }
}

pub impl Researcher {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String, prob_reproduce: f64, prob_replicate: f64) -> Self {
        Self {
            id,
            prob_reproduce,
            prob_replicate,
        }
    }

    pub fn step(&mut self) {
        let study = self.academia.get_random_free_study();
    }
}

pub impl Study {
    fn published_with_prob(prob: f64) -> PublicationStatus {
        if utils::random_bool_with_prob(prob) {
            PublicationStatus::Published
        } else {
            PublicationStatus::Unpublished
        }
    }

    pub fn determine_publication_status(&mut self) {
        let prob_pub_sig = 1.0;
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
                }
            }
        }
    }
}