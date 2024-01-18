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

```text
This is a simple TCP client that sends fake data to a TCP server.

Usage:
--host, -h <ipv4 address>. Default is 127.0.0.1
--ports, -p <port numbers>. Default is 80
--number, -n <number of requests>. Default is 10,000
```
