use super::AppState;
use crate::error::Error;
use crate::v1::Resource;
use aide::{
    axum::{routing::get_with, ApiRouter, IntoApiResponse},
    transform::TransformOperation,
    OperationOutput,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use pokemonle_lib::{
    database::{
        handler::DatabaseHandlerWithLocale,
        pagination::{Paginated, PaginatedResource},
    },
    model::{LanguageName, Languaged},
};
use pokemonle_trait::StructName;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
struct Language {
    lang: i32,
}

#[derive(Deserialize, JsonSchema)]
struct SearchQuery {
    q: Option<String>,
}

pub fn api_routers<T, H, F>(handler_fn: F) -> ApiRouter<AppState>
where
    T: StructName + OperationOutput + Serialize + JsonSchema + Clone + Send + Sync + 'static,
    <T as OperationOutput>::Inner: Serialize + From<T>,
    H: DatabaseHandlerWithLocale<Resource = T> + Sync + 'static,
    F: Fn(AppState) -> H + Clone + Copy + Send + Sync + 'static,
{
    api_routers_with_transform(handler_fn, |o| o)
}

pub fn api_routers_with_transform<T, H, F, O>(handler_fn: F, transform: O) -> ApiRouter<AppState>
where
    T: StructName + OperationOutput + Serialize + JsonSchema + Clone + Send + Sync + 'static,
    <T as OperationOutput>::Inner: Serialize + From<T>,
    H: DatabaseHandlerWithLocale<Resource = T> + Sync + 'static,
    F: Fn(AppState) -> H + Clone + Copy + Send + Sync + 'static,
    O: FnOnce(TransformOperation) -> TransformOperation + Clone + Copy,
{
    use super::openapi::{get_item_by_id_docs, list_items_docs};

    async fn list<T, H>(
        State(state): State<AppState>,
        Path(Language { lang }): Path<Language>,
        Query(pagination): Query<Paginated>,
        Query(search): Query<SearchQuery>,
        handle_fn: impl Fn(AppState) -> H,
    ) -> impl IntoApiResponse
    where
        T: Serialize + Clone + Send + Sync + StructName,
        H: DatabaseHandlerWithLocale<Resource = T>,
    {
        let handler = handle_fn(state);

        let PaginatedResource {
            data,
            page,
            per_page,
            total_items,
            total_pages,
        } = handler.get_all_resources_with_locale(pagination, lang, search.q);
        (
            StatusCode::OK,
            Json(PaginatedResource {
                data: data
                    .into_iter()
                    .map(|(resource, name)| Languaged {
                        item: resource,
                        name,
                    })
                    .collect(),
                page,
                per_page,
                total_items,
                total_pages,
            }),
        )
    }

    async fn get<T, H>(
        State(state): State<AppState>,
        Path(Language { lang }): Path<Language>,
        Path(resource): Path<Resource>,
        handler_fn: impl Fn(AppState) -> H,
    ) -> impl IntoApiResponse
    where
        T: Serialize + Clone + Send + Sync + StructName,
        H: DatabaseHandlerWithLocale<Resource = T>,
    {
        let handler = handler_fn(state);
        let struct_name = T::struct_name();

        match handler.get_resource_by_id_with_locale(resource.id, lang) {
            Some(resource) => (StatusCode::OK, Json(resource)).into_response(),
            None => {
                let err = Error::ResourceNotFound(format!(
                    "{} with id {} not found",
                    struct_name, resource.id
                ));
                err.into_response()
            }
        }
    }

    ApiRouter::new()
        .api_route(
            "/",
            get_with(
                move |(state, lang, pagination, search)| {
                    list::<T, H>(state, lang, pagination, search, handler_fn)
                },
                move |op| transform(list_items_docs::<T>(op)),
            ),
        )
        .api_route(
            "/{id}",
            get_with(
                move |(state, lang, id)| get::<T, H>(state, lang, id, handler_fn),
                // get_item_by_id_docs with transform
                move |op| transform(get_item_by_id_docs::<T>(op)),
            ),
        )
}

pub fn routers() -> ApiRouter<AppState> {
    use pokemonle_lib::model::{
        Ability, Generation, Item, ItemPocket, Language, Move, PokemonSpecies, Type, Version,
    };

    let sub_routers = ApiRouter::new()
        .nest(
            "/abilities",
            api_routers::<Ability, _, _>(|state| state.pool.ability()),
        )
        .nest(
            "/generations",
            api_routers::<Generation, _, _>(|state| state.pool.generation()),
        )
        .nest(
            "/items",
            api_routers::<Item, _, _>(|state| state.pool.item()),
        )
        .nest(
            "/item-pocket",
            api_routers::<ItemPocket, _, _>(|state| state.pool.item_pocket()),
        )
        .nest(
            "/languages",
            api_routers::<Language, _, _>(|state| state.pool.language()),
        )
        .nest(
            "/moves",
            api_routers::<Move, _, _>(|state| state.pool.r#move()),
        )
        .nest(
            "/pokemon_species",
            api_routers::<PokemonSpecies, _, _>(|state| state.pool.pokemon_specie()),
        )
        .nest(
            "/types",
            api_routers::<Type, _, _>(|state| state.pool.r#type()),
        )
        .nest(
            "/versions",
            api_routers::<Version, _, _>(|state| state.pool.version()),
        );

    ApiRouter::new().nest("/{lang}", sub_routers).api_route(
        "/local-languages",
        get_with(get_local_lanuages, |op| {
            op.tag("language")
                .response_with::<200, Json<Vec<LanguageName>>, _>(|o| {
                    o.description("Get all local languages")
                })
        }),
    )
}

async fn get_local_lanuages(State(state): State<AppState>) -> impl IntoApiResponse {
    let languages = state.pool.language().get_local_lanuages();
    (StatusCode::OK, Json(languages)).into_response()
}
