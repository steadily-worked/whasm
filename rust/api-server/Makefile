export path=$(PWD)
EXPORT = export RUSTPATH=$(PWD)

migration:
	$(EXPORT) && sqlx migrate add -r ${title}
