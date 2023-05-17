// Copyright © Aptos Foundation

use aptos_indexer_grpc_cache_worker::IndexerGrpcCacheWorkerConfig;
use aptos_indexer_grpc_server_framework::RunnableConfig;
use clap::Parser;
use std::{fs, path::PathBuf, str::FromStr};
use tempfile::TempDir;
use testcontainers::{clients, Client, core::WaitFor, images::generic::GenericImage};

use super::{VALIDATOR_IMAGE_REPO, VALIDATOR_IMAGE_TAG};

// Tests that spin up a fullnode and connect to it

#[tokio::test]
pub async fn setup_indexer_grpc_all() {
    aptos_logger::Logger::init_for_testing();
    let docker = clients::Cli::default();

    // start fullnode
    let validator_config_path = PathBuf::from_str("test_fullnode_config.yaml").unwrap();
    let validator_wait_for = WaitFor::message_on_stdout("Aptos is running, press ctrl-c to exit");
    let validator_image = GenericImage::new(VALIDATOR_IMAGE_REPO, VALIDATOR_IMAGE_TAG)
        .with_entrypoint("/usr/local/bin/aptos-node --test --test-config-override /opt/aptos/etc/override.yaml")
        .with_volume(
            validator_config_path.as_os_str().to_str().unwrap(),
            "/opt/aptos/etc/override.yaml",
        ); // with a waitfor?

    println!("validator_image: {:?}", validator_image);

    let validator_container = docker.run(validator_image);
    let validator_rest_api_ipv4_port = validator_container.get_host_port_ipv4(8080);
    let validator_grpc_ipv4_port = validator_container.get_host_port_ipv4(50051);

    // start redis
    let redis_wait_for = WaitFor::message_on_stdout("Ready to accept connections");
    let redis_image = GenericImage::new("redis", "latest").with_wait_for(redis_wait_for);
    let redis_container = docker.run(redis_image);
    let redis_ipv4_port = redis_container.get_host_port_ipv4(6379);

    // create cache worker
    let file_store_dir = TempDir::new().expect("Could not create temp dir");
    let cache_worker_config = IndexerGrpcCacheWorkerConfig {
        server_name: "cache_worker".to_string(),
        fullnode_grpc_address: format!("127.0.0.1:{}", validator_grpc_ipv4_port),
        file_store_config:
            aptos_indexer_grpc_utils::config::IndexerGrpcFileStoreConfig::LocalFileStore(
                aptos_indexer_grpc_utils::config::LocalFileStore {
                    local_file_store_path: file_store_dir.path().to_path_buf(),
                },
            ),
        redis_main_instance_address: format!("127.0.0.1:{}", redis_ipv4_port),
    };
    cache_worker_config.run().await.unwrap();

    // let tmp_dir = TempDir::new().expect("Could not create temp dir");
    // let tmp_dir_path_str = tmp_dir.path().as_os_str().to_str().unwrap();

    // // create configs
    // for (i, service_name) in vec!["cache_worker", "file_store", "data_service"]
    //     .iter()
    //     .enumerate()
    // {
    //     let config = IndexerGrpcConfig {
    //         fullnode_grpc_address: Some("127.0.0.1:50051".to_string()),
    //         data_service_grpc_listen_address: Some("127.0.0.1:50052".to_string()),
    //         validator_address: format!("127.0.0.1:{}", validator_ipv4_port),
    //         file_store: IndexerGrpcFileStoreConfig::LocalFileStore(LocalFileStore {
    //             local_file_store_path: tmp_dir.path().to_path_buf(),
    //         }),
    //         health_check_port: 9090 + i as u16,
    //         whitelisted_auth_tokens: Some(vec!["dummytoken".to_string()]),
    //     };

    //     let config_path = tmp_dir
    //         .path()
    //         .join(format!("test_indexer_grpc_{}.yaml", service_name));
    //     fs::write(config_path, serde_yaml::to_string(&config).unwrap());
    // }
}
