// This file is @generated by prost-build.
/// The request for
/// [ConnectionService.CreateConnection][google.cloud.bigquery.connection.v1.ConnectionService.CreateConnection].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectionRequest {
    /// Required. Parent resource name.
    /// Must be in the format `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Connection id that should be assigned to the created connection.
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    /// Required. Connection to create.
    #[prost(message, optional, tag = "3")]
    pub connection: ::core::option::Option<Connection>,
}
/// The request for
/// [ConnectionService.GetConnection][google.cloud.bigquery.connection.v1.ConnectionService.GetConnection].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectionRequest {
    /// Required. Name of the requested connection, for example:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// [ConnectionService.ListConnections][google.cloud.bigquery.connection.v1.ConnectionService.ListConnections].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsRequest {
    /// Required. Parent resource name.
    /// Must be in the form: `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Page size.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for
/// [ConnectionService.ListConnections][google.cloud.bigquery.connection.v1.ConnectionService.ListConnections].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsResponse {
    /// Next page token.
    #[prost(string, tag = "1")]
    pub next_page_token: ::prost::alloc::string::String,
    /// List of connections.
    #[prost(message, repeated, tag = "2")]
    pub connections: ::prost::alloc::vec::Vec<Connection>,
}
/// The request for
/// [ConnectionService.UpdateConnection][google.cloud.bigquery.connection.v1.ConnectionService.UpdateConnection].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConnectionRequest {
    /// Required. Name of the connection to update, for example:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Connection containing the updated fields.
    #[prost(message, optional, tag = "2")]
    pub connection: ::core::option::Option<Connection>,
    /// Required. Update mask for the connection fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request for [ConnectionService.DeleteConnectionRequest][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectionRequest {
    /// Required. Name of the deleted connection, for example:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Configuration parameters to establish connection with an external data
