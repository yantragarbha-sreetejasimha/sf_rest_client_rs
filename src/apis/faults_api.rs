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

pub struct FaultsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> FaultsApiClient<C> {
    pub fn new(
        configuration: Rc<configuration::Configuration<C>>,
    ) -> FaultsApiClient<C> {
        FaultsApiClient { configuration }
    }
}

pub trait FaultsApi {
    fn cancel_operation(
        &self,
        api_version: &str,
        operation_id: &str,
        force: bool,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_data_loss_progress(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        operation_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PartitionDataLossProgress,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_fault_operation_list(
        &self,
        api_version: &str,
        type_filter: i32,
        state_filter: i32,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::OperationStatusList,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_node_transition_progress(
        &self,
        api_version: &str,
        node_name: &str,
        operation_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::NodeTransitionProgress,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_partition_restart_progress(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        operation_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PartitionRestartProgress,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_quorum_loss_progress(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        operation_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PartitionQuorumLossProgress,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn start_data_loss(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        operation_id: &str,
        data_loss_mode: &str,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn start_node_transition(
        &self,
        api_version: &str,
        node_name: &str,
        operation_id: &str,
        node_transition_type: &str,
        node_instance_id: &str,
        stop_duration_in_seconds: i32,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn start_partition_restart(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        operation_id: &str,
        restart_partition_mode: &str,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn start_quorum_loss(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        operation_id: &str,
        quorum_loss_mode: &str,
        quorum_loss_duration: i32,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> FaultsApi for FaultsApiClient<C> {
    fn cancel_operation(
        &self,
        api_version: &str,
        operation_id: &str,
        force: bool,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.append_pair("OperationId", operation_id);
            query.append_pair("Force", &force.to_string());
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/Faults/$/Cancel?{}",
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

    fn get_data_loss_progress(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        operation_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PartitionDataLossProgress,
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
            query.append_pair("OperationId", operation_id);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!("{}/Faults/Services/{serviceId}/$/GetPartitions/{partitionId}/$/GetDataLossProgress?{}", configuration.base_path, query_string, serviceId=service_id, partitionId=partition_id);

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
                    let parsed: Result<::models::PartitionDataLossProgress, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_fault_operation_list(
        &self,
        api_version: &str,
        type_filter: i32,
        state_filter: i32,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::OperationStatusList,
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
            query.append_pair("TypeFilter", &type_filter.to_string());
            query.append_pair("StateFilter", &state_filter.to_string());
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str =
            format!("{}/Faults/?{}", configuration.base_path, query_string);

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
                    let parsed: Result<::models::OperationStatusList, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_node_transition_progress(
        &self,
        api_version: &str,
        node_name: &str,
        operation_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::NodeTransitionProgress,
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
            query.append_pair("OperationId", operation_id);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/Faults/Nodes/{nodeName}/$/GetTransitionProgress?{}",
            configuration.base_path,
            query_string,
            nodeName = node_name
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
                    let parsed: Result<::models::NodeTransitionProgress, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_partition_restart_progress(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        operation_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PartitionRestartProgress,
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
            query.append_pair("OperationId", operation_id);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!("{}/Faults/Services/{serviceId}/$/GetPartitions/{partitionId}/$/GetRestartProgress?{}", configuration.base_path, query_string, serviceId=service_id, partitionId=partition_id);

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
                    let parsed: Result<::models::PartitionRestartProgress, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn get_quorum_loss_progress(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        operation_id: &str,
        timeout: i64,
    ) -> Box<
        dyn Future<
            Item = ::models::PartitionQuorumLossProgress,
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
            query.append_pair("OperationId", operation_id);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!("{}/Faults/Services/{serviceId}/$/GetPartitions/{partitionId}/$/GetQuorumLossProgress?{}", configuration.base_path, query_string, serviceId=service_id, partitionId=partition_id);

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
                        ::models::PartitionQuorumLossProgress,
                        _,
                    > = serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }

    fn start_data_loss(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        operation_id: &str,
        data_loss_mode: &str,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.append_pair("OperationId", operation_id);
            query.append_pair("DataLossMode", data_loss_mode);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!("{}/Faults/Services/{serviceId}/$/GetPartitions/{partitionId}/$/StartDataLoss?{}", configuration.base_path, query_string, serviceId=service_id, partitionId=partition_id);

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

    fn start_node_transition(
        &self,
        api_version: &str,
        node_name: &str,
        operation_id: &str,
        node_transition_type: &str,
        node_instance_id: &str,
        stop_duration_in_seconds: i32,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.append_pair("OperationId", operation_id);
            query.append_pair("NodeTransitionType", node_transition_type);
            query.append_pair("NodeInstanceId", node_instance_id);
            query.append_pair(
                "StopDurationInSeconds",
                &stop_duration_in_seconds.to_string(),
            );
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!(
            "{}/Faults/Nodes/{nodeName}/$/StartTransition/?{}",
            configuration.base_path,
            query_string,
            nodeName = node_name
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

    fn start_partition_restart(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        operation_id: &str,
        restart_partition_mode: &str,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.append_pair("OperationId", operation_id);
            query.append_pair("RestartPartitionMode", restart_partition_mode);
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!("{}/Faults/Services/{serviceId}/$/GetPartitions/{partitionId}/$/StartRestart?{}", configuration.base_path, query_string, serviceId=service_id, partitionId=partition_id);

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

    fn start_quorum_loss(
        &self,
        api_version: &str,
        service_id: &str,
        partition_id: &str,
        operation_id: &str,
        quorum_loss_mode: &str,
        quorum_loss_duration: i32,
        timeout: i64,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> =
            self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query =
                ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("api-version", api_version);
            query.append_pair("OperationId", operation_id);
            query.append_pair("QuorumLossMode", quorum_loss_mode);
            query.append_pair(
                "QuorumLossDuration",
                &quorum_loss_duration.to_string(),
            );
            query.append_pair("timeout", &timeout.to_string());
            query.finish()
        };
        let uri_str = format!("{}/Faults/Services/{serviceId}/$/GetPartitions/{partitionId}/$/StartQuorumLoss?{}", configuration.base_path, query_string, serviceId=service_id, partitionId=partition_id);

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