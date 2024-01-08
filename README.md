# Fake Log Generator

> [!NOTE]
>
> It's a project for me to practice Rust.

This is a fake json log generator for testing purposes.
It generates fake logs in the following format:

```json
{
  "id": "1",
  "company": "example inc.",
  "name": "John Doe",
  "age": 24,
  "address": "Amsterdam, Netherlands",
  "phone": "+31 6 12345678",
  "email": "example@email.com",
  "introduction": "Hello, my name is John Doe and I'm a software engineer.",
  "timestamp": "2020-01-01T00:00:00.000Z"
}
```

> [!NOTE]
>
> Currently, it can only be used to send fixed-format logs.

## Usage

You can use the `--file` option to specify the connection settings file to read.

```bash
fake-log-generator --file ./connection_settings.toml
```

```toml
# connection_settings.toml example
[host]
protocol = "tcp"
ip = "127.0.0.1"
ports = [12201, 12202, 12203, 12204]
total_requests = 10000
```

> [!WARNING]
>
> The connection settings file must be in the TOML format.
> You can find the example settings file `connection_settings.toml` in the repo.