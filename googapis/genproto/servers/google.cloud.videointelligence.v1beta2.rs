// This file is @generated by prost-build.
/// Video annotation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateVideoRequest {
    /// Input video location. Currently, only
    /// [Google Cloud Storage](<https://cloud.google.com/storage/>) URIs are
    /// supported, which must be specified in the following format:
    /// `gs://bucket-id/object-id` (other URI formats return
    /// [google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT]). For
    /// more information, see [Request
    /// URIs](<https://cloud.google.com/storage/docs/request-endpoints>). A video URI
    /// may include wildcards in `object-id`, and thus identify multiple videos.
    /// Supported wildcards: '*' to match 0 or more characters;
    /// '?' to match 1 character. If unset, the input video should be embedded
    /// in the request as `input_content`. If set, `input_content` should be unset.
    #[prost(string, tag = "1")]
    pub input_uri: ::prost::alloc::string::String,
    /// The video data bytes.
    /// If unset, the input video(s) should be specified via `input_uri`.
    /// If set, `input_uri` should be unset.
    #[prost(bytes = "vec", tag = "6")]
    pub input_content: ::prost::alloc::vec::Vec<u8>,
    /// Required. Requested video annotation features.
    #[prost(enumeration = "Feature", repeated, packed = "false", tag = "2")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    /// Additional video context and/or feature-specific parameters.
    #[prost(message, optional, tag = "3")]
    pub video_context: ::core::option::Option<VideoContext>,
    /// Optional. Location where the output (in JSON format) should be stored.
    /// Currently, only [Google Cloud Storage](<https://cloud.google.com/storage/>)
    /// URIs are supported, which must be specified in the following format:
    /// `gs://bucket-id/object-id` (other URI formats return
    /// [google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT]). For
    /// more information, see [Request
    /// URIs](<https://cloud.google.com/storage/docs/request-endpoints>).
    #[prost(string, tag = "4")]
    pub output_uri: ::prost::alloc::string::String,
    /// Optional. Cloud region where annotation should take place. Supported cloud
    /// regions: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region
    /// is specified, a region will be determined based on video file location.
    #[prost(string, tag = "5")]
    pub location_id: ::prost::alloc::string::String,
}
/// Video context and/or feature-specific parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoContext {
    /// Video segments to annotate. The segments may overlap and are not required
    /// to be contiguous or span the whole video. If unspecified, each video is
    /// treated as a single segment.
    #[prost(message, repeated, tag = "1")]
    pub segments: ::prost::alloc::vec::Vec<VideoSegment>,
    /// Config for LABEL_DETECTION.
    #[prost(message, optional, tag = "2")]
    pub label_detection_config: ::core::option::Option<LabelDetectionConfig>,
    /// Config for SHOT_CHANGE_DETECTION.
    #[prost(message, optional, tag = "3")]
    pub shot_change_detection_config: ::core::option::Option<ShotChangeDetectionConfig>,
    /// Config for EXPLICIT_CONTENT_DETECTION.
    #[prost(message, optional, tag = "4")]
    pub explicit_content_detection_config: ::core::option::Option<
        ExplicitContentDetectionConfig,
    >,
    /// Config for FACE_DETECTION.
    #[prost(message, optional, tag = "5")]
    pub face_detection_config: ::core::option::Option<FaceDetectionConfig>,
}
/// Config for LABEL_DETECTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelDetectionConfig {
    /// What labels should be detected with LABEL_DETECTION, in addition to
    /// video-level labels or segment-level labels.
    /// If unspecified, defaults to `SHOT_MODE`.
    #[prost(enumeration = "LabelDetectionMode", tag = "1")]
    pub label_detection_mode: i32,
    /// Whether the video has been shot from a stationary (i.e. non-moving) camera.
    /// When set to true, might improve detection accuracy for moving objects.
    /// Should be used with `SHOT_AND_FRAME_MODE` enabled.
    #[prost(bool, tag = "2")]
    pub stationary_camera: bool,
    /// Model to use for label detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag = "3")]
    pub model: ::prost::alloc::string::String,
}
/// Config for SHOT_CHANGE_DETECTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShotChangeDetectionConfig {
    /// Model to use for shot change detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
}
/// Config for EXPLICIT_CONTENT_DETECTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitContentDetectionConfig {
    /// Model to use for explicit content detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
}
/// Config for FACE_DETECTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceDetectionConfig {
    /// Model to use for face detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Whether bounding boxes be included in the face annotation output.
    #[prost(bool, tag = "2")]
    pub include_bounding_boxes: bool,
}
/// Video segment.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct VideoSegment {
    /// Time-offset, relative to the beginning of the video,
    /// corresponding to the start of the segment (inclusive).
    #[prost(message, optional, tag = "1")]
    pub start_time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Time-offset, relative to the beginning of the video,
    /// corresponding to the end of the segment (inclusive).
    #[prost(message, optional, tag = "2")]
    pub end_time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Video segment level annotation results for label detection.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LabelSegment {
    /// Video segment where a label was detected.
    #[prost(message, optional, tag = "1")]
    pub segment: ::core::option::Option<VideoSegment>,
    /// Confidence that the label is accurate. Range: \[0, 1\].
    #[prost(float, tag = "2")]
    pub confidence: f32,
}
/// Video frame level annotation results for label detection.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct LabelFrame {
    /// Time-offset, relative to the beginning of the video, corresponding to the
    /// video frame for this location.
    #[prost(message, optional, tag = "1")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Confidence that the label is accurate. Range: \[0, 1\].
    #[prost(float, tag = "2")]
    pub confidence: f32,
}
/// Detected entity from video analysis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// Opaque entity ID. Some IDs may be available in
    /// [Google Knowledge Graph Search
    /// API](<https://developers.google.com/knowledge-graph/>).
    #[prost(string, tag = "1")]
    pub entity_id: ::prost::alloc::string::String,
    /// Textual description, e.g. `Fixed-gear bicycle`.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Language code for `description` in BCP-47 format.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// Label annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelAnnotation {
    /// Detected entity.
    #[prost(message, optional, tag = "1")]
    pub entity: ::core::option::Option<Entity>,
    /// Common categories for the detected entity.
    /// E.g. when the label is `Terrier` the category is likely `dog`. And in some
    /// cases there might be more than one categories e.g. `Terrier` could also be
    /// a `pet`.
    #[prost(message, repeated, tag = "2")]
    pub category_entities: ::prost::alloc::vec::Vec<Entity>,
    /// All video segments where a label was detected.
    #[prost(message, repeated, tag = "3")]
    pub segments: ::prost::alloc::vec::Vec<LabelSegment>,
    /// All video frames where a label was detected.
    #[prost(message, repeated, tag = "4")]
    pub frames: ::prost::alloc::vec::Vec<LabelFrame>,
}
/// Video frame level annotation results for explicit content.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ExplicitContentFrame {
    /// Time-offset, relative to the beginning of the video, corresponding to the
    /// video frame for this location.
    #[prost(message, optional, tag = "1")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Likelihood of the pornography content..
    #[prost(enumeration = "Likelihood", tag = "2")]
    pub pornography_likelihood: i32,
}
/// Explicit content annotation (based on per-frame visual signals only).
/// If no explicit content has been detected in a frame, no annotations are
/// present for that frame.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitContentAnnotation {
    /// All video frames where explicit content was detected.
    #[prost(message, repeated, tag = "1")]
    pub frames: ::prost::alloc::vec::Vec<ExplicitContentFrame>,
}
/// Normalized bounding box.
/// The normalized vertex coordinates are relative to the original image.
/// Range: \[0, 1\].
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct NormalizedBoundingBox {
    /// Left X coordinate.
    #[prost(float, tag = "1")]
    pub left: f32,
    /// Top Y coordinate.
    #[prost(float, tag = "2")]
    pub top: f32,
    /// Right X coordinate.
    #[prost(float, tag = "3")]
    pub right: f32,
    /// Bottom Y coordinate.
    #[prost(float, tag = "4")]
    pub bottom: f32,
}
/// Video segment level annotation results for face detection.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct FaceSegment {
    /// Video segment where a face was detected.
    #[prost(message, optional, tag = "1")]
    pub segment: ::core::option::Option<VideoSegment>,
}
/// Video frame level annotation results for face detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceFrame {
    /// Normalized Bounding boxes in a frame.
    /// There can be more than one boxes if the same face is detected in multiple
    /// locations within the current frame.
    #[prost(message, repeated, tag = "1")]
    pub normalized_bounding_boxes: ::prost::alloc::vec::Vec<NormalizedBoundingBox>,
    /// Time-offset, relative to the beginning of the video,
    /// corresponding to the video frame for this location.
    #[prost(message, optional, tag = "2")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Face annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceAnnotation {
    /// Thumbnail of a representative face view (in JPEG format).
    #[prost(bytes = "vec", tag = "1")]
    pub thumbnail: ::prost::alloc::vec::Vec<u8>,
    /// All video segments where a face was detected.
    #[prost(message, repeated, tag = "2")]
    pub segments: ::prost::alloc::vec::Vec<FaceSegment>,
    /// All video frames where a face was detected.
    #[prost(message, repeated, tag = "3")]
    pub frames: ::prost::alloc::vec::Vec<FaceFrame>,
}
/// Annotation results for a single video.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoAnnotationResults {
    /// Video file location in
    /// [Google Cloud Storage](<https://cloud.google.com/storage/>).
    #[prost(string, tag = "1")]
    pub input_uri: ::prost::alloc::string::String,
    /// Label annotations on video level or user specified segment level.
    /// There is exactly one element for each unique label.
    #[prost(message, repeated, tag = "2")]
    pub segment_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Label annotations on shot level.
    /// There is exactly one element for each unique label.
    #[prost(message, repeated, tag = "3")]
    pub shot_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Label annotations on frame level.
    /// There is exactly one element for each unique label.
    #[prost(message, repeated, tag = "4")]
    pub frame_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Face annotations. There is exactly one element for each unique face.
    #[prost(message, repeated, tag = "5")]
    pub face_annotations: ::prost::alloc::vec::Vec<FaceAnnotation>,
    /// Shot annotations. Each shot is represented as a video segment.
    #[prost(message, repeated, tag = "6")]
    pub shot_annotations: ::prost::alloc::vec::Vec<VideoSegment>,
    /// Explicit content annotation.
    #[prost(message, optional, tag = "7")]
    pub explicit_annotation: ::core::option::Option<ExplicitContentAnnotation>,
    /// If set, indicates an error. Note that for a single `AnnotateVideoRequest`
    /// some videos may succeed and some may fail.
    #[prost(message, optional, tag = "9")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// Video annotation response. Included in the `response`
