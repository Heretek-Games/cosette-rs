#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerEventLogSendV1 {}
/// Nested message and enum types in `PlayerEventLogSendV1`.
pub mod player_event_log_send_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, repeated, tag = "1")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {}
}
/// Generated client implementations.
pub mod player_event_log_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PlayerEventLogClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayerEventLogClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PlayerEventLogClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> PlayerEventLogClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            PlayerEventLogClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn send_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::player_event_log_send_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_event_log_send_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/PlayerEventLog/SendV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("PlayerEventLog", "SendV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod player_event_log_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PlayerEventLogServer.
    #[async_trait]
    pub trait PlayerEventLog: Send + Sync + 'static {
        async fn send_v1(
            &self,
            request: tonic::Request<super::player_event_log_send_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_event_log_send_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PlayerEventLogServer<T: PlayerEventLog> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PlayerEventLog> PlayerEventLogServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlayerEventLogServer<T>
    where
        T: PlayerEventLog,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/PlayerEventLog/SendV1" => {
                    #[allow(non_camel_case_types)]
                    struct SendV1Svc<T: PlayerEventLog>(pub Arc<T>);
                    impl<
                        T: PlayerEventLog,
                    > tonic::server::UnaryService<
                        super::player_event_log_send_v1::Request,
                    > for SendV1Svc<T> {
                        type Response = super::player_event_log_send_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_event_log_send_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).send_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PlayerEventLog> Clone for PlayerEventLogServer<T> {
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
    impl<T: PlayerEventLog> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PlayerEventLog> tonic::server::NamedService for PlayerEventLogServer<T> {
        const NAME: &'static str = "PlayerEventLog";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginBonusCountUpProgressV1 {}
/// Nested message and enum types in `LoginBonusCountUpProgressV1`.
pub mod login_bonus_count_up_progress_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, repeated, tag = "1")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub counted_up_login_bonuses: ::prost::alloc::vec::Vec<
            super::super::resource::login_bonus::v1::LoginBonus,
        >,
        #[prost(message, repeated, tag = "2")]
        pub player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginBonusGetAvailableV1 {}
/// Nested message and enum types in `LoginBonusGetAvailableV1`.
pub mod login_bonus_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "2")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub login_bonuses: ::prost::alloc::vec::Vec<
            super::super::resource::login_bonus::v1::LoginBonus,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginBonusGetReceivedV1 {}
/// Nested message and enum types in `LoginBonusGetReceivedV1`.
pub mod login_bonus_get_received_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub login_bonuses: ::prost::alloc::vec::Vec<
            super::super::resource::login_bonus::v1::LoginBonus,
        >,
    }
}
/// Generated client implementations.
pub mod login_bonus_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct LoginBonusClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LoginBonusClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> LoginBonusClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> LoginBonusClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            LoginBonusClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn count_up_progress_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::login_bonus_count_up_progress_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::login_bonus_count_up_progress_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/LoginBonus/CountUpProgressV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("LoginBonus", "CountUpProgressV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::login_bonus_get_available_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::login_bonus_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/LoginBonus/GetAvailableV1",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("LoginBonus", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_received_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::login_bonus_get_received_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::login_bonus_get_received_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LoginBonus/GetReceivedV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("LoginBonus", "GetReceivedV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod login_bonus_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LoginBonusServer.
    #[async_trait]
    pub trait LoginBonus: Send + Sync + 'static {
        async fn count_up_progress_v1(
            &self,
            request: tonic::Request<super::login_bonus_count_up_progress_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::login_bonus_count_up_progress_v1::Response>,
            tonic::Status,
        >;
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::login_bonus_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::login_bonus_get_available_v1::Response>,
            tonic::Status,
        >;
        async fn get_received_v1(
            &self,
            request: tonic::Request<super::login_bonus_get_received_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::login_bonus_get_received_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct LoginBonusServer<T: LoginBonus> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LoginBonus> LoginBonusServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LoginBonusServer<T>
    where
        T: LoginBonus,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/LoginBonus/CountUpProgressV1" => {
                    #[allow(non_camel_case_types)]
                    struct CountUpProgressV1Svc<T: LoginBonus>(pub Arc<T>);
                    impl<
                        T: LoginBonus,
                    > tonic::server::UnaryService<
                        super::login_bonus_count_up_progress_v1::Request,
                    > for CountUpProgressV1Svc<T> {
                        type Response = super::login_bonus_count_up_progress_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::login_bonus_count_up_progress_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).count_up_progress_v1(request).await
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
                        let inner = inner.0;
                        let method = CountUpProgressV1Svc(inner);
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
                "/LoginBonus/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: LoginBonus>(pub Arc<T>);
                    impl<
                        T: LoginBonus,
                    > tonic::server::UnaryService<
                        super::login_bonus_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::login_bonus_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::login_bonus_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                "/LoginBonus/GetReceivedV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetReceivedV1Svc<T: LoginBonus>(pub Arc<T>);
                    impl<
                        T: LoginBonus,
                    > tonic::server::UnaryService<
                        super::login_bonus_get_received_v1::Request,
                    > for GetReceivedV1Svc<T> {
                        type Response = super::login_bonus_get_received_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::login_bonus_get_received_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_received_v1(request).await
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
                        let inner = inner.0;
                        let method = GetReceivedV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: LoginBonus> Clone for LoginBonusServer<T> {
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
    impl<T: LoginBonus> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LoginBonus> tonic::server::NamedService for LoginBonusServer<T> {
        const NAME: &'static str = "LoginBonus";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameMessageGetUnreadMessageV1 {}
/// Nested message and enum types in `GameMessageGetUnreadMessageV1`.
pub mod game_message_get_unread_message_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "2")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub messages: ::prost::alloc::vec::Vec<
            super::super::resource::game_message::v1::GameMessage,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameMessageGetMessageV1 {}
/// Nested message and enum types in `GameMessageGetMessageV1`.
pub mod game_message_get_message_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "2")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub messages: ::prost::alloc::vec::Vec<
            super::super::resource::game_message::v1::GameMessage,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameMessageReceiveMessageV1 {}
/// Nested message and enum types in `GameMessageReceiveMessageV1`.
pub mod game_message_receive_message_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub message_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {}
}
/// Generated client implementations.
pub mod game_message_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct GameMessageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GameMessageClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GameMessageClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> GameMessageClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            GameMessageClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_unread_message_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::game_message_get_unread_message_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::game_message_get_unread_message_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/GameMessage/GetUnreadMessageV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("GameMessage", "GetUnreadMessageV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_message_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::game_message_get_message_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::game_message_get_message_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/GameMessage/GetMessageV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("GameMessage", "GetMessageV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn receive_game_message_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::game_message_receive_message_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::game_message_receive_message_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/GameMessage/ReceiveGameMessageV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("GameMessage", "ReceiveGameMessageV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod game_message_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GameMessageServer.
    #[async_trait]
    pub trait GameMessage: Send + Sync + 'static {
        async fn get_unread_message_v1(
            &self,
            request: tonic::Request<super::game_message_get_unread_message_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::game_message_get_unread_message_v1::Response>,
            tonic::Status,
        >;
        async fn get_message_v1(
            &self,
            request: tonic::Request<super::game_message_get_message_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::game_message_get_message_v1::Response>,
            tonic::Status,
        >;
        async fn receive_game_message_v1(
            &self,
            request: tonic::Request<super::game_message_receive_message_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::game_message_receive_message_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct GameMessageServer<T: GameMessage> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GameMessage> GameMessageServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GameMessageServer<T>
    where
        T: GameMessage,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/GameMessage/GetUnreadMessageV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetUnreadMessageV1Svc<T: GameMessage>(pub Arc<T>);
                    impl<
                        T: GameMessage,
                    > tonic::server::UnaryService<
                        super::game_message_get_unread_message_v1::Request,
                    > for GetUnreadMessageV1Svc<T> {
                        type Response = super::game_message_get_unread_message_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::game_message_get_unread_message_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_unread_message_v1(request).await
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
                        let inner = inner.0;
                        let method = GetUnreadMessageV1Svc(inner);
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
                "/GameMessage/GetMessageV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetMessageV1Svc<T: GameMessage>(pub Arc<T>);
                    impl<
                        T: GameMessage,
                    > tonic::server::UnaryService<
                        super::game_message_get_message_v1::Request,
                    > for GetMessageV1Svc<T> {
                        type Response = super::game_message_get_message_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::game_message_get_message_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_message_v1(request).await
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
                        let inner = inner.0;
                        let method = GetMessageV1Svc(inner);
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
                "/GameMessage/ReceiveGameMessageV1" => {
                    #[allow(non_camel_case_types)]
                    struct ReceiveGameMessageV1Svc<T: GameMessage>(pub Arc<T>);
                    impl<
                        T: GameMessage,
                    > tonic::server::UnaryService<
                        super::game_message_receive_message_v1::Request,
                    > for ReceiveGameMessageV1Svc<T> {
                        type Response = super::game_message_receive_message_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::game_message_receive_message_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).receive_game_message_v1(request).await
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
                        let inner = inner.0;
                        let method = ReceiveGameMessageV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: GameMessage> Clone for GameMessageServer<T> {
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
    impl<T: GameMessage> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GameMessage> tonic::server::NamedService for GameMessageServer<T> {
        const NAME: &'static str = "GameMessage";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoxLootBoxGetAvailableV1 {}
/// Nested message and enum types in `BoxLootBoxGetAvailableV1`.
pub mod box_loot_box_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "2")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub box_loot_box_products: ::prost::alloc::vec::Vec<
            super::super::resource::box_loot_box::v1::BoxLootBoxProduct,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoxLootBoxPurchaseV1 {}
/// Nested message and enum types in `BoxLootBoxPurchaseV1`.
pub mod box_loot_box_purchase_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub box_loot_box_product_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(sint32, tag = "3")]
        pub num: i32,
        #[prost(enumeration = "request::ResourceType", tag = "4")]
        pub resource_type: i32,
        #[prost(message, repeated, tag = "5")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    /// Nested message and enum types in `Request`.
    pub mod request {
        #[derive(serde::Serialize, serde::Deserialize)]
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
        pub enum ResourceType {
            VirtualCurrency = 0,
            PlayerKeyValue = 1,
        }
        impl ResourceType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResourceType::VirtualCurrency => "VIRTUAL_CURRENCY",
                    ResourceType::PlayerKeyValue => "PLAYER_KEY_VALUE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "VIRTUAL_CURRENCY" => Some(Self::VirtualCurrency),
                    "PLAYER_KEY_VALUE" => Some(Self::PlayerKeyValue),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub box_loot_box_contents: ::prost::alloc::vec::Vec<
            super::super::resource::box_loot_box::v1::BoxLootBoxContent,
        >,
        #[prost(message, repeated, tag = "3")]
        pub player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, repeated, tag = "4")]
        pub extra_player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(sint32, tag = "5")]
        pub remain_num: i32,
        #[prost(bool, tag = "6")]
        pub is_reset: bool,
        #[prost(message, optional, tag = "7")]
        pub wallet: ::core::option::Option<super::super::resource::wallet::v3::Wallet>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoxLootBoxResetV1 {}
/// Nested message and enum types in `BoxLootBoxResetV1`.
pub mod box_loot_box_reset_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub box_loot_box_product_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {}
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoxLootBoxGetDetailV1 {}
/// Nested message and enum types in `BoxLootBoxGetDetailV1`.
pub mod box_loot_box_get_detail_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub box_loot_box_product_id: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub box_loot_box_product: ::core::option::Option<
            super::super::resource::box_loot_box::v1::BoxLootBoxProduct,
        >,
        #[prost(message, repeated, tag = "2")]
        pub box_loot_box_contents: ::prost::alloc::vec::Vec<
            super::super::resource::box_loot_box::v1::BoxLootBoxContentWithPlayerRemainContent,
        >,
        #[prost(sint64, tag = "3")]
        pub player_item_amount: i64,
        #[prost(sint64, tag = "4")]
        pub player_box_reset_count: i64,
    }
}
/// Generated client implementations.
pub mod box_loot_box_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BoxLootBoxClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BoxLootBoxClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BoxLootBoxClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> BoxLootBoxClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            BoxLootBoxClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::box_loot_box_get_available_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::box_loot_box_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/BoxLootBox/GetAvailableV1",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("BoxLootBox", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn purchase_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::box_loot_box_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::box_loot_box_purchase_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxLootBox/PurchaseV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("BoxLootBox", "PurchaseV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::box_loot_box_reset_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::box_loot_box_reset_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxLootBox/ResetV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("BoxLootBox", "ResetV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_detail_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::box_loot_box_get_detail_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::box_loot_box_get_detail_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BoxLootBox/GetDetailV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("BoxLootBox", "GetDetailV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod box_loot_box_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BoxLootBoxServer.
    #[async_trait]
    pub trait BoxLootBox: Send + Sync + 'static {
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::box_loot_box_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::box_loot_box_get_available_v1::Response>,
            tonic::Status,
        >;
        async fn purchase_v1(
            &self,
            request: tonic::Request<super::box_loot_box_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::box_loot_box_purchase_v1::Response>,
            tonic::Status,
        >;
        async fn reset_v1(
            &self,
            request: tonic::Request<super::box_loot_box_reset_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::box_loot_box_reset_v1::Response>,
            tonic::Status,
        >;
        async fn get_detail_v1(
            &self,
            request: tonic::Request<super::box_loot_box_get_detail_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::box_loot_box_get_detail_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct BoxLootBoxServer<T: BoxLootBox> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BoxLootBox> BoxLootBoxServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BoxLootBoxServer<T>
    where
        T: BoxLootBox,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/BoxLootBox/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: BoxLootBox>(pub Arc<T>);
                    impl<
                        T: BoxLootBox,
                    > tonic::server::UnaryService<
                        super::box_loot_box_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::box_loot_box_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::box_loot_box_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                "/BoxLootBox/PurchaseV1" => {
                    #[allow(non_camel_case_types)]
                    struct PurchaseV1Svc<T: BoxLootBox>(pub Arc<T>);
                    impl<
                        T: BoxLootBox,
                    > tonic::server::UnaryService<
                        super::box_loot_box_purchase_v1::Request,
                    > for PurchaseV1Svc<T> {
                        type Response = super::box_loot_box_purchase_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::box_loot_box_purchase_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).purchase_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PurchaseV1Svc(inner);
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
                "/BoxLootBox/ResetV1" => {
                    #[allow(non_camel_case_types)]
                    struct ResetV1Svc<T: BoxLootBox>(pub Arc<T>);
                    impl<
                        T: BoxLootBox,
                    > tonic::server::UnaryService<super::box_loot_box_reset_v1::Request>
                    for ResetV1Svc<T> {
                        type Response = super::box_loot_box_reset_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::box_loot_box_reset_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).reset_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResetV1Svc(inner);
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
                "/BoxLootBox/GetDetailV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetDetailV1Svc<T: BoxLootBox>(pub Arc<T>);
                    impl<
                        T: BoxLootBox,
                    > tonic::server::UnaryService<
                        super::box_loot_box_get_detail_v1::Request,
                    > for GetDetailV1Svc<T> {
                        type Response = super::box_loot_box_get_detail_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::box_loot_box_get_detail_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_detail_v1(request).await
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
                        let inner = inner.0;
                        let method = GetDetailV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: BoxLootBox> Clone for BoxLootBoxServer<T> {
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
    impl<T: BoxLootBox> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BoxLootBox> tonic::server::NamedService for BoxLootBoxServer<T> {
        const NAME: &'static str = "BoxLootBox";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerRankingRegisterV1 {}
/// Nested message and enum types in `PlayerRankingRegisterV1`.
pub mod player_ranking_register_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub ranking_key: ::prost::alloc::string::String,
        #[prost(int64, tag = "2")]
        pub score: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {}
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerRankingRegisterV2 {}
/// Nested message and enum types in `PlayerRankingRegisterV2`.
pub mod player_ranking_register_v2 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub ranking_key: ::prost::alloc::string::String,
        #[prost(int64, tag = "2")]
        pub current_score: i64,
        #[prost(int64, tag = "3")]
        pub delta_score: i64,
        #[prost(string, tag = "4")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "5")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {}
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerRankingGetTopRankingV1 {}
/// Nested message and enum types in `PlayerRankingGetTopRankingV1`.
pub mod player_ranking_get_top_ranking_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub ranking_key: ::prost::alloc::string::String,
        #[prost(int64, tag = "2")]
        pub start: i64,
        #[prost(int64, tag = "3")]
        pub count: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_rankings: ::prost::alloc::vec::Vec<
            super::super::resource::player_ranking::v1::PlayerRankInfo,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerRankingGetRankingAroundMeV1 {}
/// Nested message and enum types in `PlayerRankingGetRankingAroundMeV1`.
pub mod player_ranking_get_ranking_around_me_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub ranking_key: ::prost::alloc::string::String,
        #[prost(int64, tag = "2")]
        pub range: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_rankings: ::prost::alloc::vec::Vec<
            super::super::resource::player_ranking::v1::PlayerRankInfo,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerRankingGetAvailableV1 {}
/// Nested message and enum types in `PlayerRankingGetAvailableV1`.
pub mod player_ranking_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "2")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub ranking_master: ::prost::alloc::vec::Vec<
            super::super::resource::player_ranking::v1::RankingMaster,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerRankingReceivePrizeV1 {}
/// Nested message and enum types in `PlayerRankingReceivePrizeV1`.
pub mod player_ranking_receive_prize_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub ranking_key: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub player_ranking: ::core::option::Option<
            super::super::resource::player_ranking::v1::PlayerRankInfo,
        >,
        #[prost(message, repeated, tag = "2")]
        pub inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
    }
}
/// Generated client implementations.
pub mod player_ranking_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PlayerRankingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayerRankingClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PlayerRankingClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> PlayerRankingClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            PlayerRankingClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn register_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::player_ranking_register_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_ranking_register_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/PlayerRanking/RegisterV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("PlayerRanking", "RegisterV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn register_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::player_ranking_register_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_ranking_register_v2::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/PlayerRanking/RegisterV2");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("PlayerRanking", "RegisterV2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_top_ranking_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_ranking_get_top_ranking_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_ranking_get_top_ranking_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerRanking/GetTopRankingV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerRanking", "GetTopRankingV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_ranking_around_me_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_ranking_get_ranking_around_me_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_ranking_get_ranking_around_me_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerRanking/GetRankingAroundMeV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerRanking", "GetRankingAroundMeV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_ranking_get_available_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_ranking_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerRanking/GetAvailableV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerRanking", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn receive_prize_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_ranking_receive_prize_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_ranking_receive_prize_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerRanking/ReceivePrizeV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerRanking", "ReceivePrizeV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod player_ranking_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PlayerRankingServer.
    #[async_trait]
    pub trait PlayerRanking: Send + Sync + 'static {
        async fn register_v1(
            &self,
            request: tonic::Request<super::player_ranking_register_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_ranking_register_v1::Response>,
            tonic::Status,
        >;
        async fn register_v2(
            &self,
            request: tonic::Request<super::player_ranking_register_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_ranking_register_v2::Response>,
            tonic::Status,
        >;
        async fn get_top_ranking_v1(
            &self,
            request: tonic::Request<super::player_ranking_get_top_ranking_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_ranking_get_top_ranking_v1::Response>,
            tonic::Status,
        >;
        async fn get_ranking_around_me_v1(
            &self,
            request: tonic::Request<
                super::player_ranking_get_ranking_around_me_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_ranking_get_ranking_around_me_v1::Response>,
            tonic::Status,
        >;
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::player_ranking_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_ranking_get_available_v1::Response>,
            tonic::Status,
        >;
        async fn receive_prize_v1(
            &self,
            request: tonic::Request<super::player_ranking_receive_prize_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_ranking_receive_prize_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PlayerRankingServer<T: PlayerRanking> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PlayerRanking> PlayerRankingServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlayerRankingServer<T>
    where
        T: PlayerRanking,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/PlayerRanking/RegisterV1" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterV1Svc<T: PlayerRanking>(pub Arc<T>);
                    impl<
                        T: PlayerRanking,
                    > tonic::server::UnaryService<
                        super::player_ranking_register_v1::Request,
                    > for RegisterV1Svc<T> {
                        type Response = super::player_ranking_register_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_ranking_register_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).register_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterV1Svc(inner);
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
                "/PlayerRanking/RegisterV2" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterV2Svc<T: PlayerRanking>(pub Arc<T>);
                    impl<
                        T: PlayerRanking,
                    > tonic::server::UnaryService<
                        super::player_ranking_register_v2::Request,
                    > for RegisterV2Svc<T> {
                        type Response = super::player_ranking_register_v2::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_ranking_register_v2::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).register_v2(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterV2Svc(inner);
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
                "/PlayerRanking/GetTopRankingV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetTopRankingV1Svc<T: PlayerRanking>(pub Arc<T>);
                    impl<
                        T: PlayerRanking,
                    > tonic::server::UnaryService<
                        super::player_ranking_get_top_ranking_v1::Request,
                    > for GetTopRankingV1Svc<T> {
                        type Response = super::player_ranking_get_top_ranking_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_ranking_get_top_ranking_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_top_ranking_v1(request).await
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
                        let inner = inner.0;
                        let method = GetTopRankingV1Svc(inner);
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
                "/PlayerRanking/GetRankingAroundMeV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetRankingAroundMeV1Svc<T: PlayerRanking>(pub Arc<T>);
                    impl<
                        T: PlayerRanking,
                    > tonic::server::UnaryService<
                        super::player_ranking_get_ranking_around_me_v1::Request,
                    > for GetRankingAroundMeV1Svc<T> {
                        type Response = super::player_ranking_get_ranking_around_me_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_ranking_get_ranking_around_me_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_ranking_around_me_v1(request).await
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
                        let inner = inner.0;
                        let method = GetRankingAroundMeV1Svc(inner);
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
                "/PlayerRanking/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: PlayerRanking>(pub Arc<T>);
                    impl<
                        T: PlayerRanking,
                    > tonic::server::UnaryService<
                        super::player_ranking_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::player_ranking_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_ranking_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                "/PlayerRanking/ReceivePrizeV1" => {
                    #[allow(non_camel_case_types)]
                    struct ReceivePrizeV1Svc<T: PlayerRanking>(pub Arc<T>);
                    impl<
                        T: PlayerRanking,
                    > tonic::server::UnaryService<
                        super::player_ranking_receive_prize_v1::Request,
                    > for ReceivePrizeV1Svc<T> {
                        type Response = super::player_ranking_receive_prize_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_ranking_receive_prize_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).receive_prize_v1(request).await
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
                        let inner = inner.0;
                        let method = ReceivePrizeV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PlayerRanking> Clone for PlayerRankingServer<T> {
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
    impl<T: PlayerRanking> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PlayerRanking> tonic::server::NamedService for PlayerRankingServer<T> {
        const NAME: &'static str = "PlayerRanking";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerVersionGetV1 {}
/// Nested message and enum types in `ServerVersionGetV1`.
pub mod server_version_get_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub server_version: ::prost::alloc::string::String,
    }
}
/// Generated client implementations.
pub mod server_version_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ServerVersionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServerVersionClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ServerVersionClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> ServerVersionClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            ServerVersionClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::server_version_get_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::server_version_get_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ServerVersion/GetV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("ServerVersion", "GetV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod server_version_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ServerVersionServer.
    #[async_trait]
    pub trait ServerVersion: Send + Sync + 'static {
        async fn get_v1(
            &self,
            request: tonic::Request<super::server_version_get_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::server_version_get_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ServerVersionServer<T: ServerVersion> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ServerVersion> ServerVersionServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ServerVersionServer<T>
    where
        T: ServerVersion,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/ServerVersion/GetV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetV1Svc<T: ServerVersion>(pub Arc<T>);
                    impl<
                        T: ServerVersion,
                    > tonic::server::UnaryService<super::server_version_get_v1::Request>
                    for GetV1Svc<T> {
                        type Response = super::server_version_get_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::server_version_get_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ServerVersion> Clone for ServerVersionServer<T> {
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
    impl<T: ServerVersion> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ServerVersion> tonic::server::NamedService for ServerVersionServer<T> {
        const NAME: &'static str = "ServerVersion";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnouncementGetAvailableV1 {}
/// Nested message and enum types in `AnnouncementGetAvailableV1`.
pub mod announcement_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub language_code: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "3")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub announcements: ::prost::alloc::vec::Vec<
            super::super::resource::announcement::v1::Announcement,
        >,
        #[prost(string, tag = "2")]
        pub base_image_url: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
/// Generated client implementations.
pub mod announcement_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AnnouncementClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AnnouncementClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AnnouncementClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> AnnouncementClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            AnnouncementClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::announcement_get_available_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::announcement_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Announcement/GetAvailableV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Announcement", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod announcement_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AnnouncementServer.
    #[async_trait]
    pub trait Announcement: Send + Sync + 'static {
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::announcement_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::announcement_get_available_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AnnouncementServer<T: Announcement> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Announcement> AnnouncementServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AnnouncementServer<T>
    where
        T: Announcement,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/Announcement/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: Announcement>(pub Arc<T>);
                    impl<
                        T: Announcement,
                    > tonic::server::UnaryService<
                        super::announcement_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::announcement_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::announcement_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Announcement> Clone for AnnouncementServer<T> {
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
    impl<T: Announcement> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Announcement> tonic::server::NamedService for AnnouncementServer<T> {
        const NAME: &'static str = "Announcement";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestorationValidateBannedWordsV1 {}
/// Nested message and enum types in `RestorationValidateBannedWordsV1`.
pub mod restoration_validate_banned_words_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub language_codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, tag = "2")]
        pub word: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(bool, tag = "1")]
        pub contains_banned_word: bool,
        #[prost(string, tag = "2")]
        pub word: ::prost::alloc::string::String,
    }
}
/// Generated client implementations.
pub mod restoration_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RestorationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RestorationClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RestorationClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> RestorationClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            RestorationClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn validate_banned_words_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::restoration_validate_banned_words_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::restoration_validate_banned_words_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Restoration/ValidateBannedWordsV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Restoration", "ValidateBannedWordsV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod restoration_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RestorationServer.
    #[async_trait]
    pub trait Restoration: Send + Sync + 'static {
        async fn validate_banned_words_v1(
            &self,
            request: tonic::Request<super::restoration_validate_banned_words_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::restoration_validate_banned_words_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct RestorationServer<T: Restoration> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Restoration> RestorationServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RestorationServer<T>
    where
        T: Restoration,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/Restoration/ValidateBannedWordsV1" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateBannedWordsV1Svc<T: Restoration>(pub Arc<T>);
                    impl<
                        T: Restoration,
                    > tonic::server::UnaryService<
                        super::restoration_validate_banned_words_v1::Request,
                    > for ValidateBannedWordsV1Svc<T> {
                        type Response = super::restoration_validate_banned_words_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::restoration_validate_banned_words_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).validate_banned_words_v1(request).await
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
                        let inner = inner.0;
                        let method = ValidateBannedWordsV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Restoration> Clone for RestorationServer<T> {
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
    impl<T: Restoration> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Restoration> tonic::server::NamedService for RestorationServer<T> {
        const NAME: &'static str = "Restoration";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxGetAvailableV1 {}
/// Nested message and enum types in `StepUpLootBoxGetAvailableV1`.
pub mod step_up_loot_box_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "2")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub step_up_loot_box_products: ::prost::alloc::vec::Vec<
            super::super::resource::step_up_loot_box::v1::StepUpLootBoxProductWithPlayerStepNum,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxPurchaseV1 {}
/// Nested message and enum types in `StepUpLootBoxPurchaseV1`.
pub mod step_up_loot_box_purchase_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub step_up_loot_box_product_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(enumeration = "request::ResourceType", tag = "3")]
        pub resource_type: i32,
        #[prost(message, repeated, tag = "4")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    /// Nested message and enum types in `Request`.
    pub mod request {
        #[derive(serde::Serialize, serde::Deserialize)]
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
        pub enum ResourceType {
            VirtualCurrency = 0,
            PlayerKeyValue = 1,
        }
        impl ResourceType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResourceType::VirtualCurrency => "VIRTUAL_CURRENCY",
                    ResourceType::PlayerKeyValue => "PLAYER_KEY_VALUE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "VIRTUAL_CURRENCY" => Some(Self::VirtualCurrency),
                    "PLAYER_KEY_VALUE" => Some(Self::PlayerKeyValue),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub step_up_loot_box_contents: ::prost::alloc::vec::Vec<
            super::super::resource::step_up_loot_box::v1::StepUpLootBoxContent,
        >,
        #[prost(message, repeated, tag = "3")]
        pub player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, repeated, tag = "4")]
        pub extra_player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, optional, tag = "6")]
        pub wallet: ::core::option::Option<super::super::resource::wallet::v3::Wallet>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxGetProbabilityV1 {}
/// Nested message and enum types in `StepUpLootBoxGetProbabilityV1`.
pub mod step_up_loot_box_get_probability_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub step_up_loot_box_product_id: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub step_up_loot_box_probability: ::core::option::Option<
            super::super::resource::step_up_loot_box::v1::StepUpLootBoxProbability,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxGetDetailV1 {}
/// Nested message and enum types in `StepUpLootBoxGetDetailV1`.
pub mod step_up_loot_box_get_detail_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub step_up_loot_box_product_id: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub step_up_loot_box_product: ::core::option::Option<
            super::super::resource::step_up_loot_box::v1::StepUpLootBoxProductWithPlayerStepNum,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxV2GetAvailableV1 {}
/// Nested message and enum types in `StepUpLootBoxV2GetAvailableV1`.
pub mod step_up_loot_box_v2_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "2")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub step_up_loot_box_products: ::prost::alloc::vec::Vec<
            super::super::resource::step_up_loot_box::v2::StepUpLootBoxProduct,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxV2PurchaseV1 {}
/// Nested message and enum types in `StepUpLootBoxV2PurchaseV1`.
pub mod step_up_loot_box_v2_purchase_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub step_up_loot_box_product_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(enumeration = "request::ResourceType", tag = "3")]
        pub resource_type: i32,
        #[prost(string, tag = "4")]
        pub purchase_token: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "5")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    /// Nested message and enum types in `Request`.
    pub mod request {
        #[derive(serde::Serialize, serde::Deserialize)]
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
        pub enum ResourceType {
            VirtualCurrency = 0,
            PlayerKeyValue = 1,
        }
        impl ResourceType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResourceType::VirtualCurrency => "VIRTUAL_CURRENCY",
                    ResourceType::PlayerKeyValue => "PLAYER_KEY_VALUE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "VIRTUAL_CURRENCY" => Some(Self::VirtualCurrency),
                    "PLAYER_KEY_VALUE" => Some(Self::PlayerKeyValue),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub step_up_loot_box_contents: ::prost::alloc::vec::Vec<
            super::super::resource::step_up_loot_box::v2::StepUpLootBoxContent,
        >,
        #[prost(message, repeated, tag = "3")]
        pub player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, repeated, tag = "4")]
        pub extra_player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, optional, tag = "5")]
        pub wallet: ::core::option::Option<super::super::resource::wallet::v3::Wallet>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxV2GetProbabilityV1 {}
/// Nested message and enum types in `StepUpLootBoxV2GetProbabilityV1`.
pub mod step_up_loot_box_v2_get_probability_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub step_up_loot_box_product_ids: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub step_up_loot_box_probabilities: ::prost::alloc::vec::Vec<
            super::super::resource::step_up_loot_box::v2::StepUpLootBoxProbability,
        >,
        #[prost(string, tag = "2")]
        pub purchase_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxV2GetDetailV1 {}
/// Nested message and enum types in `StepUpLootBoxV2GetDetailV1`.
pub mod step_up_loot_box_v2_get_detail_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub step_up_loot_box_product_ids: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub step_up_loot_box_products: ::prost::alloc::vec::Vec<
            super::super::resource::step_up_loot_box::v2::StepUpLootBoxProduct,
        >,
    }
}
/// Generated client implementations.
pub mod step_up_loot_box_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct StepUpLootBoxClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StepUpLootBoxClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StepUpLootBoxClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> StepUpLootBoxClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            StepUpLootBoxClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::step_up_loot_box_get_available_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/StepUpLootBox/GetAvailableV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("StepUpLootBox", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn purchase_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::step_up_loot_box_purchase_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_purchase_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/StepUpLootBox/PurchaseV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("StepUpLootBox", "PurchaseV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_probability_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::step_up_loot_box_get_probability_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_get_probability_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/StepUpLootBox/GetProbabilityV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("StepUpLootBox", "GetProbabilityV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_detail_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::step_up_loot_box_get_detail_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_get_detail_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/StepUpLootBox/GetDetailV1",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("StepUpLootBox", "GetDetailV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod step_up_loot_box_v2_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct StepUpLootBoxV2Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StepUpLootBoxV2Client<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StepUpLootBoxV2Client<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> StepUpLootBoxV2Client<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            StepUpLootBoxV2Client::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::step_up_loot_box_v2_get_available_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_v2_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/StepUpLootBoxV2/GetAvailableV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("StepUpLootBoxV2", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn purchase_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::step_up_loot_box_v2_purchase_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_v2_purchase_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/StepUpLootBoxV2/PurchaseV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("StepUpLootBoxV2", "PurchaseV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_probability_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::step_up_loot_box_v2_get_probability_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_v2_get_probability_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/StepUpLootBoxV2/GetProbabilityV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("StepUpLootBoxV2", "GetProbabilityV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_detail_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::step_up_loot_box_v2_get_detail_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_v2_get_detail_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/StepUpLootBoxV2/GetDetailV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("StepUpLootBoxV2", "GetDetailV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod step_up_loot_box_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StepUpLootBoxServer.
    #[async_trait]
    pub trait StepUpLootBox: Send + Sync + 'static {
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::step_up_loot_box_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_get_available_v1::Response>,
            tonic::Status,
        >;
        async fn purchase_v1(
            &self,
            request: tonic::Request<super::step_up_loot_box_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_purchase_v1::Response>,
            tonic::Status,
        >;
        async fn get_probability_v1(
            &self,
            request: tonic::Request<super::step_up_loot_box_get_probability_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_get_probability_v1::Response>,
            tonic::Status,
        >;
        async fn get_detail_v1(
            &self,
            request: tonic::Request<super::step_up_loot_box_get_detail_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_get_detail_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct StepUpLootBoxServer<T: StepUpLootBox> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: StepUpLootBox> StepUpLootBoxServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StepUpLootBoxServer<T>
    where
        T: StepUpLootBox,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/StepUpLootBox/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: StepUpLootBox>(pub Arc<T>);
                    impl<
                        T: StepUpLootBox,
                    > tonic::server::UnaryService<
                        super::step_up_loot_box_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::step_up_loot_box_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::step_up_loot_box_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                "/StepUpLootBox/PurchaseV1" => {
                    #[allow(non_camel_case_types)]
                    struct PurchaseV1Svc<T: StepUpLootBox>(pub Arc<T>);
                    impl<
                        T: StepUpLootBox,
                    > tonic::server::UnaryService<
                        super::step_up_loot_box_purchase_v1::Request,
                    > for PurchaseV1Svc<T> {
                        type Response = super::step_up_loot_box_purchase_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::step_up_loot_box_purchase_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).purchase_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PurchaseV1Svc(inner);
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
                "/StepUpLootBox/GetProbabilityV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetProbabilityV1Svc<T: StepUpLootBox>(pub Arc<T>);
                    impl<
                        T: StepUpLootBox,
                    > tonic::server::UnaryService<
                        super::step_up_loot_box_get_probability_v1::Request,
                    > for GetProbabilityV1Svc<T> {
                        type Response = super::step_up_loot_box_get_probability_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::step_up_loot_box_get_probability_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_probability_v1(request).await
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
                        let inner = inner.0;
                        let method = GetProbabilityV1Svc(inner);
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
                "/StepUpLootBox/GetDetailV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetDetailV1Svc<T: StepUpLootBox>(pub Arc<T>);
                    impl<
                        T: StepUpLootBox,
                    > tonic::server::UnaryService<
                        super::step_up_loot_box_get_detail_v1::Request,
                    > for GetDetailV1Svc<T> {
                        type Response = super::step_up_loot_box_get_detail_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::step_up_loot_box_get_detail_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_detail_v1(request).await
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
                        let inner = inner.0;
                        let method = GetDetailV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: StepUpLootBox> Clone for StepUpLootBoxServer<T> {
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
    impl<T: StepUpLootBox> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StepUpLootBox> tonic::server::NamedService for StepUpLootBoxServer<T> {
        const NAME: &'static str = "StepUpLootBox";
    }
}
/// Generated server implementations.
pub mod step_up_loot_box_v2_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StepUpLootBoxV2Server.
    #[async_trait]
    pub trait StepUpLootBoxV2: Send + Sync + 'static {
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::step_up_loot_box_v2_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_v2_get_available_v1::Response>,
            tonic::Status,
        >;
        async fn purchase_v1(
            &self,
            request: tonic::Request<super::step_up_loot_box_v2_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_v2_purchase_v1::Response>,
            tonic::Status,
        >;
        async fn get_probability_v1(
            &self,
            request: tonic::Request<
                super::step_up_loot_box_v2_get_probability_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_v2_get_probability_v1::Response>,
            tonic::Status,
        >;
        async fn get_detail_v1(
            &self,
            request: tonic::Request<super::step_up_loot_box_v2_get_detail_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::step_up_loot_box_v2_get_detail_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct StepUpLootBoxV2Server<T: StepUpLootBoxV2> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: StepUpLootBoxV2> StepUpLootBoxV2Server<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StepUpLootBoxV2Server<T>
    where
        T: StepUpLootBoxV2,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/StepUpLootBoxV2/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: StepUpLootBoxV2>(pub Arc<T>);
                    impl<
                        T: StepUpLootBoxV2,
                    > tonic::server::UnaryService<
                        super::step_up_loot_box_v2_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::step_up_loot_box_v2_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::step_up_loot_box_v2_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                "/StepUpLootBoxV2/PurchaseV1" => {
                    #[allow(non_camel_case_types)]
                    struct PurchaseV1Svc<T: StepUpLootBoxV2>(pub Arc<T>);
                    impl<
                        T: StepUpLootBoxV2,
                    > tonic::server::UnaryService<
                        super::step_up_loot_box_v2_purchase_v1::Request,
                    > for PurchaseV1Svc<T> {
                        type Response = super::step_up_loot_box_v2_purchase_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::step_up_loot_box_v2_purchase_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).purchase_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PurchaseV1Svc(inner);
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
                "/StepUpLootBoxV2/GetProbabilityV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetProbabilityV1Svc<T: StepUpLootBoxV2>(pub Arc<T>);
                    impl<
                        T: StepUpLootBoxV2,
                    > tonic::server::UnaryService<
                        super::step_up_loot_box_v2_get_probability_v1::Request,
                    > for GetProbabilityV1Svc<T> {
                        type Response = super::step_up_loot_box_v2_get_probability_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::step_up_loot_box_v2_get_probability_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_probability_v1(request).await
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
                        let inner = inner.0;
                        let method = GetProbabilityV1Svc(inner);
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
                "/StepUpLootBoxV2/GetDetailV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetDetailV1Svc<T: StepUpLootBoxV2>(pub Arc<T>);
                    impl<
                        T: StepUpLootBoxV2,
                    > tonic::server::UnaryService<
                        super::step_up_loot_box_v2_get_detail_v1::Request,
                    > for GetDetailV1Svc<T> {
                        type Response = super::step_up_loot_box_v2_get_detail_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::step_up_loot_box_v2_get_detail_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_detail_v1(request).await
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
                        let inner = inner.0;
                        let method = GetDetailV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: StepUpLootBoxV2> Clone for StepUpLootBoxV2Server<T> {
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
    impl<T: StepUpLootBoxV2> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StepUpLootBoxV2> tonic::server::NamedService for StepUpLootBoxV2Server<T> {
        const NAME: &'static str = "StepUpLootBoxV2";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemAuthorizeCnV1 {}
/// Nested message and enum types in `SystemAuthorizeCnV1`.
pub mod system_authorize_cn_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub device_account: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub device_password: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "3")]
        pub device_info: ::core::option::Option<
            super::super::resource::system::v1::DeviceInfo,
        >,
        #[prost(string, tag = "4")]
        pub id_token: ::prost::alloc::string::String,
        #[prost(int64, tag = "5")]
        pub initial_realm_id: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub session_token: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub player_id: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemAuthorizeV3 {}
/// Nested message and enum types in `SystemAuthorizeV3`.
pub mod system_authorize_v3 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub device_account: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub device_password: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "3")]
        pub device_info: ::core::option::Option<
            super::super::resource::system::v1::DeviceInfo,
        >,
        #[prost(string, tag = "4")]
        pub id_token: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub session_token: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub player_id: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemAuthorizeV2 {}
/// Nested message and enum types in `SystemAuthorizeV2`.
pub mod system_authorize_v2 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub device_account: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub device_password: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "3")]
        pub device_info: ::core::option::Option<
            super::super::resource::system::v1::DeviceInfo,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub session_token: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub player_id: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemGetPlayerStatusV1 {}
/// Nested message and enum types in `SystemGetPlayerStatusV1`.
pub mod system_get_player_status_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(sint64, tag = "1")]
        pub player_status: i64,
        #[prost(int64, tag = "2")]
        pub total_login_days: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemGetPlayerStatusV2 {}
/// Nested message and enum types in `SystemGetPlayerStatusV2`.
pub mod system_get_player_status_v2 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_statuses: ::prost::alloc::vec::Vec<
            super::super::resource::system::v1::PlayerStatus,
        >,
        #[prost(message, repeated, tag = "2")]
        pub ban_statuses: ::prost::alloc::vec::Vec<
            super::super::resource::system::v1::BanStatus,
        >,
        #[prost(int64, tag = "3")]
        pub total_login_days: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemSetPlayerStatusV1 {}
/// Nested message and enum types in `SystemSetPlayerStatusV1`.
pub mod system_set_player_status_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, optional, tag = "1")]
        pub player_status: ::core::option::Option<
            super::super::resource::system::v1::PlayerStatus,
        >,
        #[prost(message, repeated, tag = "2")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_statuses: ::prost::alloc::vec::Vec<
            super::super::resource::system::v1::PlayerStatus,
        >,
        #[prost(message, repeated, tag = "2")]
        pub ban_statuses: ::prost::alloc::vec::Vec<
            super::super::resource::system::v1::BanStatus,
        >,
        #[prost(int64, tag = "3")]
        pub total_login_days: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemDeletePlayerStatusV1 {}
/// Nested message and enum types in `SystemDeletePlayerStatusV1`.
pub mod system_delete_player_status_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, optional, tag = "1")]
        pub player_status: ::core::option::Option<
            super::super::resource::system::v1::PlayerStatus,
        >,
        #[prost(message, repeated, tag = "2")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {}
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRealmsV1 {}
/// Nested message and enum types in `GetRealmsV1`.
pub mod get_realms_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub realms: ::prost::alloc::vec::Vec<super::super::resource::realm::v1::Realm>,
    }
}
/// Generated client implementations.
pub mod system_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SystemClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SystemClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SystemClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> SystemClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            SystemClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn authorize_cn_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::system_authorize_cn_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::system_authorize_cn_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/System/AuthorizeCnV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("System", "AuthorizeCnV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn authorize_v3(
            &mut self,
            request: impl tonic::IntoRequest<super::system_authorize_v3::Request>,
        ) -> std::result::Result<
            tonic::Response<super::system_authorize_v3::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/System/AuthorizeV3");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("System", "AuthorizeV3"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn authorize_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::system_authorize_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::system_authorize_v2::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/System/AuthorizeV2");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("System", "AuthorizeV2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_player_status_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::system_get_player_status_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::system_get_player_status_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/System/GetPlayerStatusV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("System", "GetPlayerStatusV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_player_status_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::system_get_player_status_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::system_get_player_status_v2::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/System/GetPlayerStatusV2");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("System", "GetPlayerStatusV2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_player_status_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::system_set_player_status_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::system_set_player_status_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/System/SetPlayerStatusV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("System", "SetPlayerStatusV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_player_status_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::system_delete_player_status_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::system_delete_player_status_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/System/DeletePlayerStatusV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("System", "DeletePlayerStatusV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_realms_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::get_realms_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::get_realms_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/System/GetRealmsV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("System", "GetRealmsV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod system_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SystemServer.
    #[async_trait]
    pub trait System: Send + Sync + 'static {
        async fn authorize_cn_v1(
            &self,
            request: tonic::Request<super::system_authorize_cn_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::system_authorize_cn_v1::Response>,
            tonic::Status,
        >;
        async fn authorize_v3(
            &self,
            request: tonic::Request<super::system_authorize_v3::Request>,
        ) -> std::result::Result<
            tonic::Response<super::system_authorize_v3::Response>,
            tonic::Status,
        >;
        async fn authorize_v2(
            &self,
            request: tonic::Request<super::system_authorize_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::system_authorize_v2::Response>,
            tonic::Status,
        >;
        async fn get_player_status_v1(
            &self,
            request: tonic::Request<super::system_get_player_status_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::system_get_player_status_v1::Response>,
            tonic::Status,
        >;
        async fn get_player_status_v2(
            &self,
            request: tonic::Request<super::system_get_player_status_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::system_get_player_status_v2::Response>,
            tonic::Status,
        >;
        async fn set_player_status_v1(
            &self,
            request: tonic::Request<super::system_set_player_status_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::system_set_player_status_v1::Response>,
            tonic::Status,
        >;
        async fn delete_player_status_v1(
            &self,
            request: tonic::Request<super::system_delete_player_status_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::system_delete_player_status_v1::Response>,
            tonic::Status,
        >;
        async fn get_realms_v1(
            &self,
            request: tonic::Request<super::get_realms_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::get_realms_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct SystemServer<T: System> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: System> SystemServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SystemServer<T>
    where
        T: System,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/System/AuthorizeCnV1" => {
                    #[allow(non_camel_case_types)]
                    struct AuthorizeCnV1Svc<T: System>(pub Arc<T>);
                    impl<
                        T: System,
                    > tonic::server::UnaryService<super::system_authorize_cn_v1::Request>
                    for AuthorizeCnV1Svc<T> {
                        type Response = super::system_authorize_cn_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::system_authorize_cn_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).authorize_cn_v1(request).await
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
                        let inner = inner.0;
                        let method = AuthorizeCnV1Svc(inner);
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
                "/System/AuthorizeV3" => {
                    #[allow(non_camel_case_types)]
                    struct AuthorizeV3Svc<T: System>(pub Arc<T>);
                    impl<
                        T: System,
                    > tonic::server::UnaryService<super::system_authorize_v3::Request>
                    for AuthorizeV3Svc<T> {
                        type Response = super::system_authorize_v3::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::system_authorize_v3::Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).authorize_v3(request).await
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
                        let inner = inner.0;
                        let method = AuthorizeV3Svc(inner);
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
                "/System/AuthorizeV2" => {
                    #[allow(non_camel_case_types)]
                    struct AuthorizeV2Svc<T: System>(pub Arc<T>);
                    impl<
                        T: System,
                    > tonic::server::UnaryService<super::system_authorize_v2::Request>
                    for AuthorizeV2Svc<T> {
                        type Response = super::system_authorize_v2::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::system_authorize_v2::Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).authorize_v2(request).await
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
                        let inner = inner.0;
                        let method = AuthorizeV2Svc(inner);
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
                "/System/GetPlayerStatusV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetPlayerStatusV1Svc<T: System>(pub Arc<T>);
                    impl<
                        T: System,
                    > tonic::server::UnaryService<
                        super::system_get_player_status_v1::Request,
                    > for GetPlayerStatusV1Svc<T> {
                        type Response = super::system_get_player_status_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::system_get_player_status_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_player_status_v1(request).await
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
                        let inner = inner.0;
                        let method = GetPlayerStatusV1Svc(inner);
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
                "/System/GetPlayerStatusV2" => {
                    #[allow(non_camel_case_types)]
                    struct GetPlayerStatusV2Svc<T: System>(pub Arc<T>);
                    impl<
                        T: System,
                    > tonic::server::UnaryService<
                        super::system_get_player_status_v2::Request,
                    > for GetPlayerStatusV2Svc<T> {
                        type Response = super::system_get_player_status_v2::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::system_get_player_status_v2::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_player_status_v2(request).await
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
                        let inner = inner.0;
                        let method = GetPlayerStatusV2Svc(inner);
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
                "/System/SetPlayerStatusV1" => {
                    #[allow(non_camel_case_types)]
                    struct SetPlayerStatusV1Svc<T: System>(pub Arc<T>);
                    impl<
                        T: System,
                    > tonic::server::UnaryService<
                        super::system_set_player_status_v1::Request,
                    > for SetPlayerStatusV1Svc<T> {
                        type Response = super::system_set_player_status_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::system_set_player_status_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_player_status_v1(request).await
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
                        let inner = inner.0;
                        let method = SetPlayerStatusV1Svc(inner);
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
                "/System/DeletePlayerStatusV1" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePlayerStatusV1Svc<T: System>(pub Arc<T>);
                    impl<
                        T: System,
                    > tonic::server::UnaryService<
                        super::system_delete_player_status_v1::Request,
                    > for DeletePlayerStatusV1Svc<T> {
                        type Response = super::system_delete_player_status_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::system_delete_player_status_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_player_status_v1(request).await
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
                        let inner = inner.0;
                        let method = DeletePlayerStatusV1Svc(inner);
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
                "/System/GetRealmsV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetRealmsV1Svc<T: System>(pub Arc<T>);
                    impl<
                        T: System,
                    > tonic::server::UnaryService<super::get_realms_v1::Request>
                    for GetRealmsV1Svc<T> {
                        type Response = super::get_realms_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::get_realms_v1::Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_realms_v1(request).await
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
                        let inner = inner.0;
                        let method = GetRealmsV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: System> Clone for SystemServer<T> {
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
    impl<T: System> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: System> tonic::server::NamedService for SystemServer<T> {
        const NAME: &'static str = "System";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerInventoryGetInventoriesV1 {}
/// Nested message and enum types in `PlayerInventoryGetInventoriesV1`.
pub mod player_inventory_get_inventories_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, optional, tag = "1")]
        pub criterion: ::core::option::Option<
            super::super::resource::player_inventory::v1::Criterion,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerInventoryGetReceivedInventoriesV1 {}
/// Nested message and enum types in `PlayerInventoryGetReceivedInventoriesV1`.
pub mod player_inventory_get_received_inventories_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(fixed32, tag = "1")]
        pub count: u32,
        #[prost(string, repeated, tag = "2")]
        pub search_labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, tag = "3")]
        pub page_token: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub received_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::ReceivedPlayerInventory,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerInventoryReceiveV1 {}
/// Nested message and enum types in `PlayerInventoryReceiveV1`.
pub mod player_inventory_receive_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub delete_inventory_ids: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        #[prost(message, repeated, tag = "2")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "3")]
        pub next_revision: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub previous_revision: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "5")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "2")]
        pub revision: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "3")]
        pub wallet: ::core::option::Option<super::super::resource::wallet::v3::Wallet>,
        #[prost(message, repeated, tag = "4")]
        pub player_key_values: ::prost::alloc::vec::Vec<
            super::super::resource::player_key_value::v1::PlayerKeyValue,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerInventoryGetInventoriesAndCountV1 {}
/// Nested message and enum types in `PlayerInventoryGetInventoriesAndCountV1`.
pub mod player_inventory_get_inventories_and_count_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, optional, tag = "1")]
        pub criterion: ::core::option::Option<
            super::super::resource::player_inventory::v1::Criterion,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(sint64, tag = "2")]
        pub inventories_count: i64,
        #[prost(string, tag = "3")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
/// Generated client implementations.
pub mod player_inventory_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PlayerInventoryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayerInventoryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PlayerInventoryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> PlayerInventoryClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            PlayerInventoryClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_inventories_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_inventory_get_inventories_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_inventory_get_inventories_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerInventory/GetInventoriesV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerInventory", "GetInventoriesV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_received_inventories_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_inventory_get_received_inventories_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_inventory_get_received_inventories_v1::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerInventory/GetReceivedInventoriesV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerInventory", "GetReceivedInventoriesV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn receive_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::player_inventory_receive_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_inventory_receive_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerInventory/ReceiveV1",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("PlayerInventory", "ReceiveV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_inventories_and_count_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_inventory_get_inventories_and_count_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_inventory_get_inventories_and_count_v1::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerInventory/GetInventoriesAndCountV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerInventory", "GetInventoriesAndCountV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod player_inventory_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PlayerInventoryServer.
    #[async_trait]
    pub trait PlayerInventory: Send + Sync + 'static {
        async fn get_inventories_v1(
            &self,
            request: tonic::Request<super::player_inventory_get_inventories_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_inventory_get_inventories_v1::Response>,
            tonic::Status,
        >;
        async fn get_received_inventories_v1(
            &self,
            request: tonic::Request<
                super::player_inventory_get_received_inventories_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_inventory_get_received_inventories_v1::Response,
            >,
            tonic::Status,
        >;
        async fn receive_v1(
            &self,
            request: tonic::Request<super::player_inventory_receive_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_inventory_receive_v1::Response>,
            tonic::Status,
        >;
        async fn get_inventories_and_count_v1(
            &self,
            request: tonic::Request<
                super::player_inventory_get_inventories_and_count_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_inventory_get_inventories_and_count_v1::Response,
            >,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PlayerInventoryServer<T: PlayerInventory> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PlayerInventory> PlayerInventoryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlayerInventoryServer<T>
    where
        T: PlayerInventory,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/PlayerInventory/GetInventoriesV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetInventoriesV1Svc<T: PlayerInventory>(pub Arc<T>);
                    impl<
                        T: PlayerInventory,
                    > tonic::server::UnaryService<
                        super::player_inventory_get_inventories_v1::Request,
                    > for GetInventoriesV1Svc<T> {
                        type Response = super::player_inventory_get_inventories_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_inventory_get_inventories_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_inventories_v1(request).await
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
                        let inner = inner.0;
                        let method = GetInventoriesV1Svc(inner);
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
                "/PlayerInventory/GetReceivedInventoriesV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetReceivedInventoriesV1Svc<T: PlayerInventory>(pub Arc<T>);
                    impl<
                        T: PlayerInventory,
                    > tonic::server::UnaryService<
                        super::player_inventory_get_received_inventories_v1::Request,
                    > for GetReceivedInventoriesV1Svc<T> {
                        type Response = super::player_inventory_get_received_inventories_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_inventory_get_received_inventories_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_received_inventories_v1(request).await
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
                        let inner = inner.0;
                        let method = GetReceivedInventoriesV1Svc(inner);
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
                "/PlayerInventory/ReceiveV1" => {
                    #[allow(non_camel_case_types)]
                    struct ReceiveV1Svc<T: PlayerInventory>(pub Arc<T>);
                    impl<
                        T: PlayerInventory,
                    > tonic::server::UnaryService<
                        super::player_inventory_receive_v1::Request,
                    > for ReceiveV1Svc<T> {
                        type Response = super::player_inventory_receive_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_inventory_receive_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).receive_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReceiveV1Svc(inner);
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
                "/PlayerInventory/GetInventoriesAndCountV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetInventoriesAndCountV1Svc<T: PlayerInventory>(pub Arc<T>);
                    impl<
                        T: PlayerInventory,
                    > tonic::server::UnaryService<
                        super::player_inventory_get_inventories_and_count_v1::Request,
                    > for GetInventoriesAndCountV1Svc<T> {
                        type Response = super::player_inventory_get_inventories_and_count_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_inventory_get_inventories_and_count_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_inventories_and_count_v1(request).await
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
                        let inner = inner.0;
                        let method = GetInventoriesAndCountV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PlayerInventory> Clone for PlayerInventoryServer<T> {
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
    impl<T: PlayerInventory> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PlayerInventory> tonic::server::NamedService for PlayerInventoryServer<T> {
        const NAME: &'static str = "PlayerInventory";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameProductGetAvailableV1 {}
/// Nested message and enum types in `GameProductGetAvailableV1`.
pub mod game_product_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "2")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub game_products: ::prost::alloc::vec::Vec<
            super::super::resource::game_product::v1::GameProduct,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameProductPurchaseV1 {}
/// Nested message and enum types in `GameProductPurchaseV1`.
pub mod game_product_purchase_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub game_product_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(enumeration = "request::ResourceType", tag = "3")]
        pub resource_type: i32,
    }
    /// Nested message and enum types in `Request`.
    pub mod request {
        #[derive(serde::Serialize, serde::Deserialize)]
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
        pub enum ResourceType {
            VirtualCurrency = 0,
            PlayerKeyValue = 1,
        }
        impl ResourceType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResourceType::VirtualCurrency => "VIRTUAL_CURRENCY",
                    ResourceType::PlayerKeyValue => "PLAYER_KEY_VALUE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "VIRTUAL_CURRENCY" => Some(Self::VirtualCurrency),
                    "PLAYER_KEY_VALUE" => Some(Self::PlayerKeyValue),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub player_inventory: ::core::option::Option<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, optional, tag = "3")]
        pub game_product: ::core::option::Option<
            super::super::resource::game_product::v1::GameProduct,
        >,
        #[prost(message, optional, tag = "4")]
        pub wallet: ::core::option::Option<super::super::resource::wallet::v3::Wallet>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameProductPurchaseV2 {}
/// Nested message and enum types in `GameProductPurchaseV2`.
pub mod game_product_purchase_v2 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub game_product_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(enumeration = "request::ResourceType", tag = "3")]
        pub resource_type: i32,
        #[prost(sint64, tag = "4")]
        pub purchase_num: i64,
        #[prost(message, repeated, tag = "5")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    /// Nested message and enum types in `Request`.
    pub mod request {
        #[derive(serde::Serialize, serde::Deserialize)]
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
        pub enum ResourceType {
            VirtualCurrency = 0,
            PlayerKeyValue = 1,
        }
        impl ResourceType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResourceType::VirtualCurrency => "VIRTUAL_CURRENCY",
                    ResourceType::PlayerKeyValue => "PLAYER_KEY_VALUE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "VIRTUAL_CURRENCY" => Some(Self::VirtualCurrency),
                    "PLAYER_KEY_VALUE" => Some(Self::PlayerKeyValue),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, optional, tag = "3")]
        pub game_product: ::core::option::Option<
            super::super::resource::game_product::v1::GameProduct,
        >,
        #[prost(message, optional, tag = "4")]
        pub wallet: ::core::option::Option<super::super::resource::wallet::v3::Wallet>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameProductPurchaseAndSavePlayerStorageV1 {}
/// Nested message and enum types in `GameProductPurchaseAndSavePlayerStorageV1`.
pub mod game_product_purchase_and_save_player_storage_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub game_product_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "3")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "4")]
        pub next_revision: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub previous_revision: ::prost::alloc::string::String,
        #[prost(enumeration = "request::ResourceType", tag = "6")]
        pub resource_type: i32,
    }
    /// Nested message and enum types in `Request`.
    pub mod request {
        #[derive(serde::Serialize, serde::Deserialize)]
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
        pub enum ResourceType {
            VirtualCurrency = 0,
            PlayerKeyValue = 1,
        }
        impl ResourceType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResourceType::VirtualCurrency => "VIRTUAL_CURRENCY",
                    ResourceType::PlayerKeyValue => "PLAYER_KEY_VALUE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "VIRTUAL_CURRENCY" => Some(Self::VirtualCurrency),
                    "PLAYER_KEY_VALUE" => Some(Self::PlayerKeyValue),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub player_inventory: ::core::option::Option<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, optional, tag = "3")]
        pub game_product: ::core::option::Option<
            super::super::resource::game_product::v1::GameProduct,
        >,
        #[prost(message, optional, tag = "4")]
        pub wallet: ::core::option::Option<super::super::resource::wallet::v3::Wallet>,
        #[prost(message, repeated, tag = "5")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "6")]
        pub revision: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameProductPurchaseAndSavePlayerStorageV2 {}
/// Nested message and enum types in `GameProductPurchaseAndSavePlayerStorageV2`.
pub mod game_product_purchase_and_save_player_storage_v2 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub game_product_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "3")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "4")]
        pub next_revision: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub previous_revision: ::prost::alloc::string::String,
        #[prost(enumeration = "request::ResourceType", tag = "6")]
        pub resource_type: i32,
        #[prost(sint64, tag = "7")]
        pub purchase_num: i64,
        #[prost(message, repeated, tag = "8")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    /// Nested message and enum types in `Request`.
    pub mod request {
        #[derive(serde::Serialize, serde::Deserialize)]
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
        pub enum ResourceType {
            VirtualCurrency = 0,
            PlayerKeyValue = 1,
        }
        impl ResourceType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResourceType::VirtualCurrency => "VIRTUAL_CURRENCY",
                    ResourceType::PlayerKeyValue => "PLAYER_KEY_VALUE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "VIRTUAL_CURRENCY" => Some(Self::VirtualCurrency),
                    "PLAYER_KEY_VALUE" => Some(Self::PlayerKeyValue),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, optional, tag = "3")]
        pub game_product: ::core::option::Option<
            super::super::resource::game_product::v1::GameProduct,
        >,
        #[prost(message, optional, tag = "4")]
        pub wallet: ::core::option::Option<super::super::resource::wallet::v3::Wallet>,
        #[prost(message, repeated, tag = "5")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "6")]
        pub revision: ::prost::alloc::string::String,
    }
}
/// Generated client implementations.
pub mod game_product_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct GameProductClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GameProductClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GameProductClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> GameProductClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            GameProductClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::game_product_get_available_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::game_product_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/GameProduct/GetAvailableV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("GameProduct", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn purchase_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::game_product_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::game_product_purchase_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/GameProduct/PurchaseV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("GameProduct", "PurchaseV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn purchase_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::game_product_purchase_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::game_product_purchase_v2::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/GameProduct/PurchaseV2");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("GameProduct", "PurchaseV2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn purchase_and_save_player_storage_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::game_product_purchase_and_save_player_storage_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::game_product_purchase_and_save_player_storage_v1::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/GameProduct/PurchaseAndSavePlayerStorageV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("GameProduct", "PurchaseAndSavePlayerStorageV1"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn purchase_and_save_player_storage_v2(
            &mut self,
            request: impl tonic::IntoRequest<
                super::game_product_purchase_and_save_player_storage_v2::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::game_product_purchase_and_save_player_storage_v2::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/GameProduct/PurchaseAndSavePlayerStorageV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("GameProduct", "PurchaseAndSavePlayerStorageV2"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod game_product_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GameProductServer.
    #[async_trait]
    pub trait GameProduct: Send + Sync + 'static {
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::game_product_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::game_product_get_available_v1::Response>,
            tonic::Status,
        >;
        async fn purchase_v1(
            &self,
            request: tonic::Request<super::game_product_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::game_product_purchase_v1::Response>,
            tonic::Status,
        >;
        async fn purchase_v2(
            &self,
            request: tonic::Request<super::game_product_purchase_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::game_product_purchase_v2::Response>,
            tonic::Status,
        >;
        async fn purchase_and_save_player_storage_v1(
            &self,
            request: tonic::Request<
                super::game_product_purchase_and_save_player_storage_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::game_product_purchase_and_save_player_storage_v1::Response,
            >,
            tonic::Status,
        >;
        async fn purchase_and_save_player_storage_v2(
            &self,
            request: tonic::Request<
                super::game_product_purchase_and_save_player_storage_v2::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::game_product_purchase_and_save_player_storage_v2::Response,
            >,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct GameProductServer<T: GameProduct> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GameProduct> GameProductServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GameProductServer<T>
    where
        T: GameProduct,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/GameProduct/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: GameProduct>(pub Arc<T>);
                    impl<
                        T: GameProduct,
                    > tonic::server::UnaryService<
                        super::game_product_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::game_product_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::game_product_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                "/GameProduct/PurchaseV1" => {
                    #[allow(non_camel_case_types)]
                    struct PurchaseV1Svc<T: GameProduct>(pub Arc<T>);
                    impl<
                        T: GameProduct,
                    > tonic::server::UnaryService<
                        super::game_product_purchase_v1::Request,
                    > for PurchaseV1Svc<T> {
                        type Response = super::game_product_purchase_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::game_product_purchase_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).purchase_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PurchaseV1Svc(inner);
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
                "/GameProduct/PurchaseV2" => {
                    #[allow(non_camel_case_types)]
                    struct PurchaseV2Svc<T: GameProduct>(pub Arc<T>);
                    impl<
                        T: GameProduct,
                    > tonic::server::UnaryService<
                        super::game_product_purchase_v2::Request,
                    > for PurchaseV2Svc<T> {
                        type Response = super::game_product_purchase_v2::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::game_product_purchase_v2::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).purchase_v2(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PurchaseV2Svc(inner);
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
                "/GameProduct/PurchaseAndSavePlayerStorageV1" => {
                    #[allow(non_camel_case_types)]
                    struct PurchaseAndSavePlayerStorageV1Svc<T: GameProduct>(pub Arc<T>);
                    impl<
                        T: GameProduct,
                    > tonic::server::UnaryService<
                        super::game_product_purchase_and_save_player_storage_v1::Request,
                    > for PurchaseAndSavePlayerStorageV1Svc<T> {
                        type Response = super::game_product_purchase_and_save_player_storage_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::game_product_purchase_and_save_player_storage_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).purchase_and_save_player_storage_v1(request).await
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
                        let inner = inner.0;
                        let method = PurchaseAndSavePlayerStorageV1Svc(inner);
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
                "/GameProduct/PurchaseAndSavePlayerStorageV2" => {
                    #[allow(non_camel_case_types)]
                    struct PurchaseAndSavePlayerStorageV2Svc<T: GameProduct>(pub Arc<T>);
                    impl<
                        T: GameProduct,
                    > tonic::server::UnaryService<
                        super::game_product_purchase_and_save_player_storage_v2::Request,
                    > for PurchaseAndSavePlayerStorageV2Svc<T> {
                        type Response = super::game_product_purchase_and_save_player_storage_v2::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::game_product_purchase_and_save_player_storage_v2::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).purchase_and_save_player_storage_v2(request).await
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
                        let inner = inner.0;
                        let method = PurchaseAndSavePlayerStorageV2Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: GameProduct> Clone for GameProductServer<T> {
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
    impl<T: GameProduct> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GameProduct> tonic::server::NamedService for GameProductServer<T> {
        const NAME: &'static str = "GameProduct";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPreferenceGetPreferenceV1 {}
/// Nested message and enum types in `PlayerPreferenceGetPreferenceV1`.
pub mod player_preference_get_preference_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub preference: ::core::option::Option<
            super::super::resource::player_preference::v1::PlayerPreference,
        >,
        #[prost(string, tag = "2")]
        pub player_short_id: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPreferenceSetPreferenceV1 {}
/// Nested message and enum types in `PlayerPreferenceSetPreferenceV1`.
pub mod player_preference_set_preference_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, optional, tag = "1")]
        pub preference: ::core::option::Option<
            super::super::resource::player_preference::v1::PlayerPreference,
        >,
        #[prost(message, repeated, tag = "2")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub preference: ::core::option::Option<
            super::super::resource::player_preference::v1::PlayerPreference,
        >,
        #[prost(string, tag = "2")]
        pub player_short_id: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPreferenceSetPreferenceAndSavePlayerStorageV1 {}
/// Nested message and enum types in `PlayerPreferenceSetPreferenceAndSavePlayerStorageV1`.
pub mod player_preference_set_preference_and_save_player_storage_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, optional, tag = "1")]
        pub preference: ::core::option::Option<
            super::super::resource::player_preference::v1::PlayerPreference,
        >,
        #[prost(message, repeated, tag = "2")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "3")]
        pub next_revision: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub previous_revision: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub preference: ::core::option::Option<
            super::super::resource::player_preference::v1::PlayerPreference,
        >,
        #[prost(string, tag = "2")]
        pub player_short_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "3")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "4")]
        pub revision: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPreferenceSetPreferenceAndSavePlayerStorageV2 {}
/// Nested message and enum types in `PlayerPreferenceSetPreferenceAndSavePlayerStorageV2`.
pub mod player_preference_set_preference_and_save_player_storage_v2 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, optional, tag = "1")]
        pub preference: ::core::option::Option<
            super::super::resource::player_preference::v1::PlayerPreference,
        >,
        #[prost(message, repeated, tag = "2")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "3")]
        pub next_revision: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub previous_revision: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "5")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub preference: ::core::option::Option<
            super::super::resource::player_preference::v1::PlayerPreference,
        >,
        #[prost(string, tag = "2")]
        pub player_tmp_key: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "3")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "4")]
        pub revision: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPreferenceGetMonthlyBillingLimitV1 {}
/// Nested message and enum types in `PlayerPreferenceGetMonthlyBillingLimitV1`.
pub mod player_preference_get_monthly_billing_limit_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(int64, tag = "1")]
        pub min_age: i64,
        #[prost(int64, tag = "2")]
        pub max_age: i64,
        #[prost(int64, tag = "3")]
        pub monthly_billing_limit: i64,
        #[prost(bool, tag = "4")]
        pub exits_monthly_billing_limit: bool,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPreferenceGetMonthlyPurchaseSummaryV1 {}
/// Nested message and enum types in `PlayerPreferenceGetMonthlyPurchaseSummaryV1`.
pub mod player_preference_get_monthly_purchase_summary_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(int64, tag = "1")]
        pub monthly_billing_limit: i64,
        #[prost(int64, tag = "2")]
        pub paid_amount: i64,
    }
}
/// Generated client implementations.
pub mod player_preference_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PlayerPreferenceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayerPreferenceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PlayerPreferenceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> PlayerPreferenceClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            PlayerPreferenceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_preference_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_preference_get_preference_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_preference_get_preference_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerPreference/GetPreferenceV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerPreference", "GetPreferenceV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_preference_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_preference_set_preference_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_preference_set_preference_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerPreference/SetPreferenceV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerPreference", "SetPreferenceV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_preference_and_save_player_storage_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_preference_set_preference_and_save_player_storage_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_preference_set_preference_and_save_player_storage_v1::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerPreference/SetPreferenceAndSavePlayerStorageV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "PlayerPreference",
                        "SetPreferenceAndSavePlayerStorageV1",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_preference_and_save_player_storage_v2(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_preference_set_preference_and_save_player_storage_v2::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_preference_set_preference_and_save_player_storage_v2::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerPreference/SetPreferenceAndSavePlayerStorageV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "PlayerPreference",
                        "SetPreferenceAndSavePlayerStorageV2",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_monthly_billing_limit_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_preference_get_monthly_billing_limit_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_preference_get_monthly_billing_limit_v1::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerPreference/GetMonthlyBillingLimitV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerPreference", "GetMonthlyBillingLimitV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_monthly_purchase_summary_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_preference_get_monthly_purchase_summary_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_preference_get_monthly_purchase_summary_v1::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerPreference/GetMonthlyPurchaseSummaryV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("PlayerPreference", "GetMonthlyPurchaseSummaryV1"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod player_preference_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PlayerPreferenceServer.
    #[async_trait]
    pub trait PlayerPreference: Send + Sync + 'static {
        async fn get_preference_v1(
            &self,
            request: tonic::Request<super::player_preference_get_preference_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_preference_get_preference_v1::Response>,
            tonic::Status,
        >;
        async fn set_preference_v1(
            &self,
            request: tonic::Request<super::player_preference_set_preference_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_preference_set_preference_v1::Response>,
            tonic::Status,
        >;
        async fn set_preference_and_save_player_storage_v1(
            &self,
            request: tonic::Request<
                super::player_preference_set_preference_and_save_player_storage_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_preference_set_preference_and_save_player_storage_v1::Response,
            >,
            tonic::Status,
        >;
        async fn set_preference_and_save_player_storage_v2(
            &self,
            request: tonic::Request<
                super::player_preference_set_preference_and_save_player_storage_v2::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_preference_set_preference_and_save_player_storage_v2::Response,
            >,
            tonic::Status,
        >;
        async fn get_monthly_billing_limit_v1(
            &self,
            request: tonic::Request<
                super::player_preference_get_monthly_billing_limit_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_preference_get_monthly_billing_limit_v1::Response,
            >,
            tonic::Status,
        >;
        async fn get_monthly_purchase_summary_v1(
            &self,
            request: tonic::Request<
                super::player_preference_get_monthly_purchase_summary_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_preference_get_monthly_purchase_summary_v1::Response,
            >,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PlayerPreferenceServer<T: PlayerPreference> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PlayerPreference> PlayerPreferenceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlayerPreferenceServer<T>
    where
        T: PlayerPreference,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/PlayerPreference/GetPreferenceV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetPreferenceV1Svc<T: PlayerPreference>(pub Arc<T>);
                    impl<
                        T: PlayerPreference,
                    > tonic::server::UnaryService<
                        super::player_preference_get_preference_v1::Request,
                    > for GetPreferenceV1Svc<T> {
                        type Response = super::player_preference_get_preference_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_preference_get_preference_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_preference_v1(request).await
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
                        let inner = inner.0;
                        let method = GetPreferenceV1Svc(inner);
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
                "/PlayerPreference/SetPreferenceV1" => {
                    #[allow(non_camel_case_types)]
                    struct SetPreferenceV1Svc<T: PlayerPreference>(pub Arc<T>);
                    impl<
                        T: PlayerPreference,
                    > tonic::server::UnaryService<
                        super::player_preference_set_preference_v1::Request,
                    > for SetPreferenceV1Svc<T> {
                        type Response = super::player_preference_set_preference_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_preference_set_preference_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_preference_v1(request).await
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
                        let inner = inner.0;
                        let method = SetPreferenceV1Svc(inner);
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
                "/PlayerPreference/SetPreferenceAndSavePlayerStorageV1" => {
                    #[allow(non_camel_case_types)]
                    struct SetPreferenceAndSavePlayerStorageV1Svc<T: PlayerPreference>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PlayerPreference,
                    > tonic::server::UnaryService<
                        super::player_preference_set_preference_and_save_player_storage_v1::Request,
                    > for SetPreferenceAndSavePlayerStorageV1Svc<T> {
                        type Response = super::player_preference_set_preference_and_save_player_storage_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_preference_set_preference_and_save_player_storage_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .set_preference_and_save_player_storage_v1(request)
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
                        let inner = inner.0;
                        let method = SetPreferenceAndSavePlayerStorageV1Svc(inner);
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
                "/PlayerPreference/SetPreferenceAndSavePlayerStorageV2" => {
                    #[allow(non_camel_case_types)]
                    struct SetPreferenceAndSavePlayerStorageV2Svc<T: PlayerPreference>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PlayerPreference,
                    > tonic::server::UnaryService<
                        super::player_preference_set_preference_and_save_player_storage_v2::Request,
                    > for SetPreferenceAndSavePlayerStorageV2Svc<T> {
                        type Response = super::player_preference_set_preference_and_save_player_storage_v2::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_preference_set_preference_and_save_player_storage_v2::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .set_preference_and_save_player_storage_v2(request)
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
                        let inner = inner.0;
                        let method = SetPreferenceAndSavePlayerStorageV2Svc(inner);
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
                "/PlayerPreference/GetMonthlyBillingLimitV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetMonthlyBillingLimitV1Svc<T: PlayerPreference>(pub Arc<T>);
                    impl<
                        T: PlayerPreference,
                    > tonic::server::UnaryService<
                        super::player_preference_get_monthly_billing_limit_v1::Request,
                    > for GetMonthlyBillingLimitV1Svc<T> {
                        type Response = super::player_preference_get_monthly_billing_limit_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_preference_get_monthly_billing_limit_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_monthly_billing_limit_v1(request).await
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
                        let inner = inner.0;
                        let method = GetMonthlyBillingLimitV1Svc(inner);
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
                "/PlayerPreference/GetMonthlyPurchaseSummaryV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetMonthlyPurchaseSummaryV1Svc<T: PlayerPreference>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PlayerPreference,
                    > tonic::server::UnaryService<
                        super::player_preference_get_monthly_purchase_summary_v1::Request,
                    > for GetMonthlyPurchaseSummaryV1Svc<T> {
                        type Response = super::player_preference_get_monthly_purchase_summary_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_preference_get_monthly_purchase_summary_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_monthly_purchase_summary_v1(request).await
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
                        let inner = inner.0;
                        let method = GetMonthlyPurchaseSummaryV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PlayerPreference> Clone for PlayerPreferenceServer<T> {
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
    impl<T: PlayerPreference> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PlayerPreference> tonic::server::NamedService for PlayerPreferenceServer<T> {
        const NAME: &'static str = "PlayerPreference";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerAchievementGetAvailableV1 {}
/// Nested message and enum types in `PlayerAchievementGetAvailableV1`.
pub mod player_achievement_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, optional, tag = "1")]
        pub criterion: ::core::option::Option<
            super::super::resource::achievement::v1::Criterion,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_achievements: ::prost::alloc::vec::Vec<
            super::super::resource::player_achievement::v1::PlayerAchievement,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerAchievementGetAvailableByIDsV1 {}
/// Nested message and enum types in `PlayerAchievementGetAvailableByIDsV1`.
pub mod player_achievement_get_available_by_i_ds_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub achievement_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_achievements: ::prost::alloc::vec::Vec<
            super::super::resource::player_achievement::v1::PlayerAchievement,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerAchievementUnlockV1 {}
/// Nested message and enum types in `PlayerAchievementUnlockV1`.
pub mod player_achievement_unlock_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub achievement_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_achievements: ::prost::alloc::vec::Vec<
            super::super::resource::player_achievement::v1::PlayerAchievement,
        >,
        #[prost(message, repeated, tag = "2")]
        pub inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerAchievementUnlockAndSavePlayerStorageV1 {}
/// Nested message and enum types in `PlayerAchievementUnlockAndSavePlayerStorageV1`.
pub mod player_achievement_unlock_and_save_player_storage_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub achievement_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "3")]
        pub next_revision: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub previous_revision: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_achievements: ::prost::alloc::vec::Vec<
            super::super::resource::player_achievement::v1::PlayerAchievement,
        >,
        #[prost(message, repeated, tag = "2")]
        pub inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, repeated, tag = "3")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "4")]
        pub revision: ::prost::alloc::string::String,
    }
}
/// Generated client implementations.
pub mod player_achievement_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PlayerAchievementClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayerAchievementClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PlayerAchievementClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> PlayerAchievementClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            PlayerAchievementClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_achievement_get_available_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_achievement_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerAchievement/GetAvailableV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerAchievement", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_available_by_i_ds_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_achievement_get_available_by_i_ds_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_achievement_get_available_by_i_ds_v1::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerAchievement/GetAvailableByIDsV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerAchievement", "GetAvailableByIDsV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlock_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_achievement_unlock_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_achievement_unlock_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerAchievement/UnlockV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerAchievement", "UnlockV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlock_and_save_player_storage_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_achievement_unlock_and_save_player_storage_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_achievement_unlock_and_save_player_storage_v1::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerAchievement/UnlockAndSavePlayerStorageV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("PlayerAchievement", "UnlockAndSavePlayerStorageV1"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod player_achievement_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PlayerAchievementServer.
    #[async_trait]
    pub trait PlayerAchievement: Send + Sync + 'static {
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::player_achievement_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_achievement_get_available_v1::Response>,
            tonic::Status,
        >;
        async fn get_available_by_i_ds_v1(
            &self,
            request: tonic::Request<
                super::player_achievement_get_available_by_i_ds_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_achievement_get_available_by_i_ds_v1::Response,
            >,
            tonic::Status,
        >;
        async fn unlock_v1(
            &self,
            request: tonic::Request<super::player_achievement_unlock_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_achievement_unlock_v1::Response>,
            tonic::Status,
        >;
        async fn unlock_and_save_player_storage_v1(
            &self,
            request: tonic::Request<
                super::player_achievement_unlock_and_save_player_storage_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_achievement_unlock_and_save_player_storage_v1::Response,
            >,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PlayerAchievementServer<T: PlayerAchievement> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PlayerAchievement> PlayerAchievementServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlayerAchievementServer<T>
    where
        T: PlayerAchievement,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/PlayerAchievement/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: PlayerAchievement>(pub Arc<T>);
                    impl<
                        T: PlayerAchievement,
                    > tonic::server::UnaryService<
                        super::player_achievement_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::player_achievement_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_achievement_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                "/PlayerAchievement/GetAvailableByIDsV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableByIDsV1Svc<T: PlayerAchievement>(pub Arc<T>);
                    impl<
                        T: PlayerAchievement,
                    > tonic::server::UnaryService<
                        super::player_achievement_get_available_by_i_ds_v1::Request,
                    > for GetAvailableByIDsV1Svc<T> {
                        type Response = super::player_achievement_get_available_by_i_ds_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_achievement_get_available_by_i_ds_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_by_i_ds_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableByIDsV1Svc(inner);
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
                "/PlayerAchievement/UnlockV1" => {
                    #[allow(non_camel_case_types)]
                    struct UnlockV1Svc<T: PlayerAchievement>(pub Arc<T>);
                    impl<
                        T: PlayerAchievement,
                    > tonic::server::UnaryService<
                        super::player_achievement_unlock_v1::Request,
                    > for UnlockV1Svc<T> {
                        type Response = super::player_achievement_unlock_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_achievement_unlock_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).unlock_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnlockV1Svc(inner);
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
                "/PlayerAchievement/UnlockAndSavePlayerStorageV1" => {
                    #[allow(non_camel_case_types)]
                    struct UnlockAndSavePlayerStorageV1Svc<T: PlayerAchievement>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PlayerAchievement,
                    > tonic::server::UnaryService<
                        super::player_achievement_unlock_and_save_player_storage_v1::Request,
                    > for UnlockAndSavePlayerStorageV1Svc<T> {
                        type Response = super::player_achievement_unlock_and_save_player_storage_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_achievement_unlock_and_save_player_storage_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).unlock_and_save_player_storage_v1(request).await
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
                        let inner = inner.0;
                        let method = UnlockAndSavePlayerStorageV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PlayerAchievement> Clone for PlayerAchievementServer<T> {
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
    impl<T: PlayerAchievement> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PlayerAchievement> tonic::server::NamedService
    for PlayerAchievementServer<T> {
        const NAME: &'static str = "PlayerAchievement";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateBannedWordV1 {}
/// Nested message and enum types in `ValidateBannedWordV1`.
pub mod validate_banned_word_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub language_code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, tag = "2")]
        pub text: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(bool, tag = "1")]
        pub ok: bool,
        #[prost(string, tag = "2")]
        pub reason: ::prost::alloc::string::String,
    }
}
/// Generated client implementations.
pub mod banned_word_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BannedWordClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BannedWordClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BannedWordClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> BannedWordClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            BannedWordClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn validate_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::validate_banned_word_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::validate_banned_word_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/BannedWord/ValidateV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("BannedWord", "ValidateV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod banned_word_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BannedWordServer.
    #[async_trait]
    pub trait BannedWord: Send + Sync + 'static {
        async fn validate_v1(
            &self,
            request: tonic::Request<super::validate_banned_word_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::validate_banned_word_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct BannedWordServer<T: BannedWord> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BannedWord> BannedWordServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BannedWordServer<T>
    where
        T: BannedWord,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/BannedWord/ValidateV1" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateV1Svc<T: BannedWord>(pub Arc<T>);
                    impl<
                        T: BannedWord,
                    > tonic::server::UnaryService<
                        super::validate_banned_word_v1::Request,
                    > for ValidateV1Svc<T> {
                        type Response = super::validate_banned_word_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::validate_banned_word_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).validate_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ValidateV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: BannedWord> Clone for BannedWordServer<T> {
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
    impl<T: BannedWord> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BannedWord> tonic::server::NamedService for BannedWordServer<T> {
        const NAME: &'static str = "BannedWord";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalLoginBonusGetAvailableV1 {}
/// Nested message and enum types in `TotalLoginBonusGetAvailableV1`.
pub mod total_login_bonus_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub total_login_bonuses: ::prost::alloc::vec::Vec<
            super::super::resource::total_login_bonus::v1::TotalLoginBonus,
        >,
        #[prost(int64, tag = "2")]
        pub total_login_days: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalLoginBonusReceiveV1 {}
/// Nested message and enum types in `TotalLoginBonusReceiveV1`.
pub mod total_login_bonus_receive_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, repeated, tag = "1")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub total_login_bonuses: ::prost::alloc::vec::Vec<
            super::super::resource::total_login_bonus::v1::TotalLoginBonus,
        >,
        #[prost(message, repeated, tag = "2")]
        pub player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(int64, tag = "3")]
        pub total_login_days: i64,
    }
}
/// Generated client implementations.
pub mod total_login_bonus_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct TotalLoginBonusClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TotalLoginBonusClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TotalLoginBonusClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> TotalLoginBonusClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            TotalLoginBonusClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::total_login_bonus_get_available_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::total_login_bonus_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/TotalLoginBonus/GetAvailableV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("TotalLoginBonus", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn receive_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::total_login_bonus_receive_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::total_login_bonus_receive_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/TotalLoginBonus/ReceiveV1",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("TotalLoginBonus", "ReceiveV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod total_login_bonus_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TotalLoginBonusServer.
    #[async_trait]
    pub trait TotalLoginBonus: Send + Sync + 'static {
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::total_login_bonus_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::total_login_bonus_get_available_v1::Response>,
            tonic::Status,
        >;
        async fn receive_v1(
            &self,
            request: tonic::Request<super::total_login_bonus_receive_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::total_login_bonus_receive_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct TotalLoginBonusServer<T: TotalLoginBonus> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TotalLoginBonus> TotalLoginBonusServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TotalLoginBonusServer<T>
    where
        T: TotalLoginBonus,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/TotalLoginBonus/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: TotalLoginBonus>(pub Arc<T>);
                    impl<
                        T: TotalLoginBonus,
                    > tonic::server::UnaryService<
                        super::total_login_bonus_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::total_login_bonus_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::total_login_bonus_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                "/TotalLoginBonus/ReceiveV1" => {
                    #[allow(non_camel_case_types)]
                    struct ReceiveV1Svc<T: TotalLoginBonus>(pub Arc<T>);
                    impl<
                        T: TotalLoginBonus,
                    > tonic::server::UnaryService<
                        super::total_login_bonus_receive_v1::Request,
                    > for ReceiveV1Svc<T> {
                        type Response = super::total_login_bonus_receive_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::total_login_bonus_receive_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).receive_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReceiveV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: TotalLoginBonus> Clone for TotalLoginBonusServer<T> {
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
    impl<T: TotalLoginBonus> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TotalLoginBonus> tonic::server::NamedService for TotalLoginBonusServer<T> {
        const NAME: &'static str = "TotalLoginBonus";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendGetListV1 {}
/// Nested message and enum types in `FriendGetListV1`.
pub mod friend_get_list_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub player_storage_keys: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        #[prost(string, tag = "2")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "3")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub friends: ::prost::alloc::vec::Vec<
            super::super::resource::friend::v1::Player,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendSendRequestV1 {}
/// Nested message and enum types in `FriendSendRequestV1`.
pub mod friend_send_request_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub receiver_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub schema_key: ::prost::alloc::string::String,
        #[prost(bytes = "vec", tag = "3")]
        pub value: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, repeated, tag = "4")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(bool, tag = "1")]
        pub is_approved: bool,
        #[prost(string, tag = "2")]
        pub player_id: ::prost::alloc::string::String,
        #[prost(enumeration = "response::FailureReason", tag = "3")]
        pub failure_reason: i32,
    }
    /// Nested message and enum types in `Response`.
    pub mod response {
        #[derive(serde::Serialize, serde::Deserialize)]
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
        pub enum FailureReason {
            NotFail = 0,
            SenderHasNoFriendVacancy = 1,
            SenderHasNoSendFriendRequestVacancy = 2,
            ReceiverHasNoReceiveFriendRequestVacancy = 5,
            ReceiverNotAllowRequest = 6,
            RerequestHasEnoughTime = 7,
            ReceiverHasNoFriendVacancy = 8,
            ReceiverIsBannedPlayer = 9,
        }
        impl FailureReason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    FailureReason::NotFail => "NOT_FAIL",
                    FailureReason::SenderHasNoFriendVacancy => {
                        "SENDER_HAS_NO_FRIEND_VACANCY"
                    }
                    FailureReason::SenderHasNoSendFriendRequestVacancy => {
                        "SENDER_HAS_NO_SEND_FRIEND_REQUEST_VACANCY"
                    }
                    FailureReason::ReceiverHasNoReceiveFriendRequestVacancy => {
                        "RECEIVER_HAS_NO_RECEIVE_FRIEND_REQUEST_VACANCY"
                    }
                    FailureReason::ReceiverNotAllowRequest => {
                        "RECEIVER_NOT_ALLOW_REQUEST"
                    }
                    FailureReason::RerequestHasEnoughTime => "REREQUEST_HAS_ENOUGH_TIME",
                    FailureReason::ReceiverHasNoFriendVacancy => {
                        "RECEIVER_HAS_NO_FRIEND_VACANCY"
                    }
                    FailureReason::ReceiverIsBannedPlayer => "RECEIVER_IS_BANNED_PLAYER",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "NOT_FAIL" => Some(Self::NotFail),
                    "SENDER_HAS_NO_FRIEND_VACANCY" => {
                        Some(Self::SenderHasNoFriendVacancy)
                    }
                    "SENDER_HAS_NO_SEND_FRIEND_REQUEST_VACANCY" => {
                        Some(Self::SenderHasNoSendFriendRequestVacancy)
                    }
                    "RECEIVER_HAS_NO_RECEIVE_FRIEND_REQUEST_VACANCY" => {
                        Some(Self::ReceiverHasNoReceiveFriendRequestVacancy)
                    }
                    "RECEIVER_NOT_ALLOW_REQUEST" => Some(Self::ReceiverNotAllowRequest),
                    "REREQUEST_HAS_ENOUGH_TIME" => Some(Self::RerequestHasEnoughTime),
                    "RECEIVER_HAS_NO_FRIEND_VACANCY" => {
                        Some(Self::ReceiverHasNoFriendVacancy)
                    }
                    "RECEIVER_IS_BANNED_PLAYER" => Some(Self::ReceiverIsBannedPlayer),
                    _ => None,
                }
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendApproveRequestV1 {}
/// Nested message and enum types in `FriendApproveRequestV1`.
pub mod friend_approve_request_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub player_id: ::prost::alloc::string::String,
        #[prost(bool, tag = "2")]
        pub approve: bool,
        #[prost(message, repeated, tag = "3")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub player_id: ::prost::alloc::string::String,
        #[prost(enumeration = "response::FailureReason", tag = "3")]
        pub failure_reason: i32,
    }
    /// Nested message and enum types in `Response`.
    pub mod response {
        #[derive(serde::Serialize, serde::Deserialize)]
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
        pub enum FailureReason {
            NotFail = 0,
            ReceiverHasNoFriendVacancy = 1,
            SenderHasNoFriendVacancy = 2,
            ReceiverNotAllowRequest = 3,
        }
        impl FailureReason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    FailureReason::NotFail => "NOT_FAIL",
                    FailureReason::ReceiverHasNoFriendVacancy => {
                        "RECEIVER_HAS_NO_FRIEND_VACANCY"
                    }
                    FailureReason::SenderHasNoFriendVacancy => {
                        "SENDER_HAS_NO_FRIEND_VACANCY"
                    }
                    FailureReason::ReceiverNotAllowRequest => {
                        "RECEIVER_NOT_ALLOW_REQUEST"
                    }
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "NOT_FAIL" => Some(Self::NotFail),
                    "RECEIVER_HAS_NO_FRIEND_VACANCY" => {
                        Some(Self::ReceiverHasNoFriendVacancy)
                    }
                    "SENDER_HAS_NO_FRIEND_VACANCY" => {
                        Some(Self::SenderHasNoFriendVacancy)
                    }
                    "RECEIVER_NOT_ALLOW_REQUEST" => Some(Self::ReceiverNotAllowRequest),
                    _ => None,
                }
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendDeleteV1 {}
/// Nested message and enum types in `FriendDeleteV1`.
pub mod friend_delete_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub player_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {}
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendGetReceivedRequestsListV1 {}
/// Nested message and enum types in `FriendGetReceivedRequestsListV1`.
pub mod friend_get_received_requests_list_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub player_storage_keys: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        #[prost(string, tag = "2")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "3")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub received_friend_requests: ::prost::alloc::vec::Vec<
            super::super::resource::friend::v1::ReceivedFriendRequest,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendGetMyRequestsListV1 {}
/// Nested message and enum types in `FriendGetMyRequestsListV1`.
pub mod friend_get_my_requests_list_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub player_storage_keys: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        #[prost(string, tag = "2")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "3")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub my_friend_requests: ::prost::alloc::vec::Vec<
            super::super::resource::friend::v1::MyFriendRequest,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendDeleteMyRequestV1 {}
/// Nested message and enum types in `FriendDeleteMyRequestV1`.
pub mod friend_delete_my_request_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub player_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub player_id: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendApproveRequestsV1 {}
/// Nested message and enum types in `FriendApproveRequestsV1`.
pub mod friend_approve_requests_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub player_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(bool, tag = "2")]
        pub approve: bool,
        #[prost(message, repeated, tag = "3")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, repeated, tag = "1")]
        pub player_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub failures: ::prost::alloc::vec::Vec<response::Failure>,
    }
    /// Nested message and enum types in `Response`.
    pub mod response {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Failure {
            #[prost(string, tag = "1")]
            pub player_id: ::prost::alloc::string::String,
            #[prost(enumeration = "failure::FailureReason", tag = "3")]
            pub failure_reason: i32,
        }
        /// Nested message and enum types in `Failure`.
        pub mod failure {
            #[derive(serde::Serialize, serde::Deserialize)]
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
            pub enum FailureReason {
                ReceiverHasNoFriendVacancy = 0,
                SenderHasNoFriendVacancy = 1,
                ReceiverNotAllowRequest = 2,
                ReceiverHasNoFriendsVacancy = 3,
            }
            impl FailureReason {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        FailureReason::ReceiverHasNoFriendVacancy => {
                            "RECEIVER_HAS_NO_FRIEND_VACANCY"
                        }
                        FailureReason::SenderHasNoFriendVacancy => {
                            "SENDER_HAS_NO_FRIEND_VACANCY"
                        }
                        FailureReason::ReceiverNotAllowRequest => {
                            "RECEIVER_NOT_ALLOW_REQUEST"
                        }
                        FailureReason::ReceiverHasNoFriendsVacancy => {
                            "RECEIVER_HAS_NO_FRIENDS_VACANCY"
                        }
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "RECEIVER_HAS_NO_FRIEND_VACANCY" => {
                            Some(Self::ReceiverHasNoFriendVacancy)
                        }
                        "SENDER_HAS_NO_FRIEND_VACANCY" => {
                            Some(Self::SenderHasNoFriendVacancy)
                        }
                        "RECEIVER_NOT_ALLOW_REQUEST" => {
                            Some(Self::ReceiverNotAllowRequest)
                        }
                        "RECEIVER_HAS_NO_FRIENDS_VACANCY" => {
                            Some(Self::ReceiverHasNoFriendsVacancy)
                        }
                        _ => None,
                    }
                }
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendDeleteMyRequestsV1 {}
/// Nested message and enum types in `FriendDeleteMyRequestsV1`.
pub mod friend_delete_my_requests_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub player_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, repeated, tag = "1")]
        pub player_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Generated client implementations.
pub mod friend_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct FriendClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FriendClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> FriendClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> FriendClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            FriendClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_list_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::friend_get_list_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_get_list_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/Friend/GetListV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("Friend", "GetListV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_request_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::friend_send_request_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_send_request_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/Friend/SendRequestV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("Friend", "SendRequestV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn approve_request_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::friend_approve_request_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_approve_request_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/Friend/ApproveRequestV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("Friend", "ApproveRequestV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_friend_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::friend_delete_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_delete_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/Friend/DeleteFriendV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("Friend", "DeleteFriendV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_received_requests_list_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::friend_get_received_requests_list_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::friend_get_received_requests_list_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Friend/GetReceivedRequestsListV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Friend", "GetReceivedRequestsListV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_sent_requests_list_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::friend_get_my_requests_list_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::friend_get_my_requests_list_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Friend/GetSentRequestsListV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Friend", "GetSentRequestsListV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_sent_request_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::friend_delete_my_request_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_delete_my_request_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Friend/DeleteSentRequestV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Friend", "DeleteSentRequestV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn approve_requests_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::friend_approve_requests_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_approve_requests_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/Friend/ApproveRequestsV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("Friend", "ApproveRequestsV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_sent_requests_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::friend_delete_my_requests_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::friend_delete_my_requests_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Friend/DeleteSentRequestsV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Friend", "DeleteSentRequestsV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod friend_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with FriendServer.
    #[async_trait]
    pub trait Friend: Send + Sync + 'static {
        async fn get_list_v1(
            &self,
            request: tonic::Request<super::friend_get_list_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_get_list_v1::Response>,
            tonic::Status,
        >;
        async fn send_request_v1(
            &self,
            request: tonic::Request<super::friend_send_request_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_send_request_v1::Response>,
            tonic::Status,
        >;
        async fn approve_request_v1(
            &self,
            request: tonic::Request<super::friend_approve_request_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_approve_request_v1::Response>,
            tonic::Status,
        >;
        async fn delete_friend_v1(
            &self,
            request: tonic::Request<super::friend_delete_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_delete_v1::Response>,
            tonic::Status,
        >;
        async fn get_received_requests_list_v1(
            &self,
            request: tonic::Request<super::friend_get_received_requests_list_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_get_received_requests_list_v1::Response>,
            tonic::Status,
        >;
        async fn get_sent_requests_list_v1(
            &self,
            request: tonic::Request<super::friend_get_my_requests_list_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_get_my_requests_list_v1::Response>,
            tonic::Status,
        >;
        async fn delete_sent_request_v1(
            &self,
            request: tonic::Request<super::friend_delete_my_request_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_delete_my_request_v1::Response>,
            tonic::Status,
        >;
        async fn approve_requests_v1(
            &self,
            request: tonic::Request<super::friend_approve_requests_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_approve_requests_v1::Response>,
            tonic::Status,
        >;
        async fn delete_sent_requests_v1(
            &self,
            request: tonic::Request<super::friend_delete_my_requests_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::friend_delete_my_requests_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct FriendServer<T: Friend> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Friend> FriendServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FriendServer<T>
    where
        T: Friend,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/Friend/GetListV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetListV1Svc<T: Friend>(pub Arc<T>);
                    impl<
                        T: Friend,
                    > tonic::server::UnaryService<super::friend_get_list_v1::Request>
                    for GetListV1Svc<T> {
                        type Response = super::friend_get_list_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::friend_get_list_v1::Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_list_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetListV1Svc(inner);
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
                "/Friend/SendRequestV1" => {
                    #[allow(non_camel_case_types)]
                    struct SendRequestV1Svc<T: Friend>(pub Arc<T>);
                    impl<
                        T: Friend,
                    > tonic::server::UnaryService<super::friend_send_request_v1::Request>
                    for SendRequestV1Svc<T> {
                        type Response = super::friend_send_request_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::friend_send_request_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).send_request_v1(request).await
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
                        let inner = inner.0;
                        let method = SendRequestV1Svc(inner);
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
                "/Friend/ApproveRequestV1" => {
                    #[allow(non_camel_case_types)]
                    struct ApproveRequestV1Svc<T: Friend>(pub Arc<T>);
                    impl<
                        T: Friend,
                    > tonic::server::UnaryService<
                        super::friend_approve_request_v1::Request,
                    > for ApproveRequestV1Svc<T> {
                        type Response = super::friend_approve_request_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::friend_approve_request_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).approve_request_v1(request).await
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
                        let inner = inner.0;
                        let method = ApproveRequestV1Svc(inner);
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
                "/Friend/DeleteFriendV1" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteFriendV1Svc<T: Friend>(pub Arc<T>);
                    impl<
                        T: Friend,
                    > tonic::server::UnaryService<super::friend_delete_v1::Request>
                    for DeleteFriendV1Svc<T> {
                        type Response = super::friend_delete_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::friend_delete_v1::Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_friend_v1(request).await
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
                        let inner = inner.0;
                        let method = DeleteFriendV1Svc(inner);
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
                "/Friend/GetReceivedRequestsListV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetReceivedRequestsListV1Svc<T: Friend>(pub Arc<T>);
                    impl<
                        T: Friend,
                    > tonic::server::UnaryService<
                        super::friend_get_received_requests_list_v1::Request,
                    > for GetReceivedRequestsListV1Svc<T> {
                        type Response = super::friend_get_received_requests_list_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::friend_get_received_requests_list_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_received_requests_list_v1(request).await
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
                        let inner = inner.0;
                        let method = GetReceivedRequestsListV1Svc(inner);
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
                "/Friend/GetSentRequestsListV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetSentRequestsListV1Svc<T: Friend>(pub Arc<T>);
                    impl<
                        T: Friend,
                    > tonic::server::UnaryService<
                        super::friend_get_my_requests_list_v1::Request,
                    > for GetSentRequestsListV1Svc<T> {
                        type Response = super::friend_get_my_requests_list_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::friend_get_my_requests_list_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_sent_requests_list_v1(request).await
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
                        let inner = inner.0;
                        let method = GetSentRequestsListV1Svc(inner);
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
                "/Friend/DeleteSentRequestV1" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSentRequestV1Svc<T: Friend>(pub Arc<T>);
                    impl<
                        T: Friend,
                    > tonic::server::UnaryService<
                        super::friend_delete_my_request_v1::Request,
                    > for DeleteSentRequestV1Svc<T> {
                        type Response = super::friend_delete_my_request_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::friend_delete_my_request_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_sent_request_v1(request).await
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
                        let inner = inner.0;
                        let method = DeleteSentRequestV1Svc(inner);
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
                "/Friend/ApproveRequestsV1" => {
                    #[allow(non_camel_case_types)]
                    struct ApproveRequestsV1Svc<T: Friend>(pub Arc<T>);
                    impl<
                        T: Friend,
                    > tonic::server::UnaryService<
                        super::friend_approve_requests_v1::Request,
                    > for ApproveRequestsV1Svc<T> {
                        type Response = super::friend_approve_requests_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::friend_approve_requests_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).approve_requests_v1(request).await
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
                        let inner = inner.0;
                        let method = ApproveRequestsV1Svc(inner);
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
                "/Friend/DeleteSentRequestsV1" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSentRequestsV1Svc<T: Friend>(pub Arc<T>);
                    impl<
                        T: Friend,
                    > tonic::server::UnaryService<
                        super::friend_delete_my_requests_v1::Request,
                    > for DeleteSentRequestsV1Svc<T> {
                        type Response = super::friend_delete_my_requests_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::friend_delete_my_requests_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_sent_requests_v1(request).await
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
                        let inner = inner.0;
                        let method = DeleteSentRequestsV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Friend> Clone for FriendServer<T> {
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
    impl<T: Friend> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Friend> tonic::server::NamedService for FriendServer<T> {
        const NAME: &'static str = "Friend";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerStorageSetEntriesV2 {}
/// Nested message and enum types in `PlayerStorageSetEntriesV2`.
pub mod player_storage_set_entries_v2 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, repeated, tag = "1")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "2")]
        pub next_revision: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub previous_revision: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "4")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "2")]
        pub revision: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerStorageGetEntriesV2 {}
/// Nested message and enum types in `PlayerStorageGetEntriesV2`.
pub mod player_storage_get_entries_v2 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, repeated, tag = "1")]
        pub criteria: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Criterion,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "2")]
        pub revision: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerStorageGetOtherPlayerEntriesV2 {}
/// Nested message and enum types in `PlayerStorageGetOtherPlayerEntriesV2`.
pub mod player_storage_get_other_player_entries_v2 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub player_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub criteria: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Criterion,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
    }
}
/// Generated client implementations.
pub mod player_storage_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PlayerStorageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayerStorageClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PlayerStorageClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> PlayerStorageClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            PlayerStorageClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn set_entries_v2(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_storage_set_entries_v2::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_storage_set_entries_v2::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerStorage/SetEntriesV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerStorage", "SetEntriesV2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_entries_v2(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_storage_get_entries_v2::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_storage_get_entries_v2::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerStorage/GetEntriesV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerStorage", "GetEntriesV2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_other_player_entries_v2(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_storage_get_other_player_entries_v2::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_storage_get_other_player_entries_v2::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerStorage/GetOtherPlayerEntriesV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerStorage", "GetOtherPlayerEntriesV2"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod player_storage_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PlayerStorageServer.
    #[async_trait]
    pub trait PlayerStorage: Send + Sync + 'static {
        async fn set_entries_v2(
            &self,
            request: tonic::Request<super::player_storage_set_entries_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_storage_set_entries_v2::Response>,
            tonic::Status,
        >;
        async fn get_entries_v2(
            &self,
            request: tonic::Request<super::player_storage_get_entries_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_storage_get_entries_v2::Response>,
            tonic::Status,
        >;
        async fn get_other_player_entries_v2(
            &self,
            request: tonic::Request<
                super::player_storage_get_other_player_entries_v2::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::player_storage_get_other_player_entries_v2::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PlayerStorageServer<T: PlayerStorage> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PlayerStorage> PlayerStorageServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlayerStorageServer<T>
    where
        T: PlayerStorage,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/PlayerStorage/SetEntriesV2" => {
                    #[allow(non_camel_case_types)]
                    struct SetEntriesV2Svc<T: PlayerStorage>(pub Arc<T>);
                    impl<
                        T: PlayerStorage,
                    > tonic::server::UnaryService<
                        super::player_storage_set_entries_v2::Request,
                    > for SetEntriesV2Svc<T> {
                        type Response = super::player_storage_set_entries_v2::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_storage_set_entries_v2::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_entries_v2(request).await
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
                        let inner = inner.0;
                        let method = SetEntriesV2Svc(inner);
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
                "/PlayerStorage/GetEntriesV2" => {
                    #[allow(non_camel_case_types)]
                    struct GetEntriesV2Svc<T: PlayerStorage>(pub Arc<T>);
                    impl<
                        T: PlayerStorage,
                    > tonic::server::UnaryService<
                        super::player_storage_get_entries_v2::Request,
                    > for GetEntriesV2Svc<T> {
                        type Response = super::player_storage_get_entries_v2::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_storage_get_entries_v2::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_entries_v2(request).await
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
                        let inner = inner.0;
                        let method = GetEntriesV2Svc(inner);
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
                "/PlayerStorage/GetOtherPlayerEntriesV2" => {
                    #[allow(non_camel_case_types)]
                    struct GetOtherPlayerEntriesV2Svc<T: PlayerStorage>(pub Arc<T>);
                    impl<
                        T: PlayerStorage,
                    > tonic::server::UnaryService<
                        super::player_storage_get_other_player_entries_v2::Request,
                    > for GetOtherPlayerEntriesV2Svc<T> {
                        type Response = super::player_storage_get_other_player_entries_v2::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_storage_get_other_player_entries_v2::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_other_player_entries_v2(request).await
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
                        let inner = inner.0;
                        let method = GetOtherPlayerEntriesV2Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PlayerStorage> Clone for PlayerStorageServer<T> {
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
    impl<T: PlayerStorage> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PlayerStorage> tonic::server::NamedService for PlayerStorageServer<T> {
        const NAME: &'static str = "PlayerStorage";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaasProductGetAvailableByIDsV1 {}
/// Nested message and enum types in `BaasProductGetAvailableByIDsV1`.
pub mod baas_product_get_available_by_i_ds_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub product_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub baas_products: ::prost::alloc::vec::Vec<
            super::super::resource::baas_product::v1::BaasProduct,
        >,
    }
}
/// Generated client implementations.
pub mod baas_product_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BaasProductClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BaasProductClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BaasProductClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> BaasProductClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            BaasProductClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_available_by_i_ds_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::baas_product_get_available_by_i_ds_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::baas_product_get_available_by_i_ds_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/BaasProduct/GetAvailableByIDsV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("BaasProduct", "GetAvailableByIDsV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod baas_product_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BaasProductServer.
    #[async_trait]
    pub trait BaasProduct: Send + Sync + 'static {
        async fn get_available_by_i_ds_v1(
            &self,
            request: tonic::Request<
                super::baas_product_get_available_by_i_ds_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::baas_product_get_available_by_i_ds_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct BaasProductServer<T: BaasProduct> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BaasProduct> BaasProductServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BaasProductServer<T>
    where
        T: BaasProduct,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/BaasProduct/GetAvailableByIDsV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableByIDsV1Svc<T: BaasProduct>(pub Arc<T>);
                    impl<
                        T: BaasProduct,
                    > tonic::server::UnaryService<
                        super::baas_product_get_available_by_i_ds_v1::Request,
                    > for GetAvailableByIDsV1Svc<T> {
                        type Response = super::baas_product_get_available_by_i_ds_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::baas_product_get_available_by_i_ds_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_by_i_ds_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableByIDsV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: BaasProduct> Clone for BaasProductServer<T> {
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
    impl<T: BaasProduct> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BaasProduct> tonic::server::NamedService for BaasProductServer<T> {
        const NAME: &'static str = "BaasProduct";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMonthlyBillingLimitV1 {}
/// Nested message and enum types in `GetMonthlyBillingLimitV1`.
pub mod get_monthly_billing_limit_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub monthly_billing_limits: ::prost::alloc::vec::Vec<
            super::super::resource::monthly_billing_limit::v1::MonthlyBillingLimit,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMonthlyVirtualCurrencyAmountV1 {}
/// Nested message and enum types in `GetMonthlyVirtualCurrencyAmountV1`.
pub mod get_monthly_virtual_currency_amount_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(sint64, tag = "1")]
        pub paid_virtual_currency_amount: i64,
    }
}
/// Generated client implementations.
pub mod monthly_billing_limit_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MonthlyBillingLimitClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MonthlyBillingLimitClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MonthlyBillingLimitClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> MonthlyBillingLimitClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            MonthlyBillingLimitClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_monthly_billing_limit_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::get_monthly_billing_limit_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::get_monthly_billing_limit_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/MonthlyBillingLimit/GetMonthlyBillingLimitV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("MonthlyBillingLimit", "GetMonthlyBillingLimitV1"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_monthly_virtual_currency_amount_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::get_monthly_virtual_currency_amount_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::get_monthly_virtual_currency_amount_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/MonthlyBillingLimit/GetMonthlyVirtualCurrencyAmountV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "MonthlyBillingLimit",
                        "GetMonthlyVirtualCurrencyAmountV1",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod monthly_billing_limit_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MonthlyBillingLimitServer.
    #[async_trait]
    pub trait MonthlyBillingLimit: Send + Sync + 'static {
        async fn get_monthly_billing_limit_v1(
            &self,
            request: tonic::Request<super::get_monthly_billing_limit_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::get_monthly_billing_limit_v1::Response>,
            tonic::Status,
        >;
        async fn get_monthly_virtual_currency_amount_v1(
            &self,
            request: tonic::Request<
                super::get_monthly_virtual_currency_amount_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::get_monthly_virtual_currency_amount_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct MonthlyBillingLimitServer<T: MonthlyBillingLimit> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MonthlyBillingLimit> MonthlyBillingLimitServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MonthlyBillingLimitServer<T>
    where
        T: MonthlyBillingLimit,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/MonthlyBillingLimit/GetMonthlyBillingLimitV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetMonthlyBillingLimitV1Svc<T: MonthlyBillingLimit>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MonthlyBillingLimit,
                    > tonic::server::UnaryService<
                        super::get_monthly_billing_limit_v1::Request,
                    > for GetMonthlyBillingLimitV1Svc<T> {
                        type Response = super::get_monthly_billing_limit_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::get_monthly_billing_limit_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_monthly_billing_limit_v1(request).await
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
                        let inner = inner.0;
                        let method = GetMonthlyBillingLimitV1Svc(inner);
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
                "/MonthlyBillingLimit/GetMonthlyVirtualCurrencyAmountV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetMonthlyVirtualCurrencyAmountV1Svc<T: MonthlyBillingLimit>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MonthlyBillingLimit,
                    > tonic::server::UnaryService<
                        super::get_monthly_virtual_currency_amount_v1::Request,
                    > for GetMonthlyVirtualCurrencyAmountV1Svc<T> {
                        type Response = super::get_monthly_virtual_currency_amount_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::get_monthly_virtual_currency_amount_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_monthly_virtual_currency_amount_v1(request)
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
                        let inner = inner.0;
                        let method = GetMonthlyVirtualCurrencyAmountV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: MonthlyBillingLimit> Clone for MonthlyBillingLimitServer<T> {
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
    impl<T: MonthlyBillingLimit> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MonthlyBillingLimit> tonic::server::NamedService
    for MonthlyBillingLimitServer<T> {
        const NAME: &'static str = "MonthlyBillingLimit";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletGetBalancesV1 {}
/// Nested message and enum types in `WalletGetBalancesV1`.
pub mod wallet_get_balances_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub balance: ::prost::alloc::vec::Vec<
            super::super::resource::wallet::v1::Wallet,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletGetBalancesV2 {}
/// Nested message and enum types in `WalletGetBalancesV2`.
pub mod wallet_get_balances_v2 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(int64, tag = "1")]
        pub expired_at: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub total: ::core::option::Option<super::super::resource::wallet::v3::Wallet>,
        #[prost(message, optional, tag = "2")]
        pub expiration: ::core::option::Option<
            super::super::resource::wallet::v3::Wallet,
        >,
        #[prost(int64, tag = "3")]
        pub expired_at: i64,
    }
}
/// Generated client implementations.
pub mod wallet_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct WalletClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WalletClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> WalletClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> WalletClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            WalletClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_balances_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::wallet_get_balances_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::wallet_get_balances_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/Wallet/GetBalancesV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("Wallet", "GetBalancesV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_balances_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::wallet_get_balances_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::wallet_get_balances_v2::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/Wallet/GetBalancesV2");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("Wallet", "GetBalancesV2"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod wallet_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with WalletServer.
    #[async_trait]
    pub trait Wallet: Send + Sync + 'static {
        async fn get_balances_v1(
            &self,
            request: tonic::Request<super::wallet_get_balances_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::wallet_get_balances_v1::Response>,
            tonic::Status,
        >;
        async fn get_balances_v2(
            &self,
            request: tonic::Request<super::wallet_get_balances_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::wallet_get_balances_v2::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct WalletServer<T: Wallet> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Wallet> WalletServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WalletServer<T>
    where
        T: Wallet,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/Wallet/GetBalancesV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetBalancesV1Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::wallet_get_balances_v1::Request>
                    for GetBalancesV1Svc<T> {
                        type Response = super::wallet_get_balances_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::wallet_get_balances_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_balances_v1(request).await
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
                        let inner = inner.0;
                        let method = GetBalancesV1Svc(inner);
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
                "/Wallet/GetBalancesV2" => {
                    #[allow(non_camel_case_types)]
                    struct GetBalancesV2Svc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::wallet_get_balances_v2::Request>
                    for GetBalancesV2Svc<T> {
                        type Response = super::wallet_get_balances_v2::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::wallet_get_balances_v2::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_balances_v2(request).await
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
                        let inner = inner.0;
                        let method = GetBalancesV2Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Wallet> Clone for WalletServer<T> {
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
    impl<T: Wallet> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Wallet> tonic::server::NamedService for WalletServer<T> {
        const NAME: &'static str = "Wallet";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerKeyValueStoreGetPlayerKeyValuesV1 {}
/// Nested message and enum types in `PlayerKeyValueStoreGetPlayerKeyValuesV1`.
pub mod player_key_value_store_get_player_key_values_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub key: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_key_values: ::prost::alloc::vec::Vec<
            super::super::resource::player_key_value::v1::PlayerKeyValue,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerKeyValueStoreIncrementPlayerKeyValuesV1 {}
/// Nested message and enum types in `PlayerKeyValueStoreIncrementPlayerKeyValuesV1`.
pub mod player_key_value_store_increment_player_key_values_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub player_key_values: ::prost::alloc::vec::Vec<
            super::super::resource::player_key_value::v1::PlayerKeyValueIncrementInfo,
        >,
        #[prost(message, repeated, tag = "3")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_key_values: ::prost::alloc::vec::Vec<
            super::super::resource::player_key_value::v1::PlayerKeyValue,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerKeyValueStoreIncrementPlayerKeyValuesAndSavePlayerStorageV1 {}
/// Nested message and enum types in `PlayerKeyValueStoreIncrementPlayerKeyValuesAndSavePlayerStorageV1`.
pub mod player_key_value_store_increment_player_key_values_and_save_player_storage_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub player_key_values: ::prost::alloc::vec::Vec<
            super::super::resource::player_key_value::v1::PlayerKeyValueIncrementInfo,
        >,
        #[prost(message, repeated, tag = "3")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "4")]
        pub next_revision: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub previous_revision: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "6")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_key_values: ::prost::alloc::vec::Vec<
            super::super::resource::player_key_value::v1::PlayerKeyValue,
        >,
        #[prost(message, repeated, tag = "2")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "3")]
        pub revision: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerKeyValueStoreUpdatePlayerKeyValuesAndSavePlayerStorageV1 {}
/// Nested message and enum types in `PlayerKeyValueStoreUpdatePlayerKeyValuesAndSavePlayerStorageV1`.
pub mod player_key_value_store_update_player_key_values_and_save_player_storage_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub player_key_values: ::prost::alloc::vec::Vec<
            super::super::resource::player_key_value::v1::PlayerKeyValueUpdateInfo,
        >,
        #[prost(message, repeated, tag = "3")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "4")]
        pub next_revision: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub previous_revision: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "6")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_key_values: ::prost::alloc::vec::Vec<
            super::super::resource::player_key_value::v1::PlayerKeyValue,
        >,
        #[prost(message, repeated, tag = "2")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "3")]
        pub revision: ::prost::alloc::string::String,
    }
}
/// Generated client implementations.
pub mod player_key_value_store_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PlayerKeyValueStoreClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayerKeyValueStoreClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PlayerKeyValueStoreClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> PlayerKeyValueStoreClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            PlayerKeyValueStoreClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_player_key_values_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_key_value_store_get_player_key_values_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_key_value_store_get_player_key_values_v1::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerKeyValueStore/GetPlayerKeyValuesV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PlayerKeyValueStore", "GetPlayerKeyValuesV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn increment_player_key_values_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_key_value_store_increment_player_key_values_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_key_value_store_increment_player_key_values_v1::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerKeyValueStore/IncrementPlayerKeyValuesV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("PlayerKeyValueStore", "IncrementPlayerKeyValuesV1"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn increment_player_key_values_and_save_player_storage_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_key_value_store_increment_player_key_values_and_save_player_storage_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_key_value_store_increment_player_key_values_and_save_player_storage_v1::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerKeyValueStore/IncrementPlayerKeyValuesAndSavePlayerStorageV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "PlayerKeyValueStore",
                        "IncrementPlayerKeyValuesAndSavePlayerStorageV1",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_player_key_values_and_save_player_storage_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::player_key_value_store_update_player_key_values_and_save_player_storage_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_key_value_store_update_player_key_values_and_save_player_storage_v1::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PlayerKeyValueStore/UpdatePlayerKeyValuesAndSavePlayerStorageV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "PlayerKeyValueStore",
                        "UpdatePlayerKeyValuesAndSavePlayerStorageV1",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod player_key_value_store_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PlayerKeyValueStoreServer.
    #[async_trait]
    pub trait PlayerKeyValueStore: Send + Sync + 'static {
        async fn get_player_key_values_v1(
            &self,
            request: tonic::Request<
                super::player_key_value_store_get_player_key_values_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_key_value_store_get_player_key_values_v1::Response,
            >,
            tonic::Status,
        >;
        async fn increment_player_key_values_v1(
            &self,
            request: tonic::Request<
                super::player_key_value_store_increment_player_key_values_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_key_value_store_increment_player_key_values_v1::Response,
            >,
            tonic::Status,
        >;
        async fn increment_player_key_values_and_save_player_storage_v1(
            &self,
            request: tonic::Request<
                super::player_key_value_store_increment_player_key_values_and_save_player_storage_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_key_value_store_increment_player_key_values_and_save_player_storage_v1::Response,
            >,
            tonic::Status,
        >;
        async fn update_player_key_values_and_save_player_storage_v1(
            &self,
            request: tonic::Request<
                super::player_key_value_store_update_player_key_values_and_save_player_storage_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::player_key_value_store_update_player_key_values_and_save_player_storage_v1::Response,
            >,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PlayerKeyValueStoreServer<T: PlayerKeyValueStore> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PlayerKeyValueStore> PlayerKeyValueStoreServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlayerKeyValueStoreServer<T>
    where
        T: PlayerKeyValueStore,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/PlayerKeyValueStore/GetPlayerKeyValuesV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetPlayerKeyValuesV1Svc<T: PlayerKeyValueStore>(pub Arc<T>);
                    impl<
                        T: PlayerKeyValueStore,
                    > tonic::server::UnaryService<
                        super::player_key_value_store_get_player_key_values_v1::Request,
                    > for GetPlayerKeyValuesV1Svc<T> {
                        type Response = super::player_key_value_store_get_player_key_values_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_key_value_store_get_player_key_values_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_player_key_values_v1(request).await
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
                        let inner = inner.0;
                        let method = GetPlayerKeyValuesV1Svc(inner);
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
                "/PlayerKeyValueStore/IncrementPlayerKeyValuesV1" => {
                    #[allow(non_camel_case_types)]
                    struct IncrementPlayerKeyValuesV1Svc<T: PlayerKeyValueStore>(
                        pub Arc<T>,
                    );
                    impl<
                        T: PlayerKeyValueStore,
                    > tonic::server::UnaryService<
                        super::player_key_value_store_increment_player_key_values_v1::Request,
                    > for IncrementPlayerKeyValuesV1Svc<T> {
                        type Response = super::player_key_value_store_increment_player_key_values_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_key_value_store_increment_player_key_values_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).increment_player_key_values_v1(request).await
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
                        let inner = inner.0;
                        let method = IncrementPlayerKeyValuesV1Svc(inner);
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
                "/PlayerKeyValueStore/IncrementPlayerKeyValuesAndSavePlayerStorageV1" => {
                    #[allow(non_camel_case_types)]
                    struct IncrementPlayerKeyValuesAndSavePlayerStorageV1Svc<
                        T: PlayerKeyValueStore,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PlayerKeyValueStore,
                    > tonic::server::UnaryService<
                        super::player_key_value_store_increment_player_key_values_and_save_player_storage_v1::Request,
                    > for IncrementPlayerKeyValuesAndSavePlayerStorageV1Svc<T> {
                        type Response = super::player_key_value_store_increment_player_key_values_and_save_player_storage_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_key_value_store_increment_player_key_values_and_save_player_storage_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .increment_player_key_values_and_save_player_storage_v1(
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
                        let inner = inner.0;
                        let method = IncrementPlayerKeyValuesAndSavePlayerStorageV1Svc(
                            inner,
                        );
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
                "/PlayerKeyValueStore/UpdatePlayerKeyValuesAndSavePlayerStorageV1" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePlayerKeyValuesAndSavePlayerStorageV1Svc<
                        T: PlayerKeyValueStore,
                    >(
                        pub Arc<T>,
                    );
                    impl<
                        T: PlayerKeyValueStore,
                    > tonic::server::UnaryService<
                        super::player_key_value_store_update_player_key_values_and_save_player_storage_v1::Request,
                    > for UpdatePlayerKeyValuesAndSavePlayerStorageV1Svc<T> {
                        type Response = super::player_key_value_store_update_player_key_values_and_save_player_storage_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::player_key_value_store_update_player_key_values_and_save_player_storage_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .update_player_key_values_and_save_player_storage_v1(
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
                        let inner = inner.0;
                        let method = UpdatePlayerKeyValuesAndSavePlayerStorageV1Svc(
                            inner,
                        );
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PlayerKeyValueStore> Clone for PlayerKeyValueStoreServer<T> {
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
    impl<T: PlayerKeyValueStore> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PlayerKeyValueStore> tonic::server::NamedService
    for PlayerKeyValueStoreServer<T> {
        const NAME: &'static str = "PlayerKeyValueStore";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OndemandMasterGetEntriesV2 {}
/// Nested message and enum types in `OndemandMasterGetEntriesV2`.
pub mod ondemand_master_get_entries_v2 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::ondemand_master::v2::Entry,
        >,
    }
}
/// Generated client implementations.
pub mod ondemand_master_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OndemandMasterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OndemandMasterClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OndemandMasterClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> OndemandMasterClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            OndemandMasterClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_entries_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ondemand_master_get_entries_v2::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ondemand_master_get_entries_v2::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/OndemandMaster/GetEntriesV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("OndemandMaster", "GetEntriesV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod ondemand_master_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OndemandMasterServer.
    #[async_trait]
    pub trait OndemandMaster: Send + Sync + 'static {
        async fn get_entries_v1(
            &self,
            request: tonic::Request<super::ondemand_master_get_entries_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::ondemand_master_get_entries_v2::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct OndemandMasterServer<T: OndemandMaster> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: OndemandMaster> OndemandMasterServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OndemandMasterServer<T>
    where
        T: OndemandMaster,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/OndemandMaster/GetEntriesV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetEntriesV1Svc<T: OndemandMaster>(pub Arc<T>);
                    impl<
                        T: OndemandMaster,
                    > tonic::server::UnaryService<
                        super::ondemand_master_get_entries_v2::Request,
                    > for GetEntriesV1Svc<T> {
                        type Response = super::ondemand_master_get_entries_v2::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ondemand_master_get_entries_v2::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_entries_v1(request).await
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
                        let inner = inner.0;
                        let method = GetEntriesV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: OndemandMaster> Clone for OndemandMasterServer<T> {
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
    impl<T: OndemandMaster> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: OndemandMaster> tonic::server::NamedService for OndemandMasterServer<T> {
        const NAME: &'static str = "OndemandMaster";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxGetAvailableV1 {}
/// Nested message and enum types in `LootBoxGetAvailableV1`.
pub mod loot_box_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub loot_box_products: ::prost::alloc::vec::Vec<
            super::super::resource::loot_box::v1::LootBoxProduct,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxPurchaseV1 {}
/// Nested message and enum types in `LootBoxPurchaseV1`.
pub mod loot_box_purchase_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub loot_box_product_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub transaction_id: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub loot_box_contents: ::prost::alloc::vec::Vec<
            super::super::resource::loot_box::v1::LootBoxContent,
        >,
        #[prost(message, repeated, tag = "3")]
        pub player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, optional, tag = "4")]
        pub wallet: ::core::option::Option<super::super::resource::wallet::v1::Wallet>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxGetProbabilityV1 {}
/// Nested message and enum types in `LootBoxGetProbabilityV1`.
pub mod loot_box_get_probability_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub loot_box_product_id: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub loot_box_probability: ::core::option::Option<
            super::super::resource::loot_box::v1::LootBoxProbability,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxGetDetailV1 {}
/// Nested message and enum types in `LootBoxGetDetailV1`.
pub mod loot_box_get_detail_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub loot_box_product_id: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub loot_box_product: ::core::option::Option<
            super::super::resource::loot_box::v1::LootBoxProduct,
        >,
        #[prost(message, repeated, tag = "2")]
        pub loot_box_content_sets: ::prost::alloc::vec::Vec<
            super::super::resource::loot_box::v1::LootBoxContentSet,
        >,
        #[prost(bool, tag = "3")]
        pub is_limited: bool,
        #[prost(sint64, tag = "4")]
        pub purchasable_count: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxV2GetAvailableV1 {}
/// Nested message and enum types in `LootBoxV2GetAvailableV1`.
pub mod loot_box_v2_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "2")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub loot_box_products: ::prost::alloc::vec::Vec<
            super::super::resource::loot_box::v2::LootBoxProductWithPurchasedCount,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxV2PurchaseV1 {}
/// Nested message and enum types in `LootBoxV2PurchaseV1`.
pub mod loot_box_v2_purchase_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub loot_box_product_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(enumeration = "request::ResourceType", tag = "3")]
        pub resource_type: i32,
    }
    /// Nested message and enum types in `Request`.
    pub mod request {
        #[derive(serde::Serialize, serde::Deserialize)]
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
        pub enum ResourceType {
            VirtualCurrency = 0,
            PlayerKeyValue = 1,
        }
        impl ResourceType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResourceType::VirtualCurrency => "VIRTUAL_CURRENCY",
                    ResourceType::PlayerKeyValue => "PLAYER_KEY_VALUE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "VIRTUAL_CURRENCY" => Some(Self::VirtualCurrency),
                    "PLAYER_KEY_VALUE" => Some(Self::PlayerKeyValue),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub loot_box_content_set_prizes: ::prost::alloc::vec::Vec<
            super::super::resource::loot_box::v2::LootBoxContentSetPrizes,
        >,
        #[prost(message, repeated, tag = "3")]
        pub player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, repeated, tag = "4")]
        pub extra_player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, optional, tag = "5")]
        pub wallet: ::core::option::Option<super::super::resource::wallet::v3::Wallet>,
        #[prost(message, repeated, tag = "6")]
        pub pickup_extra_player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxV2GetProbabilityV1 {}
/// Nested message and enum types in `LootBoxV2GetProbabilityV1`.
pub mod loot_box_v2_get_probability_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub loot_box_product_id: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub loot_box_probability: ::core::option::Option<
            super::super::resource::loot_box::v2::LootBoxProbability,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxV2GetDetailV1 {}
/// Nested message and enum types in `LootBoxV2GetDetailV1`.
pub mod loot_box_v2_get_detail_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub loot_box_product_id: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub loot_box_product: ::core::option::Option<
            super::super::resource::loot_box::v2::LootBoxProduct,
        >,
        #[prost(message, repeated, tag = "2")]
        pub loot_box_content_sets: ::prost::alloc::vec::Vec<
            super::super::resource::loot_box::v2::LootBoxContentSet,
        >,
        #[prost(bool, tag = "3")]
        pub is_limited: bool,
        #[prost(sint64, tag = "4")]
        pub purchased_count: i64,
        #[prost(message, optional, tag = "5")]
        pub loot_box_pickup: ::core::option::Option<
            super::super::resource::loot_box::v2::LootBoxPickup,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxV3GetAvailableV1 {}
/// Nested message and enum types in `LootBoxV3GetAvailableV1`.
pub mod loot_box_v3_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "2")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub loot_box_products: ::prost::alloc::vec::Vec<
            super::super::resource::loot_box::v3::LootBoxProductWithPurchasedCount,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxV3PurchaseV1 {}
/// Nested message and enum types in `LootBoxV3PurchaseV1`.
pub mod loot_box_v3_purchase_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub loot_box_product_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(enumeration = "request::ResourceType", tag = "3")]
        pub resource_type: i32,
        #[prost(string, tag = "4")]
        pub purchase_token: ::prost::alloc::string::String,
        #[prost(int64, tag = "5")]
        pub purchase_num: i64,
        #[prost(message, repeated, tag = "6")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    /// Nested message and enum types in `Request`.
    pub mod request {
        #[derive(serde::Serialize, serde::Deserialize)]
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
        pub enum ResourceType {
            VirtualCurrency = 0,
            PlayerKeyValue = 1,
            NoConsume = 2,
        }
        impl ResourceType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResourceType::VirtualCurrency => "VIRTUAL_CURRENCY",
                    ResourceType::PlayerKeyValue => "PLAYER_KEY_VALUE",
                    ResourceType::NoConsume => "NO_CONSUME",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "VIRTUAL_CURRENCY" => Some(Self::VirtualCurrency),
                    "PLAYER_KEY_VALUE" => Some(Self::PlayerKeyValue),
                    "NO_CONSUME" => Some(Self::NoConsume),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub loot_box_content_set_prizes: ::prost::alloc::vec::Vec<
            super::super::resource::loot_box::v3::LootBoxContentSetPrizes,
        >,
        #[prost(message, repeated, tag = "3")]
        pub player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, repeated, tag = "4")]
        pub extra_player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, optional, tag = "5")]
        pub wallet: ::core::option::Option<super::super::resource::wallet::v3::Wallet>,
        #[prost(message, repeated, tag = "6")]
        pub pickup_extra_player_inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxV3GetProbabilityV1 {}
/// Nested message and enum types in `LootBoxV3GetProbabilityV1`.
pub mod loot_box_v3_get_probability_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub loot_box_product_id: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub loot_box_probability: ::core::option::Option<
            super::super::resource::loot_box::v3::LootBoxProbability,
        >,
        #[prost(string, tag = "2")]
        pub purchase_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxV3GetDetailV1 {}
/// Nested message and enum types in `LootBoxV3GetDetailV1`.
pub mod loot_box_v3_get_detail_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub loot_box_product_id: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub loot_box_product: ::core::option::Option<
            super::super::resource::loot_box::v3::LootBoxProduct,
        >,
        #[prost(message, repeated, tag = "2")]
        pub loot_box_content_sets: ::prost::alloc::vec::Vec<
            super::super::resource::loot_box::v3::LootBoxContentSet,
        >,
        #[prost(bool, tag = "3")]
        pub is_limited: bool,
        #[prost(sint64, tag = "4")]
        pub purchased_count: i64,
        #[prost(message, optional, tag = "5")]
        pub loot_box_pickup: ::core::option::Option<
            super::super::resource::loot_box::v3::LootBoxPickup,
        >,
    }
}
/// Generated client implementations.
pub mod loot_box_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct LootBoxClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LootBoxClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> LootBoxClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> LootBoxClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            LootBoxClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::loot_box_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LootBox/GetAvailableV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("LootBox", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn purchase_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::loot_box_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_purchase_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LootBox/PurchaseV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("LootBox", "PurchaseV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_probability_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::loot_box_get_probability_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_get_probability_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LootBox/GetProbabilityV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("LootBox", "GetProbabilityV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_detail_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::loot_box_get_detail_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_get_detail_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LootBox/GetDetailV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("LootBox", "GetDetailV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod loot_box_v2_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct LootBoxV2Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LootBoxV2Client<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> LootBoxV2Client<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> LootBoxV2Client<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            LootBoxV2Client::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::loot_box_v2_get_available_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v2_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LootBoxV2/GetAvailableV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("LootBoxV2", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn purchase_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::loot_box_v2_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v2_purchase_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LootBoxV2/PurchaseV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("LootBoxV2", "PurchaseV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_probability_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::loot_box_v2_get_probability_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v2_get_probability_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/LootBoxV2/GetProbabilityV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("LootBoxV2", "GetProbabilityV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_detail_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::loot_box_v2_get_detail_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v2_get_detail_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LootBoxV2/GetDetailV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("LootBoxV2", "GetDetailV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod loot_box_v3_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct LootBoxV3Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LootBoxV3Client<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> LootBoxV3Client<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> LootBoxV3Client<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            LootBoxV3Client::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::loot_box_v3_get_available_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v3_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LootBoxV3/GetAvailableV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("LootBoxV3", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn purchase_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::loot_box_v3_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v3_purchase_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LootBoxV3/PurchaseV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("LootBoxV3", "PurchaseV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_probability_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::loot_box_v3_get_probability_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v3_get_probability_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/LootBoxV3/GetProbabilityV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("LootBoxV3", "GetProbabilityV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_detail_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::loot_box_v3_get_detail_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v3_get_detail_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/LootBoxV3/GetDetailV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("LootBoxV3", "GetDetailV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod loot_box_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LootBoxServer.
    #[async_trait]
    pub trait LootBox: Send + Sync + 'static {
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::loot_box_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_get_available_v1::Response>,
            tonic::Status,
        >;
        async fn purchase_v1(
            &self,
            request: tonic::Request<super::loot_box_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_purchase_v1::Response>,
            tonic::Status,
        >;
        async fn get_probability_v1(
            &self,
            request: tonic::Request<super::loot_box_get_probability_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_get_probability_v1::Response>,
            tonic::Status,
        >;
        async fn get_detail_v1(
            &self,
            request: tonic::Request<super::loot_box_get_detail_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_get_detail_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct LootBoxServer<T: LootBox> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LootBox> LootBoxServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LootBoxServer<T>
    where
        T: LootBox,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/LootBox/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: LootBox>(pub Arc<T>);
                    impl<
                        T: LootBox,
                    > tonic::server::UnaryService<
                        super::loot_box_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::loot_box_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::loot_box_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                "/LootBox/PurchaseV1" => {
                    #[allow(non_camel_case_types)]
                    struct PurchaseV1Svc<T: LootBox>(pub Arc<T>);
                    impl<
                        T: LootBox,
                    > tonic::server::UnaryService<super::loot_box_purchase_v1::Request>
                    for PurchaseV1Svc<T> {
                        type Response = super::loot_box_purchase_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::loot_box_purchase_v1::Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).purchase_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PurchaseV1Svc(inner);
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
                "/LootBox/GetProbabilityV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetProbabilityV1Svc<T: LootBox>(pub Arc<T>);
                    impl<
                        T: LootBox,
                    > tonic::server::UnaryService<
                        super::loot_box_get_probability_v1::Request,
                    > for GetProbabilityV1Svc<T> {
                        type Response = super::loot_box_get_probability_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::loot_box_get_probability_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_probability_v1(request).await
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
                        let inner = inner.0;
                        let method = GetProbabilityV1Svc(inner);
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
                "/LootBox/GetDetailV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetDetailV1Svc<T: LootBox>(pub Arc<T>);
                    impl<
                        T: LootBox,
                    > tonic::server::UnaryService<super::loot_box_get_detail_v1::Request>
                    for GetDetailV1Svc<T> {
                        type Response = super::loot_box_get_detail_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::loot_box_get_detail_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_detail_v1(request).await
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
                        let inner = inner.0;
                        let method = GetDetailV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: LootBox> Clone for LootBoxServer<T> {
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
    impl<T: LootBox> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LootBox> tonic::server::NamedService for LootBoxServer<T> {
        const NAME: &'static str = "LootBox";
    }
}
/// Generated server implementations.
pub mod loot_box_v2_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LootBoxV2Server.
    #[async_trait]
    pub trait LootBoxV2: Send + Sync + 'static {
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::loot_box_v2_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v2_get_available_v1::Response>,
            tonic::Status,
        >;
        async fn purchase_v1(
            &self,
            request: tonic::Request<super::loot_box_v2_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v2_purchase_v1::Response>,
            tonic::Status,
        >;
        async fn get_probability_v1(
            &self,
            request: tonic::Request<super::loot_box_v2_get_probability_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v2_get_probability_v1::Response>,
            tonic::Status,
        >;
        async fn get_detail_v1(
            &self,
            request: tonic::Request<super::loot_box_v2_get_detail_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v2_get_detail_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct LootBoxV2Server<T: LootBoxV2> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LootBoxV2> LootBoxV2Server<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LootBoxV2Server<T>
    where
        T: LootBoxV2,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/LootBoxV2/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: LootBoxV2>(pub Arc<T>);
                    impl<
                        T: LootBoxV2,
                    > tonic::server::UnaryService<
                        super::loot_box_v2_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::loot_box_v2_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::loot_box_v2_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                "/LootBoxV2/PurchaseV1" => {
                    #[allow(non_camel_case_types)]
                    struct PurchaseV1Svc<T: LootBoxV2>(pub Arc<T>);
                    impl<
                        T: LootBoxV2,
                    > tonic::server::UnaryService<
                        super::loot_box_v2_purchase_v1::Request,
                    > for PurchaseV1Svc<T> {
                        type Response = super::loot_box_v2_purchase_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::loot_box_v2_purchase_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).purchase_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PurchaseV1Svc(inner);
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
                "/LootBoxV2/GetProbabilityV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetProbabilityV1Svc<T: LootBoxV2>(pub Arc<T>);
                    impl<
                        T: LootBoxV2,
                    > tonic::server::UnaryService<
                        super::loot_box_v2_get_probability_v1::Request,
                    > for GetProbabilityV1Svc<T> {
                        type Response = super::loot_box_v2_get_probability_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::loot_box_v2_get_probability_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_probability_v1(request).await
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
                        let inner = inner.0;
                        let method = GetProbabilityV1Svc(inner);
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
                "/LootBoxV2/GetDetailV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetDetailV1Svc<T: LootBoxV2>(pub Arc<T>);
                    impl<
                        T: LootBoxV2,
                    > tonic::server::UnaryService<
                        super::loot_box_v2_get_detail_v1::Request,
                    > for GetDetailV1Svc<T> {
                        type Response = super::loot_box_v2_get_detail_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::loot_box_v2_get_detail_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_detail_v1(request).await
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
                        let inner = inner.0;
                        let method = GetDetailV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: LootBoxV2> Clone for LootBoxV2Server<T> {
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
    impl<T: LootBoxV2> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LootBoxV2> tonic::server::NamedService for LootBoxV2Server<T> {
        const NAME: &'static str = "LootBoxV2";
    }
}
/// Generated server implementations.
pub mod loot_box_v3_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LootBoxV3Server.
    #[async_trait]
    pub trait LootBoxV3: Send + Sync + 'static {
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::loot_box_v3_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v3_get_available_v1::Response>,
            tonic::Status,
        >;
        async fn purchase_v1(
            &self,
            request: tonic::Request<super::loot_box_v3_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v3_purchase_v1::Response>,
            tonic::Status,
        >;
        async fn get_probability_v1(
            &self,
            request: tonic::Request<super::loot_box_v3_get_probability_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v3_get_probability_v1::Response>,
            tonic::Status,
        >;
        async fn get_detail_v1(
            &self,
            request: tonic::Request<super::loot_box_v3_get_detail_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::loot_box_v3_get_detail_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct LootBoxV3Server<T: LootBoxV3> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LootBoxV3> LootBoxV3Server<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LootBoxV3Server<T>
    where
        T: LootBoxV3,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/LootBoxV3/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: LootBoxV3>(pub Arc<T>);
                    impl<
                        T: LootBoxV3,
                    > tonic::server::UnaryService<
                        super::loot_box_v3_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::loot_box_v3_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::loot_box_v3_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                "/LootBoxV3/PurchaseV1" => {
                    #[allow(non_camel_case_types)]
                    struct PurchaseV1Svc<T: LootBoxV3>(pub Arc<T>);
                    impl<
                        T: LootBoxV3,
                    > tonic::server::UnaryService<
                        super::loot_box_v3_purchase_v1::Request,
                    > for PurchaseV1Svc<T> {
                        type Response = super::loot_box_v3_purchase_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::loot_box_v3_purchase_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).purchase_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PurchaseV1Svc(inner);
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
                "/LootBoxV3/GetProbabilityV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetProbabilityV1Svc<T: LootBoxV3>(pub Arc<T>);
                    impl<
                        T: LootBoxV3,
                    > tonic::server::UnaryService<
                        super::loot_box_v3_get_probability_v1::Request,
                    > for GetProbabilityV1Svc<T> {
                        type Response = super::loot_box_v3_get_probability_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::loot_box_v3_get_probability_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_probability_v1(request).await
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
                        let inner = inner.0;
                        let method = GetProbabilityV1Svc(inner);
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
                "/LootBoxV3/GetDetailV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetDetailV1Svc<T: LootBoxV3>(pub Arc<T>);
                    impl<
                        T: LootBoxV3,
                    > tonic::server::UnaryService<
                        super::loot_box_v3_get_detail_v1::Request,
                    > for GetDetailV1Svc<T> {
                        type Response = super::loot_box_v3_get_detail_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::loot_box_v3_get_detail_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_detail_v1(request).await
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
                        let inner = inner.0;
                        let method = GetDetailV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: LootBoxV3> Clone for LootBoxV3Server<T> {
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
    impl<T: LootBoxV3> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LootBoxV3> tonic::server::NamedService for LootBoxV3Server<T> {
        const NAME: &'static str = "LootBoxV3";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpinionGetUrlV1 {}
/// Nested message and enum types in `OpinionGetUrlV1`.
pub mod opinion_get_url_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub url: ::prost::alloc::string::String,
    }
}
/// Generated client implementations.
pub mod opinion_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OpinionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OpinionClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OpinionClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> OpinionClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            OpinionClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_url_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::opinion_get_url_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::opinion_get_url_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/Opinion/GetUrlV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("Opinion", "GetUrlV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod opinion_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OpinionServer.
    #[async_trait]
    pub trait Opinion: Send + Sync + 'static {
        async fn get_url_v1(
            &self,
            request: tonic::Request<super::opinion_get_url_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::opinion_get_url_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct OpinionServer<T: Opinion> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Opinion> OpinionServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OpinionServer<T>
    where
        T: Opinion,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/Opinion/GetUrlV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetUrlV1Svc<T: Opinion>(pub Arc<T>);
                    impl<
                        T: Opinion,
                    > tonic::server::UnaryService<super::opinion_get_url_v1::Request>
                    for GetUrlV1Svc<T> {
                        type Response = super::opinion_get_url_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::opinion_get_url_v1::Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_url_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUrlV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Opinion> Clone for OpinionServer<T> {
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
    impl<T: Opinion> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Opinion> tonic::server::NamedService for OpinionServer<T> {
        const NAME: &'static str = "Opinion";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEventGetAvailableV1 {}
/// Nested message and enum types in `OpEventGetAvailableV1`.
pub mod op_event_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "2")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "3")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub op_events: ::prost::alloc::vec::Vec<super::OpEventWithStatus>,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEventWithStatus {
    #[prost(message, optional, tag = "1")]
    pub op_event: ::core::option::Option<super::resource::op_event::v1::OpEvent>,
    #[prost(bool, tag = "2")]
    pub can_unlock: bool,
    #[prost(bool, tag = "3")]
    pub finished: bool,
    #[prost(message, repeated, tag = "11")]
    pub fund_unlocks: ::prost::alloc::vec::Vec<
        super::resource::op_fund_unlock::v1::OpFundUnlock,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEventPurchaseV1 {}
/// Nested message and enum types in `OpEventPurchaseV1`.
pub mod op_event_purchase_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub op_event_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub transaction_id: ::prost::alloc::string::String,
        #[prost(enumeration = "request::ResourceType", tag = "3")]
        pub resource_type: i32,
        #[prost(string, tag = "4")]
        pub purchase_token: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "5")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    /// Nested message and enum types in `Request`.
    pub mod request {
        #[derive(serde::Serialize, serde::Deserialize)]
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
        pub enum ResourceType {
            VirtualCurrency = 0,
            PlayerKeyValue = 1,
            NoConsume = 2,
        }
        impl ResourceType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ResourceType::VirtualCurrency => "VIRTUAL_CURRENCY",
                    ResourceType::PlayerKeyValue => "PLAYER_KEY_VALUE",
                    ResourceType::NoConsume => "NO_CONSUME",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "VIRTUAL_CURRENCY" => Some(Self::VirtualCurrency),
                    "PLAYER_KEY_VALUE" => Some(Self::PlayerKeyValue),
                    "NO_CONSUME" => Some(Self::NoConsume),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub player_op_event_history: ::core::option::Option<
            super::super::resource::player_op_event_history::v1::PlayerOpEventHistory,
        >,
        #[prost(message, optional, tag = "2")]
        pub wallet: ::core::option::Option<super::super::resource::wallet::v3::Wallet>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockFundV1 {}
/// Nested message and enum types in `UnlockFundV1`.
pub mod unlock_fund_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub op_event_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub op_fund_unlock_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "3")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, optional, tag = "1")]
        pub player_op_fund_unlock_history: ::core::option::Option<
            super::super::resource::player_op_fund_unlock_history::v1::PlayerOpFundUnlockHistory,
        >,
        #[prost(message, repeated, tag = "2")]
        pub inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
    }
}
/// Generated client implementations.
pub mod op_event_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OpEventClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OpEventClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OpEventClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> OpEventClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            OpEventClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::op_event_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::op_event_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/OpEvent/GetAvailableV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("OpEvent", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn purchase_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::op_event_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::op_event_purchase_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/OpEvent/PurchaseV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("OpEvent", "PurchaseV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlock_fund_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::unlock_fund_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::unlock_fund_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/OpEvent/UnlockFundV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("OpEvent", "UnlockFundV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod op_event_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OpEventServer.
    #[async_trait]
    pub trait OpEvent: Send + Sync + 'static {
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::op_event_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::op_event_get_available_v1::Response>,
            tonic::Status,
        >;
        async fn purchase_v1(
            &self,
            request: tonic::Request<super::op_event_purchase_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::op_event_purchase_v1::Response>,
            tonic::Status,
        >;
        async fn unlock_fund_v1(
            &self,
            request: tonic::Request<super::unlock_fund_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::unlock_fund_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct OpEventServer<T: OpEvent> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: OpEvent> OpEventServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OpEventServer<T>
    where
        T: OpEvent,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/OpEvent/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: OpEvent>(pub Arc<T>);
                    impl<
                        T: OpEvent,
                    > tonic::server::UnaryService<
                        super::op_event_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::op_event_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::op_event_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                "/OpEvent/PurchaseV1" => {
                    #[allow(non_camel_case_types)]
                    struct PurchaseV1Svc<T: OpEvent>(pub Arc<T>);
                    impl<
                        T: OpEvent,
                    > tonic::server::UnaryService<super::op_event_purchase_v1::Request>
                    for PurchaseV1Svc<T> {
                        type Response = super::op_event_purchase_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::op_event_purchase_v1::Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).purchase_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PurchaseV1Svc(inner);
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
                "/OpEvent/UnlockFundV1" => {
                    #[allow(non_camel_case_types)]
                    struct UnlockFundV1Svc<T: OpEvent>(pub Arc<T>);
                    impl<
                        T: OpEvent,
                    > tonic::server::UnaryService<super::unlock_fund_v1::Request>
                    for UnlockFundV1Svc<T> {
                        type Response = super::unlock_fund_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::unlock_fund_v1::Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).unlock_fund_v1(request).await
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
                        let inner = inner.0;
                        let method = UnlockFundV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: OpEvent> Clone for OpEventServer<T> {
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
    impl<T: OpEvent> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: OpEvent> tonic::server::NamedService for OpEventServer<T> {
        const NAME: &'static str = "OpEvent";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerSearchV1 {}
/// Nested message and enum types in `PlayerSearchV1`.
pub mod player_search_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub player_short_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub criteria: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Criterion,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub player_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "3")]
        pub nickname: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerSearchV2 {}
/// Nested message and enum types in `PlayerSearchV2`.
pub mod player_search_v2 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, tag = "1")]
        pub player_id: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub criteria: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Criterion,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(string, tag = "1")]
        pub player_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub player_short_id: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub nickname: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "4")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
    }
}
/// Generated client implementations.
pub mod player_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PlayerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PlayerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> PlayerClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            PlayerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn search_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::player_search_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_search_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/Player/SearchV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("Player", "SearchV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn search_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::player_search_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_search_v2::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/Player/SearchV2");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("Player", "SearchV2"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod player_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PlayerServer.
    #[async_trait]
    pub trait Player: Send + Sync + 'static {
        async fn search_v1(
            &self,
            request: tonic::Request<super::player_search_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_search_v1::Response>,
            tonic::Status,
        >;
        async fn search_v2(
            &self,
            request: tonic::Request<super::player_search_v2::Request>,
        ) -> std::result::Result<
            tonic::Response<super::player_search_v2::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PlayerServer<T: Player> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Player> PlayerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlayerServer<T>
    where
        T: Player,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/Player/SearchV1" => {
                    #[allow(non_camel_case_types)]
                    struct SearchV1Svc<T: Player>(pub Arc<T>);
                    impl<
                        T: Player,
                    > tonic::server::UnaryService<super::player_search_v1::Request>
                    for SearchV1Svc<T> {
                        type Response = super::player_search_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::player_search_v1::Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).search_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchV1Svc(inner);
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
                "/Player/SearchV2" => {
                    #[allow(non_camel_case_types)]
                    struct SearchV2Svc<T: Player>(pub Arc<T>);
                    impl<
                        T: Player,
                    > tonic::server::UnaryService<super::player_search_v2::Request>
                    for SearchV2Svc<T> {
                        type Response = super::player_search_v2::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::player_search_v2::Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).search_v2(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchV2Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Player> Clone for PlayerServer<T> {
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
    impl<T: Player> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Player> tonic::server::NamedService for PlayerServer<T> {
        const NAME: &'static str = "Player";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushNotificationSetConfigV1 {}
/// Nested message and enum types in `PushNotificationSetConfigV1`.
pub mod push_notification_set_config_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(bool, tag = "1")]
        pub receivable: bool,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(bool, tag = "1")]
        pub receivable: bool,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushNotificationGetConfigV1 {}
/// Nested message and enum types in `PushNotificationGetConfigV1`.
pub mod push_notification_get_config_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {}
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(bool, tag = "1")]
        pub receivable: bool,
    }
}
/// Generated client implementations.
pub mod push_notification_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PushNotificationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PushNotificationClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PushNotificationClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> PushNotificationClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            PushNotificationClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn set_config_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::push_notification_set_config_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::push_notification_set_config_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PushNotification/SetConfigV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PushNotification", "SetConfigV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_config_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::push_notification_get_config_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::push_notification_get_config_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/PushNotification/GetConfigV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("PushNotification", "GetConfigV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod push_notification_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PushNotificationServer.
    #[async_trait]
    pub trait PushNotification: Send + Sync + 'static {
        async fn set_config_v1(
            &self,
            request: tonic::Request<super::push_notification_set_config_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::push_notification_set_config_v1::Response>,
            tonic::Status,
        >;
        async fn get_config_v1(
            &self,
            request: tonic::Request<super::push_notification_get_config_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::push_notification_get_config_v1::Response>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct PushNotificationServer<T: PushNotification> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PushNotification> PushNotificationServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PushNotificationServer<T>
    where
        T: PushNotification,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/PushNotification/SetConfigV1" => {
                    #[allow(non_camel_case_types)]
                    struct SetConfigV1Svc<T: PushNotification>(pub Arc<T>);
                    impl<
                        T: PushNotification,
                    > tonic::server::UnaryService<
                        super::push_notification_set_config_v1::Request,
                    > for SetConfigV1Svc<T> {
                        type Response = super::push_notification_set_config_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::push_notification_set_config_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_config_v1(request).await
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
                        let inner = inner.0;
                        let method = SetConfigV1Svc(inner);
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
                "/PushNotification/GetConfigV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetConfigV1Svc<T: PushNotification>(pub Arc<T>);
                    impl<
                        T: PushNotification,
                    > tonic::server::UnaryService<
                        super::push_notification_get_config_v1::Request,
                    > for GetConfigV1Svc<T> {
                        type Response = super::push_notification_get_config_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::push_notification_get_config_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_config_v1(request).await
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
                        let inner = inner.0;
                        let method = GetConfigV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PushNotification> Clone for PushNotificationServer<T> {
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
    impl<T: PushNotification> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PushNotification> tonic::server::NamedService for PushNotificationServer<T> {
        const NAME: &'static str = "PushNotification";
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AchievementGetAvailableV1 {}
/// Nested message and enum types in `AchievementGetAvailableV1`.
pub mod achievement_get_available_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(message, optional, tag = "1")]
        pub criterion: ::core::option::Option<
            super::super::resource::achievement::v1::Criterion,
        >,
        #[prost(string, tag = "2")]
        pub page_token: ::prost::alloc::string::String,
        #[prost(uint64, tag = "3")]
        pub max_results: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub achievements: ::prost::alloc::vec::Vec<
            super::super::resource::achievement::v1::Achievement,
        >,
        #[prost(string, tag = "2")]
        pub next_page_token: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub prev_page_token: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AchievementGetAvailableByIDsV1 {}
/// Nested message and enum types in `AchievementGetAvailableByIDsV1`.
pub mod achievement_get_available_by_i_ds_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub achievement_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub achievements: ::prost::alloc::vec::Vec<
            super::super::resource::achievement::v1::Achievement,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AchievementUnlockV1 {}
/// Nested message and enum types in `AchievementUnlockV1`.
pub mod achievement_unlock_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub achievement_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_achievements: ::prost::alloc::vec::Vec<
            super::super::resource::player_achievement::v1::PlayerAchievement,
        >,
        #[prost(message, repeated, tag = "2")]
        pub inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AchievementUnlockAndSavePlayerStorageV1 {}
/// Nested message and enum types in `AchievementUnlockAndSavePlayerStorageV1`.
pub mod achievement_unlock_and_save_player_storage_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(string, repeated, tag = "1")]
        pub achievement_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, repeated, tag = "2")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "3")]
        pub next_revision: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub previous_revision: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "5")]
        pub player_event_logs: ::prost::alloc::vec::Vec<
            super::super::resource::player_event_log::v1::PlayerEventLog,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(message, repeated, tag = "1")]
        pub player_achievements: ::prost::alloc::vec::Vec<
            super::super::resource::player_achievement::v1::PlayerAchievement,
        >,
        #[prost(message, repeated, tag = "2")]
        pub inventories: ::prost::alloc::vec::Vec<
            super::super::resource::player_inventory::v1::PlayerInventory,
        >,
        #[prost(message, repeated, tag = "3")]
        pub entries: ::prost::alloc::vec::Vec<
            super::super::resource::player_storage::v2::Entry,
        >,
        #[prost(string, tag = "4")]
        pub revision: ::prost::alloc::string::String,
    }
}
/// Generated client implementations.
pub mod achievement_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AchievementClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AchievementClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AchievementClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> AchievementClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            AchievementClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_available_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::achievement_get_available_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::achievement_get_available_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Achievement/GetAvailableV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Achievement", "GetAvailableV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_available_by_i_ds_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::achievement_get_available_by_i_ds_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<super::achievement_get_available_by_i_ds_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Achievement/GetAvailableByIDsV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Achievement", "GetAvailableByIDsV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlock_v1(
            &mut self,
            request: impl tonic::IntoRequest<super::achievement_unlock_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::achievement_unlock_v1::Response>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/Achievement/UnlockV1");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("Achievement", "UnlockV1"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlock_and_save_player_storage_v1(
            &mut self,
            request: impl tonic::IntoRequest<
                super::achievement_unlock_and_save_player_storage_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::achievement_unlock_and_save_player_storage_v1::Response,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Achievement/UnlockAndSavePlayerStorageV1",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Achievement", "UnlockAndSavePlayerStorageV1"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod achievement_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AchievementServer.
    #[async_trait]
    pub trait Achievement: Send + Sync + 'static {
        async fn get_available_v1(
            &self,
            request: tonic::Request<super::achievement_get_available_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::achievement_get_available_v1::Response>,
            tonic::Status,
        >;
        async fn get_available_by_i_ds_v1(
            &self,
            request: tonic::Request<super::achievement_get_available_by_i_ds_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::achievement_get_available_by_i_ds_v1::Response>,
            tonic::Status,
        >;
        async fn unlock_v1(
            &self,
            request: tonic::Request<super::achievement_unlock_v1::Request>,
        ) -> std::result::Result<
            tonic::Response<super::achievement_unlock_v1::Response>,
            tonic::Status,
        >;
        async fn unlock_and_save_player_storage_v1(
            &self,
            request: tonic::Request<
                super::achievement_unlock_and_save_player_storage_v1::Request,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::achievement_unlock_and_save_player_storage_v1::Response,
            >,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AchievementServer<T: Achievement> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Achievement> AchievementServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AchievementServer<T>
    where
        T: Achievement,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/Achievement/GetAvailableV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableV1Svc<T: Achievement>(pub Arc<T>);
                    impl<
                        T: Achievement,
                    > tonic::server::UnaryService<
                        super::achievement_get_available_v1::Request,
                    > for GetAvailableV1Svc<T> {
                        type Response = super::achievement_get_available_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::achievement_get_available_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableV1Svc(inner);
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
                "/Achievement/GetAvailableByIDsV1" => {
                    #[allow(non_camel_case_types)]
                    struct GetAvailableByIDsV1Svc<T: Achievement>(pub Arc<T>);
                    impl<
                        T: Achievement,
                    > tonic::server::UnaryService<
                        super::achievement_get_available_by_i_ds_v1::Request,
                    > for GetAvailableByIDsV1Svc<T> {
                        type Response = super::achievement_get_available_by_i_ds_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::achievement_get_available_by_i_ds_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_available_by_i_ds_v1(request).await
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
                        let inner = inner.0;
                        let method = GetAvailableByIDsV1Svc(inner);
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
                "/Achievement/UnlockV1" => {
                    #[allow(non_camel_case_types)]
                    struct UnlockV1Svc<T: Achievement>(pub Arc<T>);
                    impl<
                        T: Achievement,
                    > tonic::server::UnaryService<super::achievement_unlock_v1::Request>
                    for UnlockV1Svc<T> {
                        type Response = super::achievement_unlock_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::achievement_unlock_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).unlock_v1(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnlockV1Svc(inner);
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
                "/Achievement/UnlockAndSavePlayerStorageV1" => {
                    #[allow(non_camel_case_types)]
                    struct UnlockAndSavePlayerStorageV1Svc<T: Achievement>(pub Arc<T>);
                    impl<
                        T: Achievement,
                    > tonic::server::UnaryService<
                        super::achievement_unlock_and_save_player_storage_v1::Request,
                    > for UnlockAndSavePlayerStorageV1Svc<T> {
                        type Response = super::achievement_unlock_and_save_player_storage_v1::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::achievement_unlock_and_save_player_storage_v1::Request,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).unlock_and_save_player_storage_v1(request).await
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
                        let inner = inner.0;
                        let method = UnlockAndSavePlayerStorageV1Svc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Achievement> Clone for AchievementServer<T> {
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
    impl<T: Achievement> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Achievement> tonic::server::NamedService for AchievementServer<T> {
        const NAME: &'static str = "Achievement";
    }
}

// AUTO-GENERATED SUBMODULES - DO NOT EDIT

pub mod debug;
