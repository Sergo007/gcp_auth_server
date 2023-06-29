# gcp_auth_server

gcp_auth_server for postman requests

## How to use

gcloud should be installed

upload exec file for your operation system from this_repo/bin/{macos,windows}/

exec `gcp_auth_server`

add js code to prescript in postmen

```javascript
pm.sendRequest(
  "http://localhost:9090/gcloud/print_identity_token",
  function (err, response) {
    if (!err) {
      let token = response.json().token;
      pm.variables.set("access_token", token);
      pm.request.headers.add("Authorization: Bearer " + token);
    }
  }
);
```

# build for windows on mac

`brew install mingw-w64`
`rustup target add x86_64-pc-windows-gnu`
`cargo build --target x86_64-pc-windows-gnu --release`

# build

`cargo build --release`

## Debug OPTIONS

```bash
curl -i -X OPTIONS -H "Origin: http://127.0.0.1:9090" \
    -H 'Access-Control-Request-Method: POST' \
    -H 'Access-Control-Request-Headers: Content-Type' \
    "http://localhost:8080"
```

## Next implementation

1.  investigate [Concorde TSP Solver](https://www.math.uwaterloo.ca/tsp/concorde/gui/gui.htm)
2.  impement algorithms used concorde for TSP on rust
