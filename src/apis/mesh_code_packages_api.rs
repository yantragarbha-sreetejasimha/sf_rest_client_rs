/*
 * Service Fabric Client APIs
 *
 * Service Fabric REST Client APIs allows management of Service Fabric clusters, applications and services.
 *
 * OpenAPI spec version: 6.4.0.36
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

pub struct MeshCodePackagesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> MeshCodePackagesApiClient<C> {
    pub fn new(
        configuration: Rc<configuration::Configuration<C>>,
    ) -> MeshCodePackagesApiClient<C> {
        MeshCodePackagesApiClient { configuration }
    }
}

pub trait MeshCodePackagesApi {
    fn mesh_code_package_get_container_logs(
        &self,
        api_version: &str,
        application_resource_name: &str,
        service_resource_name: &str,
        replica_name: &str,
        code_package_name: &str,
        tail: &str,
    ) -> Box<
        dyn Future<
            Item = ::models::ContainerLogs,
            Error = Error<serde_json::Value>,
        >,
    >;
}

impl<C: hyper::client::Connect> MeshCodePackagesApi
    for MeshCodePackagesApiClient<C>
{
    fn mesh_code_package_get_container_logs(
        &self,
        api_version: &str,
        application_resource_name: &str,
        service_resource_name: &str,
        replica_name: &str,
        code_package_name: &str,
        tail: &str,
    ) -> Box<
        dyn Future<
            Item = ::models::ContainerLogs,
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
            query.append_pair("Tail", tail);
            query.finish()
        };
        let uri_str = format!("{}/Resources/Applications/{applicationResourceName}/Services/{serviceResourceName}/Replicas/{replicaName}/CodePackages/{codePackageName}/Logs?{}", configuration.base_path, query_string, applicationResourceName=application_resource_name, serviceResourceName=service_resource_name, replicaName=replica_name, codePackageName=code_package_name);

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
                    let parsed: Result<::models::ContainerLogs, _> =
                        serde_json::from_slice(&body);
                    parsed.map_err(Error::from)
                }),
        )
    }
}
