projectname=hackers

build:
	dx build --release --platform web

upload: build
	rclone -P --config ./rclone.conf copy dist/. r2:${projectname}

fetch-config:
	echo "[r2]" > rclone.conf
	vault kv get -mount=kv -field=r2accesstoken "cloudflare/alexis.lowe@protonmail.com" >> rclone.conf