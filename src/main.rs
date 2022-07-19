use libcnb::{
    build::{BuildContext, BuildResult, BuildResultBuilder},
    data::{layer_content_metadata::LayerTypes, layer_name},
    detect::{DetectContext, DetectResult, DetectResultBuilder},
    generic::{GenericError, GenericMetadata, GenericPlatform},
    layer::{
        ExistingLayerStrategy, Layer, LayerData, LayerResult, LayerResultBuilder, MetadataMigration,
    },
    Buildpack,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

struct SampleBuildpack;

impl Buildpack for SampleBuildpack {
    type Platform = GenericPlatform;
    type Metadata = GenericMetadata;
    type Error = GenericError;

    fn detect(&self, _ctx: DetectContext<Self>) -> libcnb::Result<DetectResult, Self::Error> {
        DetectResultBuilder::pass().build()
    }
    fn build(&self, ctx: BuildContext<Self>) -> libcnb::Result<BuildResult, Self::Error> {
        ctx.handle_layer(layer_name!("counting"), CountingLayer)?;
        BuildResultBuilder::new().build()
    }
}

struct CountingLayer;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct CountingMetadata {
    counter_u64: u64,
    counter_i32: i32,
}

impl Layer for CountingLayer {
    type Buildpack = SampleBuildpack;
    type Metadata = CountingMetadata;
    fn types(&self) -> LayerTypes {
        LayerTypes {
            build: true,
            launch: true,
            cache: true,
        }
    }

    fn existing_layer_strategy(
        &self,
        _ctx: &BuildContext<Self::Buildpack>,
        _layer_data: &LayerData<Self::Metadata>,
    ) -> Result<ExistingLayerStrategy, GenericError> {
        Ok(ExistingLayerStrategy::Update)
    }

    fn create(
        &self,
        _ctx: &BuildContext<Self::Buildpack>,
        _path: &Path,
    ) -> Result<LayerResult<Self::Metadata>, GenericError> {
        LayerResultBuilder::new(CountingMetadata {
            counter_u64: 1,
            counter_i32: 1,
        })
        .build()
    }

    fn update(
        &self,
        _ctx: &BuildContext<Self::Buildpack>,
        layer_data: &LayerData<Self::Metadata>,
    ) -> Result<LayerResult<Self::Metadata>, GenericError> {
        LayerResultBuilder::new(CountingMetadata {
            counter_u64: layer_data.content_metadata.metadata.counter_u64 + 1,
            counter_i32: layer_data.content_metadata.metadata.counter_i32 + 1,
        })
        .build()
    }

    fn migrate_incompatible_metadata(
        &self,
        _ctx: &BuildContext<Self::Buildpack>,
        metadata: &GenericMetadata,
    ) -> Result<MetadataMigration<Self::Metadata>, <Self::Buildpack as Buildpack>::Error> {
        println!("Previous layer data invalid!!!");
        println!("metadata: {metadata:?}");
        Ok(MetadataMigration::RecreateLayer)
    }
}

libcnb::buildpack_main!(SampleBuildpack);
