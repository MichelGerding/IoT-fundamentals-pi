use actix_web::{post, web};

use crate::{ai, AppState};

use serde::Deserialize;

use smartcore::linear::logistic_regression::LogisticRegressionSolverName;

use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::str;


#[derive(Deserialize)]
pub(crate) struct TrainForm {
    persist: Option<bool>,
}

#[post("/train")]
pub(crate) async fn train(
    form: web::Form<TrainForm>,
    app_state: web::Data<AppState>,
) -> Result<String, Box<dyn Error>> {
    let form = form.into_inner();
    let persist = form.persist.unwrap_or(false);

    // train a new model
    let new_model = ai::main::train_custom(LogisticRegressionSolverName::LBFGS)?;
    // save the model
    if persist {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        ai::main::save_model(&new_model, &format!("./src/ai/model-{}.bin", now))?;
    }

    // store the new model
    let mut model = app_state.model.lock().unwrap();
    *model = new_model;

    Ok("".to_string())
}

