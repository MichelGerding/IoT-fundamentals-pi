use std::error::Error;
use std::f64;

use smartcore::linalg::basic::matrix::DenseMatrix;
// Linear Regression
use smartcore::linear::logistic_regression::{LogisticRegression, LogisticRegressionParameters, LogisticRegressionSolverName};

pub enum AIModel {
    Smartcore(LogisticRegression<f64, u8, DenseMatrix<f64>, Vec<u8>>),
}

impl AIModel {
    pub fn predict(&self, inputs: Vec<f64>) -> u8 {
        match self {
            AIModel::Smartcore(model) => {
                let x = DenseMatrix::from_2d_vec(&vec![inputs]);
                let y_pred = model.predict(&x).unwrap();
                y_pred[0]
            }
        }
    }
}


pub(crate) fn save_model(
    model: &AIModel,
    path: &str) -> Result<(), Box<dyn Error>> {
    match model {
        AIModel::Smartcore(model) => {
            let bytes = bincode::serialize(&model)?;
            std::fs::write(path, bytes)?;
        }
    }


    Ok(())
}

pub(crate) fn load_model(path: &str, backend: &str) -> Result<AIModel, Box<dyn Error>> {
    match backend {
        "smartcore" => {
            let bytes = std::fs::read(path)?;
            let model = bincode::deserialize(&bytes)?;
            Ok(AIModel::Smartcore(model))
        }
        e => {
            Err(format!("backend {} not supported", e).into())
        }
    }
}

pub(crate) fn train_custom(solver: LogisticRegressionSolverName) -> Result<AIModel, Box<dyn Error>> {

    // load the dataset from the csv
    let mut xd: Vec<Vec<f64>> = Vec::new();
    let mut yd: Vec<u8> = Vec::new();

    let mut rdr = csv::Reader::from_path("./src/ai/dataset.csv")?;
    for result in rdr.records() {
        let record = result?;
        let mut x: Vec<f64> = Vec::new();

        // parse the measurements from the csv
        x.push(record[2].trim_start().parse::<f64>().unwrap());
        x.push(record[3].trim_start().parse::<f64>().unwrap() / 10.);
        x.push(record[5].trim_start().parse::<f64>().unwrap() / 10.);
        x.push(record[6].trim_start().parse::<f64>().unwrap());

        xd.push(x);

        // if there is more then 0.5 mm of rain we will consider it rain
        // the dataset does not offer a higher resolution then 0.5 mm
        // it does provide a -1 if there is no rain but using that we have a 100% false positive rate
        if record[4].trim_start().parse::<f64>().unwrap() > 0.0 {
            yd.push(1);
        } else {
            yd.push(0);
        }
    }
    yd.remove(0);
    xd.pop();

    // count the amount of yd which are greater then 0
    let mut count = 0;
    for i in 0..yd.len() {
        if yd[i] > 0 {
            count += 1;
        }
    }

    // make sure there are count amount of rain and count amount of no rain
    let mut xd2: Vec<Vec<f64>> = Vec::new();
    let mut yd2: Vec<u8> = Vec::new();
    let mut count2 = 0;

    for i in 0..yd.len() {
        if yd[i] > 0 {
            xd2.push(xd[i].clone());
            yd2.push(yd[i].clone());
        } else if count2 < count {
            xd2.push(xd[i].clone());
            yd2.push(yd[i].clone());
            count2 += 1;
        }
    }


    let x = DenseMatrix::from_2d_vec(&xd2);
    let lr = LogisticRegression::fit(&x, &yd2,
       LogisticRegressionParameters {
           solver: solver.clone(),
           alpha: 0.0001,
       })?;

    let y_pred = lr.predict(&x)?;

    // get the mean squared error for values with rain
    let mut avg_answer = 0.0;

    let mut rain_correct = 0;
    let mut no_rain_correct = 0;


    for i in 0..yd2.len() {
        if yd2[i] > 0{
            if y_pred[i] == yd2[i] {
                rain_correct += 1;
            }
        } else {
            if y_pred[i] == yd2[i] {
                no_rain_correct += 1;
            }
        }



        avg_answer += y_pred[i] as f64 / yd2.len() as f64;
    }

    println!("model trained {:?}:
    - rain correct: {:.3}%
    - no rain correct: {:.3}%
    - total correct: {:.3}%
    - avg pred: {:.3}",
        solver,
        rain_correct as f64 / count as f64 * 100.0,
        no_rain_correct as f64 / (yd2.len() - count) as f64 * 100.0,
        (rain_correct + no_rain_correct) as f64 / yd2.len() as f64 * 100.0,
        avg_answer);


    return Ok(AIModel::Smartcore(lr));
}

pub(crate) fn train() -> Result<
    AIModel,
    Box<dyn Error>> {
    train_custom(LogisticRegressionSolverName::LBFGS)
}