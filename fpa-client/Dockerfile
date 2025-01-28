FROM node:22.13.1 AS build
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm install
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=build /app/dist/fpa-client/browser /usr/share/nginx/html

EXPOSE 80
