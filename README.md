# Bokeh

Bokeh is an image/media server written in rust. It's just a little pet project of mine.

## Installation

Run ``cargo build --release`` and use the compiled binary. To compile diesel you will need the postgres lib headers.
By default, diesel will run any applicable migrations on each run, so you do not need anything else.

## Usage

### Docker

The only officially supported method of running via docker is via `docker compose`.
Echo the following into `.env` in the source directory:
```env
POSTGRES_PASSWORD=<pw>
POSTGRES_USER=<user>
POSTGRES_DB=<user>
```

Then run ``docker compose up``. Note that by default the binds caddy (the webserver that handles proxying to containers) to
port ``9863``.


### Manually

#### Frontend

Change to the frontend directory and run ``yarn install``, then ``yarn build``, then to run the webserver run ``node build``
in the frontend/ directory.

#### Backend

Run the compiled binary. You can optionally provide a path to your config if you'd like, like ``bokeh <path/to/config>``

## Configuration

Create a file named ``config.toml`` (default name) and fill it out with the content:
```toml
database_url="" # the postgres connection url
invite_key="" # the key to check against when registering users
bind_addr="" # the address to bind to, i.e 127.0.0.1:3545
base_url="" # the url to use for the og:x properties
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)