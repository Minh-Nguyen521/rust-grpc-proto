// @generated
/// Generated client implementations.
pub mod event_provider_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct EventProviderApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EventProviderApiClient<tonic::transport::Channel> {
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
    impl<T> EventProviderApiClient<T>
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
        ) -> EventProviderApiClient<InterceptedService<T, F>>
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
            EventProviderApiClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_latest_height(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLatestHeightRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLatestHeightResponse>,
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
                "/event_provider_api.EventProviderAPI/GetLatestHeight",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "event_provider_api.EventProviderAPI",
                        "GetLatestHeight",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn stream_block_events(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamBlockEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamBlockEventsResponse>>,
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
                "/event_provider_api.EventProviderAPI/StreamBlockEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "event_provider_api.EventProviderAPI",
                        "StreamBlockEvents",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn get_block_events_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockEventsRpcRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlockEventsRpcResponse>,
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
                "/event_provider_api.EventProviderAPI/GetBlockEventsRPC",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "event_provider_api.EventProviderAPI",
                        "GetBlockEventsRPC",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_custom_events_rpc(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomEventsRpcRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomEventsRpcResponse>,
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
                "/event_provider_api.EventProviderAPI/GetCustomEventsRPC",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "event_provider_api.EventProviderAPI",
                        "GetCustomEventsRPC",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_abci_block_events(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAbciBlockEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAbciBlockEventsResponse>,
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
                "/event_provider_api.EventProviderAPI/GetABCIBlockEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "event_provider_api.EventProviderAPI",
                        "GetABCIBlockEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_abci_block_events_at_height(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAbciBlockEventsAtHeightRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAbciBlockEventsAtHeightResponse>,
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
                "/event_provider_api.EventProviderAPI/GetABCIBlockEventsAtHeight",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "event_provider_api.EventProviderAPI",
                        "GetABCIBlockEventsAtHeight",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod event_provider_api_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EventProviderApiServer.
    #[async_trait]
    pub trait EventProviderApi: Send + Sync + 'static {
        async fn get_latest_height(
            &self,
            request: tonic::Request<super::GetLatestHeightRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLatestHeightResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the StreamBlockEvents method.
        type StreamBlockEventsStream: futures_core::Stream<
                Item = std::result::Result<
                    super::StreamBlockEventsResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn stream_block_events(
            &self,
            request: tonic::Request<super::StreamBlockEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::StreamBlockEventsStream>,
            tonic::Status,
        >;
        async fn get_block_events_rpc(
            &self,
            request: tonic::Request<super::GetBlockEventsRpcRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlockEventsRpcResponse>,
            tonic::Status,
        >;
        async fn get_custom_events_rpc(
            &self,
            request: tonic::Request<super::GetCustomEventsRpcRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomEventsRpcResponse>,
            tonic::Status,
        >;
        async fn get_abci_block_events(
            &self,
            request: tonic::Request<super::GetAbciBlockEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAbciBlockEventsResponse>,
            tonic::Status,
        >;
        async fn get_abci_block_events_at_height(
            &self,
            request: tonic::Request<super::GetAbciBlockEventsAtHeightRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAbciBlockEventsAtHeightResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct EventProviderApiServer<T: EventProviderApi> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: EventProviderApi> EventProviderApiServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for EventProviderApiServer<T>
    where
        T: EventProviderApi,
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
                "/event_provider_api.EventProviderAPI/GetLatestHeight" => {
                    #[allow(non_camel_case_types)]
                    struct GetLatestHeightSvc<T: EventProviderApi>(pub Arc<T>);
                    impl<
                        T: EventProviderApi,
                    > tonic::server::UnaryService<super::GetLatestHeightRequest>
                    for GetLatestHeightSvc<T> {
                        type Response = super::GetLatestHeightResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLatestHeightRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_latest_height(request).await
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
                        let method = GetLatestHeightSvc(inner);
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
                "/event_provider_api.EventProviderAPI/StreamBlockEvents" => {
                    #[allow(non_camel_case_types)]
                    struct StreamBlockEventsSvc<T: EventProviderApi>(pub Arc<T>);
                    impl<
                        T: EventProviderApi,
                    > tonic::server::ServerStreamingService<
                        super::StreamBlockEventsRequest,
                    > for StreamBlockEventsSvc<T> {
                        type Response = super::StreamBlockEventsResponse;
                        type ResponseStream = T::StreamBlockEventsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamBlockEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_block_events(request).await
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
                        let method = StreamBlockEventsSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/event_provider_api.EventProviderAPI/GetBlockEventsRPC" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockEventsRPCSvc<T: EventProviderApi>(pub Arc<T>);
                    impl<
                        T: EventProviderApi,
                    > tonic::server::UnaryService<super::GetBlockEventsRpcRequest>
                    for GetBlockEventsRPCSvc<T> {
                        type Response = super::GetBlockEventsRpcResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockEventsRpcRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_block_events_rpc(request).await
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
                        let method = GetBlockEventsRPCSvc(inner);
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
                "/event_provider_api.EventProviderAPI/GetCustomEventsRPC" => {
                    #[allow(non_camel_case_types)]
                    struct GetCustomEventsRPCSvc<T: EventProviderApi>(pub Arc<T>);
                    impl<
                        T: EventProviderApi,
                    > tonic::server::UnaryService<super::GetCustomEventsRpcRequest>
                    for GetCustomEventsRPCSvc<T> {
                        type Response = super::GetCustomEventsRpcResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCustomEventsRpcRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_custom_events_rpc(request).await
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
                        let method = GetCustomEventsRPCSvc(inner);
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
                "/event_provider_api.EventProviderAPI/GetABCIBlockEvents" => {
                    #[allow(non_camel_case_types)]
                    struct GetABCIBlockEventsSvc<T: EventProviderApi>(pub Arc<T>);
                    impl<
                        T: EventProviderApi,
                    > tonic::server::UnaryService<super::GetAbciBlockEventsRequest>
                    for GetABCIBlockEventsSvc<T> {
                        type Response = super::GetAbciBlockEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAbciBlockEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_abci_block_events(request).await
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
                        let method = GetABCIBlockEventsSvc(inner);
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
                "/event_provider_api.EventProviderAPI/GetABCIBlockEventsAtHeight" => {
                    #[allow(non_camel_case_types)]
                    struct GetABCIBlockEventsAtHeightSvc<T: EventProviderApi>(
                        pub Arc<T>,
                    );
                    impl<
                        T: EventProviderApi,
                    > tonic::server::UnaryService<
                        super::GetAbciBlockEventsAtHeightRequest,
                    > for GetABCIBlockEventsAtHeightSvc<T> {
                        type Response = super::GetAbciBlockEventsAtHeightResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetAbciBlockEventsAtHeightRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_abci_block_events_at_height(request).await
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
                        let method = GetABCIBlockEventsAtHeightSvc(inner);
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
    impl<T: EventProviderApi> Clone for EventProviderApiServer<T> {
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
    impl<T: EventProviderApi> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EventProviderApi> tonic::server::NamedService for EventProviderApiServer<T> {
        const NAME: &'static str = "event_provider_api.EventProviderAPI";
    }
}
