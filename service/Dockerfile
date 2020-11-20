FROM node:15.2.1-alpine3.12 AS builder

WORKDIR /universe

# First get all the dependencies
COPY package.json package-lock.json /universe/
RUN npm install

# Then build the code
COPY src/ /universe/src/
COPY tsconfig.json /universe/
RUN npm run build

FROM node:15.2.1-alpine3.12 AS service

WORKDIR /universe

# First get systemd dependencies
RUN apk add --no-cache openssl curl

ENV DOCKERIZE_VERSION v0.6.1
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && tar -C /usr/local/bin -xzvf dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && rm dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz

# Then get all the software dependencies
COPY package.json package-lock.json /universe/
RUN npm install --production

# Then copy the built code
COPY --from=builder /universe/target /universe/target

# Required environment properties
ENV PORT=8000
ENV DATABASE_URL=
ENV DOCKERIZE_ARGS=

# Set up healthchecks
HEALTHCHECK --interval=30s --start-period=5s CMD curl -f http://localhost:$PORT/health || exit 1

# Set up ability to run
CMD dockerize $DOCKERIZE_ARGS node target/build/index.js