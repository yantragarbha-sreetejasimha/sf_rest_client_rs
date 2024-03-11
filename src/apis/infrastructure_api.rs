/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.2.0.46
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::borrow::Borrow;
use std::borrow::Cow;
use std::rc::Rc;

use hyper;
use serde_json;

use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{configuration, Error};

pub struct InfrastructureApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> InfrastructureApiClient<C> {
    pub fn new(
        configuration: Rc<configuration::Configuration<C>>,
    ) -> InfrastructureApiClient<C> {
        InfrastructureApiClient { configuration }
    }
}

pub trait InfrastructureApi {
    fn invoke_infrastructure_command(
        &self,
        api_version: &str,
        command: &str,
        service_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::InfrastructureServiceResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn invoke_infrastructure_query(
        &self,
        api_version: &str,
        command: &str,
        service_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::InfrastructureServiceResponse,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::Connect> InfrastructureApi
    for InfrastructureApiClient<C>
{
    fn invoke_infrastructure_command(
        &self,
        api_version: &str,
        command: &str,
        service_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::InfrastructureServiceResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.append_pair("Command", command);
            query.append_pair("ServiceId", service_id);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/$/InvokeInfrastructureCommand?{}",
            configuration.base_path, query_string
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(Error::from)
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(Error::from)
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<
                        ::models::InfrastructureServiceResponse,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn invoke_infrastructure_query(
        &self,
        api_version: &str,
        command: &str,
        service_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::InfrastructureServiceResponse,
            Error = Error<serde_json::Value>,
        >,
    > {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.append_pair("Command", command);
            query.append_pair("ServiceId", service_id);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/$/InvokeInfrastructureQuery?{}",
            configuration.base_path, query_string
        );

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut()
                .set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        // send request
        Box::new(
            configuration
                .client
                .request(req)
                .map_err(Error::from)
                .and_then(|resp| {
                    let status = resp.status();
                    resp.body()
                        .concat2()
                        .and_then(move |body| Ok((status, body)))
                        .map_err(Error::from)
                })
                .and_then(|(status, body)| {
                    if status.is_success() {
                        Ok(body)
                    } else {
                        Err(Error::from((status, &*body)))
                    }
                })
                .and_then(|body| {
                    let parsed: Result<
                        ::models::InfrastructureServiceResponse,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }
}
