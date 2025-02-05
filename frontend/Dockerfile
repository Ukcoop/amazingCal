FROM node:lts-alpine AS build

WORKDIR /app

COPY . .

RUN npm install 

ENV NODE_ENV=production

RUN npm run build

FROM node:lts-alpine AS runner

WORKDIR /app

COPY --from=build /app/package.json /app/package-lock.json ./
COPY --from=build /app/.next ./.next
COPY --from=build /app/public ./public
COPY --from=build /app/node_modules ./node_modules

EXPOSE 3000

CMD ["npm", "start"]
