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

pub struct ServiceApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ServiceApiClient<C> {
    pub fn new(
        configuration: Rc<configuration::Configuration<C>>,
    ) -> ServiceApiClient<C> {
        ServiceApiClient { configuration }
    }
}

pub trait ServiceApi {
    fn create_service(
        &self,
        api_version: &str,
        application_id: &str,
        service_description: ::models::ServiceDescription,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_service_from_template(
        &self,
        api_version: &str,
        application_id: &str,
        service_from_template_description: ::models::ServiceFromTemplateDescription,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_service(
        &self,
        api_version: &str,
        service_id: &str,
        force_remove: bool,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_application_name_info(
        &self,
        api_version: &str,
        service_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ApplicationNameInfo,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_service_description(
        &self,
        api_version: &str,
        service_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceDescription,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_service_health(
        &self,
        api_version: &str,
        service_id: &str,
        events_health_state_filter: i32,
        partitions_health_state_filter: i32,
        exclude_health_statistics: bool,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceHealth,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_service_health_using_policy(
        &self,
        api_version: &str,
        service_id: &str,
        events_health_state_filter: i32,
        partitions_health_state_filter: i32,
        application_health_policy: ::models::ApplicationHealthPolicy,
        exclude_health_statistics: bool,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceHealth,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_service_info(
        &self,
        application_id: &str,
        service_id: &str,
        api_version: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceInfo,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_service_info_list(
        &self,
        application_id: &str,
        api_version: &str,
        service_type_name: &str,
        continuation_token: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PagedServiceInfoList,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_unplaced_replica_information(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        only_query_primaries: bool,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::UnplacedReplicaInformation,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn report_service_health(
        &self,
        api_version: &str,
        service_id: &str,
        health_information: ::models::HealthInformation,
        immediate: bool,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn resolve_service(
        &self,
        api_version: &str,
        service_id: &str,
        partition_key_type: i32,
        partition_key_value: &str,
        previous_rsp_version: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ResolvedServicePartition,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn update_service(
        &self,
        api_version: &str,
        service_id: &str,
        service_update_description: ::models::ServiceUpdateDescription,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> ServiceApi for ServiceApiClient<C> {
    fn create_service(
        &self,
        api_version: &str,
        application_id: &str,
        service_description: ::models::ServiceDescription,
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
            "{}/Applications/{applicationId}/$/GetServices/$/Create?{}",
            configuration.base_path,
            query_string,
            applicationId = application_id
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

        let serialized = serde_json::to_string(&service_description).unwrap();
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

    fn create_service_from_template(
        &self,
        api_version: &str,
        application_id: &str,
        service_from_template_description: ::models::ServiceFromTemplateDescription,
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
        let uri_str = format!("{}/Applications/{applicationId}/$/GetServices/$/CreateFromTemplate?{}", configuration.base_path, query_string, applicationId=application_id);

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
            serde_json::to_string(&service_from_template_description).unwrap();
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

    fn delete_service(
        &self,
        api_version: &str,
        service_id: &str,
        force_remove: bool,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.append_pair("ForceRemove", &force_remove.to_string());
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/Services/{serviceId}/$/Delete?{}",
            configuration.base_path,
            query_string,
            serviceId = service_id
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

    fn get_application_name_info(
        &self,
        api_version: &str,
        service_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ApplicationNameInfo,
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
            "{}/Services/{serviceId}/$/GetApplicationName?{}",
            configuration.base_path,
            query_string,
            serviceId = service_id
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
                    let parsed: Result<::models::ApplicationNameInfo, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_service_description(
        &self,
        api_version: &str,
        service_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceDescription,
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
            "{}/Services/{serviceId}/$/GetDescription?{}",
            configuration.base_path,
            query_string,
            serviceId = service_id
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
                    let parsed: Result<::models::ServiceDescription, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_service_health(
        &self,
        api_version: &str,
        service_id: &str,
        events_health_state_filter: i32,
        partitions_health_state_filter: i32,
        exclude_health_statistics: bool,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceHealth,
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
                "EventsHealthStateFilter",
                &events_health_state_filter.to_string(),
            );
            query.append_pair(
                "PartitionsHealthStateFilter",
                &partitions_health_state_filter.to_string(),
            );
            query.append_pair(
                "ExcludeHealthStatistics",
                &exclude_health_statistics.to_string(),
            );
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/Services/{serviceId}/$/GetHealth?{}",
            configuration.base_path,
            query_string,
            serviceId = service_id
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
                    let parsed: Result<::models::ServiceHealth, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_service_health_using_policy(
        &self,
        api_version: &str,
        service_id: &str,
        events_health_state_filter: i32,
        partitions_health_state_filter: i32,
        application_health_policy: ::models::ApplicationHealthPolicy,
        exclude_health_statistics: bool,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceHealth,
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
            query.append_pair(
                "EventsHealthStateFilter",
                &events_health_state_filter.to_string(),
            );
            query.append_pair(
                "PartitionsHealthStateFilter",
                &partitions_health_state_filter.to_string(),
            );
            query.append_pair(
                "ExcludeHealthStatistics",
                &exclude_health_statistics.to_string(),
            );
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/Services/{serviceId}/$/GetHealth?{}",
            configuration.base_path,
            query_string,
            serviceId = service_id
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
            serde_json::to_string(&application_health_policy).unwrap();
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
                    let parsed: Result<::models::ServiceHealth, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_service_info(
        &self,
        application_id: &str,
        service_id: &str,
        api_version: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ServiceInfo,
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
            "{}/Applications/{applicationId}/$/GetServices/{serviceId}?{}",
            configuration.base_path,
            query_string,
            applicationId = application_id,
            serviceId = service_id
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
                    let parsed: Result<::models::ServiceInfo, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_service_info_list(
        &self,
        application_id: &str,
        api_version: &str,
        service_type_name: &str,
        continuation_token: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PagedServiceInfoList,
            Error = Error<serde_json::Value>,
        >,
    > {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("ServiceTypeName", service_type_name);
            query.append_pair("api-version", api_version);
            query.append_pair("ContinuationToken", continuation_token);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/Applications/{applicationId}/$/GetServices?{}",
            configuration.base_path,
            query_string,
            applicationId = application_id
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
                    let parsed: Result<::models::PagedServiceInfoList, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_unplaced_replica_information(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        only_query_primaries: bool,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::UnplacedReplicaInformation,
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
            query.append_pair("PartitionId", partition_id);
            query.append_pair(
                "OnlyQueryPrimaries",
                &only_query_primaries.to_string(),
            );
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/Services/{serviceId}/$/GetUnplacedReplicaInformation?{}",
            configuration.base_path,
            query_string,
            serviceId = service_id
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
                        ::models::UnplacedReplicaInformation,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn report_service_health(
        &self,
        api_version: &str,
        service_id: &str,
        health_information: ::models::HealthInformation,
        immediate: bool,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.append_pair("Immediate", &immediate.to_string());
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/Services/{serviceId}/$/ReportHealth?{}",
            configuration.base_path,
            query_string,
            serviceId = service_id
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

        let serialized = serde_json::to_string(&health_information).unwrap();
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

    fn resolve_service(
        &self,
        api_version: &str,
        service_id: &str,
        partition_key_type: i32,
        partition_key_value: &str,
        previous_rsp_version: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ResolvedServicePartition,
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
                "PartitionKeyType",
                &partition_key_type.to_string(),
            );
            query.append_pair("PartitionKeyValue", partition_key_value);
            query.append_pair("PreviousRspVersion", previous_rsp_version);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/Services/{serviceId}/$/ResolvePartition?{}",
            configuration.base_path,
            query_string,
            serviceId = service_id
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
                    let parsed: Result<::models::ResolvedServicePartition, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn update_service(
        &self,
        api_version: &str,
        service_id: &str,
        service_update_description: ::models::ServiceUpdateDescription,
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
            "{}/Services/{serviceId}/$/Update?{}",
            configuration.base_path,
            query_string,
            serviceId = service_id
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
            serde_json::to_string(&service_update_description).unwrap();
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
