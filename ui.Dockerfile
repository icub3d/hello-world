# syntax=docker/dockerfile:1.3

FROM node:17 AS builder

WORKDIR /usr/src/hello-world/

COPY ui/package.json ui/yarn.lock .
RUN --mount=type=cache,target=/usr/src/hello-world/node_modules \
	yarn install
COPY ui/ .

RUN --mount=type=cache,target=/usr/src/hello-world/node_modules \
	yarn docker && ls build

FROM nginx

COPY --from=builder /usr/src/hello-world/build /usr/share/nginx/html
COPY ui/default.conf.template /etc/nginx/templates/
