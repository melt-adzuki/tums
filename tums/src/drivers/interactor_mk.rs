use serde_json::json;

use crate::{confs::CONFS, domain::interactor::Interactor};

pub(crate) struct InteractorMisskeyImpl {
    client: reqwest::Client,
}

impl InteractorMisskeyImpl {
    pub(crate) fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Interactor for InteractorMisskeyImpl {
    async fn announce(&self, content: String) -> anyhow::Result<()> {
        self.client
            .post(format!("https://{}/api/notes/create", CONFS.mk_endpnt))
            .json(&json!({
                "i": CONFS.mk_token,
                "text": content,
                "visibility": "home",
            }))
            .send()
            .await?;

        Ok(())
    }

    async fn reply(&self, content: String, reply_id: String) -> anyhow::Result<()> {
        self.client
            .post(format!("https://{}/api/notes/create", CONFS.mk_endpnt))
            .json(&json!({
                "i": CONFS.mk_token,
                "text": content,
                "replyId": reply_id,
                "visibility": "home",
            }))
            .send()
            .await?;

        Ok(())
    }

    async fn ask_yes_no(
        &self,
        content: String,
        reply_id: String,
    ) -> anyhow::Result<crate::domain::interactor::YesNo> {
        todo!()
    }
}