/// source, except the credential attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    /// The resource name of the connection in the form of:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User provided display name for the connection.
    #[prost(string, tag = "2")]
    pub friendly_name: ::prost::alloc::string::String,
    /// User provided description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The creation timestamp of the connection.
    #[prost(int64, tag = "5")]
    pub creation_time: i64,
    /// Output only. The last update timestamp of the connection.
    #[prost(int64, tag = "6")]
    pub last_modified_time: i64,
    /// Output only. True, if credential is configured for this connection.
    #[prost(bool, tag = "7")]
    pub has_credential: bool,
    /// Properties specific to the underlying data source.
    #[prost(oneof = "connection::Properties", tags = "4, 8, 11, 21, 22, 23, 24")]
    pub properties: ::core::option::Option<connection::Properties>,
}
/// Nested message and enum types in `Connection`.
pub mod connection {
    /// Properties specific to the underlying data source.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Properties {
        /// Cloud SQL properties.
        #[prost(message, tag = "4")]
        CloudSql(super::CloudSqlProperties),
        /// Amazon Web Services (AWS) properties.
        #[prost(message, tag = "8")]
        Aws(super::AwsProperties),
        /// Azure properties.
        #[prost(message, tag = "11")]
        Azure(super::AzureProperties),
        /// Cloud Spanner properties.
        #[prost(message, tag = "21")]
        CloudSpanner(super::CloudSpannerProperties),
        /// Cloud Resource properties.
        #[prost(message, tag = "22")]
        CloudResource(super::CloudResourceProperties),
        /// Spark properties.
        #[prost(message, tag = "23")]
        Spark(super::SparkProperties),
        /// Optional. Salesforce DataCloud properties. This field is intended for
        /// use only by Salesforce partner projects. This field contains properties
        /// for your Salesforce DataCloud connection.
        #[prost(message, tag = "24")]
        SalesforceDataCloud(super::SalesforceDataCloudProperties),
    }
}
/// Connection properties specific to the Cloud SQL.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlProperties {
    /// Cloud SQL instance ID in the form `project:location:instance`.
    #[prost(string, tag = "1")]
    pub instance_id: ::prost::alloc::string::String,
    /// Database name.
    #[prost(string, tag = "2")]
    pub database: ::prost::alloc::string::String,
    /// Type of the Cloud SQL database.
    #[prost(enumeration = "cloud_sql_properties::DatabaseType", tag = "3")]
    pub r#type: i32,
    /// Input only. Cloud SQL credential.
    #[prost(message, optional, tag = "4")]
    pub credential: ::core::option::Option<CloudSqlCredential>,
    /// Output only. The account ID of the service used for the purpose of this
    /// connection.
    ///
    /// When the connection is used in the context of an operation in
    /// BigQuery, this service account will serve as the identity being used for
    /// connecting to the CloudSQL instance specified in this connection.
    #[prost(string, tag = "5")]
    pub service_account_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CloudSqlProperties`.
pub mod cloud_sql_properties {
    /// Supported Cloud SQL database types.
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
    pub enum DatabaseType {
        /// Unspecified database type.
        Unspecified = 0,
        /// Cloud SQL for PostgreSQL.
        Postgres = 1,
        /// Cloud SQL for MySQL.
        Mysql = 2,
    }
    impl DatabaseType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "DATABASE_TYPE_UNSPECIFIED",
                Self::Postgres => "POSTGRES",
                Self::Mysql => "MYSQL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATABASE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "POSTGRES" => Some(Self::Postgres),
                "MYSQL" => Some(Self::Mysql),
                _ => None,
            }
        }
    }
}
/// Credential info for the Cloud SQL.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlCredential {
    /// The username for the credential.
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    /// The password for the credential.
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
/// Connection properties specific to Cloud Spanner.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSpannerProperties {
    /// Cloud Spanner database in the form `project/instance/database'
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// If parallelism should be used when reading from Cloud Spanner
    #[prost(bool, tag = "2")]
    pub use_parallelism: bool,
    /// Allows setting max parallelism per query when executing on Spanner
    /// independent compute resources. If unspecified, default values of
    /// parallelism are chosen that are dependent on the Cloud Spanner instance
    /// configuration.
    ///
    /// REQUIRES: `use_parallelism` must be set.
    /// REQUIRES: Either `use_data_boost` or `use_serverless_analytics` must be
    /// set.
    #[prost(int32, tag = "5")]
    pub max_parallelism: i32,
    /// If the serverless analytics service should be used to read data from Cloud
    /// Spanner.
    /// Note: `use_parallelism` must be set when using serverless analytics.
    #[prost(bool, tag = "3")]
    pub use_serverless_analytics: bool,
    /// If set, the request will be executed via Spanner independent compute
    /// resources.
    /// REQUIRES: `use_parallelism` must be set.
    ///
    /// NOTE: `use_serverless_analytics` will be deprecated. Prefer
    /// `use_data_boost` over `use_serverless_analytics`.
    #[prost(bool, tag = "6")]
    pub use_data_boost: bool,
    /// Optional. Cloud Spanner database role for fine-grained access control.
    /// The Cloud Spanner admin should have provisioned the database role with
    /// appropriate permissions, such as `SELECT` and `INSERT`. Other users should
    /// only use roles provided by their Cloud Spanner admins.
    ///
    /// For more details, see \[About fine-grained access control\]
    /// (<https://cloud.google.com/spanner/docs/fgac-about>).
    ///
    /// REQUIRES: The database role name must start with a letter, and can only
    /// contain letters, numbers, and underscores.
    #[prost(string, tag = "4")]
    pub database_role: ::prost::alloc::string::String,
}
/// Connection properties specific to Amazon Web Services (AWS).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsProperties {
    /// Authentication method chosen at connection creation.
    #[prost(oneof = "aws_properties::AuthenticationMethod", tags = "2, 3")]
    pub authentication_method: ::core::option::Option<
        aws_properties::AuthenticationMethod,
    >,
}
/// Nested message and enum types in `AwsProperties`.
pub mod aws_properties {
    /// Authentication method chosen at connection creation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AuthenticationMethod {
        /// Authentication using Google owned AWS IAM user's access key to assume
        /// into customer's AWS IAM Role.
        /// Deprecated, do not use.
        #[prost(message, tag = "2")]
        CrossAccountRole(super::AwsCrossAccountRole),
        /// Authentication using Google owned service account to assume into
        /// customer's AWS IAM Role.
        #[prost(message, tag = "3")]
        AccessRole(super::AwsAccessRole),
    }
}
/// Authentication method for Amazon Web Services (AWS) that uses Google owned
/// AWS IAM user's access key to assume into customer's AWS IAM Role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsCrossAccountRole {
    /// The user’s AWS IAM Role that trusts the Google-owned AWS IAM user
    /// Connection.
    #[prost(string, tag = "1")]
    pub iam_role_id: ::prost::alloc::string::String,
    /// Output only. Google-owned AWS IAM User for a Connection.
    #[prost(string, tag = "2")]
    pub iam_user_id: ::prost::alloc::string::String,
    /// Output only. A Google-generated id for representing Connection’s identity
    /// in AWS. External Id is also used for preventing the Confused Deputy
    /// Problem. See
    /// <https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_create_for-user_externalid.html>
    #[prost(string, tag = "3")]
    pub external_id: ::prost::alloc::string::String,
}
/// Authentication method for Amazon Web Services (AWS) that uses Google owned
/// Google service account to assume into customer's AWS IAM Role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsAccessRole {
    /// The user’s AWS IAM Role that trusts the Google-owned AWS IAM user
    /// Connection.
    #[prost(string, tag = "1")]
    pub iam_role_id: ::prost::alloc::string::String,
    /// A unique Google-owned and Google-generated identity for the Connection.
    /// This identity will be used to access the user's AWS IAM Role.
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
}
/// Container for connection properties specific to Azure.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureProperties {
    /// Output only. The name of the Azure Active Directory Application.
    #[prost(string, tag = "1")]
    pub application: ::prost::alloc::string::String,
    /// Output only. The client id of the Azure Active Directory Application.
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
    /// Output only. The object id of the Azure Active Directory Application.
    #[prost(string, tag = "3")]
    pub object_id: ::prost::alloc::string::String,
    /// The id of customer's directory that host the data.
    #[prost(string, tag = "4")]
    pub customer_tenant_id: ::prost::alloc::string::String,
    /// The URL user will be redirected to after granting consent during connection
    /// setup.
    #[prost(string, tag = "5")]
    pub redirect_uri: ::prost::alloc::string::String,
    /// The client ID of the user's Azure Active Directory Application used for a
    /// federated connection.
    #[prost(string, tag = "6")]
    pub federated_application_client_id: ::prost::alloc::string::String,
    /// Output only. A unique Google-owned and Google-generated identity for the
    /// Connection. This identity will be used to access the user's Azure Active
    /// Directory Application.
    #[prost(string, tag = "7")]
    pub identity: ::prost::alloc::string::String,
}
/// Container for connection properties for delegation of access to GCP
/// resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudResourceProperties {
    /// Output only. The account ID of the service created for the purpose of this
    /// connection.
    ///
    /// The service account does not have any permissions associated with it
    /// when it is created. After creation, customers delegate permissions
    /// to the service account. When the connection is used in the context of an
    /// operation in BigQuery, the service account will be used to connect to the
    /// desired resources in GCP.
    ///
    /// The account ID is in the form of:
    ///    <service-1234>@gcp-sa-bigquery-cloudresource.iam.gserviceaccount.com
    #[prost(string, tag = "1")]
    pub service_account_id: ::prost::alloc::string::String,
}
/// Configuration of the Dataproc Metastore Service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetastoreServiceConfig {
    /// Optional. Resource name of an existing Dataproc Metastore service.
    ///
    /// Example:
    ///
    /// * `projects/\[project_id\]/locations/\[region\]/services/\[service_id\]`
    #[prost(string, tag = "1")]
    pub metastore_service: ::prost::alloc::string::String,
}
/// Configuration of the Spark History Server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SparkHistoryServerConfig {
    /// Optional. Resource name of an existing Dataproc Cluster to act as a Spark
    /// History Server for the connection.
    ///
    /// Example:
    ///
    /// * `projects/\[project_id\]/regions/\[region\]/clusters/\[cluster_name\]`
    #[prost(string, tag = "1")]
    pub dataproc_cluster: ::prost::alloc::string::String,
}
/// Container for connection properties to execute stored procedures for Apache
/// Spark.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SparkProperties {
    /// Output only. The account ID of the service created for the purpose of this
    /// connection.
    ///
    /// The service account does not have any permissions associated with it when
    /// it is created. After creation, customers delegate permissions to the
    /// service account. When the connection is used in the context of a stored
    /// procedure for Apache Spark in BigQuery, the service account is used to
    /// connect to the desired resources in Google Cloud.
    ///
    /// The account ID is in the form of:
    /// bqcx-<projectnumber>-<uniqueid>@gcp-sa-bigquery-consp.iam.gserviceaccount.com
    #[prost(string, tag = "1")]
    pub service_account_id: ::prost::alloc::string::String,
    /// Optional. Dataproc Metastore Service configuration for the connection.
    #[prost(message, optional, tag = "3")]
    pub metastore_service_config: ::core::option::Option<MetastoreServiceConfig>,
    /// Optional. Spark History Server configuration for the connection.
    #[prost(message, optional, tag = "4")]
    pub spark_history_server_config: ::core::option::Option<SparkHistoryServerConfig>,
}
/// Connection properties specific to Salesforce DataCloud. This is intended for
/// use only by Salesforce partner projects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SalesforceDataCloudProperties {
    /// The URL to the user's Salesforce DataCloud instance.
    #[prost(string, tag = "1")]
    pub instance_uri: ::prost::alloc::string::String,
    /// Output only. A unique Google-owned and Google-generated service account
    /// identity for the connection.
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    /// The ID of the user's Salesforce tenant.
    #[prost(string, tag = "3")]
    pub tenant_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod connection_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages external data source connections and credentials.
    #[derive(Debug, Clone)]
    pub struct ConnectionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConnectionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ConnectionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ConnectionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Creates a new connection.
        pub async fn create_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectionRequest>,
        ) -> std::result::Result<tonic::Response<super::Connection>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/CreateConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1.ConnectionService",
                        "CreateConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns specified connection.
        pub async fn get_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectionRequest>,
        ) -> std::result::Result<tonic::Response<super::Connection>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/GetConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1.ConnectionService",
                        "GetConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a list of connections in the given project.
        pub async fn list_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListConnectionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/ListConnections",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1.ConnectionService",
                        "ListConnections",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified connection. For security reasons, also resets
        /// credential if connection properties are in the update field mask.
        pub async fn update_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConnectionRequest>,
        ) -> std::result::Result<tonic::Response<super::Connection>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/UpdateConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1.ConnectionService",
                        "UpdateConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes connection and associated credential.
        pub async fn delete_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectionRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/DeleteConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1.ConnectionService",
                        "DeleteConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the access control policy for a resource.
        /// Returns an empty policy if the resource exists and does not have a policy
        /// set.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1.ConnectionService",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the access control policy on the specified resource. Replaces any
        /// existing policy.
        ///
        /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1.ConnectionService",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns permissions that a caller has on the specified resource.
        /// If the resource does not exist, this will return an empty set of
        /// permissions, not a `NOT_FOUND` error.
        ///
        /// Note: This operation is designed to be used for building permission-aware
        /// UIs and command-line tools, not for authorization checking. This operation
        /// may "fail open" without warning.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.connection.v1.ConnectionService",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}