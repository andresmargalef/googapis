// This file is @generated by prost-build.
/// Message describing ClientGateway object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientGateway {
    /// Required. name of resource. The name is ignored during creation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. \[Output only\] Create time stamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. \[Output only\] Update time stamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The operational state of the gateway.
    #[prost(enumeration = "client_gateway::State", tag = "4")]
    pub state: i32,
    /// Output only. A unique identifier for the instance generated by the system.
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
    /// Output only. The client connector service name that the client gateway is
    /// associated to. Client Connector Services, named as follows:
    ///    `projects/{project_id}/locations/{location_id}/client_connector_services/{client_connector_service_id}`.
    #[prost(string, tag = "6")]
    pub client_connector_service: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ClientGateway`.
pub mod client_gateway {
    /// Represents the different states of a gateway.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Gateway is being created.
        Creating = 1,
        /// Gateway is being updated.
        Updating = 2,
        /// Gateway is being deleted.
        Deleting = 3,
        /// Gateway is running.
        Running = 4,
        /// Gateway is down and may be restored in the future.
        /// This happens when CCFE sends ProjectState = OFF.
        Down = 5,
        /// ClientGateway encountered an error and is in indeterministic state.
        Error = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "STATE_UNSPECIFIED",
                Self::Creating => "CREATING",
                Self::Updating => "UPDATING",
                Self::Deleting => "DELETING",
                Self::Running => "RUNNING",
                Self::Down => "DOWN",
                Self::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "RUNNING" => Some(Self::Running),
                "DOWN" => Some(Self::Down),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// Message for requesting list of ClientGateways.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClientGatewaysRequest {
    /// Required. Parent value for ListClientGatewaysRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing ClientGateways.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClientGatewaysResponse {
    /// The list of ClientGateway.
    #[prost(message, repeated, tag = "1")]
    pub client_gateways: ::prost::alloc::vec::Vec<ClientGateway>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a ClientGateway.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClientGatewayRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a ClientGateway.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClientGatewayRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. User-settable client gateway resource ID.
    ///   * Must start with a letter.
    ///   * Must contain between 4-63 characters from `/[a-z][0-9]-/`.
    ///   * Must end with a number or a letter.
    #[prost(string, tag = "2")]
    pub client_gateway_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub client_gateway: ::core::option::Option<ClientGateway>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Message for deleting a ClientGateway
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClientGatewayRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientGatewayOperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have been cancelled successfully
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated server implementations.
pub mod client_gateways_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ClientGatewaysServiceServer.
    #[async_trait]
    pub trait ClientGatewaysService: std::marker::Send + std::marker::Sync + 'static {
        /// Lists ClientGateways in a given project and location.
        async fn list_client_gateways(
            &self,
            request: tonic::Request<super::ListClientGatewaysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListClientGatewaysResponse>,
            tonic::Status,
        >;
        /// Gets details of a single ClientGateway.
        async fn get_client_gateway(
            &self,
            request: tonic::Request<super::GetClientGatewayRequest>,
        ) -> std::result::Result<tonic::Response<super::ClientGateway>, tonic::Status>;
        /// Creates a new ClientGateway in a given project and location.
        async fn create_client_gateway(
            &self,
            request: tonic::Request<super::CreateClientGatewayRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        /// Deletes a single ClientGateway.
        async fn delete_client_gateway(
            &self,
            request: tonic::Request<super::DeleteClientGatewayRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    /// API Overview:
    ///
    /// The `beyondcorp.googleapis.com` service implements the Google Cloud
    /// BeyondCorp API.
    ///
    /// Data Model:
    ///
    /// The ClientGatewaysService exposes the following resources:
    ///
    /// * Client Gateways, named as follows:
    ///   `projects/{project_id}/locations/{location_id}/clientGateways/{client_gateway_id}`.
    #[derive(Debug)]
    pub struct ClientGatewaysServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ClientGatewaysServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for ClientGatewaysServiceServer<T>
    where
        T: ClientGatewaysService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/google.cloud.beyondcorp.clientgateways.v1.ClientGatewaysService/ListClientGateways" => {
                    #[allow(non_camel_case_types)]
                    struct ListClientGatewaysSvc<T: ClientGatewaysService>(pub Arc<T>);
                    impl<
                        T: ClientGatewaysService,
                    > tonic::server::UnaryService<super::ListClientGatewaysRequest>
                    for ListClientGatewaysSvc<T> {
                        type Response = super::ListClientGatewaysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListClientGatewaysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ClientGatewaysService>::list_client_gateways(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListClientGatewaysSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.beyondcorp.clientgateways.v1.ClientGatewaysService/GetClientGateway" => {
                    #[allow(non_camel_case_types)]
                    struct GetClientGatewaySvc<T: ClientGatewaysService>(pub Arc<T>);
                    impl<
                        T: ClientGatewaysService,
                    > tonic::server::UnaryService<super::GetClientGatewayRequest>
                    for GetClientGatewaySvc<T> {
                        type Response = super::ClientGateway;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetClientGatewayRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ClientGatewaysService>::get_client_gateway(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetClientGatewaySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.beyondcorp.clientgateways.v1.ClientGatewaysService/CreateClientGateway" => {
                    #[allow(non_camel_case_types)]
                    struct CreateClientGatewaySvc<T: ClientGatewaysService>(pub Arc<T>);
                    impl<
                        T: ClientGatewaysService,
                    > tonic::server::UnaryService<super::CreateClientGatewayRequest>
                    for CreateClientGatewaySvc<T> {
                        type Response = super::super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateClientGatewayRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ClientGatewaysService>::create_client_gateway(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateClientGatewaySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.beyondcorp.clientgateways.v1.ClientGatewaysService/DeleteClientGateway" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteClientGatewaySvc<T: ClientGatewaysService>(pub Arc<T>);
                    impl<
                        T: ClientGatewaysService,
                    > tonic::server::UnaryService<super::DeleteClientGatewayRequest>
                    for DeleteClientGatewaySvc<T> {
                        type Response = super::super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteClientGatewayRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ClientGatewaysService>::delete_client_gateway(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteClientGatewaySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for ClientGatewaysServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "google.cloud.beyondcorp.clientgateways.v1.ClientGatewaysService";
    impl<T> tonic::server::NamedService for ClientGatewaysServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}