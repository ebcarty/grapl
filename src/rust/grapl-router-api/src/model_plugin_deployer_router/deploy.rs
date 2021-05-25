use crate::make_request;
use actix_web::{post, Error, HttpResponse, Responder};
use::reqwest;
use actix_web::body::Body;
use serde::{Serialize, Deserialize};
// use actix_web::error::InternalError;
// use actix_web::http::StatusCode;
// use actix_web::web::Json;

#[derive(Serialize, Deserialize)]
pub struct DeployRequest{
    name: String,
}

#[derive(thiserror::Error, Debug)]
pub enum CustomError {

    #[error("Request Error")]
    RequestError(#[from] reqwest::Error),

    #[error("Invalid schema contents")]
    InvalidSchema,

    #[error("Unable to read schema contents")]
    ReadError,

    #[error("Internal Server Error")]
    ServerError,



}



// actix procedural macros that route incoming http requests
#[post("/modelPluginDeployer/deploy")]
pub async fn grapl_model_plugin_deployer(body: actix_web::web::Json<DeployRequest>) -> impl Responder {
    // CALL MODEL-PLUGIN-DEPlOYER GRPC CLIENT
    let body = body.into_inner();
    let response = make_request("deploy", body)
        .await;

    match response{
        Ok(response) => HttpResponse::Ok().json(response),

        Err(CustomError::InvalidSchema) => {
            HttpResponse::BadRequest()
                .finish()
        }

        Err(CustomError::ReadError) => {
            HttpResponse::Conflict()
                .finish()
        }

        Err(CustomError::ServerError) => {
            HttpResponse::BadRequest()
            .finish()
        }

        Err(CustomError::RequestError(_)) =>  HttpResponse::InternalServerError().finish(),
    }
}


// We will make a post request to our new actix server
// This will route us to the appropriate model plugin deployer service.

// we come in on a path. Based on that path, we route the request to the appropriate service.

// route to the graplModelPluginDeployer
// setup & write tests with an http client
// use grcp client for model-plugin-deployer
// X set up docker stuff

// every service can have a directory
// every route in service can have a file
