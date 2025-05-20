use sea_orm::{EntityTrait, Order, QueryOrder};

use super::{
    entity::{self, prelude::*},
    DatabaseClient,
};
use crate::{localized_resource_handler, prelude::*};

localized_resource_handler!(
    Versions,
    VersionNames,
    entity::version_names::Column::LocalLanguageId,
    entity::version_names::Column::Name
);

impl DatabaseClient {
    pub async fn get_latest_version(&self) -> Result<entity::versions::Model> {
        Versions::find()
            .order_by(entity::versions::Column::Id, Order::Desc)
            .one(&self.conn)
            .await
            .map_err(Error::ConnectionError)?
            .ok_or(Error::ResourceNotFound(
                "Latest version not found".to_string(),
            ))
    }

    pub async fn get_latest_version_group(&self) -> Result<entity::version_groups::Model> {
        VersionGroups::find()
            .order_by(entity::version_groups::Column::Id, Order::Desc)
            .one(&self.conn)
            .await
            .map_err(Error::ConnectionError)?
            .ok_or(Error::ResourceNotFound(
                "Latest version group not found".to_string(),
            ))
    }
}
