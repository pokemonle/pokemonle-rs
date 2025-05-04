use diesel::dsl::count;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use super::{DatabaseConnection, DatabaseHandler};

use crate::database::pagination::{Paginated, PaginatedResource};
use crate::database::schema::items::dsl::*;
use crate::model::{Item, ItemCategory, ItemPocket};

pub struct ItemHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl ItemHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        ItemHandler { connection }
    }

    pub fn category_handler(&self) -> ItemCategoryHandler {
        ItemCategoryHandler::new(self.connection.clone())
    }

    pub fn pocket_handler(&self) -> ItemPocketHandler {
        ItemPocketHandler::new(self.connection.clone())
    }
}

impl DatabaseHandler for ItemHandler {
    type Resource = Item;

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        let mut conn = self.connection.get().unwrap();

        let total_items = items.select(count(id)).first::<i64>(&mut conn).unwrap();
        let total_pages = pagination.pages(total_items);

        let _items = items
            .select(Item::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading items");

        PaginatedResource {
            data: _items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        items
            .filter(id.eq(resource_id))
            .select(Item::as_select())
            .first::<Item>(&mut self.connection.get().unwrap())
            .ok()
    }
}

pub struct ItemCategoryHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl ItemCategoryHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        ItemCategoryHandler { connection }
    }
}

impl DatabaseHandler for ItemCategoryHandler {
    type Resource = ItemCategory;

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        use crate::database::schema::item_categories::dsl::*;
        let mut conn = self.connection.get().unwrap();

        let total_items = item_categories
            .select(count(id))
            .first::<i64>(&mut conn)
            .unwrap();
        let total_pages = pagination.pages(total_items);

        let _items = item_categories
            .select(ItemCategory::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading item categories");

        PaginatedResource {
            data: _items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        use crate::database::schema::item_categories::dsl::*;
        item_categories
            .filter(id.eq(resource_id))
            .select(ItemCategory::as_select())
            .first::<ItemCategory>(&mut self.connection.get().unwrap())
            .ok()
    }
}

// ItemPocket
pub struct ItemPocketHandler {
    pub connection: Pool<ConnectionManager<DatabaseConnection>>,
}

impl ItemPocketHandler {
    pub fn new(connection: Pool<ConnectionManager<DatabaseConnection>>) -> Self {
        ItemPocketHandler { connection }
    }
}

impl DatabaseHandler for ItemPocketHandler {
    type Resource = ItemPocket;

    fn get_all_resources(&self, pagination: Paginated) -> PaginatedResource<Self::Resource> {
        use crate::database::schema::item_pockets::dsl::*;
        let mut conn = self.connection.get().unwrap();

        let total_items = item_pockets
            .select(count(id))
            .first::<i64>(&mut conn)
            .unwrap();
        let total_pages = pagination.pages(total_items);

        let _items = item_pockets
            .select(ItemPocket::as_select())
            .limit(pagination.limit())
            .offset(pagination.offset())
            .load(&mut conn)
            .expect("Error loading item pockets");

        PaginatedResource {
            data: _items,
            total_pages,
            total_items,
            page: pagination.page,
            per_page: pagination.per_page,
        }
    }

    fn get_resource_by_id(&self, resource_id: i32) -> Option<Self::Resource> {
        use crate::database::schema::item_pockets::dsl::*;
        item_pockets
            .filter(id.eq(resource_id))
            .select(ItemPocket::as_select())
            .first::<ItemPocket>(&mut self.connection.get().unwrap())
            .ok()
    }
}
