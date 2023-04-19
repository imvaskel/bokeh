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

CMD [ "yarn", "start" ]

