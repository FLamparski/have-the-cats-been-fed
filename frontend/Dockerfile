FROM node:20 as builder
WORKDIR /app
COPY . .
RUN npm ci
RUN npm run build

FROM node:20-alpine as runtime
LABEL description="Frontend for the app that lets everyone know when the cats have been fed" \
      org.opencontainers.image.authors="filip@filipwieland.com"
COPY --from=builder /app/.output /app
ENTRYPOINT [ "node", "/app/server/index.mjs" ]
EXPOSE 3000
