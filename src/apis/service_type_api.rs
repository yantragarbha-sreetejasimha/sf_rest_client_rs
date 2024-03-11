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

use hyper;
use serde_json;

use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{configuration, Error};

pub struct ServiceTypeApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ServiceTypeApiClient<C> {
    pub fn new(
        configuration: Rc<configuration::Configuration<C>>,
    ) -> ServiceTypeApiClient<C> {
        ServiceTypeApiClient { configuration }
    }
}

pub trait ServiceTypeApi {
    fn get_deployed_service_type_info_by_name(
        &self,
        api_version: &str,
        node_name: &str,
        application_id: &str,
        service_type_name: &str,
        service_manifest_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::DeployedServiceTypeInfoList,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_deployed_service_type_info_list(
        &self,
        api_version: &str,
        node_name: &str,
        application_id: &str,
        service_manifest_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::DeployedServiceTypeInfoList,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_service_manifest(
        &self,
        api_version: &str,
        application_type_name: &str,
        application_type_version: &str,
        service_manifest_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceTypeManifest,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_service_type_info_by_name(
        &self,
        api_version: &str,
        application_type_name: &str,
        application_type_version: &str,
        service_type_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceTypeInfo,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_service_type_info_list(
        &self,
        api_version: &str,
        application_type_name: &str,
        application_type_version: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceTypeInfoList,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::Connect> ServiceTypeApi for ServiceTypeApiClient<C> {
    fn get_deployed_service_type_info_by_name(
        &self,
        api_version: &str,
        node_name: &str,
        application_id: &str,
        service_type_name: &str,
        service_manifest_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::DeployedServiceTypeInfoList,
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
            query.append_pair("ServiceManifestName", service_manifest_name);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!("{}/Nodes/{nodeName}/$/GetApplications/{applicationId}/$/GetServiceTypes/{serviceTypeName}?{}", configuration.base_path, query_string, nodeName=node_name, applicationId=application_id, serviceTypeName=service_type_name);

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
                        ::models::DeployedServiceTypeInfoList,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_deployed_service_type_info_list(
        &self,
        api_version: &str,
        node_name: &str,
        application_id: &str,
        service_manifest_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::DeployedServiceTypeInfoList,
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
            query.append_pair("ServiceManifestName", service_manifest_name);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!("{}/Nodes/{nodeName}/$/GetApplications/{applicationId}/$/GetServiceTypes?{}", configuration.base_path, query_string, nodeName=node_name, applicationId=application_id);

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
                        ::models::DeployedServiceTypeInfoList,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_service_manifest(
        &self,
        api_version: &str,
        application_type_name: &str,
        application_type_version: &str,
        service_manifest_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceTypeManifest,
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
            query.append_pair("ServiceManifestName", service_manifest_name);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/ApplicationTypes/{applicationTypeName}/$/GetServiceManifest?{}",
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
                    let parsed: Result<::models::ServiceTypeManifest, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_service_type_info_by_name(
        &self,
        api_version: &str,
        application_type_name: &str,
        application_type_version: &str,
        service_type_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceTypeInfo,
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
        let uri_str = format!("{}/ApplicationTypes/{applicationTypeName}/$/GetServiceTypes/{serviceTypeName}?{}", configuration.base_path, query_string, applicationTypeName=application_type_name, serviceTypeName=service_type_name);

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
                    let parsed: Result<::models::ServiceTypeInfo, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_service_type_info_list(
        &self,
        api_version: &str,
        application_type_name: &str,
        application_type_version: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceTypeInfoList,
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
        let uri_str = format!(
            "{}/ApplicationTypes/{applicationTypeName}/$/GetServiceTypes?{}",
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
                    let parsed: Result<::models::ServiceTypeInfoList, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }
}
