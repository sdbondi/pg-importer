Postgres importer 

Usage:

```bash
./pg-importer import -f pg_dump_file.sql -o output_dump_file.sql
pg-importer -f=/Users/stanleybondi/Downloads/onionshare_qxca2g/yatproduction-complete-pre.sql -o /tmp/deleteme.sql --exclude-schema=geolite --exclude-extension=ip4r --exclude-tabledata=visits

psql < output_dump_file.sql

```
