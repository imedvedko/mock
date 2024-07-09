# Mock
[![Build Status](https://app.travis-ci.com/imedvedko/mock.svg?branch=master)](https://app.travis-ci.com/imedvedko/mock)

It is a server-side stub service for easy testing third-party integrations in complex software systems

## Runbook

### Build docker image

```shell
docker build -t mock .
```

### Run docker container

```bash
docker run --rm -t -p 8888:8888 mock
```

### Create you first mock

```bash
curl -X 'POST' \
  'http://localhost:8888/mocks/first/data' \
  -H 'Authorization: Bearer token' \
  -d 'It'\''s my first mock'
```

### Call your first mock

```bash
curl http://localhost:8888/mocks/first/call
```

### Open mock service in your favorite browser and explore Swagger UI

```
http://localhost:8888/swagger-ui/index.html
```
