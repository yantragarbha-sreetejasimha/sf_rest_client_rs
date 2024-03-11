if [ ! -f "Cargo.toml" ]; then
    docker run --rm -v ${PWD}:/local swaggerapi/swagger-codegen-cli generate \
        -i https://raw.githubusercontent.com/Azure/azure-rest-api-specs/main/specification/servicefabric/data-plane/Microsoft.ServiceFabric/stable/7.2/servicefabric.json \
        -l rust \
        -o /local
fi


sudo chown ${USER} -R ./ && sudo chmod u+rwx -R ./

# Patch: Cargo.toml
cat <<EOF > Cargo.toml
[package]
name = "sf_rest_client_rs"
version = "1.0.0"
authors = ["sgemaraju@microsoft.com"]

[dependencies]
serde = "1.0"
serde_derive = "1.0"
serde_yaml = "0.7"
serde_json = "1.0"
base64 = "~0.7.0"
futures = "0.1.16"
hyper = "0.11.6"
url = "1.5"

[dev-dependencies]
tokio-core = "*"
EOF

cat <<EOF > rustfmt.toml
max_width = 80
EOF

function fix_file() {
    echo "Working on $1"
    sed -i 's/async/_async/g' $1
    sed -i 's/_+async/_async/g' $1
    sed -i 's/set__*/set_/g' $1
    sed -i 's/get__*/get_/g' $1
    sed -i 's/with__*/with_/g' $1
    sed -i 's/match__*/match_/g' $1
    sed -i 's/reset__*/reset_/g' $1
}

# Fix code
CWD=$(pwd)
folders=("src/models/" "src/apis/")
for folder in "${folders[@]}"; do
    cd $CWD/$folder
    echo "Working on fixing files in $(pwd)"
    for file in $(ls); do
        fix_file $file &
    done
done

wait

cd $CWD

cargo clippy --fix --allow-dirty --allow-no-vcs
cargo fmt
cargo b && cargo b --release