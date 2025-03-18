// @generated
/// Generated client implementations.
pub mod injective_campaign_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct InjectiveCampaignRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InjectiveCampaignRpcClient<tonic::transport::Channel> {
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
    impl<T> InjectiveCampaignRpcClient<T>
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
        ) -> InjectiveCampaignRpcClient<InterceptedService<T, F>>
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
            InjectiveCampaignRpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn ranking(
            &mut self,
            request: impl tonic::IntoRequest<super::RankingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RankingResponse>,
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
                "/injective_campaign_rpc.InjectiveCampaignRPC/Ranking",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_campaign_rpc.InjectiveCampaignRPC",
                        "Ranking",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn campaigns(
            &mut self,
            request: impl tonic::IntoRequest<super::CampaignsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CampaignsResponse>,
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
                "/injective_campaign_rpc.InjectiveCampaignRPC/Campaigns",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_campaign_rpc.InjectiveCampaignRPC",
                        "Campaigns",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn campaigns_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::CampaignsV2Request>,
        ) -> std::result::Result<
            tonic::Response<super::CampaignsV2Response>,
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
                "/injective_campaign_rpc.InjectiveCampaignRPC/CampaignsV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_campaign_rpc.InjectiveCampaignRPC",
                        "CampaignsV2",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_guilds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGuildsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGuildsResponse>,
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
                "/injective_campaign_rpc.InjectiveCampaignRPC/ListGuilds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_campaign_rpc.InjectiveCampaignRPC",
                        "ListGuilds",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_guild_members(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGuildMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGuildMembersResponse>,
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
                "/injective_campaign_rpc.InjectiveCampaignRPC/ListGuildMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_campaign_rpc.InjectiveCampaignRPC",
                        "ListGuildMembers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_guild_member(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGuildMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGuildMemberResponse>,
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
                "/injective_campaign_rpc.InjectiveCampaignRPC/GetGuildMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "injective_campaign_rpc.InjectiveCampaignRPC",
                        "GetGuildMember",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod injective_campaign_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InjectiveCampaignRpcServer.
    #[async_trait]
    pub trait InjectiveCampaignRpc: Send + Sync + 'static {
        async fn ranking(
            &self,
            request: tonic::Request<super::RankingRequest>,
        ) -> std::result::Result<tonic::Response<super::RankingResponse>, tonic::Status>;
        async fn campaigns(
            &self,
            request: tonic::Request<super::CampaignsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CampaignsResponse>,
            tonic::Status,
        >;
        async fn campaigns_v2(
            &self,
            request: tonic::Request<super::CampaignsV2Request>,
        ) -> std::result::Result<
            tonic::Response<super::CampaignsV2Response>,
            tonic::Status,
        >;
        async fn list_guilds(
            &self,
            request: tonic::Request<super::ListGuildsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGuildsResponse>,
            tonic::Status,
        >;
        async fn list_guild_members(
            &self,
            request: tonic::Request<super::ListGuildMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGuildMembersResponse>,
            tonic::Status,
        >;
        async fn get_guild_member(
            &self,
            request: tonic::Request<super::GetGuildMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGuildMemberResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct InjectiveCampaignRpcServer<T: InjectiveCampaignRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InjectiveCampaignRpc> InjectiveCampaignRpcServer<T> {
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
    for InjectiveCampaignRpcServer<T>
    where
        T: InjectiveCampaignRpc,
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
                "/injective_campaign_rpc.InjectiveCampaignRPC/Ranking" => {
                    #[allow(non_camel_case_types)]
                    struct RankingSvc<T: InjectiveCampaignRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveCampaignRpc,
                    > tonic::server::UnaryService<super::RankingRequest>
                    for RankingSvc<T> {
                        type Response = super::RankingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RankingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).ranking(request).await };
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
                        let method = RankingSvc(inner);
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
                "/injective_campaign_rpc.InjectiveCampaignRPC/Campaigns" => {
                    #[allow(non_camel_case_types)]
                    struct CampaignsSvc<T: InjectiveCampaignRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveCampaignRpc,
                    > tonic::server::UnaryService<super::CampaignsRequest>
                    for CampaignsSvc<T> {
                        type Response = super::CampaignsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CampaignsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).campaigns(request).await };
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
                        let method = CampaignsSvc(inner);
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
                "/injective_campaign_rpc.InjectiveCampaignRPC/CampaignsV2" => {
                    #[allow(non_camel_case_types)]
                    struct CampaignsV2Svc<T: InjectiveCampaignRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveCampaignRpc,
                    > tonic::server::UnaryService<super::CampaignsV2Request>
                    for CampaignsV2Svc<T> {
                        type Response = super::CampaignsV2Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CampaignsV2Request>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).campaigns_v2(request).await
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
                        let method = CampaignsV2Svc(inner);
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
                "/injective_campaign_rpc.InjectiveCampaignRPC/ListGuilds" => {
                    #[allow(non_camel_case_types)]
                    struct ListGuildsSvc<T: InjectiveCampaignRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveCampaignRpc,
                    > tonic::server::UnaryService<super::ListGuildsRequest>
                    for ListGuildsSvc<T> {
                        type Response = super::ListGuildsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListGuildsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_guilds(request).await };
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
                        let method = ListGuildsSvc(inner);
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
                "/injective_campaign_rpc.InjectiveCampaignRPC/ListGuildMembers" => {
                    #[allow(non_camel_case_types)]
                    struct ListGuildMembersSvc<T: InjectiveCampaignRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveCampaignRpc,
                    > tonic::server::UnaryService<super::ListGuildMembersRequest>
                    for ListGuildMembersSvc<T> {
                        type Response = super::ListGuildMembersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListGuildMembersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_guild_members(request).await
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
                        let method = ListGuildMembersSvc(inner);
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
                "/injective_campaign_rpc.InjectiveCampaignRPC/GetGuildMember" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuildMemberSvc<T: InjectiveCampaignRpc>(pub Arc<T>);
                    impl<
                        T: InjectiveCampaignRpc,
                    > tonic::server::UnaryService<super::GetGuildMemberRequest>
                    for GetGuildMemberSvc<T> {
                        type Response = super::GetGuildMemberResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGuildMemberRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_guild_member(request).await
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
                        let method = GetGuildMemberSvc(inner);
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
    impl<T: InjectiveCampaignRpc> Clone for InjectiveCampaignRpcServer<T> {
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
    impl<T: InjectiveCampaignRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InjectiveCampaignRpc> tonic::server::NamedService
    for InjectiveCampaignRpcServer<T> {
        const NAME: &'static str = "injective_campaign_rpc.InjectiveCampaignRPC";
    }
}
