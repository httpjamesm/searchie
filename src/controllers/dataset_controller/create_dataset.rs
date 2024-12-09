use super::DatasetController;
use anyhow::Result;
use small_world_rs::{
    distance_metric::{CosineDistance, DistanceMetric},
    world::world::World,
};
impl DatasetController {
    pub async fn create_dataset(&self, name: &str) -> Result<String> {
        // force name to be lowercase
        let dataset_id = self
            .dataset_repository
            .create_dataset(name.to_lowercase().as_str())
            .await?;

        // create the world file for the dataset
        let world = World::new(24, 50, 40, DistanceMetric::Cosine(CosineDistance)).unwrap();
        // dump to file
        std::fs::write(
            &format!("indices/{}.smallworld", dataset_id),
            world.dump().unwrap(),
        )
        .unwrap();
        self.worlds.lock().await.insert(dataset_id.clone(), world);

        Ok(dataset_id)
    }
}
