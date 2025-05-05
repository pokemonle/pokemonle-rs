use crate::database::schema::{item_categories, item_pockets, items};
use crate::impl_database_handler;
use crate::model::{Item, ItemCategory, ItemPocket};

impl_database_handler!(ItemHandler, Item, items::dsl::items, items::dsl::id);

impl_database_handler!(
    ItemCategoryHandler,
    ItemCategory,
    item_categories::dsl::item_categories,
    item_categories::dsl::id
);

impl_database_handler!(
    ItemPocketHandler,
    ItemPocket,
    item_pockets::dsl::item_pockets,
    item_pockets::dsl::id
);
