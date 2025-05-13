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
            ) -> $crate::database::pagination::PaginatedResource<Self::Resource> {
                // These imports are needed within the generated function scope
                use diesel::dsl::count_star;
                use diesel::prelude::*;

                let resource_name = stringify!($resource); // For error messages
                let mut conn = self
                    .connection
                    .get()
                    .expect("Failed to get DB connection from pool");

                // Use count_star() as per the example
                let total_items_query = $table.select(count_star());
                let total_items = total_items_query
                    .first::<i64>(&mut conn)
                    .expect(&format!("Error counting {}", resource_name));

                let total_pages = pagination.pages(total_items);

                let items_query = $table
                    .select(<$resource>::as_select()) // Use the resource type passed to macro
                    .limit(pagination.limit())
                    .offset(pagination.offset());

                let items = items_query
                    .load::<Self::Resource>(&mut conn) // Load the associated Resource type
                    .expect(&format!("Error loading {}", resource_name));

                $crate::database::pagination::PaginatedResource {
                    data: items,
                    total_pages,
                    total_items,
                    page: pagination.page,
                    per_page: pagination.per_page,
                }
            }

            fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
                // These imports are needed within the generated function scope
                use diesel::prelude::*;

                let resource_name = stringify!($resource); // For error messages
                let mut conn = self
                    .connection
                    .get()
                    .expect("Failed to get DB connection from pool");

                let query = $table
                    .filter($id_column.eq(resource_id)) // Use the ID column path passed to macro
                    .select(<$resource>::as_select()); // Use the resource type

                query
                    .first::<Self::Resource>(&mut conn) // Load the associated Resource type
                    .optional() // Turns Err(NotFound) into Ok(None), propagates other errors
                    .expect(&format!(
                        "Database error retrieving {} by ID",
                        resource_name
                    )) // Panics only on actual DB errors
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
            ) -> $crate::database::pagination::PaginatedResource<(Self::Resource, String)> {
                use diesel::dsl::count_star;
                use diesel::prelude::*;

                let resource_name = stringify!($resource);
                let mut conn = self
                    .connection
                    .get()
                    .expect("Failed to get DB connection from pool");

                if let Some(query) = query {
                    let total_items_query = $table
                        .inner_join($names_table.on($id_column.eq($names_id_column)))
                        .filter($names_language_column.eq(locale_id))
                        .filter($names_name_column.like(format!("%{}%", query)))
                        .select(count_star());
                    let total_items = total_items_query
                        .first::<i64>(&mut conn)
                        .expect(&format!("Error counting {}", resource_name));

                    let total_pages = pagination.pages(total_items);

                    let items = $table
                        .inner_join($names_table.on($id_column.eq($names_id_column)))
                        .filter($names_language_column.eq(locale_id))
                        .filter($names_name_column.like(format!("%{}%", query)))
                        .select((<$resource>::as_select(), $names_name_column))
                        .limit(pagination.limit())
                        .offset(pagination.offset())
                        .load::<(Self::Resource, String)>(&mut conn)
                        .expect(&format!("Error loading {} with locale", resource_name));

                    $crate::database::pagination::PaginatedResource {
                        data: items,
                        total_pages,
                        total_items,
                        page: pagination.page,
                        per_page: pagination.per_page,
                    }
                } else {
                    // Count total items
                    let total_items_query = $table.select(count_star());
                    let total_items = total_items_query
                        .first::<i64>(&mut conn)
                        .expect(&format!("Error counting {}", resource_name));

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
                        .expect(&format!("Error loading {} with locale", resource_name));

                    $crate::database::pagination::PaginatedResource {
                        data: items,
                        total_pages,
                        total_items,
                        page: pagination.page,
                        per_page: pagination.per_page,
                    }
                }
            }

            fn get_resource_by_id_with_locale(
                &self,
                resource_id: i32,
                locale_id: i32,
            ) -> Option<(Self::Resource, String)> {
                use diesel::prelude::*;

                let resource_name = stringify!($resource);
                let mut conn = self
                    .connection
                    .get()
                    .expect("Failed to get DB connection from pool");

                let query = $table
                    .inner_join($names_table.on($id_column.eq($names_id_column)))
                    .filter($id_column.eq(resource_id))
                    .filter($names_language_column.eq(locale_id))
                    .select((<$resource>::as_select(), $names_name_column));

                query
                    .first::<(Self::Resource, String)>(&mut conn)
                    .optional()
                    .expect(&format!(
                        "Database error retrieving {} by ID with locale",
                        resource_name
                    ))
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
            ) -> $crate::database::pagination::PaginatedResource<$crate::model::ResourceDescription>
            {
                use diesel::dsl::count_star;
                use diesel::prelude::*;

                let resource_name = stringify!($resource);
                let mut conn = self
                    .connection
                    .get()
                    .expect("Failed to get DB connection from pool");

                let total_items_query = $flavor_table
                    .filter($flavor_language_column.eq(locale_id))
                    .filter($flavor_id_column.eq(resource_id))
                    .select(count_star());
                let total_items = total_items_query
                    .first::<i64>(&mut conn)
                    .expect(&format!("Error counting {}", resource_name));
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
                    .expect(&format!("Error loading {} with flavor text", resource_name))
                    .into_iter()
                    .map(|(flavor_text, version_group, language)| {
                        $crate::model::ResourceDescription {
                            description: flavor_text,
                            version: $crate::model::DescriptionVersion::VersionGroup(version_group),
                            language,
                        }
                    })
                    .collect();

                $crate::database::pagination::PaginatedResource {
                    data: resources,
                    total_pages,
                    total_items,
                    page: pagination.page,
                    per_page: pagination.per_page,
                }
            }

            fn get_latest_flavor_text(
                &self,
                resource_id: i32,
                locale_id: i32,
            ) -> Option<$crate::model::ResourceDescription> {
                use diesel::prelude::*;
                let mut conn = self
                    .connection
                    .get()
                    .expect("Failed to get DB connection from pool");
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
                    .ok()
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
