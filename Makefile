watch:
	cargo watch -q -c -w src/ -w .cargo/ -x run

watch-test:
	cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

run:
	export DATABASE_URL="postgres://fl0user:EWr86qDkyGLj@ep-floral-forest-04275288.eu-central-1.aws.neon.fl0.io:5432/database?sslmode=require" 
	export SERVICE_PWD_KEY="EWzNyWp4L6Xq4YomnET-2PeHLTJwBfxTqxiSNbP9pAlZdTl_NIahKO8U4wPuPEuTMuj5VzFBHL_FAnQfqj5nvg" 
	export SERVICE_TOKEN_KEY="hmS6JpebyH1srAZwK0WmGkQdQHducUZnhUAgaJnqp8U_xv1htL0x8X04hcTS_N1kwfVQ00ZToJNVbzTLoUa11Q" 
	export SERVICE_TOKEN_DURATION_SEC="1800" 
	cargo run
