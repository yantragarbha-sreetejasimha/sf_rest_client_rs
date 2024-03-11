/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.0
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

pub struct ComposeDeploymentApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ComposeDeploymentApiClient<C> {
    pub fn new(
        configuration: Rc<configuration::Configuration<C>>,
    ) -> ComposeDeploymentApiClient<C> {
        ComposeDeploymentApiClient { configuration }
    }
}

pub trait ComposeDeploymentApi {
    fn create_compose_deployment(
        &self,
        api_version: &str,
        create_compose_deployment_description: ::models::CreateComposeDeploymentDescription,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_compose_deployment_status(
        &self,
        api_version: &str,
        deployment_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ComposeDeploymentStatusInfo,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_compose_deployment_status_list(
        &self,
        api_version: &str,
        continuation_token: &str,
        max_results: i64,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PagedComposeDeploymentStatusInfoList,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_compose_deployment_upgrade_progress(
        &self,
        api_version: &str,
        deployment_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ComposeDeploymentUpgradeProgressInfo,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn remove_compose_deployment(
        &self,
        api_version: &str,
        deployment_name: &str,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn start_compose_deployment_upgrade(
        &self,
        api_version: &str,
        deployment_name: &str,
        compose_deployment_upgrade_description: ::models::ComposeDeploymentUpgradeDescription,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn start_rollback_compose_deployment_upgrade(
        &self,
        api_version: &str,
        deployment_name: &str,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> ComposeDeploymentApi
    for ComposeDeploymentApiClient<C>
{
    fn create_compose_deployment(
        &self,
        api_version: &str,
        create_compose_deployment_description: ::models::CreateComposeDeploymentDescription,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Put;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/ComposeDeployments/$/Create?{}",
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

        let serialized =
            serde_json::to_string(&create_compose_deployment_description)
                .unwrap();
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
                .and_then(|_| futures::future::ok(())),
        )
    }

    fn get_compose_deployment_status(
        &self,
        api_version: &str,
        deployment_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ComposeDeploymentStatusInfo,
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
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/ComposeDeployments/{deploymentName}?{}",
            configuration.base_path,
            query_string,
            deploymentName = deployment_name
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
                        ::models::ComposeDeploymentStatusInfo,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_compose_deployment_status_list(
        &self,
        api_version: &str,
        continuation_token: &str,
        max_results: i64,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PagedComposeDeploymentStatusInfoList,
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
            query.append_pair("ContinuationToken", continuation_token);
            query.append_pair("MaxResults", &max_results.to_string());
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/ComposeDeployments?{}",
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
                        ::models::PagedComposeDeploymentStatusInfoList,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_compose_deployment_upgrade_progress(
        &self,
        api_version: &str,
        deployment_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ComposeDeploymentUpgradeProgressInfo,
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
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/ComposeDeployments/{deploymentName}/$/GetUpgradeProgress?{}",
            configuration.base_path,
            query_string,
            deploymentName = deployment_name
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
                        ::models::ComposeDeploymentUpgradeProgressInfo,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn remove_compose_deployment(
        &self,
        api_version: &str,
        deployment_name: &str,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/ComposeDeployments/{deploymentName}/$/Delete?{}",
            configuration.base_path,
            query_string,
            deploymentName = deployment_name
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
                .and_then(|_| futures::future::ok(())),
        )
    }

    fn start_compose_deployment_upgrade(
        &self,
        api_version: &str,
        deployment_name: &str,
        compose_deployment_upgrade_description: ::models::ComposeDeploymentUpgradeDescription,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/ComposeDeployments/{deploymentName}/$/Upgrade?{}",
            configuration.base_path,
            query_string,
            deploymentName = deployment_name
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

        let serialized =
            serde_json::to_string(&compose_deployment_upgrade_description)
                .unwrap();
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
                .and_then(|_| futures::future::ok(())),
        )
    }

    fn start_rollback_compose_deployment_upgrade(
        &self,
        api_version: &str,
        deployment_name: &str,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/ComposeDeployments/{deploymentName}/$/RollbackUpgrade?{}",
            configuration.base_path,
            query_string,
            deploymentName = deployment_name
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
                .and_then(|_| futures::future::ok(())),
        )
    }
}
