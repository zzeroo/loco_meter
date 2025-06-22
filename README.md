# Zählerstände

Build with love and [Loco](https://loco.rs). It's running on Rust.


## Quick Start

```sh
cargo loco start
```


## Development

```sh
➜  src loco new
✔ ❯ App name? · loco_meter
✔ ❯ What would you like to build? · Saas App with server side rendering
✔ ❯ Select a DB Provider · Sqlite
✔ ❯ Select your background worker type · Async (in-process tokio async tasks)
```


```sh
cargo loco generate scaffold -k htmx meter_types meter_type:string!
cargo loco generate scaffold -k htmx units unit:string!
cargo loco generate scaffold -k htmx meters meter_type:references number:int! installation_date:date! removal_date:date!
cargo loco generate scaffold -k htmx meter_entries meter:references unit:references entry:float!
```
