SINGLEPAGE=babiano

SINGLEPAGE=$SINGLEPAGE perseus --wasm-opt-version version_118 export --release
cp -r ./static/* "./dist/exported/$SINGLEPAGE"
mv ./dist/exported/.perseus/bundle.wasm "./dist/exported/$SINGLEPAGE"
mv ./dist/exported/.perseus/bundle.js "./dist/exported/$SINGLEPAGE"
sed -i "s/.perseus\/bundle/$SINGLEPAGE\/bundle/g" "./dist/exported/$SINGLEPAGE/index.html"
