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

use futures;
use futures::{Future, Stream};
use hyper;
use serde_json;

use hyper::header::UserAgent;

use super::{configuration, Error};

pub struct MeshSecretValuesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> MeshSecretValuesApiClient<C> {
    pub fn new(
        configuration: Rc<configuration::Configuration<C>>,
    ) -> MeshSecretValuesApiClient<C> {
        MeshSecretValuesApiClient { configuration }
    }
}

pub trait MeshSecretValuesApi {
    fn mesh_secret_value_add_value(
        &self,
        api_version: &str,
        secret_resource_name: &str,
        secret_value_resource_name: &str,
        secret_value_resource_description: ::models::SecretValueResourceDescription,
    ) -> Box<
        dyn Future<
            Item = ::models::SecretValueResourceDescription,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn mesh_secret_value_delete(
        &self,
        api_version: &str,
        secret_resource_name: &str,
        secret_value_resource_name: &str,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn mesh_secret_value_get(
        &self,
        api_version: &str,
        secret_resource_name: &str,
        secret_value_resource_name: &str,
    ) -> Box<
        dyn Future<
            Item = ::models::SecretValueResourceDescription,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn mesh_secret_value_list(
        &self,
        api_version: &str,
        secret_resource_name: &str,
    ) -> Box<
        dyn Future<
            Item = ::models::PagedSecretValueResourceDescriptionList,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn mesh_secret_value_show(
        &self,
        api_version: &str,
        secret_resource_name: &str,
        secret_value_resource_name: &str,
    ) -> Box<
        dyn Future<
            Item = ::models::SecretValue,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::Connect> MeshSecretValuesApi
    for MeshSecretValuesApiClient<C>
{
    fn mesh_secret_value_add_value(
        &self,
        api_version: &str,
        secret_resource_name: &str,
        secret_value_resource_name: &str,
        secret_value_resource_description: ::models::SecretValueResourceDescription,
    ) -> Box<
        dyn Future<
            Item = ::models::SecretValueResourceDescription,
            Error = Error<serde_json::Value>,
        >,
    > {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Put;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.finish()
        };
        let uri_str = format!("{}/Resources/Secrets/{secretResourceName}/values/{secretValueResourceName}?{}", configuration.base_path, query_string, secretResourceName=secret_resource_name, secretValueResourceName=secret_value_resource_name);

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

        let serialized =
            serde_json::to_string(&secret_value_resource_description).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut()
            .set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

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
                        ::models::SecretValueResourceDescription,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn mesh_secret_value_delete(
        &self,
        api_version: &str,
        secret_resource_name: &str,
        secret_value_resource_name: &str,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Delete;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.finish()
        };
        let uri_str = format!("{}/Resources/Secrets/{secretResourceName}/values/{secretValueResourceName}?{}", configuration.base_path, query_string, secretResourceName=secret_resource_name, secretValueResourceName=secret_value_resource_name);

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
                .and_then(|_| futures::future::ok(())),
        )
    }

    fn mesh_secret_value_get(
        &self,
        api_version: &str,
        secret_resource_name: &str,
        secret_value_resource_name: &str,
    ) -> Box<
        dyn Future<
            Item = ::models::SecretValueResourceDescription,
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
            query.finish()
        };
        let uri_str = format!("{}/Resources/Secrets/{secretResourceName}/values/{secretValueResourceName}?{}", configuration.base_path, query_string, secretResourceName=secret_resource_name, secretValueResourceName=secret_value_resource_name);

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
                        ::models::SecretValueResourceDescription,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn mesh_secret_value_list(
        &self,
        api_version: &str,
        secret_resource_name: &str,
    ) -> Box<
        dyn Future<
            Item = ::models::PagedSecretValueResourceDescriptionList,
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
            query.finish()
        };
        let uri_str = format!(
            "{}/Resources/Secrets/{secretResourceName}/values?{}",
            configuration.base_path,
            query_string,
            secretResourceName = secret_resource_name
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
                        ::models::PagedSecretValueResourceDescriptionList,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn mesh_secret_value_show(
        &self,
        api_version: &str,
        secret_resource_name: &str,
        secret_value_resource_name: &str,
    ) -> Box<
        dyn Future<
            Item = ::models::SecretValue,
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
            query.finish()
        };
        let uri_str = format!("{}/Resources/Secrets/{secretResourceName}/values/{secretValueResourceName}/list_value?{}", configuration.base_path, query_string, secretResourceName=secret_resource_name, secretValueResourceName=secret_value_resource_name);

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
                    let parsed: Result<::models::SecretValue, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }
}
