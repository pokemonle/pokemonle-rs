#[macro_export]
macro_rules! define_extra_struct {
    ($name:ident { $($field:ident: $type:ty),* }) => {
        #[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema,aide::OperationIo)]
        pub struct $name<T> {
            #[serde(flatten)]
            pub item: T,
            $(
                pub $field: $type,
            )*
        }


    };
}

#[macro_export]
macro_rules! localized_resource_handler {
    ($entity:ty, $name_entity:ty, $lang_column:path) => {
        #[async_trait::async_trait]
        impl $crate::database::r#trait::LocalizedResourceHandler<$entity, $name_entity>
            for $crate::database::DatabaseClient
        {
            async fn list_with_pagination(
                &self,
                page: u64,
                limit: u64,
                lang: i32,
            ) -> $crate::error::Result<
                $crate::types::response::PaginatedResource<
                    $crate::types::WithName<<$entity as $crate::sea_orm::EntityTrait>::Model>,
                >,
            > {
                use sea_orm::prelude::*;
                let paginator = <$entity>::find()
                    .find_also_related(<$name_entity>::default())
                    .filter($lang_column.eq(lang))
                    .paginate(&self.conn, limit);

                let data = paginator
                    .fetch_page(page - 1)
                    .await?
                    .into_iter()
                    .map(|(entity, name_entity)| {
                        let name = match name_entity {
                            Some(ne) => ne.name,
                            None => entity.identifier.clone(),
                        };

                        $crate::types::WithName::new(entity, name)
                    })
                    .collect();

                let total = paginator.num_items_and_pages().await?;

                Ok($crate::types::response::PaginatedResource {
                    data,
                    page,
                    per_page: limit,
                    total_pages: total.number_of_pages,
                    total_items: total.number_of_items,
                })
            }

            async fn get_by_id(
                &self,
                id: i32,
                lang: i32,
            ) -> $crate::error::Result<
                $crate::types::WithName<<$entity as $crate::sea_orm::EntityTrait>::Model>,
            > {
                use sea_orm::prelude::*;
                let (entity, name_entity) = <$entity>::find_by_id(id)
                    .find_also_related(<$name_entity>::default())
                    .filter($lang_column.eq(lang))
                    .one(&self.conn)
                    .await
                    .map_err($crate::error::Error::ConnectionError)?
                    .ok_or($crate::error::Error::ResourceNotFound(format!(
                        "resource {} not found",
                        id
                    )))?;

                let name = match name_entity {
                    Some(ne) => ne.name,
                    None => entity.identifier.clone(),
                };

                Ok($crate::types::WithName::new(entity, name))
            }
        }
    };
}
