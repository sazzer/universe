# Build the app
FROM node:15.5.0-alpine3.12 AS builder

WORKDIR /universe

COPY package.json package-lock.json tsconfig.json /universe/

RUN npm install

COPY public /universe/public/
COPY src /universe/src/

RUN npm run build

# Create deployable image
FROM beamaustralia/react-env:latest

WORKDIR /var/www

COPY --from=builder /universe/build /var/www

ENV REACT_APP_URL_BASE=NOT_SPECIFIED
