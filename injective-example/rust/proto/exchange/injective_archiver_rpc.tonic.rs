// @generated
/// Generated client implementations.
pub mod injective_archiver_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct InjectiveArchiverRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InjectiveArchiverRpcClient<tonic::transport::Channel> {
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
    impl<T> InjectiveArchiverRpcClient<T>
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
        ) -> InjectiveArchiverRpcClient<InterceptedService<T, F>>
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
            InjectiveArchiverRpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn balance(
            &mut self,
            request: impl tonic::IntoRequest<super::BalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BalanceResponse>,
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/Balance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_archiver_rpc.InjectiveArchiverRPC",
                        "Balance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn rpnl(
            &mut self,
            request: impl tonic::IntoRequest<super::RpnlRequest>,
        ) -> std::result::Result<tonic::Response<super::RpnlResponse>, tonic::Status> {
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/Rpnl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_archiver_rpc.InjectiveArchiverRPC",
                        "Rpnl",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn volumes(
            &mut self,
            request: impl tonic::IntoRequest<super::VolumesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VolumesResponse>,
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/Volumes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_archiver_rpc.InjectiveArchiverRPC",
                        "Volumes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn pnl_leaderboard(
            &mut self,
            request: impl tonic::IntoRequest<super::PnlLeaderboardRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PnlLeaderboardResponse>,
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/PnlLeaderboard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_archiver_rpc.InjectiveArchiverRPC",
                        "PnlLeaderboard",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn vol_leaderboard(
            &mut self,
            request: impl tonic::IntoRequest<super::VolLeaderboardRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VolLeaderboardResponse>,
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/VolLeaderboard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_archiver_rpc.InjectiveArchiverRPC",
                        "VolLeaderboard",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn pnl_leaderboard_fixed_resolution(
            &mut self,
            request: impl tonic::IntoRequest<super::PnlLeaderboardFixedResolutionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PnlLeaderboardFixedResolutionResponse>,
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/PnlLeaderboardFixedResolution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_archiver_rpc.InjectiveArchiverRPC",
                        "PnlLeaderboardFixedResolution",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn vol_leaderboard_fixed_resolution(
            &mut self,
            request: impl tonic::IntoRequest<super::VolLeaderboardFixedResolutionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VolLeaderboardFixedResolutionResponse>,
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/VolLeaderboardFixedResolution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_archiver_rpc.InjectiveArchiverRPC",
                        "VolLeaderboardFixedResolution",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn denom_holders(
            &mut self,
            request: impl tonic::IntoRequest<super::DenomHoldersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DenomHoldersResponse>,
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/DenomHolders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_archiver_rpc.InjectiveArchiverRPC",
                        "DenomHolders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn historical_trades(
            &mut self,
            request: impl tonic::IntoRequest<super::HistoricalTradesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HistoricalTradesResponse>,
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/HistoricalTrades",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_archiver_rpc.InjectiveArchiverRPC",
                        "HistoricalTrades",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod injective_archiver_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InjectiveArchiverRpcServer.
    #[async_trait]
    pub trait InjectiveArchiverRpc: Send + Sync + 'static {
        async fn balance(
            &self,
            request: tonic::Request<super::BalanceRequest>,
        ) -> std::result::Result<tonic::Response<super::BalanceResponse>, tonic::Status>;
        async fn rpnl(
            &self,
            request: tonic::Request<super::RpnlRequest>,
        ) -> std::result::Result<tonic::Response<super::RpnlResponse>, tonic::Status>;
        async fn volumes(
            &self,
            request: tonic::Request<super::VolumesRequest>,
        ) -> std::result::Result<tonic::Response<super::VolumesResponse>, tonic::Status>;
        async fn pnl_leaderboard(
            &self,
            request: tonic::Request<super::PnlLeaderboardRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PnlLeaderboardResponse>,
            tonic::Status,
        >;
        async fn vol_leaderboard(
            &self,
            request: tonic::Request<super::VolLeaderboardRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VolLeaderboardResponse>,
            tonic::Status,
        >;
        async fn pnl_leaderboard_fixed_resolution(
            &self,
            request: tonic::Request<super::PnlLeaderboardFixedResolutionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PnlLeaderboardFixedResolutionResponse>,
            tonic::Status,
        >;
        async fn vol_leaderboard_fixed_resolution(
            &self,
            request: tonic::Request<super::VolLeaderboardFixedResolutionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VolLeaderboardFixedResolutionResponse>,
            tonic::Status,
        >;
        async fn denom_holders(
            &self,
            request: tonic::Request<super::DenomHoldersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DenomHoldersResponse>,
            tonic::Status,
        >;
        async fn historical_trades(
            &self,
            request: tonic::Request<super::HistoricalTradesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HistoricalTradesResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct InjectiveArchiverRpcServer<T: InjectiveArchiverRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InjectiveArchiverRpc> InjectiveArchiverRpcServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for InjectiveArchiverRpcServer<T>
    where
        T: InjectiveArchiverRpc,
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/Balance" => {
                    #[allow(non_camel_case_types)]
                    struct BalanceSvc<T: InjectiveArchiverRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveArchiverRpc,
                    > tonic::server::UnaryService<super::BalanceRequest>
                    for BalanceSvc<T> {
                        type Response = super::BalanceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BalanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).balance(request).await };
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
                        let method = BalanceSvc(inner);
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/Rpnl" => {
                    #[allow(non_camel_case_types)]
                    struct RpnlSvc<T: InjectiveArchiverRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveArchiverRpc,
                    > tonic::server::UnaryService<super::RpnlRequest> for RpnlSvc<T> {
                        type Response = super::RpnlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RpnlRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).rpnl(request).await };
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
                        let method = RpnlSvc(inner);
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/Volumes" => {
                    #[allow(non_camel_case_types)]
                    struct VolumesSvc<T: InjectiveArchiverRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveArchiverRpc,
                    > tonic::server::UnaryService<super::VolumesRequest>
                    for VolumesSvc<T> {
                        type Response = super::VolumesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolumesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).volumes(request).await };
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
                        let method = VolumesSvc(inner);
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/PnlLeaderboard" => {
                    #[allow(non_camel_case_types)]
                    struct PnlLeaderboardSvc<T: InjectiveArchiverRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveArchiverRpc,
                    > tonic::server::UnaryService<super::PnlLeaderboardRequest>
                    for PnlLeaderboardSvc<T> {
                        type Response = super::PnlLeaderboardResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PnlLeaderboardRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).pnl_leaderboard(request).await
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
                        let method = PnlLeaderboardSvc(inner);
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/VolLeaderboard" => {
                    #[allow(non_camel_case_types)]
                    struct VolLeaderboardSvc<T: InjectiveArchiverRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveArchiverRpc,
                    > tonic::server::UnaryService<super::VolLeaderboardRequest>
                    for VolLeaderboardSvc<T> {
                        type Response = super::VolLeaderboardResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VolLeaderboardRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).vol_leaderboard(request).await
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
                        let method = VolLeaderboardSvc(inner);
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/PnlLeaderboardFixedResolution" => {
                    #[allow(non_camel_case_types)]
                    struct PnlLeaderboardFixedResolutionSvc<T: InjectiveArchiverRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InjectiveArchiverRpc,
                    > tonic::server::UnaryService<
                        super::PnlLeaderboardFixedResolutionRequest,
                    > for PnlLeaderboardFixedResolutionSvc<T> {
                        type Response = super::PnlLeaderboardFixedResolutionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PnlLeaderboardFixedResolutionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).pnl_leaderboard_fixed_resolution(request).await
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
                        let method = PnlLeaderboardFixedResolutionSvc(inner);
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/VolLeaderboardFixedResolution" => {
                    #[allow(non_camel_case_types)]
                    struct VolLeaderboardFixedResolutionSvc<T: InjectiveArchiverRpc>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InjectiveArchiverRpc,
                    > tonic::server::UnaryService<
                        super::VolLeaderboardFixedResolutionRequest,
                    > for VolLeaderboardFixedResolutionSvc<T> {
                        type Response = super::VolLeaderboardFixedResolutionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::VolLeaderboardFixedResolutionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).vol_leaderboard_fixed_resolution(request).await
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
                        let method = VolLeaderboardFixedResolutionSvc(inner);
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/DenomHolders" => {
                    #[allow(non_camel_case_types)]
                    struct DenomHoldersSvc<T: InjectiveArchiverRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveArchiverRpc,
                    > tonic::server::UnaryService<super::DenomHoldersRequest>
                    for DenomHoldersSvc<T> {
                        type Response = super::DenomHoldersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DenomHoldersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).denom_holders(request).await
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
                        let method = DenomHoldersSvc(inner);
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
                "/injective_archiver_rpc.InjectiveArchiverRPC/HistoricalTrades" => {
                    #[allow(non_camel_case_types)]
                    struct HistoricalTradesSvc<T: InjectiveArchiverRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveArchiverRpc,
                    > tonic::server::UnaryService<super::HistoricalTradesRequest>
                    for HistoricalTradesSvc<T> {
                        type Response = super::HistoricalTradesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HistoricalTradesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).historical_trades(request).await
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
                        let method = HistoricalTradesSvc(inner);
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
    impl<T: InjectiveArchiverRpc> Clone for InjectiveArchiverRpcServer<T> {
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
    impl<T: InjectiveArchiverRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InjectiveArchiverRpc> tonic::server::NamedService
    for InjectiveArchiverRpcServer<T> {
        const NAME: &'static str = "injective_archiver_rpc.InjectiveArchiverRPC";
    }
}
