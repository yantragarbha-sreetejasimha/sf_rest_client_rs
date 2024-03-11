/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 8.2
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

pub struct ChaosApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ChaosApiClient<C> {
    pub fn new(
        configuration: Rc<configuration::Configuration<C>>,
    ) -> ChaosApiClient<C> {
        ChaosApiClient { configuration }
    }
}

pub trait ChaosApi {
    fn get_chaos(
        &self,
        api_version: &str,
        timeout: i64,
    ) -> Box<dyn Future<Item = ::models::Chaos, Error = Error<serde_json::Value>>>;
    fn get_chaos_events(
        &self,
        api_version: &str,
        continuation_token: &str,
        start_time_utc: &str,
        end_time_utc: &str,
        max_results: i64,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ChaosEventsSegment,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_chaos_schedule(
        &self,
        api_version: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ChaosScheduleDescription,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn post_chaos_schedule(
        &self,
        api_version: &str,
        chaos_schedule: ::models::ChaosScheduleDescription,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn start_chaos(
        &self,
        api_version: &str,
        chaos_parameters: ::models::ChaosParameters,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn stop_chaos(
        &self,
        api_version: &str,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> ChaosApi for ChaosApiClient<C> {
    fn get_chaos(
        &self,
        api_version: &str,
        timeout: i64,
    ) -> Box<dyn Future<Item = ::models::Chaos, Error = Error<serde_json::Value>>>
    {
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
        let uri_str =
            format!("{}/Tools/Chaos?{}", configuration.base_path, query_string);

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
                    let parsed: Result<::models::Chaos, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_chaos_events(
        &self,
        api_version: &str,
        continuation_token: &str,
        start_time_utc: &str,
        end_time_utc: &str,
        max_results: i64,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ChaosEventsSegment,
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
            query.append_pair("StartTimeUtc", start_time_utc);
            query.append_pair("EndTimeUtc", end_time_utc);
            query.append_pair("MaxResults", &max_results.to_string());
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/Tools/Chaos/Events?{}",
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
                    let parsed: Result<::models::ChaosEventsSegment, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_chaos_schedule(
        &self,
        api_version: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ChaosScheduleDescription,
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
            "{}/Tools/Chaos/Schedule?{}",
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
                    let parsed: Result<::models::ChaosScheduleDescription, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn post_chaos_schedule(
        &self,
        api_version: &str,
        chaos_schedule: ::models::ChaosScheduleDescription,
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
            "{}/Tools/Chaos/Schedule?{}",
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

        let serialized = serde_json::to_string(&chaos_schedule).unwrap();
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

    fn start_chaos(
        &self,
        api_version: &str,
        chaos_parameters: ::models::ChaosParameters,
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
            "{}/Tools/Chaos/$/Start?{}",
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

        let serialized = serde_json::to_string(&chaos_parameters).unwrap();
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

    fn stop_chaos(
        &self,
        api_version: &str,
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
            "{}/Tools/Chaos/$/Stop?{}",
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
                .and_then(|_| futures::future::ok(())),
        )
    }
}
