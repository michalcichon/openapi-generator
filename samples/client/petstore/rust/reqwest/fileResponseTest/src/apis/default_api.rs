/*
 * File Response Test
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;
use std::borrow::Borrow;
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct DefaultApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl DefaultApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> DefaultApiClient {
        DefaultApiClient {
            configuration,
        }
    }
}


pub trait DefaultApi {
    fn fileresponsetest(&self, ) -> Result<std::path::PathBuf, Error<serde_json::Value>>;
}

impl DefaultApi for DefaultApiClient {
    fn fileresponsetest(&self, ) -> Result<std::path::PathBuf, Error<serde_json::Value>> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/tests/fileResponse", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        let mut resp = client.execute(req)?;
        if resp.status().is_success() {
            Ok(resp.json()?)
        } else {
            let status = resp.status();
            let content = resp.text()?;
            let entity: Option<serde_json::Value> = serde_json::from_str(&content).ok();
            let error = crate::apis::ResponseErrorContent { status, content, entity };
            Err(Error::ResponseError(error))
        }
    }

}
