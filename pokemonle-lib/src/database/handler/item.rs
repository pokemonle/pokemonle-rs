use crate::database::schema::{
    item_categories, item_names, item_pocket_names, item_pockets, items,
};
use crate::model::{Item, ItemCategory, ItemPocket};
use crate::{impl_database_handler, impl_database_locale_handler};

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

impl_database_locale_handler!(
    ItemHandler,
    Item,
    items::dsl::items,
    items::dsl::id,
    item_names::dsl::item_names,
    item_names::dsl::item_id,
    item_names::dsl::name,
    item_names::dsl::local_language_id
);

impl_database_locale_handler!(
    ItemPocketHandler,
    ItemPocket,
    item_pockets::dsl::item_pockets,
    item_pockets::dsl::id,
    item_pocket_names::dsl::item_pocket_names,
    item_pocket_names::dsl::item_pocket_id,
    item_pocket_names::dsl::name,
    item_pocket_names::dsl::local_language_id
);
