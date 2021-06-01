Postgres importer 

Usage:

```bash
./pg-importer import -f pg_dump_file.sql -o output_dump_file.sql
pg-importer -f=/Path/To/Export.sql -o /tmp/deleteme.sql --exclude-schema=geolite --exclude-extension=ip4r --exclude-tabledata=visits

psql < output_dump_file.sql

```


NOTE: `--exclude-tabledata` was added because the parser has a bug that I have not had time to dig into ;)
