// This file is @generated by prost-build.
/// Metadata common to all Datastore Admin operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonMetadata {
    /// The time that work began on the operation.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation ended, either successfully or otherwise.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The type of the operation. Can be used as a filter in
    /// ListOperationsRequest.
    #[prost(enumeration = "OperationType", tag = "3")]
    pub operation_type: i32,
    /// The client-assigned labels which were provided when the operation was
    /// created. May also include additional labels.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The current state of the Operation.
    #[prost(enumeration = "common_metadata::State", tag = "5")]
    pub state: i32,
}
/// Nested message and enum types in `CommonMetadata`.
pub mod common_metadata {
    /// The various possible states for an ongoing Operation.
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
        /// Unspecified.
        Unspecified = 0,
        /// Request is being prepared for processing.
        Initializing = 1,
        /// Request is actively being processed.
        Processing = 2,
        /// Request is in the process of being cancelled after user called
        /// google.longrunning.Operations.CancelOperation on the operation.
        Cancelling = 3,
        /// Request has been processed and is in its finalization stage.
        Finalizing = 4,
        /// Request has completed successfully.
        Successful = 5,
        /// Request has finished being processed, but encountered an error.
        Failed = 6,
        /// Request has finished being cancelled after user called
        /// google.longrunning.Operations.CancelOperation.
        Cancelled = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "STATE_UNSPECIFIED",
                Self::Initializing => "INITIALIZING",
                Self::Processing => "PROCESSING",
                Self::Cancelling => "CANCELLING",
                Self::Finalizing => "FINALIZING",
                Self::Successful => "SUCCESSFUL",
                Self::Failed => "FAILED",
                Self::Cancelled => "CANCELLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "INITIALIZING" => Some(Self::Initializing),
                "PROCESSING" => Some(Self::Processing),
                "CANCELLING" => Some(Self::Cancelling),
                "FINALIZING" => Some(Self::Finalizing),
                "SUCCESSFUL" => Some(Self::Successful),
                "FAILED" => Some(Self::Failed),
                "CANCELLED" => Some(Self::Cancelled),
                _ => None,
            }
        }
    }
}
/// Measures the progress of a particular metric.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Progress {
    /// The amount of work that has been completed. Note that this may be greater
    /// than work_estimated.
    #[prost(int64, tag = "1")]
    pub work_completed: i64,
    /// An estimate of how much work needs to be performed. May be zero if the
    /// work estimate is unavailable.
    #[prost(int64, tag = "2")]
    pub work_estimated: i64,
}
/// The request for
/// [google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities][google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportEntitiesRequest {
    /// Project ID against which to make the request.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Client-assigned labels.
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Description of what data from the project is included in the export.
    #[prost(message, optional, tag = "3")]
    pub entity_filter: ::core::option::Option<EntityFilter>,
    /// Location for the export metadata and data files.
    ///
    /// The full resource URL of the external storage location. Currently, only
    /// Google Cloud Storage is supported. So output_url_prefix should be of the
    /// form: `gs://BUCKET_NAME\[/NAMESPACE_PATH\]`, where `BUCKET_NAME` is the
    /// name of the Cloud Storage bucket and `NAMESPACE_PATH` is an optional Cloud
    /// Storage namespace path (this is not a Cloud Datastore namespace). For more
    /// information about Cloud Storage namespace paths, see
    /// [Object name
    /// considerations](<https://cloud.google.com/storage/docs/naming#object-considerations>).
    ///
    /// The resulting files will be nested deeper than the specified URL prefix.
    /// The final output URL will be provided in the
    /// [google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url][google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url]
    /// field. That value should be used for subsequent ImportEntities operations.
    ///
    /// By nesting the data files deeper, the same Cloud Storage bucket can be used
    /// in multiple ExportEntities operations without conflict.
    #[prost(string, tag = "4")]
    pub output_url_prefix: ::prost::alloc::string::String,
}
/// The request for
/// [google.datastore.admin.v1beta1.DatastoreAdmin.ImportEntities][google.datastore.admin.v1beta1.DatastoreAdmin.ImportEntities].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportEntitiesRequest {
    /// Project ID against which to make the request.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Client-assigned labels.
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The full resource URL of the external storage location. Currently, only
    /// Google Cloud Storage is supported. So input_url should be of the form:
    /// `gs://BUCKET_NAME\[/NAMESPACE_PATH\]/OVERALL_EXPORT_METADATA_FILE`, where
    /// `BUCKET_NAME` is the name of the Cloud Storage bucket, `NAMESPACE_PATH` is
    /// an optional Cloud Storage namespace path (this is not a Cloud Datastore
    /// namespace), and `OVERALL_EXPORT_METADATA_FILE` is the metadata file written
    /// by the ExportEntities operation. For more information about Cloud Storage
    /// namespace paths, see
    /// [Object name
    /// considerations](<https://cloud.google.com/storage/docs/naming#object-considerations>).
    ///
    /// For more information, see
    /// [google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url][google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url].
    #[prost(string, tag = "3")]
    pub input_url: ::prost::alloc::string::String,
    /// Optionally specify which kinds/namespaces are to be imported. If provided,
    /// the list must be a subset of the EntityFilter used in creating the export,
    /// otherwise a FAILED_PRECONDITION error will be returned. If no filter is
    /// specified then all entities from the export are imported.
    #[prost(message, optional, tag = "4")]
    pub entity_filter: ::core::option::Option<EntityFilter>,
}
/// The response for
/// [google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities][google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportEntitiesResponse {
    /// Location of the output metadata file. This can be used to begin an import
    /// into Cloud Datastore (this project or another project). See
    /// [google.datastore.admin.v1beta1.ImportEntitiesRequest.input_url][google.datastore.admin.v1beta1.ImportEntitiesRequest.input_url].
    /// Only present if the operation completed successfully.
    #[prost(string, tag = "1")]
    pub output_url: ::prost::alloc::string::String,
}
/// Metadata for ExportEntities operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportEntitiesMetadata {
    /// Metadata common to all Datastore Admin operations.
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonMetadata>,
    /// An estimate of the number of entities processed.
    #[prost(message, optional, tag = "2")]
    pub progress_entities: ::core::option::Option<Progress>,
    /// An estimate of the number of bytes processed.
    #[prost(message, optional, tag = "3")]
    pub progress_bytes: ::core::option::Option<Progress>,
    /// Description of which entities are being exported.
    #[prost(message, optional, tag = "4")]
    pub entity_filter: ::core::option::Option<EntityFilter>,
    /// Location for the export metadata and data files. This will be the same
    /// value as the
    /// [google.datastore.admin.v1beta1.ExportEntitiesRequest.output_url_prefix][google.datastore.admin.v1beta1.ExportEntitiesRequest.output_url_prefix]
    /// field. The final output location is provided in
    /// [google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url][google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url].
    #[prost(string, tag = "5")]
    pub output_url_prefix: ::prost::alloc::string::String,
}
/// Metadata for ImportEntities operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportEntitiesMetadata {
    /// Metadata common to all Datastore Admin operations.
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonMetadata>,
    /// An estimate of the number of entities processed.
    #[prost(message, optional, tag = "2")]
    pub progress_entities: ::core::option::Option<Progress>,
    /// An estimate of the number of bytes processed.
    #[prost(message, optional, tag = "3")]
    pub progress_bytes: ::core::option::Option<Progress>,
    /// Description of which entities are being imported.
    #[prost(message, optional, tag = "4")]
    pub entity_filter: ::core::option::Option<EntityFilter>,
    /// The location of the import metadata file. This will be the same value as
    /// the
    /// [google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url][google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url]
    /// field.
    #[prost(string, tag = "5")]
    pub input_url: ::prost::alloc::string::String,
}
/// Identifies a subset of entities in a project. This is specified as
/// combinations of kinds and namespaces (either or both of which may be all, as
/// described in the following examples).
/// Example usage:
///
/// Entire project:
///    kinds=\[\], namespace_ids=\[\]
///
/// Kinds Foo and Bar in all namespaces:
///    kinds=\['Foo', 'Bar'\], namespace_ids=\[\]
///
/// Kinds Foo and Bar only in the default namespace:
///    kinds=\['Foo', 'Bar'\], namespace_ids=\[''\]
///
/// Kinds Foo and Bar in both the default and Baz namespaces:
///    kinds=\['Foo', 'Bar'\], namespace_ids=\['', 'Baz'\]
///
/// The entire Baz namespace:
///    kinds=\[\], namespace_ids=\['Baz'\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityFilter {
    /// If empty, then this represents all kinds.
    #[prost(string, repeated, tag = "1")]
    pub kinds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An empty list represents all namespaces. This is the preferred
    /// usage for projects that don't use namespaces.
    ///
    /// An empty string element represents the default namespace. This should be
    /// used if the project has data in non-default namespaces, but doesn't want to
    /// include them.
    /// Each namespace in this list must be unique.
    #[prost(string, repeated, tag = "2")]
    pub namespace_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Operation types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationType {
    /// Unspecified.
    Unspecified = 0,
    /// ExportEntities.
    ExportEntities = 1,
    /// ImportEntities.
    ImportEntities = 2,
}
impl OperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "OPERATION_TYPE_UNSPECIFIED",
            Self::ExportEntities => "EXPORT_ENTITIES",
            Self::ImportEntities => "IMPORT_ENTITIES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "EXPORT_ENTITIES" => Some(Self::ExportEntities),
            "IMPORT_ENTITIES" => Some(Self::ImportEntities),
            _ => None,
        }
    }
}
/// Generated server implementations.
pub mod datastore_admin_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DatastoreAdminServer.
    #[async_trait]
    pub trait DatastoreAdmin: std::marker::Send + std::marker::Sync + 'static {
        /// Exports a copy of all or a subset of entities from Google Cloud Datastore
        /// to another storage system, such as Google Cloud Storage. Recent updates to
        /// entities may not be reflected in the export. The export occurs in the
        /// background and its progress can be monitored and managed via the
        /// Operation resource that is created. The output of an export may only be
        /// used once the associated operation is done. If an export operation is
        /// cancelled before completion it may leave partial data behind in Google
        /// Cloud Storage.
        async fn export_entities(
            &self,
            request: tonic::Request<super::ExportEntitiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        /// Imports entities into Google Cloud Datastore. Existing entities with the
        /// same key are overwritten. The import occurs in the background and its
        /// progress can be monitored and managed via the Operation resource that is
        /// created. If an ImportEntities operation is cancelled, it is possible
        /// that a subset of the data has already been imported to Cloud Datastore.
        async fn import_entities(
            &self,
            request: tonic::Request<super::ImportEntitiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    /// Google Cloud Datastore Admin API
    ///
    /// The Datastore Admin API provides several admin services for Cloud Datastore.
    ///
    /// -----------------------------------------------------------------------------
    /// ## Concepts
    ///
    /// Project, namespace, kind, and entity as defined in the Google Cloud Datastore
    /// API.
    ///
    /// Operation: An Operation represents work being performed in the background.
    ///
    /// EntityFilter: Allows specifying a subset of entities in a project. This is
    /// specified as a combination of kinds and namespaces (either or both of which
    /// may be all).
    ///
    /// -----------------------------------------------------------------------------
    /// ## Services
    ///
    /// # Export/Import
    ///
    /// The Export/Import service provides the ability to copy all or a subset of
    /// entities to/from Google Cloud Storage.
    ///
    /// Exported data may be imported into Cloud Datastore for any Google Cloud
    /// Platform project. It is not restricted to the export source project. It is
    /// possible to export from one project and then import into another.
    ///
    /// Exported data can also be loaded into Google BigQuery for analysis.
    ///
    /// Exports and imports are performed asynchronously. An Operation resource is
    /// created for each export/import. The state (including any errors encountered)
    /// of the export/import may be queried via the Operation resource.
    ///
    /// # Operation
    ///
    /// The Operations collection provides a record of actions performed for the
    /// specified project (including any operations in progress). Operations are not
    /// created directly but through calls on other collections or resources.
    ///
    /// An operation that is not yet done may be cancelled. The request to cancel is
    /// asynchronous and the operation may continue to run for some time after the
    /// request to cancel is made.
    ///
    /// An operation that is done may be deleted so that it is no longer listed as
    /// part of the Operation collection.
    ///
    /// ListOperations returns all pending operations, but not completed operations.
    ///
    /// Operations are created by service DatastoreAdmin,
    /// but are accessed via service google.longrunning.Operations.
    #[derive(Debug)]
    pub struct DatastoreAdminServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> DatastoreAdminServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DatastoreAdminServer<T>
    where
        T: DatastoreAdmin,
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
                "/google.datastore.admin.v1beta1.DatastoreAdmin/ExportEntities" => {
                    #[allow(non_camel_case_types)]
                    struct ExportEntitiesSvc<T: DatastoreAdmin>(pub Arc<T>);
                    impl<
                        T: DatastoreAdmin,
                    > tonic::server::UnaryService<super::ExportEntitiesRequest>
                    for ExportEntitiesSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportEntitiesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DatastoreAdmin>::export_entities(&inner, request)
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
                        let method = ExportEntitiesSvc(inner);
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
                "/google.datastore.admin.v1beta1.DatastoreAdmin/ImportEntities" => {
                    #[allow(non_camel_case_types)]
                    struct ImportEntitiesSvc<T: DatastoreAdmin>(pub Arc<T>);
                    impl<
                        T: DatastoreAdmin,
                    > tonic::server::UnaryService<super::ImportEntitiesRequest>
                    for ImportEntitiesSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportEntitiesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DatastoreAdmin>::import_entities(&inner, request)
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
                        let method = ImportEntitiesSvc(inner);
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
    impl<T> Clone for DatastoreAdminServer<T> {
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
    pub const SERVICE_NAME: &str = "google.datastore.admin.v1beta1.DatastoreAdmin";
    impl<T> tonic::server::NamedService for DatastoreAdminServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}