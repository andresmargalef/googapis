// This file is @generated by prost-build.
/// A request to the SnapToRoads method, requesting that a sequence of points be
/// snapped to road segments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapToRoadsRequest {
    /// The path to be snapped as a series of lat, lng points. Specified as
    /// a string of the format: lat,lng|lat,lng|...
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// Whether to interpolate the points to return full road geometry.
    #[prost(bool, tag = "2")]
    pub interpolate: bool,
    /// The asset ID of the asset to which this path relates. This is used for
    /// abuse detection purposes for clients with asset-based SKUs.
    #[prost(string, tag = "3")]
    pub asset_id: ::prost::alloc::string::String,
    /// The type of travel being tracked. This will constrain the paths we snap to.
    #[prost(enumeration = "TravelMode", tag = "4")]
    pub travel_mode: i32,
}
/// A snapped point object, representing the result of snapping.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnappedPoint {
    /// The lat,lng of the snapped location.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The index into the original path of the equivalent pre-snapped point.
    /// This allows for identification of points which have been interpolated if
    /// this index is missing.
    #[prost(message, optional, tag = "2")]
    pub original_index: ::core::option::Option<u32>,
    /// The place ID for this snapped location (road segment). These are the same
    /// as are currently used by the Places API.
    #[prost(string, tag = "3")]
    pub place_id: ::prost::alloc::string::String,
}
/// The response from the SnapToRoads method, returning a sequence of snapped
/// points.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapToRoadsResponse {
    /// A list of snapped points.
    #[prost(message, repeated, tag = "1")]
    pub snapped_points: ::prost::alloc::vec::Vec<SnappedPoint>,
    /// User-visible warning message, if any, which can be shown alongside a valid
    /// result.
    #[prost(string, tag = "2")]
    pub warning_message: ::prost::alloc::string::String,
}
/// A request to the ListNearestRoads method, requesting that a sequence of
/// points be snapped individually to the road segment that each is closest to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNearestRoadsRequest {
    /// The points to be snapped as a series of lat, lng points. Specified as
    /// a string of the format: lat,lng|lat,lng|...
    #[prost(string, tag = "1")]
    pub points: ::prost::alloc::string::String,
    /// The type of travel being tracked. This will constrain the roads we snap to.
    #[prost(enumeration = "TravelMode", tag = "2")]
    pub travel_mode: i32,
}
/// The response from the ListNearestRoads method, returning a list of snapped
/// points.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNearestRoadsResponse {
    /// A list of snapped points.
    #[prost(message, repeated, tag = "1")]
    pub snapped_points: ::prost::alloc::vec::Vec<SnappedPoint>,
}
/// An enum representing the mode of travel used for snapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TravelMode {
    Unspecified = 0,
    Driving = 1,
    Cycling = 2,
    Walking = 3,
}
impl TravelMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "TRAVEL_MODE_UNSPECIFIED",
            Self::Driving => "DRIVING",
            Self::Cycling => "CYCLING",
            Self::Walking => "WALKING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRAVEL_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "DRIVING" => Some(Self::Driving),
            "CYCLING" => Some(Self::Cycling),
            "WALKING" => Some(Self::Walking),
            _ => None,
        }
    }
}
/// Generated server implementations.
pub mod roads_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RoadsServiceServer.
    #[async_trait]
    pub trait RoadsService: std::marker::Send + std::marker::Sync + 'static {
        /// This method takes a sequence of latitude,longitude points and snaps them to
        /// the most likely road segments. Optionally returns additional points giving
        /// the full road geometry. Also returns a place ID for each snapped point.
        async fn snap_to_roads(
            &self,
            request: tonic::Request<super::SnapToRoadsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SnapToRoadsResponse>,
            tonic::Status,
        >;
        /// This method takes a list of latitude,longitude points and snaps them each
        /// to their nearest road. Also returns a place ID for each snapped point.
        async fn list_nearest_roads(
            &self,
            request: tonic::Request<super::ListNearestRoadsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNearestRoadsResponse>,
            tonic::Status,
        >;
    }
    /// The Roads API maps one or more GPS coordinates to the geometry of the road
    /// and determines the speed limit along road segments.
    #[derive(Debug)]
    pub struct RoadsServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> RoadsServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RoadsServiceServer<T>
    where
        T: RoadsService,
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
                "/google.maps.roads.v1op.RoadsService/SnapToRoads" => {
                    #[allow(non_camel_case_types)]
                    struct SnapToRoadsSvc<T: RoadsService>(pub Arc<T>);
                    impl<
                        T: RoadsService,
                    > tonic::server::UnaryService<super::SnapToRoadsRequest>
                    for SnapToRoadsSvc<T> {
                        type Response = super::SnapToRoadsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SnapToRoadsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RoadsService>::snap_to_roads(&inner, request).await
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
                        let method = SnapToRoadsSvc(inner);
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
                "/google.maps.roads.v1op.RoadsService/ListNearestRoads" => {
                    #[allow(non_camel_case_types)]
                    struct ListNearestRoadsSvc<T: RoadsService>(pub Arc<T>);
                    impl<
                        T: RoadsService,
                    > tonic::server::UnaryService<super::ListNearestRoadsRequest>
                    for ListNearestRoadsSvc<T> {
                        type Response = super::ListNearestRoadsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListNearestRoadsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RoadsService>::list_nearest_roads(&inner, request)
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
                        let method = ListNearestRoadsSvc(inner);
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
    impl<T> Clone for RoadsServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "google.maps.roads.v1op.RoadsService";
    impl<T> tonic::server::NamedService for RoadsServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}