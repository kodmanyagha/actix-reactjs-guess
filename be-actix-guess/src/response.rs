#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct SingleResponse<T> {
    pub status: String,
    pub data: T,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct PaginationResponse<T> {
    pub status: String,
    pub recordsTotal: u32,
    pub recordsFiltered: u32,
    pub data: Vec<T>,
}
