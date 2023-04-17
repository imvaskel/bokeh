FROM node:19-bullseye as deps

WORKDIR /app
COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

FROM node:19-bullseye as builder

ENV NODE_ENV=production
WORKDIR /app
COPY . .
COPY --from=deps /app/node_modules ./node_modules
RUN yarn build

FROM node:19-bullseye as runner
WORKDIR /appp
COPY --from=builder /app/build ./build
COPY --from=builder /app/node_modules ./node_modules
COPY package.json ./
CMD [ "node", "build" ]

