pub mod error;

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use axum_server::Handle;
use error::ApiServerError;
use std::net::SocketAddr;
use std::path::PathBuf;
use tracing::error;

pub struct APIServer {
    address: SocketAddr,
    router: Router,
    rust_ls_config: Option<RustlsConfig>,
}

impl APIServer {
    pub fn new(address: SocketAddr, router: Router) -> Self {
        Self {
            address,
            router,
            rust_ls_config: None,
        }
    }

    pub async fn add_certs(
        mut self,
        certificate_pem_file: PathBuf,
        private_key_pem_file: PathBuf,
    ) -> Result<Self, ApiServerError> {
        match RustlsConfig::from_pem_file(certificate_pem_file, private_key_pem_file).await {
            Ok(config) => {
                self.rust_ls_config = Some(config);
                Ok(self)
            }
            Err(error) => Err(ApiServerError::CertError(error.to_string())),
        }
    }

    pub async fn start(self) -> Result<Handle, ApiServerError> {
        // Create the shutdown handle
        let server_shutdown_handle = Handle::new();

        match self.rust_ls_config {
            Some(cert_config) => {
                let listener = match tokio::net::TcpListener::bind(self.address.to_string()).await {
                    Ok(listener) => listener.into_std().unwrap(),
                    Err(error) => {
                        error!(errorMsg=%error,"APIServer listener failed to start");
                        return Err(ApiServerError::ServerError(error.to_string()));
                    }
                };

                // let listener = match tokio::net::TcpListener::bind(self.address.to_string()).await {
                //     Ok(listener) => listener.into_std().unwrap(),
                //     Err(error) => {
                //         error!(errorMsg=%error,"APIServer listener failed to start");
                //         panic!("{}", error)
                //     }
                // };

                let server = axum_server::from_tcp_rustls(listener, cert_config)
                    .handle(server_shutdown_handle.clone())
                    .serve(
                        self.router
                            .into_make_service_with_connect_info::<SocketAddr>(),
                    );

                tokio::spawn(async move {
                    if let Err(error) = server.await {
                        error!(errorMsg=%error,"APIServer failed to start");
                    }
                });
            }
            None => {
                // let listener = match tokio::net::TcpListener::bind(self.address.to_string()).await {
                //     Ok(listener) => listener.into_std().unwrap(),
                //     Err(error) => {
                //         error!(errorMsg=%error,"APIServer listener failed to start");
                //         panic!("{}", error)
                //     }
                // };

                let server = axum_server::bind(self.address)
                    .handle(server_shutdown_handle.clone())
                    .serve(
                        self.router
                            .into_make_service_with_connect_info::<SocketAddr>(),
                    );

                // tokio::spawn(async move {
                //     if let Err(error) = axum_server::bind(self.address)
                //         .handle(server_handle_clone)
                //         .serve(
                //             self.router
                //                 .into_make_service_with_connect_info::<SocketAddr>(),
                //         )
                //         .await
                //     {
                //         error!(errorMsg=%error,"APIServer failed to start");
                //     }
                // });

                // tokio::spawn(async move {
                //     if let Err(error) = axum_server::bind(self.address)
                //         .handle(server_handle_clone)
                //         .serve(
                //             self.router
                //                 .into_make_service_with_connect_info::<SocketAddr>(),
                //         )
                //         .await
                //     {
                //         error!(errorMsg=%error,"APIServer failed to start");
                //     }
                // });
                tokio::spawn(async move {
                    if let Err(error) = server.await {
                        error!(errorMsg=%error,"APIServer failed to start");
                    }
                });
            }
        }

        // if let Err(error) = axum_server::bind(self.address)
        //     .handle(server_handle_clone)
        //     .serve(
        //         self.router
        //             .into_make_service_with_connect_info::<SocketAddr>(),
        //     )
        //     .await
        // {
        //     return Err(ApiServerError::ServerError(error.to_string()));
        // }

        // tokio::spawn(async move {
        //     if let Err(error) = axum_server::bind(self.address)
        //         .handle(server_handle_clone)
        //         .serve(
        //             self.router
        //                 .into_make_service_with_connect_info::<SocketAddr>(),
        //         )
        //         .await
        //     {
        //         error!(errorMsg=%error, "Unable to startAPI Server");
        //     }
        // });

        Ok(server_shutdown_handle)
    }
}
