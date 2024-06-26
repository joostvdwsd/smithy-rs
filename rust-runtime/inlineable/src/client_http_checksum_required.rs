/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::borrow::Cow;

use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::interceptors::context::BeforeTransmitInterceptorContextMut;
use aws_smithy_runtime_api::client::interceptors::{Intercept, SharedInterceptor};
use aws_smithy_runtime_api::client::runtime_components::{
    RuntimeComponents, RuntimeComponentsBuilder,
};
use aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin;
use aws_smithy_types::base64;
use aws_smithy_types::config_bag::ConfigBag;

#[derive(Debug)]
pub(crate) struct HttpChecksumRequiredRuntimePlugin {
    runtime_components: RuntimeComponentsBuilder,
}

impl HttpChecksumRequiredRuntimePlugin {
    pub(crate) fn new() -> Self {
        Self {
            runtime_components: RuntimeComponentsBuilder::new("HttpChecksumRequiredRuntimePlugin")
                .with_interceptor(SharedInterceptor::new(HttpChecksumRequiredInterceptor)),
        }
    }
}

impl RuntimePlugin for HttpChecksumRequiredRuntimePlugin {
    fn runtime_components(
        &self,
        _: &RuntimeComponentsBuilder,
    ) -> Cow<'_, RuntimeComponentsBuilder> {
        Cow::Borrowed(&self.runtime_components)
    }
}

#[derive(Debug)]
struct HttpChecksumRequiredInterceptor;

impl Intercept for HttpChecksumRequiredInterceptor {
    fn name(&self) -> &'static str {
        "HttpChecksumRequiredInterceptor"
    }

    fn modify_before_signing(
        &self,
        context: &mut BeforeTransmitInterceptorContextMut<'_>,
        _runtime_components: &RuntimeComponents,
        _cfg: &mut ConfigBag,
    ) -> Result<(), BoxError> {
        let request = context.request_mut();
        let body_bytes = request
            .body()
            .bytes()
            .expect("checksum can only be computed for non-streaming operations");
        let checksum = <md5::Md5 as md5::Digest>::digest(body_bytes);
        request
            .headers_mut()
            .insert("content-md5", base64::encode(&checksum[..]));
        Ok(())
    }
}
