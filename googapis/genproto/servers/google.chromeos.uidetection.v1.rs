// This file is @generated by prost-build.
/// Request message for UI detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UiDetectionRequest {
    /// Required. Required field that represents a PNG image.
    #[prost(bytes = "vec", tag = "1")]
    pub image_png: ::prost::alloc::vec::Vec<u8>,
    /// Required. Required field that indicates the detection type.
    #[prost(message, optional, tag = "2")]
    pub request: ::core::option::Option<DetectionRequest>,
    /// Indicates whether to fall back to resizing the image if no elements are
    /// detected.
    #[prost(bool, optional, tag = "3")]
    pub resize_image: ::core::option::Option<bool>,
    /// Deprecated as of 2023-03-29. Use test_metadata instead.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub test_id: ::prost::alloc::string::String,
    /// Optional. Metadata about the client for analytics.
    #[prost(message, optional, tag = "5")]
    pub test_metadata: ::core::option::Option<TestMetadata>,
    /// Optional. Indicates whether to always start by resizing the image.
    #[prost(bool, tag = "6")]
    pub force_image_resizing: bool,
    /// Optional. Indicates whether to respond with the transformed image png.
    #[prost(bool, tag = "7")]
    pub return_transformed_image: bool,
}
/// Detection type specifies what to detect in the image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectionRequest {
    #[prost(oneof = "detection_request::DetectionRequestType", tags = "1, 2, 3")]
    pub detection_request_type: ::core::option::Option<
        detection_request::DetectionRequestType,
    >,
}
/// Nested message and enum types in `DetectionRequest`.
pub mod detection_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DetectionRequestType {
        /// Detection type for word detection.
        #[prost(message, tag = "1")]
        WordDetectionRequest(super::WordDetectionRequest),
        /// Detection type for text block detection.
        #[prost(message, tag = "2")]
        TextBlockDetectionRequest(super::TextBlockDetectionRequest),
        /// Detection type for custom icon detection.
        #[prost(message, tag = "3")]
        CustomIconDetectionRequest(super::CustomIconDetectionRequest),
    }
}
/// Metadata about the client test and test device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestMetadata {
    /// Name of the calling test. For example, 'tast.uidetection.BasicDetections'.
    #[prost(string, tag = "1")]
    pub test_id: ::prost::alloc::string::String,
    /// Board name of the ChromeOS device under test. For example, 'volteer'.
    #[prost(string, tag = "2")]
    pub board: ::prost::alloc::string::String,
    /// Model name of the ChromeOS device under test. For example, 'volet'.
    #[prost(string, tag = "3")]
    pub model: ::prost::alloc::string::String,
    /// ChromeOS build of the device under test.
    /// For example, 'volteer-release/R110-15275.0.0-75031-8794956681263330561'.
    #[prost(string, tag = "4")]
    pub cros_build: ::prost::alloc::string::String,
}
/// Detection type for word detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WordDetectionRequest {
    /// Required. The word to locate in the image.
    #[prost(string, tag = "1")]
    pub word: ::prost::alloc::string::String,
    /// Indicating whether the query string is a regex or not.
    #[prost(bool, tag = "2")]
    pub regex_mode: bool,
    /// Indicating whether the detection is an approximate match.
    #[prost(bool, tag = "3")]
    pub disable_approx_match: bool,
    /// Levenshtein distance threshold.
    /// Applicable only if regex_mode is False.
    #[prost(int32, optional, tag = "4")]
    pub max_edit_distance: ::core::option::Option<i32>,
}
/// Detection type for text block detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextBlockDetectionRequest {
    /// Required. The text block consisting a list of words to locate in the image.
    #[prost(string, repeated, tag = "1")]
    pub words: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Indicating whether the query string is a regex or not.
    #[prost(bool, tag = "2")]
    pub regex_mode: bool,
    /// Indicating whether the detection is an approximate match.
    #[prost(bool, tag = "3")]
    pub disable_approx_match: bool,
    /// Levenshtein distance threshold.
    /// Applicable only if regex_mode is False.
    #[prost(int32, optional, tag = "4")]
    pub max_edit_distance: ::core::option::Option<i32>,
    /// Indicating whether the detection result should only contain the specified
    /// words.
    #[prost(bool, tag = "5")]
    pub specified_words_only: bool,
}
/// Detection type for custom icon detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomIconDetectionRequest {
    /// Required. Required field that represents an icon in PNG format.
    #[prost(bytes = "vec", tag = "1")]
    pub icon_png: ::prost::alloc::vec::Vec<u8>,
    /// Set match_count to -1 to not limit the number of matches.
    #[prost(int32, tag = "2")]
    pub match_count: i32,
    /// Confidence threshold in the range \[0.0, 1.0\] below which the matches will
    /// be considered as non-existent.
    #[prost(double, tag = "3")]
    pub min_confidence_threshold: f64,
}
/// Response message for UI detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UiDetectionResponse {
    /// Locations of matching UI elements.
    #[prost(message, repeated, tag = "1")]
    pub bounding_boxes: ::prost::alloc::vec::Vec<BoundingBox>,
    /// The transformed detection image PNG, if requested and transformations were
    /// applied.
    #[prost(bytes = "vec", tag = "2")]
    pub transformed_image_png: ::prost::alloc::vec::Vec<u8>,
    /// The amount the original image was scaled by to make the transformed image.
    /// 1.0 if the detection result is not based on a resized image.
    #[prost(float, tag = "3")]
    pub resizing_scale_factor: f32,
}
/// The location of a UI element.
/// A bounding box is reprensented by its top-left point \[left, top\]
/// and its bottom-right point \[right, bottom\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingBox {
    /// The text found in the bounding box.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// The y-coordinate of the top-left point.
    #[prost(int32, tag = "2")]
    pub top: i32,
    /// The x-coordinate of the top-left point.
    #[prost(int32, tag = "3")]
    pub left: i32,
    /// The y-coordinate of the bottom-right point.
    #[prost(int32, tag = "4")]
    pub bottom: i32,
    /// The x-coordinate of the bottom-right point.
    #[prost(int32, tag = "5")]
    pub right: i32,
}
/// Generated server implementations.
pub mod ui_detection_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UiDetectionServiceServer.
    #[async_trait]
    pub trait UiDetectionService: std::marker::Send + std::marker::Sync + 'static {
        /// Runs the detection.
        async fn execute_detection(
            &self,
            request: tonic::Request<super::UiDetectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UiDetectionResponse>,
            tonic::Status,
        >;
    }
    /// Provides image-based UI detection service.
    #[derive(Debug)]
    pub struct UiDetectionServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> UiDetectionServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UiDetectionServiceServer<T>
    where
        T: UiDetectionService,
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
                "/google.chromeos.uidetection.v1.UiDetectionService/ExecuteDetection" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteDetectionSvc<T: UiDetectionService>(pub Arc<T>);
                    impl<
                        T: UiDetectionService,
                    > tonic::server::UnaryService<super::UiDetectionRequest>
                    for ExecuteDetectionSvc<T> {
                        type Response = super::UiDetectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UiDetectionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UiDetectionService>::execute_detection(
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
                        let method = ExecuteDetectionSvc(inner);
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
    impl<T> Clone for UiDetectionServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "google.chromeos.uidetection.v1.UiDetectionService";
    impl<T> tonic::server::NamedService for UiDetectionServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}