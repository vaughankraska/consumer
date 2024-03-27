# Playing with rust pulsar client consumer
## To run
* start pulsar in docker
```bash
docker run -it \
-p 6650:6650 \
-p 8080:8080 \
--mount source=pulsardata,target=/pulsar/data \
--mount source=pulsarconf,target=/pulsar/conf \
apachepulsar/pulsar:3.2.1 \
bin/pulsar standalone
```
* start consumer
```bash
RUST_LOG=[info or debug or error] cargo run --release
```
* run producer to see results
Expected format:
```text
'{"data": "hello pulsar or whatever I want"}'
```
