git clone github.com/ssravens/xivpflistbot
cd xivpflistbot
cargo install sqlx-cli

$env:DISCORD_TOKEN="<MTAxMDMzMjI4NDkwNzQ0MjE5Ng.GS3Z6e.XrEKhEQYF4E6clcUjLAKmVraidWFuzLKvM4hrk>"
$env:DATABASE_URL="sqlite:database.sqlite"
$env:MAX_LISTINGS_IN_POST=8

sqlx database create
sqlx migrate run

cargo install
cargo build --release
./target/release/ffxiv_pf_bot.exe 