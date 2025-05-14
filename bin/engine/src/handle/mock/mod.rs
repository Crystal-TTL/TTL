// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::handle::Handler;
use async_trait::async_trait;
use base::model::RequestToProcess;
use common::repo::Tx;

pub(crate) struct MockHandler {}

#[async_trait]
impl Handler for MockHandler {
    async fn handle<'a>(&self, _tx: &mut Tx<'a>, _request: RequestToProcess) {
        todo!()
    }
}
