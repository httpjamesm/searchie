use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

use crate::controllers::datapoint_controller::DatapointController;
use crate::repositories::indexing_queue_repository::IndexingQueueRepository;

pub struct IndexingWorker {
    queue_repository: Arc<IndexingQueueRepository>,
    datapoint_controller: Arc<DatapointController>,
}

impl IndexingWorker {
    pub fn new(
        queue_repository: Arc<IndexingQueueRepository>,
        datapoint_controller: Arc<DatapointController>,
    ) -> Self {
        Self {
            queue_repository,
            datapoint_controller,
        }
    }

    pub async fn start(&self) {
        loop {
            if let Err(e) = self.process_next().await {
                eprintln!("Error processing queue item: {}", e);
            }
            sleep(Duration::from_secs(1)).await;
        }
    }

    async fn process_next(&self) -> Result<()> {
        let Some(item) = self.queue_repository.get_next_pending().await? else {
            return Ok(());
        };

        self.queue_repository.mark_processing(item.id).await?;

        match self
            .datapoint_controller
            .index_datapoint(item.datapoint_id)
            .await
        {
            Ok(_) => {
                self.queue_repository.mark_completed(item.id).await?;
            }
            Err(e) => {
                self.queue_repository
                    .mark_failed(item.id, &e.to_string())
                    .await?;
            }
        }

        Ok(())
    }
}
