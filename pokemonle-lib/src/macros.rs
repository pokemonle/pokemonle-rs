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
