// This file is @generated by prost-build.
/// The group information for methods in the Merchant API. The quota is shared
/// between all methods in the group. Even if none of the methods within the
/// group have usage the information for the group is returned.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaGroup {
    /// Identifier. The resource name of the quota group.
    /// Format: accounts/{account}/quotas/{group}
    /// Note: There is no guarantee on the format of {group}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The current quota usage, meaning the number of calls already
    /// made on a given day to the methods in the group. The daily quota limits
    /// reset at at 12:00 PM midday UTC.
    #[prost(int64, tag = "2")]
    pub quota_usage: i64,
    /// Output only. The maximum number of calls allowed per day for the group.
    #[prost(int64, tag = "3")]
    pub quota_limit: i64,
    /// Output only. The maximum number of calls allowed per minute for the group.
    #[prost(int64, tag = "5")]
    pub quota_minute_limit: i64,
    /// Output only. List of all methods group quota applies to.
    #[prost(message, repeated, tag = "4")]
    pub method_details: ::prost::alloc::vec::Vec<MethodDetails>,
}
/// The method details per method in the Merchant API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodDetails {
    /// Output only. The name of the method for example `products.list`.
    #[prost(string, tag = "1")]
    pub method: ::prost::alloc::string::String,
    /// Output only. The API version that the method belongs to.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// Output only. The sub-API that the method belongs to.
    #[prost(string, tag = "3")]
    pub subapi: ::prost::alloc::string::String,
    /// Output only. The path for the method such as
    /// `products/v1/productInputs.insert`
    #[prost(string, tag = "4")]
    pub path: ::prost::alloc::string::String,
}
/// Request message for the ListQuotaGroups method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQuotaGroupsRequest {
    /// Required. The merchant account who owns the collection of method quotas
    /// Format: accounts/{account}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of quotas to return in the response, used
    /// for paging. Defaults to 500; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Token (if provided) to retrieve the subsequent page. All other
    /// parameters must match the original call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the ListMethodGroups method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQuotaGroupsResponse {
    /// The methods, current quota usage and limits per each group. The quota is
    /// shared between all methods in the group. The groups are sorted in
    /// descending order based on
    /// [quotaUsage][google.shopping.merchant.quota.v1main.QuotaGroup.quota_usage].
    #[prost(message, repeated, tag = "1")]
    pub quota_groups: ::prost::alloc::vec::Vec<QuotaGroup>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated server implementations.
pub mod quota_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QuotaServiceServer.
    #[async_trait]
    pub trait QuotaService: std::marker::Send + std::marker::Sync + 'static {
        /// Lists the daily call quota and usage per group for your Merchant
        /// Center account.
        async fn list_quota_groups(
            &self,
            request: tonic::Request<super::ListQuotaGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQuotaGroupsResponse>,
            tonic::Status,
        >;
    }
    /// Service to get method call quota information per Merchant API method.
    #[derive(Debug)]
    pub struct QuotaServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> QuotaServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QuotaServiceServer<T>
    where
        T: QuotaService,
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
                "/google.shopping.merchant.quota.v1beta.QuotaService/ListQuotaGroups" => {
                    #[allow(non_camel_case_types)]
                    struct ListQuotaGroupsSvc<T: QuotaService>(pub Arc<T>);
                    impl<
                        T: QuotaService,
                    > tonic::server::UnaryService<super::ListQuotaGroupsRequest>
                    for ListQuotaGroupsSvc<T> {
                        type Response = super::ListQuotaGroupsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListQuotaGroupsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as QuotaService>::list_quota_groups(&inner, request)
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
                        let method = ListQuotaGroupsSvc(inner);
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
    impl<T> Clone for QuotaServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "google.shopping.merchant.quota.v1beta.QuotaService";
    impl<T> tonic::server::NamedService for QuotaServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}