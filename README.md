# Fake Log Generator

> [!NOTE]
>
> This a simple project to practice Rust.

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

The program will read the `connection_settings.toml` file to get the connection settings.

```toml
[host]
protocol = "tcp"
ip = "127.0.0.1"
ports = [12201, 12202, 12203, 12204]
total_requests = 10000
```