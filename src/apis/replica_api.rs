/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.5.0.36
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

pub struct ReplicaApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ReplicaApiClient<C> {
    pub fn new(
        configuration: Rc<configuration::Configuration<C>>,
    ) -> ReplicaApiClient<C> {
        ReplicaApiClient { configuration }
    }
}

pub trait ReplicaApi {
    fn get_deployed_service_replica_detail_info(
        &self,
        api_version: &str,
        node_name: &str,
        partition_id: &str,
        replica_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::DeployedServiceReplicaDetailInfo,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_deployed_service_replica_detail_info_by_partition_id(
        &self,
        api_version: &str,
        node_name: &str,
        partition_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::DeployedServiceReplicaDetailInfo,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_deployed_service_replica_info_list(
        &self,
        api_version: &str,
        node_name: &str,
        application_id: &str,
        partition_id: &str,
        service_manifest_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::DeployedServiceReplicaInfoList,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_replica_health(
        &self,
        api_version: &str,
        partition_id: &str,
        replica_id: &str,
        events_health_state_filter: i32,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ReplicaHealth,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_replica_health_using_policy(
        &self,
        api_version: &str,
        partition_id: &str,
        replica_id: &str,
        events_health_state_filter: i32,
        application_health_policy: ::models::ApplicationHealthPolicy,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ReplicaHealth,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_replica_info(
        &self,
        api_version: &str,
        partition_id: &str,
        replica_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ReplicaInfo,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_replica_info_list(
        &self,
        api_version: &str,
        partition_id: &str,
        continuation_token: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PagedReplicaInfoList,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn remove_replica(
        &self,
        api_version: &str,
        node_name: &str,
        partition_id: &str,
        replica_id: &str,
        force_remove: bool,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn report_replica_health(
        &self,
        api_version: &str,
        partition_id: &str,
        replica_id: &str,
        service_kind: &str,
        health_information: ::models::HealthInformation,
        immediate: bool,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn restart_replica(
        &self,
        api_version: &str,
        node_name: &str,
        partition_id: &str,
        replica_id: &str,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> ReplicaApi for ReplicaApiClient<C> {
    fn get_deployed_service_replica_detail_info(
        &self,
        api_version: &str,
        node_name: &str,
        partition_id: &str,
        replica_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::DeployedServiceReplicaDetailInfo,
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
        let uri_str = format!("{}/Nodes/{nodeName}/$/GetPartitions/{partitionId}/$/GetReplicas/{replicaId}/$/GetDetail?{}", configuration.base_path, query_string, nodeName=node_name, partitionId=partition_id, replicaId=replica_id);

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
                        ::models::DeployedServiceReplicaDetailInfo,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_deployed_service_replica_detail_info_by_partition_id(
        &self,
        api_version: &str,
        node_name: &str,
        partition_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::DeployedServiceReplicaDetailInfo,
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
        let uri_str = format!("{}/Nodes/{nodeName}/$/GetPartitions/{partitionId}/$/GetReplicas?{}", configuration.base_path, query_string, nodeName=node_name, partitionId=partition_id);

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
                        ::models::DeployedServiceReplicaDetailInfo,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_deployed_service_replica_info_list(
        &self,
        api_version: &str,
        node_name: &str,
        application_id: &str,
        partition_id: &str,
        service_manifest_name: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::DeployedServiceReplicaInfoList,
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
            query.append_pair("ServiceManifestName", service_manifest_name);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!("{}/Nodes/{nodeName}/$/GetApplications/{applicationId}/$/GetReplicas?{}", configuration.base_path, query_string, nodeName=node_name, applicationId=application_id);

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
                        ::models::DeployedServiceReplicaInfoList,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_replica_health(
        &self,
        api_version: &str,
        partition_id: &str,
        replica_id: &str,
        events_health_state_filter: i32,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ReplicaHealth,
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
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!("{}/Partitions/{partitionId}/$/GetReplicas/{replicaId}/$/GetHealth?{}", configuration.base_path, query_string, partitionId=partition_id, replicaId=replica_id);

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
                    let parsed: Result<::models::ReplicaHealth, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_replica_health_using_policy(
        &self,
        api_version: &str,
        partition_id: &str,
        replica_id: &str,
        events_health_state_filter: i32,
        application_health_policy: ::models::ApplicationHealthPolicy,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ReplicaHealth,
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
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!("{}/Partitions/{partitionId}/$/GetReplicas/{replicaId}/$/GetHealth?{}", configuration.base_path, query_string, partitionId=partition_id, replicaId=replica_id);

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
                    let parsed: Result<::models::ReplicaHealth, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_replica_info(
        &self,
        api_version: &str,
        partition_id: &str,
        replica_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::ReplicaInfo,
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
            "{}/Partitions/{partitionId}/$/GetReplicas/{replicaId}?{}",
            configuration.base_path,
            query_string,
            partitionId = partition_id,
            replicaId = replica_id
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
                    let parsed: Result<::models::ReplicaInfo, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_replica_info_list(
        &self,
        api_version: &str,
        partition_id: &str,
        continuation_token: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PagedReplicaInfoList,
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
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/Partitions/{partitionId}/$/GetReplicas?{}",
            configuration.base_path,
            query_string,
            partitionId = partition_id
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
                    let parsed: Result<::models::PagedReplicaInfoList, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn remove_replica(
        &self,
        api_version: &str,
        node_name: &str,
        partition_id: &str,
        replica_id: &str,
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
        let uri_str = format!("{}/Nodes/{nodeName}/$/GetPartitions/{partitionId}/$/GetReplicas/{replicaId}/$/Delete?{}", configuration.base_path, query_string, nodeName=node_name, partitionId=partition_id, replicaId=replica_id);

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

    fn report_replica_health(
        &self,
        api_version: &str,
        partition_id: &str,
        replica_id: &str,
        service_kind: &str,
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
            query.append_pair("ServiceKind", service_kind);
            query.append_pair("Immediate", &immediate.to_string());
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!("{}/Partitions/{partitionId}/$/GetReplicas/{replicaId}/$/ReportHealth?{}", configuration.base_path, query_string, partitionId=partition_id, replicaId=replica_id);

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

    fn restart_replica(
        &self,
        api_version: &str,
        node_name: &str,
        partition_id: &str,
        replica_id: &str,
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
        let uri_str = format!("{}/Nodes/{nodeName}/$/GetPartitions/{partitionId}/$/GetReplicas/{replicaId}/$/Restart?{}", configuration.base_path, query_string, nodeName=node_name, partitionId=partition_id, replicaId=replica_id);

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
