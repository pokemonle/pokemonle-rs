.PHONY: generate-entities
generate-entities:
	sea-orm-cli generate entity \
		-o pokemonle-lib/src/database/entity \
		--with-serde both \
		--model-extra-derives schemars::JsonSchema \
		--model-extra-derives aide::OperationIo


.PHONY: generate
generate: generate-entities