/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 7.1.0.45
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

pub struct ApplicationTypeApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ApplicationTypeApiClient<C> {
    pub fn new(
        configuration: Rc<configuration::Configuration<C>>,
    ) -> ApplicationTypeApiClient<C> {
        ApplicationTypeApiClient { configuration }
    }
}

pub trait ApplicationTypeApi {
    fn get_application_manifest(
        &self,
        api_version: &str,
        application_type_name: &str,
        application_type_version: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ApplicationTypeManifest,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_application_type_info_list(
        &self,
        api_version: &str,
        application_type_definition_kind_filter: i32,
        exclude_application_parameters: bool,
        continuation_token: &str,
        max_results: i64,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PagedApplicationTypeInfoList,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_application_type_info_list_by_name(
        &self,
        api_version: &str,
        application_type_name: &str,
        application_type_version: &str,
        exclude_application_parameters: bool,
        continuation_token: &str,
        max_results: i64,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PagedApplicationTypeInfoList,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn provision_application_type(
        &self,
        api_version: &str,
        provision_application_type_description_base_required_body_param: ::models::ProvisionApplicationTypeDescriptionBase,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn unprovision_application_type(
        &self,
        api_version: &str,
        application_type_name: &str,
        unprovision_application_type_description_info: ::models::UnprovisionApplicationTypeDescriptionInfo,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> ApplicationTypeApi
    for ApplicationTypeApiClient<C>
{
    fn get_application_manifest(
        &self,
        api_version: &str,
        application_type_name: &str,
        application_type_version: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ApplicationTypeManifest,
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
            query.append_pair(
                "ApplicationTypeVersion",
                application_type_version,
            );
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!("{}/ApplicationTypes/{applicationTypeName}/$/GetApplicationManifest?{}", configuration.base_path, query_string, applicationTypeName=application_type_name);

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
                    let parsed: Result<::models::ApplicationTypeManifest, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_application_type_info_list(
        &self,
        api_version: &str,
        application_type_definition_kind_filter: i32,
        exclude_application_parameters: bool,
        continuation_token: &str,
        max_results: i64,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PagedApplicationTypeInfoList,
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
            query.append_pair(
                "ApplicationTypeDefinitionKindFilter",
                &application_type_definition_kind_filter.to_string(),
            );
            query.append_pair(
                "ExcludeApplicationParameters",
                &exclude_application_parameters.to_string(),
            );
            query.append_pair("ContinuationToken", continuation_token);
            query.append_pair("MaxResults", &max_results.to_string());
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/ApplicationTypes?{}",
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
                        ::models::PagedApplicationTypeInfoList,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_application_type_info_list_by_name(
        &self,
        api_version: &str,
        application_type_name: &str,
        application_type_version: &str,
        exclude_application_parameters: bool,
        continuation_token: &str,
        max_results: i64,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PagedApplicationTypeInfoList,
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
            query.append_pair(
                "ApplicationTypeVersion",
                application_type_version,
            );
            query.append_pair(
                "ExcludeApplicationParameters",
                &exclude_application_parameters.to_string(),
            );
            query.append_pair("ContinuationToken", continuation_token);
            query.append_pair("MaxResults", &max_results.to_string());
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/ApplicationTypes/{applicationTypeName}?{}",
            configuration.base_path,
            query_string,
            applicationTypeName = application_type_name
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
                        ::models::PagedApplicationTypeInfoList,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn provision_application_type(
        &self,
        api_version: &str,
        provision_application_type_description_base_required_body_param: ::models::ProvisionApplicationTypeDescriptionBase,
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
            "{}/ApplicationTypes/$/Provision?{}",
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

        let serialized = serde_json::to_string(
            &provision_application_type_description_base_required_body_param,
        )
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

    fn unprovision_application_type(
        &self,
        api_version: &str,
        application_type_name: &str,
        unprovision_application_type_description_info: ::models::UnprovisionApplicationTypeDescriptionInfo,
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
            "{}/ApplicationTypes/{applicationTypeName}/$/Unprovision?{}",
            configuration.base_path,
            query_string,
            applicationTypeName = application_type_name
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

        let serialized = serde_json::to_string(
            &unprovision_application_type_description_info,
        )
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
}
