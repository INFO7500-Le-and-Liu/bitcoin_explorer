# # use node 16.7 local
# FROM node:16.7

# # install angular cli
# RUN npm install -g @angular/cli@12.2.0

# # set work dir
# WORKDIR /app

# COPY package*.json ./

# RUN npm install

# COPY . /app

# # EXPOSE 4200 port
# EXPOSE 4200

# # launch the countainer
# # use: ng serve
# CMD ["ng", "serve", "--host", "0.0.0.0"]

# Heroku
# Stage 1: Build
FROM node:16.7-alpine as builder
WORKDIR /app
COPY package*.json ./
RUN npm install && npm cache clean --force
COPY . .
RUN npm run build

# Stage 2: Production Environment
FROM node:16.7-alpine
WORKDIR /app
RUN npm install -g http-server
COPY --from=builder /app/dist/WebstormProjects /app
CMD ["http-server", "-p", "$PORT", "--cors"]
