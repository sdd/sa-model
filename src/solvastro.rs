use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    pub filename: String,
    pub img_w: u32,
    pub img_h: u32,

    pub points: Vec<QueryPoint>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct QueryPoint {
    pub x: f64,
    pub y: f64,
    pub amplitude: f64,
    pub radius: f64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct VerifiedSolutionSerial {
    pub solution: Option<SolutionSerial>,
    pub matches: Vec<ItemMatchResult>,

    pub logodds: f64,
    best_idx: usize,
    pub best_logodds: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SolutionSerial {
    pub query_quad: Quad,
    pub cat_quad_idx: usize,

    pub cat_quad_identifiers: [String; 4],
    pub code_error: f64,

    pub wcs: WcsTanSerial,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ItemMatchResult {
    #[allow(dead_code)]
    Filtered, // should be used for quad stars

    Distractor,
    Conflict,
    Match {
        field: usize,
        star_idx: usize,
        logodds: f64,
        field_x: f64,
        field_y: f64,
        index_x: f64,
        index_y: f64,
        index_name: String,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Quad {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub d: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WcsTanSerial {
    pub crpix: [f64; 2],

    pub crval: CoordCelestial,

    pub cd: [f64; 4],
    pub cd_inv: [f64; 4],

    pub mtx_w2i: [f64; 9],
    pub mtx_i2w: [f64; 9],

    sip_params: Option<(Vec<f64>, Vec<f64>)>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct CoordCelestial {
    pub ra: f64,  // radians
    pub dec: f64, // radians
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SolveRequest {
    pub id: String,
    pub user_id: String,
    pub connection_id: Option<String>,
    pub query: Query,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SolveResponse {
    pub id: String,
    pub user_id: String,
    pub connection_id: Option<String>,
    pub response: Option<VerifiedSolutionSerial>,
}