/// field of the `Operation` returned by the `GetOperation`
/// call of the `google::longrunning::Operations` service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateVideoResponse {
    /// Annotation results for all videos specified in `AnnotateVideoRequest`.
    #[prost(message, repeated, tag = "1")]
    pub annotation_results: ::prost::alloc::vec::Vec<VideoAnnotationResults>,
}
/// Annotation progress for a single video.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoAnnotationProgress {
    /// Video file location in
    /// [Google Cloud Storage](<https://cloud.google.com/storage/>).
    #[prost(string, tag = "1")]
    pub input_uri: ::prost::alloc::string::String,
    /// Approximate percentage processed thus far.
    /// Guaranteed to be 100 when fully processed.
    #[prost(int32, tag = "2")]
    pub progress_percent: i32,
    /// Time when the request was received.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time of the most recent update.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Video annotation progress. Included in the `metadata`
/// field of the `Operation` returned by the `GetOperation`
/// call of the `google::longrunning::Operations` service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateVideoProgress {
    /// Progress metadata for all videos specified in `AnnotateVideoRequest`.
    #[prost(message, repeated, tag = "1")]
    pub annotation_progress: ::prost::alloc::vec::Vec<VideoAnnotationProgress>,
}
/// Video annotation feature.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Feature {
    /// Unspecified.
    Unspecified = 0,
    /// Label detection. Detect objects, such as dog or flower.
    LabelDetection = 1,
    /// Shot change detection.
    ShotChangeDetection = 2,
    /// Explicit content detection.
    ExplicitContentDetection = 3,
    /// Human face detection and tracking.
    FaceDetection = 4,
}
impl Feature {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "FEATURE_UNSPECIFIED",
            Self::LabelDetection => "LABEL_DETECTION",
            Self::ShotChangeDetection => "SHOT_CHANGE_DETECTION",
            Self::ExplicitContentDetection => "EXPLICIT_CONTENT_DETECTION",
            Self::FaceDetection => "FACE_DETECTION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FEATURE_UNSPECIFIED" => Some(Self::Unspecified),
            "LABEL_DETECTION" => Some(Self::LabelDetection),
            "SHOT_CHANGE_DETECTION" => Some(Self::ShotChangeDetection),
            "EXPLICIT_CONTENT_DETECTION" => Some(Self::ExplicitContentDetection),
            "FACE_DETECTION" => Some(Self::FaceDetection),
            _ => None,
        }
    }
}
/// Label detection mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LabelDetectionMode {
    /// Unspecified.
    Unspecified = 0,
    /// Detect shot-level labels.
    ShotMode = 1,
    /// Detect frame-level labels.
    FrameMode = 2,
    /// Detect both shot-level and frame-level labels.
    ShotAndFrameMode = 3,
}
impl LabelDetectionMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "LABEL_DETECTION_MODE_UNSPECIFIED",
            Self::ShotMode => "SHOT_MODE",
            Self::FrameMode => "FRAME_MODE",
            Self::ShotAndFrameMode => "SHOT_AND_FRAME_MODE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LABEL_DETECTION_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "SHOT_MODE" => Some(Self::ShotMode),
            "FRAME_MODE" => Some(Self::FrameMode),
            "SHOT_AND_FRAME_MODE" => Some(Self::ShotAndFrameMode),
            _ => None,
        }
    }
}
/// Bucketized representation of likelihood.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Likelihood {
    /// Unspecified likelihood.
    Unspecified = 0,
    /// Very unlikely.
    VeryUnlikely = 1,
    /// Unlikely.
    Unlikely = 2,
    /// Possible.
    Possible = 3,
    /// Likely.
    Likely = 4,
    /// Very likely.
    VeryLikely = 5,
}
impl Likelihood {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "LIKELIHOOD_UNSPECIFIED",
            Self::VeryUnlikely => "VERY_UNLIKELY",
            Self::Unlikely => "UNLIKELY",
            Self::Possible => "POSSIBLE",
            Self::Likely => "LIKELY",
            Self::VeryLikely => "VERY_LIKELY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LIKELIHOOD_UNSPECIFIED" => Some(Self::Unspecified),
            "VERY_UNLIKELY" => Some(Self::VeryUnlikely),
            "UNLIKELY" => Some(Self::Unlikely),
            "POSSIBLE" => Some(Self::Possible),
            "LIKELY" => Some(Self::Likely),
            "VERY_LIKELY" => Some(Self::VeryLikely),
            _ => None,
        }
    }
}
/// Generated server implementations.
pub mod video_intelligence_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VideoIntelligenceServiceServer.
    #[async_trait]
    pub trait VideoIntelligenceService: std::marker::Send + std::marker::Sync + 'static {
        /// Performs asynchronous video annotation. Progress and results can be
        /// retrieved through the `google.longrunning.Operations` interface.
        /// `Operation.metadata` contains `AnnotateVideoProgress` (progress).
        /// `Operation.response` contains `AnnotateVideoResponse` (results).
        async fn annotate_video(
            &self,
            request: tonic::Request<super::AnnotateVideoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    /// Service that implements Google Cloud Video Intelligence API.
    #[derive(Debug)]
    pub struct VideoIntelligenceServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> VideoIntelligenceServiceServer<T> {
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
    for VideoIntelligenceServiceServer<T>
    where
        T: VideoIntelligenceService,
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
                "/google.cloud.videointelligence.v1beta2.VideoIntelligenceService/AnnotateVideo" => {
                    #[allow(non_camel_case_types)]
                    struct AnnotateVideoSvc<T: VideoIntelligenceService>(pub Arc<T>);
                    impl<
                        T: VideoIntelligenceService,
                    > tonic::server::UnaryService<super::AnnotateVideoRequest>
                    for AnnotateVideoSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AnnotateVideoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as VideoIntelligenceService>::annotate_video(
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
                        let method = AnnotateVideoSvc(inner);
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
    impl<T> Clone for VideoIntelligenceServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "google.cloud.videointelligence.v1beta2.VideoIntelligenceService";
    impl<T> tonic::server::NamedService for VideoIntelligenceServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}