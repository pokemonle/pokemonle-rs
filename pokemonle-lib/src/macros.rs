#[macro_export]
macro_rules! define_extra_struct {
    ($name:ident { $($field:ident: $type:ty),* }) => {
        #[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema,aide::OperationIo)]
        pub struct $name<T> where T: pokemonle_trait::StructName {
            #[serde(flatten)]
            pub item: T,
            $(
                pub $field: $type,
            )*
        }

        impl<T> pokemonle_trait::StructName for $name<T> where T: pokemonle_trait::StructName {
            fn struct_name() -> &'static str {
                T::struct_name()
            }

            fn tags() -> &'static [&'static str] {
                T::tags()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_handlers {
    // Match multiple handler definitions at once
    (
        $($handler_name:ident : $module:ident :: $handler_type:ident),* $(,)?
    ) => {
        impl DatabaseClientPooled {
            // Generate all handler methods using the provided definitions
            $(
                pub fn $handler_name(&self) -> $module::$handler_type {
                    $module::$handler_type::new(self.connection.clone())
                }
            )*
        }
    };
}

#[macro_export] // Make macro available throughout the crate (and potentially others if published)
macro_rules! impl_database_handler {
    (
        $handler:ident,         // e.g., AbilityHandler
        $resource:ty,          // e.g., crate::model::Ability
        $table:path,           // e.g., crate::schema::abilities::dsl::abilities
        $id_column:path        // e.g., crate::schema::abilities::dsl::id
    ) => {
        pub struct $handler {
            pub connection: diesel::r2d2::Pool<
                diesel::r2d2::ConnectionManager<$crate::database::handler::DatabaseConnection>,
            >,
        }

        impl $handler {
            pub fn new(
                connection: diesel::r2d2::Pool<
                    diesel::r2d2::ConnectionManager<$crate::database::handler::DatabaseConnection>,
                >,
            ) -> Self {
                $handler { connection }
            }
        }
        // Implement the trait for the specified handler struct
        impl $crate::database::handler::DatabaseHandler for $handler {
            type Resource = $resource;

            fn get_all_resources(
                &self,
                pagination: $crate::database::pagination::Paginated,
            ) -> $crate::error::Result<$crate::types::response::PaginatedResource<Self::Resource>>
            {
                // These imports are needed within the generated function scope
                use diesel::dsl::count_star;
                use diesel::prelude::*;

                let mut conn = self
                    .connection
                    .get()
                    .map_err($crate::error::Error::R2D2PoolError)?;

                // Use count_star() as per the example
                let total_items_query = $table.select(count_star());
                let total_items = total_items_query
                    .first::<i64>(&mut conn)
                    .map_err($crate::error::Error::DieselError)?;

                let total_pages = pagination.pages(total_items);

                let items_query = $table
                    .select(<$resource>::as_select()) // Use the resource type passed to macro
                    .limit(pagination.limit())
                    .offset(pagination.offset());

                let items = items_query
                    .load::<Self::Resource>(&mut conn) // Load the associated Resource type
                    .map_err($crate::error::Error::DieselError)?;

                Ok($crate::types::response::PaginatedResource {
                    data: items,
                    total_pages,
                    total_items,
                    page: pagination.page,
                    per_page: pagination.per_page,
                })
            }

            fn get_resource_by_id(
                &self,
                resource_id: i32,
            ) -> $crate::error::Result<Self::Resource> {
                // These imports are needed within the generated function scope
                use diesel::prelude::*;

                let mut conn = self
                    .connection
                    .get()
                    .map_err($crate::error::Error::R2D2PoolError)?;

                let query = $table
                    .filter($id_column.eq(resource_id)) // Use the ID column path passed to macro
                    .select(<$resource>::as_select()); // Use the resource type

                query
                    .first::<Self::Resource>(&mut conn) // Load the associated Resource type
                    .map_err($crate::error::Error::DieselError) // Turns Err(NotFound) into Ok(None), propagates other errors
            }
        }
    };
}

#[macro_export]
macro_rules! impl_database_locale_handler {
    (
        $handler:ident,         // e.g., ItemHandler
        $resource:ty,          // e.g., crate::model::Item
        $table:path,           // e.g., crate::schema::items::dsl::items
        $id_column:path,       // e.g., crate::schema::items::dsl::id
        $names_table:path,     // e.g., crate::schema::item_names::dsl::item_names
        $names_id_column:path, // e.g., crate::schema::item_names::dsl::item_id
        $names_name_column:path, // e.g., crate::schema::item_names::dsl::name
        $names_language_column:path // e.g., crate::schema::item_names::dsl::local_language_id
    ) => {
        impl $crate::database::handler::DatabaseHandlerWithLocale for $handler {
            type Resource = $resource;

            fn get_all_resources_with_locale(
                &self,
                pagination: $crate::database::pagination::Paginated,
                locale_id: i32,
                query: Option<String>,
            ) -> $crate::error::Result<
                $crate::types::response::PaginatedResource<
                    $crate::model::Languaged<Self::Resource>,
                >,
            > {
                use diesel::dsl::count_star;
                use diesel::prelude::*;

                let mut conn = self.connection.get()?;

                if let Some(query) = query {
                    let total_items_query = $table
                        .inner_join($names_table.on($id_column.eq($names_id_column)))
                        .filter($names_language_column.eq(locale_id))
                        .filter($names_name_column.like(format!("%{}%", query)))
                        .select(count_star());
                    let total_items = total_items_query
                        .first::<i64>(&mut conn)
                        .map_err($crate::error::Error::DieselError)?;

                    let total_pages = pagination.pages(total_items);

                    let items = $table
                        .inner_join($names_table.on($id_column.eq($names_id_column)))
                        .filter($names_language_column.eq(locale_id))
                        .filter($names_name_column.like(format!("%{}%", query)))
                        .select((<$resource>::as_select(), $names_name_column))
                        .limit(pagination.limit())
                        .offset(pagination.offset())
                        .load::<(Self::Resource, String)>(&mut conn)
                        .map_err($crate::error::Error::DieselError)?;

                    Ok($crate::types::response::PaginatedResource {
                        data: items
                            .into_iter()
                            .map(|(resource, name)| $crate::model::Languaged {
                                item: resource,
                                name,
                            })
                            .collect(),
                        total_pages,
                        total_items,
                        page: pagination.page,
                        per_page: pagination.per_page,
                    })
                } else {
                    // Count total items
                    let total_items_query = $table.select(count_star());
                    let total_items = total_items_query
                        .first::<i64>(&mut conn)
                        .map_err($crate::error::Error::DieselError)?;

                    let total_pages = pagination.pages(total_items);

                    // Join with names table to get localized names
                    let items_query = $table
                        .inner_join($names_table.on($id_column.eq($names_id_column)))
                        .filter($names_language_column.eq(locale_id))
                        .select((<$resource>::as_select(), $names_name_column))
                        .limit(pagination.limit())
                        .offset(pagination.offset());

                    let items = items_query
                        .load::<(Self::Resource, String)>(&mut conn)
                        .map_err($crate::error::Error::DieselError)?;

                    Ok($crate::types::response::PaginatedResource {
                        data: items
                            .into_iter()
                            .map(|(resource, name)| $crate::model::Languaged {
                                item: resource,
                                name,
                            })
                            .collect(),
                        total_pages,
                        total_items,
                        page: pagination.page,
                        per_page: pagination.per_page,
                    })
                }
            }

            fn get_resource_by_id_with_locale(
                &self,
                resource_id: i32,
                locale_id: i32,
            ) -> $crate::error::Result<$crate::model::Languaged<Self::Resource>> {
                use diesel::prelude::*;

                let mut conn = self
                    .connection
                    .get()
                    .map_err($crate::error::Error::R2D2PoolError)?;

                let query = $table
                    .inner_join($names_table.on($id_column.eq($names_id_column)))
                    .filter($id_column.eq(resource_id))
                    .filter($names_language_column.eq(locale_id))
                    .select((<$resource>::as_select(), $names_name_column));

                query
                    .first::<(Self::Resource, String)>(&mut conn)
                    .map_err($crate::error::Error::DieselError)
                    .map(|(resource, name)| $crate::model::Languaged {
                        item: resource,
                        name,
                    })
            }
        }
    };
}

#[macro_export]
macro_rules! impl_database_flavor_text_handler {
    (
        $handler:ident,         // e.g., ItemHandler
        $flavor_table:path,     // e.g., crate::schema::item_flavor_text::dsl::item_flavor_text
        $flavor_id_column:path, // e.g., crate::schema::item_flavor_text::dsl::item_id
        $flavor_text_column:path, // e.g., crate::schema::item_flavor_text::dsl::flavor_text
        $flavor_language_column:path, // e.g., crate::schema::item_flavor_text::dsl::language_id
        $flavor_version_group_column:path // e.g., crate::schema::item_flavor_text::dsl::version_group_id
    ) => {
        impl $crate::database::handler::DatabaseHandlerWithFlavorText for $handler {
            fn get_all_resources_with_flavor_text(
                &self,
                resource_id: i32,
                pagination: $crate::database::pagination::Paginated,
                locale_id: i32,
            ) -> $crate::error::Result<
                $crate::types::response::PaginatedResource<$crate::model::ResourceDescription>,
            > {
                use diesel::dsl::count_star;
                use diesel::prelude::*;

                let mut conn = self
                    .connection
                    .get()
                    .map_err($crate::error::Error::R2D2PoolError)?;

                let total_items_query = $flavor_table
                    .filter($flavor_language_column.eq(locale_id))
                    .filter($flavor_id_column.eq(resource_id))
                    .select(count_star());
                let total_items = total_items_query
                    .first::<i64>(&mut conn)
                    .map_err($crate::error::Error::DieselError)?;
                let total_pages = pagination.pages(total_items);

                let resources = $flavor_table
                    .filter($flavor_language_column.eq(locale_id))
                    .filter($flavor_id_column.eq(resource_id))
                    .select((
                        $flavor_text_column,
                        $flavor_version_group_column,
                        $flavor_language_column,
                    ))
                    .limit(pagination.limit())
                    .offset(pagination.offset())
                    .load::<(String, i32, i32)>(&mut conn)
                    .map_err($crate::error::Error::DieselError)?
                    .into_iter()
                    .map(|(flavor_text, version_group, language)| {
                        $crate::model::ResourceDescription {
                            description: flavor_text,
                            version: $crate::model::DescriptionVersion::VersionGroup(version_group),
                            language,
                        }
                    })
                    .collect();

                Ok($crate::types::response::PaginatedResource {
                    data: resources,
                    total_pages,
                    total_items,
                    page: pagination.page,
                    per_page: pagination.per_page,
                })
            }

            fn get_latest_flavor_text(
                &self,
                resource_id: i32,
                locale_id: i32,
            ) -> $crate::error::Result<$crate::model::ResourceDescription> {
                use diesel::prelude::*;
                let mut conn = self
                    .connection
                    .get()
                    .map_err($crate::error::Error::R2D2PoolError)?;
                $flavor_table
                    .filter($flavor_language_column.eq(locale_id))
                    .filter($flavor_id_column.eq(resource_id))
                    .select((
                        $flavor_text_column,
                        $flavor_version_group_column,
                        $flavor_language_column,
                    ))
                    .order($flavor_version_group_column.desc())
                    .first::<(String, i32, i32)>(&mut conn)
                    .map_err($crate::error::Error::DieselError)
                    .map(|(flavor_text, version_group, language)| {
                        $crate::model::ResourceDescription {
                            description: flavor_text,
                            version: $crate::model::DescriptionVersion::VersionGroup(version_group),
                            language,
                        }
                    })
            }
        }
    };
}

#[macro_export]
macro_rules! impl_fetch_related_by_foreign_key {
    (
        // 要实现方法的 Handler 结构体名称
        // 例如: PokemonSpeciesHandler
        handler_struct: $handler_struct:ident,
        // 生成的新公开方法的名称
        // 例如: get_pokemon_forms
        method_name: $method_name:ident,
        // "父"实体 ID 的参数名称和类型
        // 例如: species_id: i32
        parent_id_arg: $parent_id_arg_name:ident: $parent_id_arg_type:ty,
        // 要获取的关联实体的 Diesel 模型/类型
        // 例如: crate::model::Pokemon
        related_resource_model: $related_resource_model:ty,
        // 关联实体表 DSL 的路径
        // 例如: crate::schema::pokemon::dsl::pokemon
        related_table_dsl: $related_table_dsl:path,
        // related_table_dsl 中引用父实体 ID 的外键列 DSL 路径
        // 例如: crate::schema::pokemon::dsl::species_id
        foreign_key_on_related_table: $foreign_key_on_related_table:path
    ) => {
        impl $handler_struct {
            /// 根据外键获取关联资源列表 (一对多关系).
            /// 例如，根据给定的 PokemonSpecies ID 获取所有 Pokemon (形态).
            pub fn $method_name(
                &self,
                $parent_id_arg_name: $parent_id_arg_type,
            ) -> Result<Vec<$related_resource_model>, diesel::result::Error> {
                use diesel::prelude::*;

                // 假设 self.connection 可用并且是一个 Pool, 类似于其他的 handlers
                let mut conn = self.connection.get()?;

                // $related_table_dsl 应该是表标识符本身
                // (例如: crate::schema::pokemon::dsl::pokemon)
                // $foreign_key_on_related_table 应该是列标识符
                // (例如: crate::schema::pokemon::dsl::species_id).
                $related_table_dsl
                    .filter($foreign_key_on_related_table.eq($parent_id_arg_name))
                    .select(<$related_resource_model>::as_select())
                    .load::<$related_resource_model>(&mut conn)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_fetch_related_through_join_table {
    (
        // 要实现方法的 Handler 结构体名称
        // 例如: PokemonHandler
        handler_struct: $handler_struct:ident,
        // 生成的新公开方法的名称
        // 例如: get_moves_for_pokemon
        method_name: $method_name:ident,
        // "主"实体 ID 的参数名称和类型
        // 例如: pokemon_id: i32
        primary_id_arg: $primary_id_arg_name:ident: $primary_id_arg_type:ty,
        // 目标关联实体的 Diesel 模型/类型
        // 例如: crate::model::Move
        related_resource_model: $related_resource_model:ty,

        // --- 连接表 (Join Table) 详情 ---
        // 连接表 DSL 的路径
        // 例如: crate::schema::pokemon_moves::dsl::pokemon_moves
        join_table_dsl: $join_table_dsl:path,
        // 连接表中引用主实体的外键列 DSL 路径
        // 例如: crate::schema::pokemon_moves::dsl::pokemon_id
        join_table_fk_to_primary: $join_table_fk_to_primary:path,
        // 连接表中引用关联实体的外键列 DSL 路径
        // 例如: crate::schema::pokemon_moves::dsl::move_id
        join_table_fk_to_related: $join_table_fk_to_related:path,

        // --- 目标关联表 (Related/Target Table) 详情 ---
        // 目标关联表 DSL 的路径
        // 例如: crate::schema::moves::dsl::moves
        related_table_dsl: $related_table_dsl:path,
        // 目标关联表的主键 (或用于连接的唯一键) DSL 路径
        // 例如: crate::schema::moves::dsl::id
        related_table_pk: $related_table_pk:path
    ) => {
        impl $handler_struct {
            /// 通过连接表获取关联资源列表 (多对多关系).
            /// 例如，根据给定的 Pokemon ID 获取其所有 Moves (招式).
            pub fn $method_name(
                &self,
                $primary_id_arg_name: $primary_id_arg_type,
            ) -> Result<Vec<$related_resource_model>, diesel::result::Error> {
                use diesel::prelude::*;

                let mut conn = self.connection.get()?;

                // $join_table_dsl, $related_table_dsl 是表标识符
                // $join_table_fk_to_primary, $join_table_fk_to_related, $related_table_pk 是列标识符.

                // 查询示例:
                // pokemon_moves.inner_join(moves.on(pokemon_moves::move_id.eq(moves::id)))
                //              .filter(pokemon_moves::pokemon_id.eq(given_pokemon_id))
                //              .select(Move::as_select())
                //              .load::<Move>(&mut conn)

                $join_table_dsl
                    .inner_join(
                        $related_table_dsl.on($join_table_fk_to_related.eq($related_table_pk)),
                    )
                    .filter($join_table_fk_to_primary.eq($primary_id_arg_name))
                    .select(<$related_resource_model>::as_select())
                    .load::<$related_resource_model>(&mut conn)
            }
        }
    };
}
