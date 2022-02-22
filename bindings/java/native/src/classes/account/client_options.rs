// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{cell::RefCell, rc::Rc, time::Duration};

use iota_wallet::client::{
    Api, BrokerOptions as BrokerOptionsRust, ClientBuilder as ClientBuilderRust,
    ClientBuilder as ClientBuilderRust,
};

use crate::Result;

pub struct BrokerOptions {
    builder: Rc<RefCell<Option<BrokerOptionsRust>>>,
}

impl BrokerOptions {
    pub fn new() -> Self {
        Self {
            builder: Rc::new(RefCell::new(Option::from(BrokerOptionsRust {
                automatic_disconnect: None,
                timeout: None,
                use_ws: None,
                port: None,
                max_reconnection_attempts: None,
            }))),
        }
    }

    fn new_with(options: BrokerOptionsRust) -> BrokerOptions {
        Self {
            builder: Rc::new(RefCell::new(Option::from(options))),
        }
    }

    pub fn automatic_disconnect(&self, disconnect: bool) -> BrokerOptions {
        let mut builder = self.builder.borrow_mut().take().unwrap();
        builder.automatic_disconnect = Some(disconnect);
        BrokerOptions::new_with(builder)
    }

    pub fn timeout(&self, timeout: Duration) -> BrokerOptions {
        let mut builder = self.builder.borrow_mut().take().unwrap();
        builder.timeout = Some(timeout);
        BrokerOptions::new_with(builder)
    }
    pub fn use_ws(&self, use_ws: bool) -> BrokerOptions {
        let mut builder = self.builder.borrow_mut().take().unwrap();
        builder.use_ws = Some(use_ws);
        BrokerOptions::new_with(builder)
    }
    pub fn port(&self, port: u16) -> BrokerOptions {
        let mut builder = self.builder.borrow_mut().take().unwrap();
        builder.port = Some(port);
        BrokerOptions::new_with(builder)
    }
    pub fn max_reconnection_attempts(&self, max_reconnection_attempts: usize) -> BrokerOptions {
        let mut builder = self.builder.borrow_mut().take().unwrap();
        builder.max_reconnection_attempts = Some(max_reconnection_attempts);
        BrokerOptions::new_with(builder)
    }
}

pub struct ClientBuilder {
    options: ClientBuilderRust,
}


impl core::fmt::Display for ClientBuilder {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}", 
            self.options
        )
    }
}

impl From<ClientBuilderRust> for ClientBuilder {
    fn from(options: ClientBuilderRust) -> Self {
        Self {
            options
        }
    }
}

impl ClientBuilder {
    pub fn to_inner(self) -> ClientBuilderRust {
        self.options
    }
}

pub struct ClientBuilder {
    builder: Rc<RefCell<Option<ClientBuilderRust>>>,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            builder: Rc::new(RefCell::new(Option::from(ClientBuilderRust::default()))),
        }
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    fn new_with_builder(builder: ClientBuilderRust) -> Self {
        Self {
            builder: Rc::new(RefCell::new(Option::from(builder))),
        }
    }

    pub fn with_primary_node(&mut self, node: &str) -> ClientBuilder {
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_primary_node(node)
            .unwrap();
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_primary_pow_node(&mut self, node: &str) -> ClientBuilder {
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_primary_pow_node(node)
            .unwrap();
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_node(&mut self, node: &str) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_node(node).unwrap();
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_node_pool_urls(&mut self, node_pool_urls: Vec<String>) -> ClientBuilder {
        let nodes_urls: Vec<&str> = node_pool_urls.iter().map(|x| &**x).collect();
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_node_pool_urls(&nodes_urls)
            .unwrap();
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_network(&mut self, network: String) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_network(network);
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_node_sync_interval(&mut self, node_sync_interval: Duration) -> ClientBuilder {
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_node_sync_interval(node_sync_interval);
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_node_sync_disabled(&mut self) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_node_sync_disabled();
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_mqtt_disabled(&mut self) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_mqtt_disabled();
        ClientBuilder::new_with_builder(new_builder)
    }

    /// Sets the MQTT broker options.
    pub fn with_mqtt_mqtt_broker_options(&mut self, options: BrokerOptions) -> ClientBuilder {
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_mqtt_mqtt_broker_options(options.builder.borrow_mut().take().unwrap());
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_local_pow(&mut self, local: bool) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_local_pow(local);
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_request_timeout(&mut self, timeout: Duration) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_request_timeout(timeout);
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn with_api_timeout(&mut self, api: Api, timeout: Duration) -> ClientBuilder {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_api_timeout(api, timeout);
        ClientBuilder::new_with_builder(new_builder)
    }

    pub fn build(&mut self) -> Result<ClientBuilder> {
        Ok(ClientBuilder {
            options: self.builder.borrow_mut().take().unwrap().build().unwrap(),
        })
    }
}
